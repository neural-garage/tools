//! # Bury - Dead Code Detector
//!
//! Bury is a blazingly fast dead code detector that uses reachability analysis
//! to find unused functions, methods, and classes in Python and TypeScript codebases.
//!
//! ## Architecture
//!
//! This tool uses the `neural-shared` library for parsing and scanning.
//! The analyzer module contains bury-specific dead code detection logic.

pub mod analyzer;
pub mod cli;

// Re-export shared types
pub use neural_shared::{Language, ParsedFile, Parser, Scanner, Symbol, SymbolKind};

// Bury-specific exports
pub use analyzer::{Analyzer, Confidence, DeadCodeFinding};

/// Result type used throughout the library
pub type Result<T> = anyhow::Result<T>;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
