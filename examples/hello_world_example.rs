use std::sync::Arc;

use bubblers::cli_builder;
use bubblers::config::{ArgConfig, CliConfig, CommandConfig};

fn main() {
    let echo_action = Arc::new(|args: &[String]| {
        println!("Echo: {}", args.join(" "));
    }) as Arc<dyn Fn(&[String]) + Send + Sync + 'static>;

    let version_action = Arc::new(|_: &[String]| {
        println!("Version 1.0.0");
    }) as Arc<dyn Fn(&[String]) + Send + Sync + 'static>;

    let mut cli = CliConfig::new("MyCLI", "1.0", "A simple CLI");

    // Set up the echo command
    let mut echo_command =
        CommandConfig::new("echo", "Echo the input back to the console", echo_action);
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
        version_action,
    ));

    let matches = cli_builder::build_cli(&cli).get_matches();

    if let Some((command_name, sub_matches)) = matches.subcommand() {
        let cmd = cli
            .commands()
            .iter()
            .find(|cmd| cmd.name() == command_name)
            .unwrap();

        let args = sub_matches
            .get_many::<String>("message")
            .unwrap_or_default()
            .cloned()
            .collect::<Vec<_>>();

        cmd.execute_action(&args);
    }
}
