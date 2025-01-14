mod loader;
use loader::load_file_content;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use tokio;
use kalosm::language::*;
use std::sync::Arc;

// First, derive an efficient parser for your structured data
#[derive(Parse, Clone, Debug)]
enum Class {
    Arret,
    Numero,
    Commentaire
}

#[derive(Parse, Clone, Debug)]
struct Response {
    classification: Class,
}

#[derive(Parser)]
#[command(name = "juridique")]
#[command(about = "A legal text processor using LLMs and regex", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Process {
        #[arg(short, long, value_name = "INPUT_FOLDER")]
        input_folder: PathBuf,
        #[arg(short, long, value_name = "MODEL")]
        model: String,
    },
    Mesure,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Process { input_folder, model } => {
            println!("Processing with model {} on folder {:?}", model, input_folder);

            // Call a loader and processing logic here
            let content_file = match load_file_content(input_folder) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error loading file content: {}", e);
                    return;
                }
            };
            // Then set up a task with a prompt and constraints
            let llm = Llama::new_chat().await.unwrap();
            let task = llm.task("Tu es un juriste qui explique quels sont les changements de chaque arret")
                .with_constraints(Arc::new(Response::new_parser()));

            // Finally, run the task
            let response = task("Donne moi chaque arret et numero correspondant avec une petite description : {content_file}").await.unwrap();
            println!("{:?}", response);
        }
        Commands::Mesure => {
            println!("Running metrics measurement...");
            // Call metric calculation logic here
        }
    }
}

