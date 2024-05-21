use bubblers::{cli_builder, config::CliConfig};

fn main() {
    // Create a new CLI configuration
    let mut cli = CliConfig::new("Text Area Config", "1.0", "A simple CLI");

    cli.add_text_area(
        "text_area",
        "small text area",
        "Write why do you like this TUI",
        6,
    );

    cli_builder::execute_cli(&cli);
}
