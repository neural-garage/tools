use clap::{Parser, Subcommand};
use neural_conductor_agent::cli::CopilotCli;
use neural_conductor_agent::{Agent, VERSION};

#[derive(Parser)]
#[command(name = "neural-conductor-agent")]
#[command(about = "Remote agent for Neural Conductor orchestration platform")]
#[command(version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// GitHub Copilot integration commands
    Copilot(CopilotCli),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Copilot(copilot)) => {
            if let Err(e) = copilot.execute().await {
                eprintln!("❌ Error: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            // Default behavior when no command is provided
            print_agent_info();
        }
    }
}

fn print_agent_info() {
    println!("neural-conductor-agent v{}", VERSION);

    let agent = Agent::new();
    println!("Agent ID: {}", agent.info().id);
    println!("Hostname: {}", agent.info().hostname);
    println!("Platform: {}", agent.info().platform);
    println!();
    println!("🚧 Agent runtime - coming soon!");
    println!();
    println!("This agent will:");
    println!("  - Listen for commands from Conductor server");
    println!("  - Manage work sessions");
    println!("  - Execute commands in isolated environments");
    println!("  - Report results back to server");
    println!();
    println!("Commands:");
    println!("  copilot login    Authenticate with GitHub Copilot");
    println!("  copilot status   Show authentication status");
    println!("  copilot test     Test Copilot API connection");
    println!("  copilot logout   Clear stored credentials");
    println!();
    println!("Part of Neural Garage 🧠🔧");
    println!("https://github.com/neural-garage/tools");
}
