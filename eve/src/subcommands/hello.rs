use clap::Command;
use regex::Regex;
use std::str;

use crate::{client::client::Client, error::error::EveError};

const TEMPLATE_CONTENT: &str = include_str!("templates/hello.txt");

pub(crate) const COMMAND: &str = "hello";

pub(crate) fn get_subcommand() -> Command {
    Command::new(COMMAND).about("Say Hello!")
}

pub(crate) fn handle_command(client: Client) -> Result<(), EveError> {
    let prompt = TEMPLATE_CONTENT;
    let response = client.get_response(&prompt, 0.8).map_err(|e| {
        EveError::new(
            ("Unable to handle OpenAI request - ".to_string() + e.to_string().as_str()).as_str(),
        )
    })?;

    let re = Regex::new(r"(?i)eve").unwrap();
    let highlighted_text = re
        .replace_all(&response, "\x1b[35m$0\x1b[0m")
        .replace("\"", "");

    println!("{}", highlighted_text);
    println!("");
    Ok(())
}
