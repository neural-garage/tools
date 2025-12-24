//! Report generation in various formats

use crate::Result;
use serde::Serialize;

pub mod json;
pub mod markdown;

pub use json::JsonReporter;
pub use markdown::MarkdownReporter;

/// Trait for analysis findings that can be reported
pub trait Finding: Serialize {
    /// Get the kind/type of the finding (e.g., "Function", "Class")
    fn kind(&self) -> String;
    /// Get the name of the symbol
    fn name(&self) -> String;
    /// Get the file path
    fn file(&self) -> String;
    /// Get the line number
    fn line(&self) -> usize;
    /// Get the column number
    fn column(&self) -> usize;
    /// Get the reason/description
    fn reason(&self) -> String;
    /// Get the confidence level
    fn confidence(&self) -> String;
}

/// Reporter trait for outputting analysis results
pub trait Reporter<T: Finding> {
    fn report(&self, findings: &[T]) -> Result<String>;
}
