use std::io::{self, Write};

use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use rustubble::input::{handle_input, TextInput};
use rustubble::text_area::{handle_text_area, TextArea};

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

pub fn text_area(label: &str, visible_lines: usize) -> Result<(), io::Error> {
    io::stdout().flush().unwrap();

    enable_raw_mode().unwrap();

    let mut text_area = TextArea::new(label, Some("Press ESC to exit."), visible_lines);
    // text_area.render(0, 1); // Initial render at position (0, 1)

    let x = 5;
    let y = 5;

    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    let text_area_value = handle_text_area(&mut text_area, x, y);

    let text_2 = format!("Input value: {:?}", text_area_value);

    println!("{}", text_2);

    disable_raw_mode().unwrap();

    Ok(())
}
