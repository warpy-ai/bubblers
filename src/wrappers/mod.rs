use std::io::{self, Write};

use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use rustubble::input::{handle_input, TextInput};
use rustubble::list::{handle_list, Item, ItemList};

pub fn show_input_form(
    placeholder: &str,
    initial_text: &str,
    label: &str,
) -> Result<String, io::Error> {
    // Flush stdout to ensure all previous logs are written to the terminal
    io::stdout().flush().unwrap();

    enable_raw_mode().unwrap();

    let mut text_input = TextInput::new(
        Some(placeholder),      // Placeholder
        2,                      // Padding
        initial_text,           // Initial text
        label,                  // Label
        Some("Ctrl+C to exit"), // Helper text
        ">",                    // Prefix
    );

    let x = 5;
    let y = 5;

    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    let text_input = handle_input(&mut text_input, x, y);
    let text_2 = format!("Input value: {:?}", text_input);

    disable_raw_mode().unwrap();
    Ok(text_2)
}

fn show_ui_terminal_app() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut list = ItemList::new(
        "Groceries".to_string(),
        vec![
            Item {
                title: "Pocky".to_string(),
                subtitle: "Expensive".to_string(),
            },
            Item {
                title: "Ginger".to_string(),
                subtitle: "Exquisite".to_string(),
            },
            Item {
                title: "Coke".to_string(),
                subtitle: "Cheap".to_string(),
            },
            Item {
                title: "Sprite".to_string(),
                subtitle: "Cheap".to_string(),
            },
        ],
    );

    let (x, y) = (5, 5);
    let list = handle_list(&mut list, x, y);
    println!("Operation completed.{:?}", list);

    disable_raw_mode()
}
