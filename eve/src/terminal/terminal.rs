use std::{io, process::Command};
use termion::{color, style};

use crate::error::error::EveError;

pub(crate) fn handle_yes_no_input() -> bool {
    loop {
        println!("Please enter yes or no (Y/n): ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => {
                return true;
            }
            "no" | "n" => {
                return false;
            }
            _ => {
                println!("Invalid input. Please try again.");
            }
        }
    }
}

pub(crate) fn print_boxed(text: &str) {
    let width = text.len() + 4;

    println!(
        "{blue}{bold}{top}{reset}",
        blue = color::Fg(color::Blue),
        bold = style::Bold,
        top = "━".repeat(width),
        reset = style::Reset
    );

    println!(
        "{blue}║ {reset}{text}{blue} ║{reset}",
        blue = color::Fg(color::Blue),
        reset = style::Reset,
        text = text
    );

    println!(
        "{blue}{bold}{bottom}{reset}",
        blue = color::Fg(color::Blue),
        bold = style::Bold,
        bottom = "━".repeat(width),
        reset = style::Reset
    );
}

pub(crate) fn run_command(command: &str) -> Result<(), EveError> {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .map_err(|_e| EveError::new("Failed to run the command."))?
        .wait()
        .map_err(|_e| EveError::new("Failed to run the command."))?;
    Ok(())
}
