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
        use std::fs;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let scanner = Scanner::new(dir.path());

        // Create test files
        let py_file = dir.path().join("test.py");
        let ts_file = dir.path().join("test.ts");
        let tsx_file = dir.path().join("test.tsx");
        let rs_file = dir.path().join("test.rs");
        let txt_file = dir.path().join("test.txt");

        fs::write(&py_file, "").unwrap();
        fs::write(&ts_file, "").unwrap();
        fs::write(&tsx_file, "").unwrap();
        fs::write(&rs_file, "").unwrap();
        fs::write(&txt_file, "").unwrap();

        assert!(scanner.is_supported_file(&py_file));
        assert!(scanner.is_supported_file(&ts_file));
        assert!(scanner.is_supported_file(&tsx_file));
        assert!(!scanner.is_supported_file(&rs_file));
        assert!(!scanner.is_supported_file(&txt_file));
    }
}
