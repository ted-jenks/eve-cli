use atty::Stream;
use std::{
    io::{self, BufRead},
    process::Command,
};
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
    let lines: Vec<&str> = text.split('\n').collect();
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0) + 4;

    println!(
        "{blue}{bold}{top}{reset}",
        blue = color::Fg(color::Blue),
        bold = style::Bold,
        top = "━".repeat(width),
        reset = style::Reset
    );

    for line in lines {
        let padding = " ".repeat(width - 4 - line.len());
        println!(
            "{blue}║ {reset}{text}{padding} {blue}║{reset}",
            blue = color::Fg(color::Blue),
            reset = style::Reset,
            text = line,
            padding = padding
        );
    }

    println!(
        "{blue}{bold}{bottom}{reset}",
        blue = color::Fg(color::Blue),
        bold = style::Bold,
        bottom = "━".repeat(width),
        reset = style::Reset
    );
}

pub(crate) fn read_stdin() -> String {
    if atty::is(Stream::Stdin) {
        return String::new(); // Return an empty string if there is no input from a pipe
    }

    let mut stdin_str = String::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read from standard input");
        stdin_str.push_str(&line);
        stdin_str.push_str("\n");
    }
    stdin_str
}

pub(crate) fn format_msg(message: &str) -> String {
    const RED: &str = "\x1b[31m";
    const YELLOW: &str = "\x1b[33m";
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";

    let formatted_response = message
        .replace("errors ", &format!("{}{}{}", RED, "errors ", RESET))
        .replace("errors:", &format!("{}{}{}", RED, "errors:", RESET))
        .replace("error ", &format!("{}{}{}", RED, "error ", RESET))
        .replace("error:", &format!("{}{}{}", RED, "error:", RESET))
        .replace("warnings ", &format!("{}{}{}", YELLOW, "warnings ", RESET))
        .replace("warnings:", &format!("{}{}{}", YELLOW, "warnings:", RESET))
        .replace("warning ", &format!("{}{}{}", YELLOW, "warning ", RESET))
        .replace("warning:", &format!("{}{}{}", YELLOW, "warning:", RESET))
        .replace("passing ", &format!("{}{}{}", GREEN, "passing ", RESET))
        .replace("passing:", &format!("{}{}{}", GREEN, "passing:", RESET))
        .replace("pass ", &format!("{}{}{}", GREEN, "pass ", RESET))
        .replace("safely", &format!("{}{}{}", GREEN, "safely", RESET))
        .replace("safe ", &format!("{}{}{}", GREEN, "safe ", RESET));

    formatted_response
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
