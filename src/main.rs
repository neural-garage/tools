use bury::cli::{Cli, Commands, OutputFormat};
use bury::report::{JsonReporter, MarkdownReporter};
use bury::{Analyzer, Reporter, Scanner};
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

    match cli.command {
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
            let analysis_path = path.as_ref().unwrap_or(&cli.path);
            
            if cli.verbose {
                println!("🔍 Scanning for files...");
            }

            let scanner = Scanner::new(analysis_path);
            let files = scanner.scan()?;

            if cli.verbose {
                println!("📁 Found {} files", files.len());
            }

            // TODO: Parse files and run analysis
            let analyzer = Analyzer::new();
            let findings = analyzer.analyze();

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
        None => {
            // Default behavior: analyze current directory
            let analysis_path = &cli.path;
            
            if cli.verbose {
                println!("🔍 Scanning for files...");
            }

            let scanner = Scanner::new(analysis_path);
            let files = scanner.scan()?;

            if cli.verbose {
                println!("📁 Found {} files", files.len());
            }

            // TODO: Parse files and run analysis
            let analyzer = Analyzer::new();
            let findings = analyzer.analyze();

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
    }
}
