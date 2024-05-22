# Bubblers

Crustacean CLI that expels bubbles

[![Rust](https://github.com/warpy-ai/bubblers/actions/workflows/rust.yml/badge.svg)](https://github.com/warpy-ai/bubblers/actions/workflows/rust.yml)

Bubblers is a simple command line interface (CLI) builder in Rust. It provides an easy way to create CLI applications with minimal configuration.

## Features

- **Simple API** for creating CLI applications
- **Support for various command types** (standard, UI, UI with return)
- **Support for different argument types**
- **Support for diverse UI elements** (input form, text area, loader, table, progress bar, timer, stopwatch, viewport, item list, menu list)
- **Execution support** for commands and UI elements

## Installation

To include Bubblers in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
bubblers = "0.1.0"
```

## Usage

### Creating a Basic CLI

Here is an example of how to create a basic CLI application using Bubblers:

```rust
use std::{sync::Arc, io};
use bubblers::{CliConfig, CommandConfig, CommandType, ArgConfig};

fn main() {
    let mut cli = CliConfig::new("bubblers_app", "1.0", "A simple CLI app using Bubblers");

    let command = CommandConfig::new_standard(
        "greet",
        "Print a greeting message",
        Arc::new(|args| {
            if let Some(name) = args.get(0) {
                println!("Hello, {}!", name);
            } else {
                println!("Hello, world!");
            }
        }),
    ).add_arg(ArgConfig {
        name: "name",
        help: "Name to greet".to_string(),
        required: false,
    });

    cli.add_command(command);
    cli.execute();
}
```

### Adding UI Elements

Bubblers supports various UI elements. Here is an example of adding an input form:

```rust
cli.add_input(
    "input",
    "Get user input",
    "Enter text here...",
    "",
    "Your Input:"
);
```

### Implementing Custom Commands

You can implement custom commands and add them to your CLI. Here's an example:

```rust
cli.add_command(CommandConfig::new_standard(
    "custom_cmd",
    "Execute a custom command",
    Arc::new(|args| {
        println!("Executing custom command with args: {:?}", args);
    }),
));
```

## Full Example

Here's a full example of a CLI application using various features of Bubblers:

```rust
use std::{io, sync::Arc};
use bubblers::{CliConfig, CommandConfig, CommandType, ArgConfig};
use crossterm::style::Color;

fn main() {
    let mut cli = CliConfig::new("bubblers_app", "1.0", "A simple CLI app using Bubblers");

    // Standard Command
    cli.add_command(CommandConfig::new_standard(
        "greet",
        "Print a greeting message",
        Arc::new(|args| {
            if let Some(name) = args.get(0) {
                println!("Hello, {}!", name);
            } else {
                println!("Hello, world!");
            }
        }),
    ).add_arg(ArgConfig {
        name: "name",
        help: "Name to greet".to_string(),
        required: false,
    }));

    // UI Command
    cli.add_input(
        "input",
        "Get user input",
        "Enter text here...",
        "",
        "Your Input:"
    );

    // UI with Return Command
    cli.add_menu_list(
        "menu",
        "Select an option",
        "Main Menu",
        "Choose one of the following:",
        vec!["Option 1".to_string(), "Option 2".to_string()]
    );

    cli.execute();
}
```

## Contributing

Contributions are welcome! Please submit a pull request or create an issue to discuss your ideas.

## License

Bubblers is licensed under the MIT License. See [LICENSE](LICENSE) for more information.
