//! Markdown reporter (human-readable format)

use super::Reporter;
use crate::analyzer::DeadCodeFinding;
use crate::Result;

pub struct MarkdownReporter;

impl Reporter for MarkdownReporter {
    fn report(&self, findings: &[DeadCodeFinding]) -> Result<String> {
        let mut output = String::new();

        output.push_str("# Dead Code Report\n\n");
        output.push_str(&format!("## Summary\n\n"));
        output.push_str(&format!("- Dead code items: {}\n\n", findings.len()));

        if findings.is_empty() {
            output.push_str("✅ No dead code found!\n");
            return Ok(output);
        }

        output.push_str("## Dead Code Details\n\n");

        for finding in findings {
            output.push_str(&format!(
                "### {}:{}\n",
                finding.symbol.location.file, finding.symbol.location.line
            ));
            output.push_str(&format!("**Type:** {:?}\n", finding.symbol.kind));
            output.push_str(&format!("**Name:** `{}`\n", finding.symbol.name));
            output.push_str(&format!("**Reason:** {}\n", finding.reason));
            output.push_str(&format!("**Confidence:** {:?}\n\n", finding.confidence));
        }

        Ok(output)
    }
}
