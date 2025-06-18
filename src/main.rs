mod cli;
mod parser;
mod translate;

use std::{fs::read_to_string, time::Instant};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;

use crate::cli::{Cli, find_target_files, get_current_directory};

fn print_json_value(key: &str, value: &serde_json::Value, indent: usize) {
    let indent_str = "  ".repeat(indent);
    match value {
        serde_json::Value::Object(map) => {
            println!("{}{}:", indent_str, key.bold());
            for (k, v) in map {
                print_json_value(k, v, indent + 1);
            }
        }
        _ => {
            println!(
                "{}{}: {}",
                indent_str,
                key.bold(),
                format!("{}", value).bright_yellow()
            );
        }
    }
}

fn main() {
    let total_start = Instant::now();
    let cli = Cli::parse();

    let config = match cli::get_config() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    if cli.verbose {
        println!("{}", "Config".green().bold());
        let value = serde_json::to_value(&config).unwrap();
        if let serde_json::Value::Object(map) = value {
            for (key, val) in map {
                print_json_value(&key, &val, 1);
            }
        }
    }

    match cli.cmd {
        cli::Command::Build => {
            println!("{} `build`", "Running".green().bold());

            let content_dir = get_current_directory().join("content");
            let targets = find_target_files(content_dir, "md");

            println!(" {} `{}` targets", "Found".green().bold(), targets.len());

            let bar = ProgressBar::new(targets.len() as u64);
            bar.set_style(
                ProgressStyle::with_template(" [{bar:57}] {pos}/{len}")
                    .unwrap()
                    .progress_chars("=> "),
            );

            for target in targets {
                let content = read_to_string(target)
                    .map_err(|e| format!("Failed to read file contents: {}", e))
                    .unwrap();

                let parser = parser::MarkdownParser::new(content);
                let nodes = parser.parse();

                for node in &nodes {
                    println!("{:?}", node);
                }

                let translator = translate::Translator::new(nodes);
                let translated = translator.translate();

                println!("{}", translated);

                bar.inc(1);
            }
            bar.finish();
        }
    }

    let total_duration = total_start.elapsed();

    if cli.verbose {
        println!(
            "{}: `{}`ms",
            "Duration".green().bold(),
            total_duration.as_millis()
        );
    }
}
