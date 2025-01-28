mod article;
mod commands;
mod models;

use commands::{parse_html_file_in_batch, process_files_in_batch};

use models::Model;

use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use kalosm::language::*;
use std::path::PathBuf;
use std::sync::Arc;
use tokio;

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
        #[arg(short, long, value_name = "OUTPUT_FOLDER")]
        output_folder: PathBuf,
        #[arg(short, long, value_name = "MODEL")]
        model: Model,
        #[arg(short, long, value_name = "N_THREADS", default_value = "1")]
        n_threads: usize,
    },
    ParseHTML {
        #[arg(short, long, value_name = "INPUT_FOLDER")]
        input_folder: PathBuf,
        #[arg(short, long, value_name = "OUTPUT_FOLDER")]
        output_folder: PathBuf,
        #[arg(short, long, value_name = "N_THREADS", default_value = "1")]
        n_threads: usize,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let pb = Arc::new(ProgressBar::new_spinner());
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    match &cli.command {
        Commands::Process {
            input_folder,
            output_folder,
            model,
            n_threads,
        } => {
            pb.set_message("Building LLM model...");
            // Then set up a task with a prompt and constraints

            let llm = Llama::builder()
                .with_source(model.get_llama_source())
                .build()
                .await
                .unwrap();

            pb.finish_with_message("Processing batch");

            match process_files_in_batch(
                &llm,
                &input_folder,
                &output_folder,
                pb.clone(),
                n_threads.clone(),
            )
            .await
            {
                Ok(_) => pb.finish_with_message("Batch processing completed successfully."),
                Err(e) => {
                    pb.finish_with_message("Error during batch processing.");
                    eprintln!("Error: {}", e);
                }
            }
        }
        Commands::ParseHTML {
            input_folder,
            output_folder,
            n_threads,
        } => {
            pb.set_message("Parsing HTML files...");

            match parse_html_file_in_batch(
                &input_folder,
                &output_folder,
                pb.clone(),
                n_threads.clone(),
            )
            .await
            {
                Ok(_) => pb.finish_with_message("Batch parsing completed successfully."),
                Err(e) => {
                    pb.finish_with_message("Error during batch parsing.");
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}
