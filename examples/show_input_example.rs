use bubblers::{
    cli_builder,
    config::{ArgConfig, CliConfig, CommandConfig},
    wrappers::show_input_form,
};
use std::sync::Arc;

fn main() {
    // Define actions
    fn echo_action(args: &[String]) {
        println!("Echo: {}", args.join(" "));
    }

    fn version_action(_: &[String]) {
        println!("Version 1.0.0");
    }

    fn install_instruction(_: &[String]) {
        println!("To install this CLI, please follow these steps...");
    }

    // Create a new CLI configuration
    let mut cli = CliConfig::new("MyCLI", "1.0", "A simple CLI");

    // Define the echo command
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

    let app_version = CommandConfig::new_standard(
        "version",
        "Display the application version",
        Arc::new(version_action),
    );

    cli.add_command(app_version);
    cli.add_command(CommandConfig::new_standard(
        "install",
        "Display installation instructions",
        Arc::new(install_instruction),
    ));

    let input_form = || show_input_form("Enter your name: ", "John Doe", "Name");

    cli.add_command(CommandConfig::new_ui_with_return(
        "input_form",
        "Display an input form",
        Arc::new(input_form),
    ));

    cli_builder::execute_cli(&cli);
}
