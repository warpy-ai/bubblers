use bubblers::{
    cli_builder,
    config::{ArgConfig, CliConfig, CommandConfig},
};
use std::sync::Arc;

fn main() {
    // Define actions as functions for better readability and reuse
    fn echo_action(args: &[String]) {
        println!("Echo: {}", args.join(" "));
    }

    fn version_action(_: &[String]) {
        println!("Version 1.0.0");
    }

    // Create a new CLI configuration
    let mut cli = CliConfig::new("MyCLI", "1.0", "A simple CLI");

    // Define the echo command with its action and argument
    let mut echo_command = CommandConfig::new_standard(
        "echo",
        "Echo the input back to the console",
        Arc::new(echo_action),
    );
    echo_command.add_arg(ArgConfig {
        name: "message",
        help: "Message to echo back".to_string(),
        required: true,
    });

    // Add commands to CLI configuration
    cli.add_command(echo_command);
    cli.add_command(CommandConfig::new_standard(
        "version",
        "Display the version information",
        Arc::new(version_action),
    ));

    // Build and parse the CLI
    cli_builder::execute_cli(&cli);
}
