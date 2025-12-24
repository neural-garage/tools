//! Shared utilities for Neural Garage analysis tools
//!
//! This library provides common functionality for analyzing code:
//! - Language detection and parsing (via tree-sitter)
//! - File scanning with .gitignore support
//! - Report generation (JSON, Markdown, Terminal)

pub mod parser;
pub mod report;
pub mod scanner;

pub use anyhow::{anyhow, Result};

/// Re-export common types
pub use parser::{Language, ParsedFile, Parser, Symbol, SymbolKind};
pub use report::{Finding, JsonReporter, MarkdownReporter, Reporter};
pub use scanner::Scanner;
