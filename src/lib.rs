//! # Bury - Dead Code Detector
//!
//! Bury is a blazingly fast dead code detector that uses reachability analysis
//! to find unused functions, methods, and classes in Python and TypeScript codebases.
//!
//! ## Architecture
//!
//! This is the core library (open source, MIT/Apache-2.0).
//! Premium features will be available as separate modules.
//!
//! ## Core Modules:
//! - `scanner`: File discovery and filtering
//! - `parser`: Language detection and AST parsing
//! - `analyzer`: Reachability analysis and dead code detection
//! - `report`: Output generation (JSON, Markdown)

pub mod analyzer;
pub mod cli;
pub mod parser;
pub mod report;
pub mod scanner;

pub use analyzer::{Analyzer, DeadCodeFinding};
pub use parser::{Language, Parser};
pub use report::{JsonReporter, Reporter};
pub use scanner::Scanner;

/// Result type used throughout the library
pub type Result<T> = anyhow::Result<T>;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
