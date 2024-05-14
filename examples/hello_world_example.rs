use bubblers::{cli_builder, config::ArgConfig, config::CliConfig, config::CommandConfig};
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
    let mut echo_command = CommandConfig::new(
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
    cli.add_command(CommandConfig::new(
        "version",
        "Display the version information",
        Arc::new(version_action),
    ));

    // Build and parse the CLI
    let matches = cli_builder::build_cli(&cli).get_matches();

    // Execute the matched command
    if let Some((command_name, sub_matches)) = matches.subcommand() {
        if let Some(cmd) = cli.commands().iter().find(|cmd| cmd.name() == command_name) {
            let args = sub_matches
                .get_many::<String>("message")
                .unwrap_or_default()
                .cloned()
                .collect::<Vec<_>>();

            cmd.execute_action(&args);
        }
    }
}
