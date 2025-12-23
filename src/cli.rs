//! CLI argument parsing and command handling

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "bury",
    version,
    about = "A blazingly fast dead code detector",
    long_about = "Bury finds unused code in your Python and TypeScript projects using reachability analysis.\n\nBury the dead code before it haunts your codebase!"
)]
pub struct Cli {
    /// Path to analyze (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Output format
    #[arg(short, long, value_name = "FORMAT", default_value = "terminal")]
    pub format: OutputFormat,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Analyze code for dead code
    Analyze {
        /// Path to analyze
        path: Option<PathBuf>,
    },
    
    /// Initialize a .bury.json config file
    Init,
    
    /// Show version information
    Version,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum OutputFormat {
    /// Pretty terminal output
    Terminal,
    /// JSON format (machine-readable, LLM-friendly)
    Json,
    /// Markdown report
    Markdown,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
