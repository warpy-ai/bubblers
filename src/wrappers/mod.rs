use std::fs;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

use crossterm::execute;
use crossterm::style::Color;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use rustubble::input::{handle_input, TextInput};
use rustubble::list::{handle_list, Item, ItemList};
use rustubble::menu_list::{handle_menu_list, Menu};
use rustubble::progress_bar::{handle_progress_bar, ProgressBar};
use rustubble::spinner::{handle_spinner, Spinner};
use rustubble::stopwatch::{handle_stopwatch, StopWatch};
use rustubble::table::{handle_table, Table};
use rustubble::text_area::{handle_text_area, TextArea};
use rustubble::timer::{handle_timer, Timer};
use rustubble::viewport::{handle_viewport, Viewport};

pub fn input_form(
    placeholder: &str,
    initial_text: &str,
    label: &str,
) -> Result<Option<String>, io::Error> {
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
    Ok(Some(text_2))
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

pub fn timed_progress(
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

pub fn timer(secs: u64, nanos: u32) -> Result<(), io::Error> {
    io::stdout().flush().unwrap();
    enable_raw_mode().unwrap();

    let duration = Duration::new(secs, nanos); // For example, 5 minutes
    let mut timer = Timer::new(duration);

    let (x, y) = (5, 5);
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    handle_timer(&mut timer, x, y);
    disable_raw_mode().unwrap();
    Ok(())
}

pub fn stopwatch() -> Result<(), io::Error> {
    io::stdout().flush().unwrap();
    enable_raw_mode().unwrap();
    let mut time = StopWatch::new();
    let (x, y) = (5, 5);
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    handle_stopwatch(&mut time, x, y);
    disable_raw_mode().unwrap();
    Ok(())
}

pub fn viewport(file_path: String) -> Result<(), io::Error> {
    io::stdout().flush().unwrap();
    enable_raw_mode().unwrap();
    let file_name = file_path.split('/').last().unwrap();
    let header = file_name.to_string();

    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let height = 20; // Assume the visible height of the content area is 5 lines
    let width = 100; // Assume the visible width of the content area is 10 characters

    let x = 5;
    let y = 5;

    let mut viewport = Viewport::new(header, content, height, width, 6);
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    handle_viewport(&mut viewport, x, y);
    disable_raw_mode().unwrap();
    Ok(())
}

pub fn item_list(list: Vec<Item>, list_title: String) -> Result<Option<String>, io::Error> {
    io::stdout().flush().unwrap();
    enable_raw_mode().unwrap();
    let mut list = ItemList::new(list_title, list);

    let (x, y) = (5, 5);
    let list = handle_list(&mut list, x, y);

    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    println!("List: {:?}", list);
    disable_raw_mode().unwrap();
    Ok(list)
}

pub fn menu_list(
    list: Vec<String>,
    list_title: String,
    list_sub_title: String,
) -> Result<Option<String>, io::Error> {
    io::stdout().flush().unwrap();
    enable_raw_mode().unwrap();
    let mut list = Menu::new(list_title, list_sub_title, list);

    let (x, y) = (5, 5);

    let selected_menu = handle_menu_list(&mut list, x, y);

    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    println!("List: {:?}", selected_menu);
    disable_raw_mode().unwrap();
    Ok(selected_menu)
}
