//! Dead code analysis using reachability

use neural_shared::{ParsedFile, Symbol};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// Dead code analyzer
pub struct Analyzer {
    /// All definitions found in the codebase
    definitions: HashMap<String, Symbol>,
    /// All usages found in the codebase (function -> [called functions])
    call_graph: HashMap<String, Vec<String>>,
    /// Entry points (functions called at the top level or from special contexts)
    entry_points: HashSet<String>,
    /// Additional manually specified entry points
    manual_entry_points: HashSet<String>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            call_graph: HashMap::new(),
            entry_points: HashSet::new(),
            manual_entry_points: HashSet::new(),
        }
    }

    /// Add entry points (functions that should always be considered used)
    pub fn add_entry_points(&mut self, entry_points: Vec<String>) {
        self.manual_entry_points.extend(entry_points);
    }

    /// Add parsed file to analysis
    pub fn add_file(&mut self, parsed: ParsedFile) {
        // Add definitions
        for def in &parsed.definitions {
            self.definitions.insert(def.name.clone(), def.clone());

            // Initialize call graph entry for this definition
            if !self.call_graph.contains_key(&def.name) {
                self.call_graph.insert(def.name.clone(), Vec::new());
            }
        }

        // Add entry points from the file
        for entry_point in &parsed.entry_points {
            self.entry_points.insert(entry_point.clone());
        }

        // For the call graph, we need to associate usages with the functions that call them
        // Since we don't track scope yet, we'll use a simple heuristic:
        // All usages in a file can potentially be called by all definitions in that file
        // This is conservative - better to mark something as alive when it might be dead
        // than to mark something as dead when it's actually alive

        // Build a list of all function calls in this file
        let mut all_calls: Vec<String> = parsed.usages.iter().map(|u| u.name.clone()).collect();
        all_calls.sort();
        all_calls.dedup();

        // Associate calls with definitions
        for def in &parsed.definitions {
            if let Some(calls) = self.call_graph.get_mut(&def.name) {
                calls.extend(all_calls.clone());
                calls.sort();
                calls.dedup();
            }
        }
    }

    /// Perform reachability analysis and return dead code
    pub fn analyze(&self) -> Vec<DeadCodeFinding> {
        // Step 1: Find all reachable symbols using BFS from entry points
        let reachable = self.find_reachable_symbols();

        // Step 2: Identify dead code - anything defined but not reachable
        let mut dead_code = Vec::new();

        for (name, symbol) in &self.definitions {
            // Skip if manually marked as entry point
            if self.manual_entry_points.contains(name) {
                continue;
            }

            // Skip if reachable
            if reachable.contains(name) {
                continue;
            }

            // This symbol is dead code
            dead_code.push(DeadCodeFinding {
                symbol: symbol.clone(),
                reason: "Not reachable from any entry point".to_string(),
                confidence: Confidence::High,
            });
        }

        dead_code
    }

    /// Find all symbols reachable from entry points using BFS
    fn find_reachable_symbols(&self) -> HashSet<String> {
        let mut reachable = HashSet::new();
        let mut queue = VecDeque::new();

        // Start with all entry points
        for entry_point in &self.entry_points {
            queue.push_back(entry_point.clone());
            reachable.insert(entry_point.clone());
        }

        for entry_point in &self.manual_entry_points {
            queue.push_back(entry_point.clone());
            reachable.insert(entry_point.clone());
        }

        // BFS traversal
        while let Some(current) = queue.pop_front() {
            // Find all functions called by the current function
            if let Some(called_functions) = self.call_graph.get(&current) {
                for called in called_functions {
                    // If we haven't seen this function yet, mark it as reachable
                    if reachable.insert(called.clone()) {
                        queue.push_back(called.clone());
                    }
                }
            }
        }

        reachable
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

impl neural_shared::report::Finding for DeadCodeFinding {
    fn kind(&self) -> String {
        format!("{:?}", self.symbol.kind)
    }

    fn name(&self) -> String {
        self.symbol.name.clone()
    }

    fn file(&self) -> String {
        self.symbol.location.file.clone()
    }

    fn line(&self) -> usize {
        self.symbol.location.line
    }

    fn column(&self) -> usize {
        self.symbol.location.column
    }

    fn reason(&self) -> String {
        self.reason.clone()
    }

    fn confidence(&self) -> String {
        format!("{:?}", self.confidence)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Confidence {
    High,
    Medium,
    Low,
}
