//! JSON reporter (LLM-friendly format)

use super::{Finding, Reporter};
use crate::Result;
use serde::{Deserialize, Serialize};

pub struct JsonReporter;

impl<T: Finding> Reporter<T> for JsonReporter {
    fn report(&self, findings: &[T]) -> Result<String> {
        let items: Vec<GenericItem> = findings
            .iter()
            .map(|f| GenericItem {
                kind: f.kind(),
                name: f.name(),
                file: f.file(),
                line: f.line(),
                column: f.column(),
                reason: f.reason(),
                confidence: f.confidence(),
            })
            .collect();

        let report = GenericReport {
            summary: Summary {
                total_findings: findings.len(),
            },
            findings: items,
        };

        Ok(serde_json::to_string_pretty(&report)?)
    }
}

/// Generic analysis report structure (LLM-friendly)
#[derive(Debug, Serialize, Deserialize)]
struct GenericReport {
    summary: Summary,
    findings: Vec<GenericItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Summary {
    total_findings: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenericItem {
    kind: String,
    name: String,
    file: String,
    line: usize,
    column: usize,
    reason: String,
    confidence: String,
}
