// src/main.rs

mod subcommands;

use clap::Command;

fn main() {
    let matches = Command::new("eve")
        .version("1.0")
        .author("Ted")
        .about("Eve is your personal command-line AI assistant.")
        .subcommands([subcommands::command::get_subcommand()])
        .get_matches();

    match matches.subcommand() {
        Some((subcommands::command::COMMAND, subcommand_matches)) => {
            subcommands::command::handle_command(subcommand_matches);
        }
        _ => println!("No valid subcommand provided."),
    }
}
