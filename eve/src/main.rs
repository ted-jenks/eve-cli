// src/main.rs

mod client;
mod config;
mod error;
mod subcommands;
mod terminal;

#[allow(deprecated)]
use std::env::home_dir;
use std::path::PathBuf;

use clap::ArgMatches;
use clap::Command;
use client::client::Client;
use config::config::Config;
use error::error::EveError;

/*
TODO:
 - eve hello for a generated
 - eve custom for a custom template
 - eve error for an error explainer
 */

fn main() {
    #[allow(deprecated)]
    let mut config_file: PathBuf = home_dir().expect("Cannot identify home directory");
    config_file.push(".evecfg");

    let matches = Command::new("eve")
        .version("1.0")
        .author("Ted")
        .about("Eve is your personal command-line AI assistant.")
        .subcommands([
            subcommands::hello::get_subcommand(),
            subcommands::config::get_subcommand(),
            subcommands::command::get_subcommand(),
        ])
        .get_matches();

    handle_configuration_mode(&matches, &config_file);

    match get_config(config_file)
        .map(|config| get_client(config))
        .and_then(|client| handle_subcommand(matches, client))
    {
        Err(e) => println!("{}", e),
        _ => return,
    }
}

fn handle_configuration_mode(matches: &ArgMatches, config_file: &PathBuf) {
    match matches.subcommand() {
        Some((subcommands::config::COMMAND, _subcommand_matches)) => {
            match subcommands::config::handle_command(config_file) {
                Err(e) => println!("{}", e),
                _ => return,
            }
        }
        _ => (),
    }
}

fn get_config(config_file: PathBuf) -> Result<Config, error::error::EveError> {
    Config::from_file(config_file)
}

fn get_client(config: Config) -> Client {
    Client::new(config.api_key(), config.model())
}

fn handle_subcommand(matches: ArgMatches, client: Client) -> Result<(), EveError> {
    match matches.subcommand() {
        Some((subcommands::hello::COMMAND, _)) => subcommands::hello::handle_command(client),
        Some((subcommands::command::COMMAND, subcommand_matches)) => {
            subcommands::command::handle_command(subcommand_matches, client)
        }
        _ => Err(EveError::new("No valid subcommand provided.")),
    }
}
