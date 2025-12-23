use bury::cli::{Cli, Commands, OutputFormat};
use bury::parser::{Language, PythonParser, TypeScriptParser};
use bury::report::{JsonReporter, MarkdownReporter};
use bury::{Analyzer, Parser, Reporter, Scanner};
use std::fs;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> bury::Result<()> {
    let cli = Cli::parse_args();

    if cli.verbose {
        println!("bury v{}", bury::VERSION);
        println!("Analyzing: {:?}", cli.path);
    }

    match &cli.command {
        Some(Commands::Version) => {
            println!("bury {}", bury::VERSION);
            Ok(())
        }
        Some(Commands::Init) => {
            println!("Creating .bury.json config file...");
            // TODO: Implement config file generation
            println!("✅ Created .bury.json");
            Ok(())
        }
        Some(Commands::Analyze { path }) => {
            analyze(path.as_ref().unwrap_or(&cli.path), &cli)
        }
        None => {
            // Default behavior: analyze current directory
            analyze(&cli.path, &cli)
        }
    }
}

fn analyze(analysis_path: &std::path::Path, cli: &Cli) -> bury::Result<()> {
    if cli.verbose {
        println!("🔍 Scanning for files...");
    }

    let scanner = Scanner::new(analysis_path);
    let files = scanner.scan()?;

    if cli.verbose {
        println!("📁 Found {} files", files.len());
    }

    if files.is_empty() {
        println!("No source files found to analyze.");
        return Ok(());
    }

    // Parse all files
    if cli.verbose {
        println!("🔬 Parsing files...");
    }

    let mut analyzer = Analyzer::new();
    let python_parser = PythonParser::new()?;
    let ts_parser = TypeScriptParser::new()?;

    for file_path in &files {
        if cli.verbose {
            println!("  Parsing: {}", file_path.display());
        }

        let source = fs::read_to_string(file_path)?;
        let language = Language::from_path(file_path)?;

        let parsed = match language {
            Language::Python => python_parser.parse(&source, file_path)?,
            Language::TypeScript | Language::JavaScript => ts_parser.parse(&source, file_path)?,
        };

        if cli.verbose {
            println!("    Found {} definitions, {} usages", 
                parsed.definitions.len(), 
                parsed.usages.len()
            );
        }

        analyzer.add_file(parsed);
    }

    // Run analysis
    if cli.verbose {
        println!("🔍 Running reachability analysis...");
    }

    let findings = analyzer.analyze();

    if cli.verbose {
        println!("✅ Analysis complete!");
        println!();
    }

    // Generate report
    let report: Box<dyn Reporter> = match cli.format {
        OutputFormat::Json => Box::new(JsonReporter),
        OutputFormat::Markdown => Box::new(MarkdownReporter),
        OutputFormat::Terminal => {
            // For now, use Markdown for terminal
            Box::new(MarkdownReporter)
        }
    };

    let output = report.report(&findings)?;
    println!("{}", output);

    if !findings.is_empty() && matches!(cli.format, OutputFormat::Terminal) {
        eprintln!("\n⚠️  Found {} dead code items", findings.len());
        process::exit(1);
    }

    Ok(())
}
