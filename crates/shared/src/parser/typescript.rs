//! TypeScript/JavaScript parser using tree-sitter

use super::{Location, ParsedFile, Parser, Symbol, SymbolKind};
use crate::Result;
use std::path::Path;
use tree_sitter::{Node, Parser as TSParser, Tree};

pub struct TypeScriptParser;

impl TypeScriptParser {
    pub fn new() -> Result<Self> {
        Ok(Self)
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
            "function_declaration" | "function" => {
                // Extract function name
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = name_node
                        .utf8_text(source.as_bytes())
                        .unwrap_or("")
                        .to_string();
                    if !name.is_empty() {
                        let pos = name_node.start_position();

                        definitions.push(Symbol::new(
                            name,
                            SymbolKind::Function,
                            Location {
                                file: file_path.to_string(),
                                line: pos.row + 1,
                                column: pos.column,
                            },
                        ));
                    }
                }
            }
            "arrow_function" => {
                // Arrow functions assigned to variables
                // We'll handle this when we process variable declarations
            }
            "method_definition" => {
                // Extract method name (inside class)
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = name_node
                        .utf8_text(source.as_bytes())
                        .unwrap_or("")
                        .to_string();
                    if !name.is_empty() {
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
            }
            "class_declaration" | "class" => {
                // Extract class name
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = name_node
                        .utf8_text(source.as_bytes())
                        .unwrap_or("")
                        .to_string();
                    if !name.is_empty() {
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
                            self.traverse_for_definitions(
                                child,
                                source,
                                file_path,
                                definitions,
                                Some(name.clone()),
                            );
                        }
                        return; // Don't traverse children again below
                    }
                }
            }
            "variable_declarator" => {
                // Handle const foo = function() {} or const foo = () => {}
                if let Some(name_node) = node.child_by_field_name("name") {
                    if let Some(value_node) = node.child_by_field_name("value") {
                        let value_kind = value_node.kind();
                        if value_kind == "function" || value_kind == "arrow_function" {
                            let name = name_node
                                .utf8_text(source.as_bytes())
                                .unwrap_or("")
                                .to_string();
                            if !name.is_empty() {
                                let pos = name_node.start_position();

                                definitions.push(Symbol::new(
                                    name,
                                    SymbolKind::Function,
                                    Location {
                                        file: file_path.to_string(),
                                        line: pos.row + 1,
                                        column: pos.column,
                                    },
                                ));
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        // Traverse children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.traverse_for_definitions(
                child,
                source,
                file_path,
                definitions,
                current_class.clone(),
            );
        }
    }

    fn extract_usages(&self, tree: &Tree, source: &str, file_path: &str) -> Vec<Symbol> {
        let mut usages = Vec::new();
        let root = tree.root_node();

        self.traverse_for_usages(root, source, file_path, &mut usages);

        usages
    }

    fn traverse_for_usages(
        &self,
        node: Node,
        source: &str,
        file_path: &str,
        usages: &mut Vec<Symbol>,
    ) {
        let kind = node.kind();

        match kind {
            "call_expression" => {
                // Extract function name being called
                if let Some(func_node) = node.child_by_field_name("function") {
                    let name = self.extract_call_name(func_node, source);
                    if !name.is_empty() {
                        let pos = func_node.start_position();
                        usages.push(Symbol::new(
                            name,
                            SymbolKind::Function,
                            Location {
                                file: file_path.to_string(),
                                line: pos.row + 1,
                                column: pos.column,
                            },
                        ));
                    }
                }
            }
            "new_expression" => {
                // Track class instantiation
                if let Some(class_node) = node.child_by_field_name("constructor") {
                    let name = class_node
                        .utf8_text(source.as_bytes())
                        .unwrap_or("")
                        .to_string();
                    if !name.is_empty() {
                        let pos = class_node.start_position();
                        usages.push(Symbol::new(
                            name,
                            SymbolKind::Class,
                            Location {
                                file: file_path.to_string(),
                                line: pos.row + 1,
                                column: pos.column,
                            },
                        ));
                    }
                }
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
            "member_expression" => {
                // For obj.method() calls, extract the method name
                if let Some(prop_node) = node.child_by_field_name("property") {
                    prop_node
                        .utf8_text(source.as_bytes())
                        .unwrap_or("")
                        .to_string()
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }

    fn extract_entry_points(&self, tree: &Tree, source: &str) -> Vec<String> {
        let mut entry_points = Vec::new();
        let root = tree.root_node();

        self.traverse_for_entry_points(root, source, &mut entry_points);

        entry_points
    }

    fn traverse_for_entry_points(&self, node: Node, source: &str, entry_points: &mut Vec<String>) {
        let kind = node.kind();

        // Detect top-level call expressions (like main())
        if kind == "expression_statement" {
            if let Some(expr) = node.child(0) {
                if expr.kind() == "call_expression" {
                    if let Some(func_node) = expr.child_by_field_name("function") {
                        let name = self.extract_call_name(func_node, source);
                        if !name.is_empty() {
                            entry_points.push(name);
                        }
                    }
                }
            }
        }

        // Detect exported functions as entry points
        if kind == "export_statement" {
            // Find function/class being exported
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "function_declaration" || child.kind() == "class_declaration" {
                    if let Some(name_node) = child.child_by_field_name("name") {
                        let name = name_node.utf8_text(source.as_bytes()).unwrap_or("");
                        if !name.is_empty() {
                            entry_points.push(name.to_string());
                        }
                    }
                }
            }
        }

        // Detect test functions (describe, it, test)
        if kind == "call_expression" {
            if let Some(func_node) = node.child_by_field_name("function") {
                let func_name = func_node.utf8_text(source.as_bytes()).unwrap_or("");
                if func_name == "describe" || func_name == "it" || func_name == "test" {
                    // Mark this as an entry point - extract the callback function
                    if let Some(args) = node.child_by_field_name("arguments") {
                        // The test callback is usually the second argument
                        let mut cursor = args.walk();
                        for child in args.children(&mut cursor) {
                            if child.kind() == "arrow_function" || child.kind() == "function" {
                                // This is a test entry point - for now, we'll just mark the test function itself
                                entry_points
                                    .push(format!("__test_callback_{}", entry_points.len()));
                            }
                        }
                    }
                }
            }
        }

        // Traverse children (but not into function bodies to avoid recursion)
        if kind != "statement_block" {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                self.traverse_for_entry_points(child, source, entry_points);
            }
        }
    }
}

impl Parser for TypeScriptParser {
    fn parse(&self, source: &str, file_path: &Path) -> Result<ParsedFile> {
        // Parser needs to be mutable, so we need to use interior mutability
        // For now, we'll create a new parser each time (not ideal but works for MVP)
        let mut parser = TSParser::new();
        parser.set_language(tree_sitter_typescript::language_typescript())?;

        let tree = parser
            .parse(source, None)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse TypeScript file"))?;

        let file_path_str = file_path.to_string_lossy().to_string();

        let definitions = self.extract_definitions(&tree, source, &file_path_str);
        let usages = self.extract_usages(&tree, source, &file_path_str);
        let entry_points = self.extract_entry_points(&tree, source);

        Ok(ParsedFile {
            path: file_path_str,
            definitions,
            usages,
            entry_points,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let parser = TypeScriptParser::new().unwrap();
        let source = r#"
function hello() {
    console.log("Hello, world!");
}
"#;
        let result = parser.parse(source, Path::new("test.ts"));
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.definitions.len(), 1);
        assert_eq!(parsed.definitions[0].name, "hello");
    }

    #[test]
    fn test_parse_arrow_function() {
        let parser = TypeScriptParser::new().unwrap();
        let source = r#"
const greet = () => {
    console.log("Hello!");
};
"#;
        let result = parser.parse(source, Path::new("test.ts"));
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.definitions.len(), 1);
        assert_eq!(parsed.definitions[0].name, "greet");
    }

    #[test]
    fn test_parse_class_with_methods() {
        let parser = TypeScriptParser::new().unwrap();
        let source = r#"
class Calculator {
    add(a: number, b: number) {
        return a + b;
    }
    
    subtract(a: number, b: number) {
        return a - b;
    }
}
"#;
        let result = parser.parse(source, Path::new("test.ts"));
        assert!(result.is_ok());

        let parsed = result.unwrap();
        // Should have 1 class + 2 methods = 3 definitions
        assert_eq!(parsed.definitions.len(), 3);
    }

    #[test]
    fn test_parse_function_calls() {
        let parser = TypeScriptParser::new().unwrap();
        let source = r#"
function foo() {
    return 42;
}

function bar() {
    foo();
    console.log("test");
}
"#;
        let result = parser.parse(source, Path::new("test.ts"));
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.definitions.len(), 2); // foo, bar
        assert!(!parsed.usages.is_empty()); // At least foo() call
    }
}
