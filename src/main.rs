mod loader;
use loader::load_file_content;

mod article;
use article::{Article, EXAMPLE_INPUT, EXAMPLE_OUTPUT};

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
        #[arg(short, long, value_name = "MODEL")]
        model: String,
    },
    Mesure,
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
            model,
        } => {
            pb.set_message("Loading file content...");

            // Call a loader and processing logic here
            let content_file = match load_file_content(input_folder) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error loading file content: {}", e);
                    return;
                }
            };

            pb.set_message("Building LLM model...");
            // Then set up a task with a prompt and constraints
            let llm = Llama::builder()
                .with_source(LlamaSource::phi_3_mini_4k_instruct())
                .build()
                .await
                .unwrap();

            pb.set_message("Creating task...");
            let task = llm
                .task("You are a Jurist. Please extract the article number, date, and content of the legal text.")
                .with_example(EXAMPLE_INPUT, EXAMPLE_OUTPUT)
                .with_constraints(Arc::new(Article::new_parser()));
            // .typed();

            pb.set_message("LLM is thinking...");

            // Run the task
            let llm_thread = task.run(format!("Extract the relevant informations (article number, date, content) from the following content: {}", content_file));

            // let _ = llm_thread.to_std_out().await;

            match llm_thread.await {
                Ok(article) => {
                    pb.finish_with_message("Response: ");
                    println!("\n{}", article.to_string());
                }
                Err(e) => {
                    eprintln!("Error processing content: {}", e);
                }
            }
        }
        Commands::Mesure => {
            println!("Running metrics measurement...");
            // Call metric calculation logic here
        }
    }
}
