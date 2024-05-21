use bubblers::{
    cli_builder,
    config::{ArgConfig, CliConfig, CommandConfig},
    wrappers::show_input_form,
};
use std::sync::Arc;

fn main() {
    // Create a new CLI configuration
    let mut cli = CliConfig::new("MyCLI", "1.0", "A simple CLI");

    cli.add_input("input_form", "Name for user", "Your name", "", "Your Name");

    cli_builder::execute_cli(&cli);
}
