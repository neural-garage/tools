//! File system scanner with .gitignore support

use crate::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

/// Scanner finds source files to analyze
pub struct Scanner {
    root: PathBuf,
}

impl Scanner {
    /// Create a new scanner for the given root directory
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    /// Scan for Python and TypeScript files
    pub fn scan(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for result in WalkBuilder::new(&self.root)
            .hidden(false) // Include hidden files
            .git_ignore(true) // Respect .gitignore
            .build()
        {
            let entry = result?;
            let path = entry.path();

            if self.is_supported_file(path) {
                files.push(path.to_path_buf());
            }
        }

        Ok(files)
    }

    /// Check if file is a supported language
    fn is_supported_file(&self, path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }

        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| matches!(ext, "py" | "ts" | "tsx" | "js" | "jsx"))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_supported_file() {
        let scanner = Scanner::new(".");
        assert!(scanner.is_supported_file(Path::new("test.py")));
        assert!(scanner.is_supported_file(Path::new("test.ts")));
        assert!(scanner.is_supported_file(Path::new("test.tsx")));
        assert!(!scanner.is_supported_file(Path::new("test.rs")));
        assert!(!scanner.is_supported_file(Path::new("test.txt")));
    }
}
