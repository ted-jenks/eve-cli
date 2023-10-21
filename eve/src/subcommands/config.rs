use clap::Command;
use std::{fs::File, io, io::Write, path::PathBuf, str};

use crate::{client::model::Model, error::error::EveError};

pub(crate) const COMMAND: &str = "config";

pub(crate) fn get_subcommand() -> Command {
    Command::new(COMMAND).about("Configure eve with you OpenAI API key and choice of model")
}

pub(crate) fn handle_command(config_file: &PathBuf) -> Result<(), EveError> {
    println!(
        "Please enter the model you wish to use. Valid entries = [{}]: ",
        Model::valid_models()
    );
    let mut model = String::new();
    io::stdin()
        .read_line(&mut model)
        .map_err(|e| EveError::new(format!("Failed to read model: {}", e).as_str()))?;

    println!("Please enter the API key: ");
    let mut api_key = String::new();
    io::stdin()
        .read_line(&mut api_key)
        .map_err(|e| EveError::new(format!("Failed to read API key: {}", e).as_str()))?;

    let model = model.trim();
    let api_key = api_key.trim();

    let mut file = File::create(config_file)
        .map_err(|e| EveError::new(format!("Failed to create file: {}", e).as_str()))?;
    write!(file, "MODEL={}\nAPI_KEY={}\n", model, api_key)
        .map_err(|e| EveError::new(format!("Failed to write to file: {}", e).as_str()))?;

    Ok(())
}
