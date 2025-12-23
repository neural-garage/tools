//! Python parser using tree-sitter

use super::{Location, ParsedFile, Parser, Symbol, SymbolKind};
use crate::Result;
use std::path::Path;
use tree_sitter::{Node, Parser as TSParser, Tree};

pub struct PythonParser {
    parser: TSParser,
}

impl PythonParser {
    pub fn new() -> Result<Self> {
        let mut parser = TSParser::new();
        parser.set_language(tree_sitter_python::language())?;
        Ok(Self { parser })
    }

    fn extract_definitions(&self, tree: &Tree, source: &str, file_path: &str) -> Vec<Symbol> {
        let mut definitions = Vec::new();
        let root = tree.root_node();
        
        self.traverse_for_definitions(root, source, file_path, &mut definitions, None);
        
        definitions
    }

    fn traverse_for_definitions(
        &self,
        node: Node,
        source: &str,
        file_path: &str,
        definitions: &mut Vec<Symbol>,
        current_class: Option<String>,
    ) {
        let kind = node.kind();
        
        match kind {
            "function_definition" => {
                // Extract function name
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                    let pos = name_node.start_position();
                    
                    let symbol_kind = if let Some(ref class_name) = current_class {
                        SymbolKind::Method {
                            class_name: class_name.clone(),
                        }
                    } else {
                        SymbolKind::Function
                    };
                    
                    definitions.push(Symbol::new(
                        name,
                        symbol_kind,
                        Location {
                            file: file_path.to_string(),
                            line: pos.row + 1,
                            column: pos.column,
                        },
                    ));
                }
            }
            "class_definition" => {
                // Extract class name
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                    let pos = name_node.start_position();
                    
                    definitions.push(Symbol::new(
                        name.clone(),
                        SymbolKind::Class,
                        Location {
                            file: file_path.to_string(),
                            line: pos.row + 1,
                            column: pos.column,
                        },
                    ));
                    
                    // Traverse class body with class context
                    let mut cursor = node.walk();
                    for child in node.children(&mut cursor) {
                        self.traverse_for_definitions(child, source, file_path, definitions, Some(name.clone()));
                    }
                    return; // Don't traverse children again below
                }
            }
            _ => {}
        }
        
        // Traverse children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.traverse_for_definitions(child, source, file_path, definitions, current_class.clone());
        }
    }

    fn extract_usages(&self, tree: &Tree, source: &str, file_path: &str) -> Vec<Symbol> {
        let mut usages = Vec::new();
        let root = tree.root_node();
        
        self.traverse_for_usages(root, source, file_path, &mut usages);
        
        usages
    }

    fn traverse_for_usages(&self, node: Node, source: &str, file_path: &str, usages: &mut Vec<Symbol>) {
        let kind = node.kind();
        
        match kind {
            "call" => {
                // Extract function name being called
                if let Some(func_node) = node.child_by_field_name("function") {
                    let name = self.extract_call_name(func_node, source);
                    if !name.is_empty() {
                        let pos = func_node.start_position();
                        usages.push(Symbol::new(
                            name,
                            SymbolKind::Function, // We don't know if it's a function or method yet
                            Location {
                                file: file_path.to_string(),
                                line: pos.row + 1,
                                column: pos.column,
                            },
                        ));
                    }
                }
            }
            "identifier" => {
                // Track variable usages (for future enhancement)
                // For now, we focus on function calls
            }
            _ => {}
        }
        
        // Traverse children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.traverse_for_usages(child, source, file_path, usages);
        }
    }

    fn extract_call_name(&self, node: Node, source: &str) -> String {
        match node.kind() {
            "identifier" => node.utf8_text(source.as_bytes()).unwrap_or("").to_string(),
            "attribute" => {
                // For obj.method() calls, extract the method name
                if let Some(attr_node) = node.child_by_field_name("attribute") {
                    attr_node.utf8_text(source.as_bytes()).unwrap_or("").to_string()
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }
}

impl Parser for PythonParser {
    fn parse(&self, source: &str, file_path: &Path) -> Result<ParsedFile> {
        // Parser needs to be mutable, so we need to use interior mutability
        // For now, we'll create a new parser each time (not ideal but works for MVP)
        let mut parser = TSParser::new();
        parser.set_language(tree_sitter_python::language())?;
        
        let tree = parser.parse(source, None)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse Python file"))?;

        let file_path_str = file_path.to_string_lossy().to_string();

        let definitions = self.extract_definitions(&tree, source, &file_path_str);
        let usages = self.extract_usages(&tree, source, &file_path_str);

        Ok(ParsedFile {
            path: file_path_str,
            definitions,
            usages,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let parser = PythonParser::new().unwrap();
        let source = r#"
def hello():
    print("Hello, world!")
"#;
        let result = parser.parse(source, Path::new("test.py"));
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        assert_eq!(parsed.definitions.len(), 1);
        assert_eq!(parsed.definitions[0].name, "hello");
    }

    #[test]
    fn test_parse_class_with_methods() {
        let parser = PythonParser::new().unwrap();
        let source = r#"
class Calculator:
    def add(self, a, b):
        return a + b
    
    def subtract(self, a, b):
        return a - b
"#;
        let result = parser.parse(source, Path::new("test.py"));
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        // Should have 1 class + 2 methods = 3 definitions
        assert_eq!(parsed.definitions.len(), 3);
    }

    #[test]
    fn test_parse_function_calls() {
        let parser = PythonParser::new().unwrap();
        let source = r#"
def foo():
    pass

def bar():
    foo()
    print("test")
"#;
        let result = parser.parse(source, Path::new("test.py"));
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        assert_eq!(parsed.definitions.len(), 2); // foo, bar
        assert!(parsed.usages.len() >= 1); // At least foo() call
    }
}
