//! Markdown reporter (human-readable format)

use super::{Finding, Reporter};
use crate::Result;

pub struct MarkdownReporter;

impl<T: Finding> Reporter<T> for MarkdownReporter {
    fn report(&self, findings: &[T]) -> Result<String> {
        let mut output = String::new();

        output.push_str("# Analysis Report\n\n");
        output.push_str("## Summary\n\n");
        output.push_str(&format!("- Total findings: {}\n\n", findings.len()));

        if findings.is_empty() {
            output.push_str("✅ No issues found!\n");
            return Ok(output);
        }

        output.push_str("## Details\n\n");

        for finding in findings {
            output.push_str(&format!("### {}:{}\n", finding.file(), finding.line()));
            output.push_str(&format!("**Type:** {}\n", finding.kind()));
            output.push_str(&format!("**Name:** `{}`\n", finding.name()));
            output.push_str(&format!("**Reason:** {}\n", finding.reason()));
            output.push_str(&format!("**Confidence:** {}\n\n", finding.confidence()));
        }

        Ok(output)
    }
}
