use neural_conductor_agent::{Agent, VERSION};

fn main() {
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
    println!("Part of Neural Garage 🧠🔧");
    println!("https://github.com/neural-garage/tools");
}
