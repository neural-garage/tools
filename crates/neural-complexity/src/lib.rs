//! # Neural Complexity - Code Complexity Analyzer
//!
//! Analyzes code complexity metrics including cyclomatic and cognitive complexity.
//!
//! Part of the Neural Garage toolkit.

pub mod analyzer;

// Re-export shared types
pub use neural_shared::{Language, ParsedFile, Parser, Scanner, Symbol, SymbolKind};

// Complexity-specific exports
pub use analyzer::{ComplexityAnalyzer, ComplexityMetrics};

/// Result type
pub type Result<T> = anyhow::Result<T>;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
