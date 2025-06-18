use std::fs;
use std::{env::current_dir, path::PathBuf};

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
    #[arg(short, long)]
    pub verbose: bool,
}

pub fn get_current_directory() -> std::path::PathBuf {
    current_dir().expect("Failed to get current directory")
}

pub fn get_config_file_path() -> std::path::PathBuf {
    get_current_directory().join("config.toml")
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub title: String,
    pub description: String,
    pub content: Content,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Content {
    pub latex_enabled: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Build,
}

pub fn get_config() -> Result<Config, String> {
    let config_path = get_config_file_path();

    let config_str = fs::read_to_string(config_path)
        .map_err(|err| format!("Failed to read config file: {}", err))?;

    let config: Config = toml::from_str(&config_str)
        .map_err(|err| format!("Failed to parse config file: {}", err))?;

    Ok(config)
}

pub fn find_target_files(dir: PathBuf, extension: &str) -> Vec<PathBuf> {
    if dir.is_dir() {
        let mut files = Vec::new();
        for entry in fs::read_dir(dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == extension) {
                files.push(path);
            } else if path.is_dir() {
                files.extend(find_target_files(path, extension));
            }
        }
        files
    } else {
        Vec::new()
    }
}
