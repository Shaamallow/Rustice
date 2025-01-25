mod loader;
use loader::load_file_content;

mod article;
use article::{Article, EXAMPLE_INPUT, EXAMPLE_OUTPUT};

use clap::{Parser, Subcommand};
use kalosm::language::*;
use std::path::PathBuf;
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
    Mesure
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Process {
            input_folder,
            model,
        } => {
            println!(
                "Processing with model {} on folder {:?}",
                model, input_folder
            );

            // Call a loader and processing logic here
            let content_file = match load_file_content(input_folder) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error loading file content: {}", e);
                    return;
                }
            };

            println!("Content Loaded, now running model...");

            // Then set up a task with a prompt and constraints
            let llm = Llama::builder()
                .with_source(LlamaSource::phi_3_mini_4k_instruct())
                .build()
                .await
                .unwrap();

            let task = llm
                .task("You are a Jurist. Please extract the article number, date, and content of the legal text.")
                .with_example(EXAMPLE_INPUT, EXAMPLE_OUTPUT)
                .typed();

            // Run the task
            println!("Getting response");
            let mut stream =
                task.run(format!("Extract the relevant informations (article number, date, content) from the following content. If you find no date, give back 0000-00-00 {}", content_file));

            stream.to_std_out().await.unwrap();
            let article: Article = stream.await.unwrap();

            println!("{}", article.to_string());
        }
        Commands::Mesure => {
            println!("Running metrics measurement...");
            // Call metric calculation logic here
        }
    }
}
