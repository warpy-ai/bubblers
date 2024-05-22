use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

use crossterm::execute;
use crossterm::style::Color;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use rustubble::input::{handle_input, TextInput};
use rustubble::progress_bar::{handle_progress_bar, ProgressBar};
use rustubble::spinner::{handle_spinner, Spinner};
use rustubble::table::{handle_table, Table};
use rustubble::text_area::{handle_text_area, TextArea};

pub fn input_form(placeholder: &str, initial_text: &str, label: &str) -> Result<String, io::Error> {
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

pub fn loader(message: String, style: String) -> Result<(), io::Error> {
    io::stdout().flush().unwrap();

    let spinner = Spinner::new(
        Color::Rgb {
            r: 0,
            g: 255,
            b: 255,
        },
        message,
        &style,
    );

    let (x, y) = (10, 10);
    handle_spinner(&spinner, x, y);

    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    println!("Operation completed.");

    Ok(())
}

pub fn table(headers: Vec<&'static str>, rows: Vec<Vec<&'static str>>) -> Result<(), io::Error> {
    io::stdout().flush().unwrap();

    enable_raw_mode().unwrap();

    let headers_str: Vec<String> = headers.iter().map(|&s| s.to_string()).collect();
    let rows_str: Vec<Vec<String>> = rows
        .iter()
        .map(|row| row.iter().map(|&s| s.to_string()).collect())
        .collect();

    let mut table = Table::new(headers_str, rows_str, 0, 3, 5);

    let (x, y) = (5, 5);
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    handle_table(&mut table, x, y);

    disable_raw_mode().unwrap();

    Ok(())
}

pub fn progress(
    prefix: &str,
    progress: f32,
    length: u16,
    start_color: Color,
    end_color: Color,
) -> Result<(), io::Error> {
    io::stdout().flush().unwrap();

    enable_raw_mode().unwrap();

    let mut progress_bar =
        ProgressBar::new(prefix.to_string(), progress, length, start_color, end_color);
    let (x, y) = (10, 10);
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();

    for i in 0..=100 {
        handle_progress_bar(&mut progress_bar, i as f32 / 100.0, x, y);

        // progress_bar.update(i);
        sleep(Duration::from_millis(10)); // Simulate time-consuming task
    }

    disable_raw_mode().unwrap();

    Ok(())
}
