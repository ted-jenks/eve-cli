use clap::Command;
use std::str;
use textwrap::fill;

use crate::terminal::terminal::{format_msg, print_boxed};
use crate::{client::client::Client, error::error::EveError};

const TEMPLATE_CONTENT: &str = include_str!("templates/error.txt");

pub(crate) const COMMAND: &str = "error";

pub(crate) fn get_subcommand() -> Command {
    Command::new(COMMAND).about("Get an explanation and suggested fix for an error message")
}

pub(crate) fn handle_command(stdin: String, client: Client) -> Result<(), EveError> {
    let prompt = TEMPLATE_CONTENT.replace("QUERY", &stdin);

    let response = client.get_response(&prompt, 0.0).map_err(|e| {
        EveError::new(
            ("Unable to handle OpenAI request - ".to_string() + e.to_string().as_str()).as_str(),
        )
    })?;

    println!("");
    print_boxed("ERROR MESSAGE");
    println!("\n{}", format_msg(&stdin));

    print_boxed("MY ANALYSIS");
    let highlighted_text = format_msg(&response);
    println!("\n{}\n", fill(&highlighted_text, 130));

    Ok(())
}
