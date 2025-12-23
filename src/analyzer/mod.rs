//! Dead code analysis using reachability

use crate::parser::{ParsedFile, Symbol};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Dead code analyzer
pub struct Analyzer {
    /// All definitions found in the codebase
    definitions: HashMap<String, Symbol>,
    /// All usages found in the codebase
    usages: HashSet<String>,
    /// Entry points (won't be marked as dead)
    entry_points: HashSet<String>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            usages: HashSet::new(),
            entry_points: HashSet::new(),
        }
    }

    /// Add entry points (functions that should always be considered used)
    pub fn add_entry_points(&mut self, entry_points: Vec<String>) {
        self.entry_points.extend(entry_points);
    }

    /// Add parsed file to analysis
    pub fn add_file(&mut self, parsed: ParsedFile) {
        // Add definitions
        for def in parsed.definitions {
            self.definitions.insert(def.name.clone(), def);
        }

        // Add usages
        for usage in parsed.usages {
            self.usages.insert(usage.name);
        }
    }

    /// Perform reachability analysis and return dead code
    pub fn analyze(&self) -> Vec<DeadCodeFinding> {
        let mut dead_code = Vec::new();

        for (name, symbol) in &self.definitions {
            // Skip entry points
            if self.entry_points.contains(name) {
                continue;
            }

            // If not used anywhere, it's dead code
            if !self.usages.contains(name) {
                dead_code.push(DeadCodeFinding {
                    symbol: symbol.clone(),
                    reason: "Not reachable from any entry point".to_string(),
                    confidence: Confidence::High,
                });
            }
        }

        dead_code
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Dead code finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadCodeFinding {
    pub symbol: Symbol,
    pub reason: String,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Confidence {
    High,
    Medium,
    Low,
}
