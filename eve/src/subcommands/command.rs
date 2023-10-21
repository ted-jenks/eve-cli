use clap::{arg, ArgMatches, Command};
use std::str;

pub const COMMAND: &str = "command";
const QUERY: &str = "query";
const YOLO: &str = "yolo";

pub fn get_subcommand() -> Command {
    Command::new(COMMAND)
        .about("Get a command to complete a task defined in natural language.")
        .arg(
            arg!(--yolo "Run the suggested command without confirmation")
                .short('y')
                .required(false),
        )
        .arg(arg!(<query> "The natural language description of the command").required(true))
}

pub fn handle_command(matches: &ArgMatches) {
    let query = matches.get_one::<String>(QUERY);
    let yolo = matches.get_one::<bool>(YOLO);
}
