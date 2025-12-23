//! Report generation in various formats

use crate::analyzer::DeadCodeFinding;
use crate::Result;
use serde::{Deserialize, Serialize};

pub mod json;
pub mod markdown;

pub use json::JsonReporter;
pub use markdown::MarkdownReporter;

/// Reporter trait for outputting analysis results
pub trait Reporter {
    fn report(&self, findings: &[DeadCodeFinding]) -> Result<String>;
}

/// Analysis report structure (LLM-friendly)
#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub summary: Summary,
    pub dead_code: Vec<DeadCodeItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    pub total_files_scanned: usize,
    pub total_definitions: usize,
    pub dead_code_count: usize,
    pub languages: LanguageStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageStats {
    pub python: usize,
    pub typescript: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeadCodeItem {
    pub kind: String,
    pub name: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub reason: String,
    pub confidence: String,
}
