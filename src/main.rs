use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

mod lexer;
use lexer::{tokenize, Token};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        source: String,
    },
    Run {
        source: String,
    },
}

fn check_extension(filename: &str) -> bool {
    let path = Path::new(filename);
    match path.extension() {
        Some(ext) => ext == "gene",
        None => false,
    }
}

fn read_source_file(path: &str) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(content) => Some(content),
        Err(_) => {
            eprintln!("Error: Failed to read file '{}'. Please check the path and try again.", path);
            None
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { source } => {
            if !check_extension(source) {
                eprintln!("Error: 'build' command only accepts files with the .gene extension.");
                std::process::exit(1);
            }

            if let Some(source_code) = read_source_file(source) {
                println!("Build command: Successfully read the file.");
                
                let tokens = tokenize(&source_code);
                for token in tokens {
                    println!("{:?}", token);
                }

            } else {
                std::process::exit(1);
            }
        }

        Commands::Run { source } => {
            if let Some(source_code) = read_source_file(source) {
                println!("Run command: Executing with the following content:\n{}", source_code);
                // TODO: Interpret and execute the source code
            } else {
                std::process::exit(1);
            }
        }
    }
}
