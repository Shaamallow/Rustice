use indicatif::ProgressBar;
use kalosm::language::*;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::article::{Article, EXAMPLE_INPUT, EXAMPLE_OUTPUT};

async fn process_file(
    llm: &Llama,
    file_path: &PathBuf,
) -> Result<Article, Box<dyn std::error::Error>> {
    // Load file content
    let content_file = std::fs::read_to_string(file_path)?;

    // Set up the task with a prompt and constraints
    let task = llm
        .task("You are a Jurist. Please extract the article number, date, and content of the legal text.")
        .with_example(EXAMPLE_INPUT, EXAMPLE_OUTPUT)
        .with_constraints(Arc::new(Article::new_parser()));

    // Run the task
    let article = task
        .run(format!(
            "Extract the relevant informations (article number, date, content) from the following content: {}",
            content_file
        ))
        .await?;

    Ok(article)
}

pub async fn process_files_in_batch(
    llm: &Llama,
    input_folder: &PathBuf,
    output_folder: &PathBuf,
    pb: Arc<ProgressBar>,
    n_threads: usize,
) -> Result<(), io::Error> {
    // Get a list of all files in the input folder

    // Check if the input is a folder
    if !input_folder.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Input must be a folder.",
        ));
    }

    let files: Vec<_> = fs::read_dir(input_folder)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map_or(false, |ext| ext == "json" || ext == "txt")
        })
        .collect();

    if files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No text files found.",
        ));
    }

    // Limit concurrent file processing (e.g., 4 files at a time)
    let semaphore = Arc::new(Semaphore::new(n_threads));

    // Process each file asynchronously
    let tasks: Vec<_> = files
        .iter()
        .map(|file| {
            let file_path = file.path();
            let output_path = output_folder.clone();
            let llm = llm.clone();
            let pb = pb.clone();
            let permit = semaphore.clone().acquire_owned();

            tokio::spawn(async move {
                let _permit = permit.await; // Wait for semaphore permit
                match process_file(&llm, &file_path).await {
                    Ok(article) => {
                        pb.println(format!("Processed file: {:?}", file_path));

                        // Save the Hashmap as a json using serde_json
                        let json = serde_json::to_string(&article).unwrap();

                        let mut output_file = output_path.join(file_path.file_name().unwrap());
                        output_file.set_extension("json");
                        fs::write(output_file, json).unwrap();
                    }
                    Err(e) => {
                        pb.println(format!("Error processing file {:?}: {}", file_path, e));
                    }
                }
            })
        })
        .collect();

    // Wait for all tasks to complete
    futures::future::join_all(tasks).await;

    Ok(())
}
