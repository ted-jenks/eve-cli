// src/main.rs

mod client;
mod config;
mod error;
mod subcommands;

use clap::Command;
use config::config::Config;

fn main() {
    match Config::from_file(".evecfg") {
        Ok(config) => match (config.model(), config.api_key()) {
            (Some(model), Some(api_key)) => {
                println!("Model: {}", model.to_string());
                println!("API Key: {}", api_key);
            }
            _ => println!("Invalid or incomplete configuration file."),
        },
        Err(e) => println!("{}", e),
    }

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
