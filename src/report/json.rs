//! JSON reporter (LLM-friendly format)

use super::{DeadCodeItem, Report, Reporter, Summary};
use crate::analyzer::DeadCodeFinding;
use crate::Result;

pub struct JsonReporter;

impl Reporter for JsonReporter {
    fn report(&self, findings: &[DeadCodeFinding]) -> Result<String> {
        let items: Vec<DeadCodeItem> = findings
            .iter()
            .map(|f| DeadCodeItem {
                kind: format!("{:?}", f.symbol.kind),
                name: f.symbol.name.clone(),
                file: f.symbol.location.file.clone(),
                line: f.symbol.location.line,
                column: f.symbol.location.column,
                reason: f.reason.clone(),
                confidence: format!("{:?}", f.confidence),
            })
            .collect();

        let report = Report {
            summary: Summary {
                total_files_scanned: 0, // TODO: track this
                total_definitions: 0,   // TODO: track this
                dead_code_count: findings.len(),
                languages: super::LanguageStats {
                    python: 0,     // TODO: track this
                    typescript: 0, // TODO: track this
                },
            },
            dead_code: items,
        };

        Ok(serde_json::to_string_pretty(&report)?)
    }
}
