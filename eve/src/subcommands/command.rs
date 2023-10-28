use clap::{arg, ArgMatches, Command};
use std::str;

use crate::{
    client::client::Client,
    error::error::EveError,
    terminal::terminal::{handle_yes_no_input, print_boxed, run_command},
};

const TEMPLATE_CONTENT: &str = include_str!("templates/command.txt");

pub(crate) const COMMAND: &str = "command";
const QUERY: &str = "query";
const YOLO: &str = "yolo";

pub(crate) fn get_subcommand() -> Command {
    Command::new(COMMAND)
        .about("Get a command to complete a task defined in natural language")
        .arg(
            arg!(--yolo "Run the suggested command without confirmation")
                .short('y')
                .required(false),
        )
        .arg(arg!(<query> "The natural language description of the command").required(true))
}

pub(crate) fn handle_command(matches: &ArgMatches, client: Client) -> Result<(), EveError> {
    let maybe_query = matches.get_one::<String>(QUERY);
    let yolo = matches.get_one::<bool>(YOLO);
    let is_yolo = match yolo {
        Some(value) => value,
        None => &false,
    };

    match maybe_query {
        Some(query) => {
            let prompt = TEMPLATE_CONTENT.replace("QUERY", query);
            let suggested_command = client.get_response(&prompt, 0.0).map_err(|e| {
                EveError::new(
                    ("Unable to handle OpenAI request - ".to_string() + e.to_string().as_str())
                        .as_str(),
                )
            })?;

            println!("\nI think something like this would work:\n");
            print_boxed(&suggested_command);
            if !*is_yolo {
                println!("\nWould you like me to run it?");
            }
            if *is_yolo || handle_yes_no_input() {
                println!("Running command...");
                run_command(&suggested_command)?;
            }
        }
        _ => println!("No command query provided."),
    }
    Ok(())
}
