//! Language detection and AST parsing using tree-sitter

use crate::Result;
use anyhow::anyhow;
use std::path::Path;

mod python;
mod typescript;

pub use python::PythonParser;
pub use typescript::TypeScriptParser;

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Python,
    TypeScript,
    JavaScript,
}

impl Language {
    /// Detect language from file extension
    pub fn from_path(path: &Path) -> Result<Self> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| anyhow!("No file extension found"))?;

        match ext {
            "py" => Ok(Language::Python),
            "ts" | "tsx" => Ok(Language::TypeScript),
            "js" | "jsx" => Ok(Language::JavaScript),
            _ => Err(anyhow!("Unsupported file extension: {}", ext)),
        }
    }
}

/// Parser trait for language-specific parsing
pub trait Parser {
    /// Parse source code and extract symbols
    fn parse(&self, source: &str, file_path: &Path) -> Result<ParsedFile>;
}

/// Parsed file containing symbols
#[derive(Debug, Clone)]
pub struct ParsedFile {
    pub path: String,
    pub definitions: Vec<Symbol>,
    pub usages: Vec<Symbol>,
    pub entry_points: Vec<String>,
}

/// Symbol represents a function, class, method, or variable
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SymbolKind {
    Function,
    Class,
    Method { class_name: String },
    Variable,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl Symbol {
    pub fn new(name: String, kind: SymbolKind, location: Location) -> Self {
        Self {
            name,
            kind,
            location,
        }
    }
}
