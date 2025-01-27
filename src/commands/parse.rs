use indicatif::ProgressBar;
use scraper::{Html, Selector, ElementRef};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Semaphore;
use regex::Regex;

fn clean_text(text: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();

    let parsed_text = text.trim().replace("\n", " ").replace("\t", " ");
    let parsed_text = re.replace_all(&parsed_text, " ").to_string();

    let fixed_text = parsed_text.replace("'", "'").replace(" ,", ",");
    fixed_text
}

pub async fn parse_html_file(
    file_path: &PathBuf,
) -> Result<HashMap<String, String>, std::io::Error> {
    let html_content = std::fs::read_to_string(file_path)?;
    let document = Html::parse_document(&html_content);

    let link_selector = Selector::parse("a").unwrap();
    let mut clean_articles = HashMap::new();

    for element in document.select(&link_selector) {
        // log the HTML of the element
        let article_name = clean_text(&element.text().collect::<Vec<_>>().join(" "));

        if let Some(parent) = element.parent() {

            let parent_element = ElementRef::wrap(parent).unwrap();
            let full_text = clean_text(&parent_element.text().collect::<Vec<_>>().join(" "));

            if !full_text.starts_with("vu") {
                clean_articles.insert(article_name.clone(), full_text);
            }
        }
    }

    Ok(clean_articles)
}

pub async fn parse_html_file_in_batch(
    input_folder: &PathBuf,
    pb: Arc<ProgressBar>,
) -> Result<(), io::Error> {
    // Get a list of all files in the input folder
    // Check if the input is a folder
    if !input_folder.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Input must be a folder.",
        ));
    }

    let files: Vec<_> = fs::read_dir(input_folder)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "html"))
        .collect();

    if files.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No HTML files found.",
        ));
    }

    // Limit concurrent file processing (e.g., 4 files at a time)
    let semaphore = Arc::new(Semaphore::new(4));

    // Process each file asynchronously
    let tasks: Vec<_> = files
        .iter()
        .map(|file| {
            let file_path = file.path();
            let pb = pb.clone();
            let permit = semaphore.clone().acquire_owned();

            tokio::spawn(async move {
                let _permit = permit.await; // Wait for semaphore permit
                match parse_html_file(&file_path).await {
                    Ok(article) => {
                        pb.println(format!("Processed file: {:?}", file_path));
                        // Save the Hashmap as a json using serde_json
                        let json = serde_json::to_string(&article).unwrap();

                        let output_file = file_path.with_extension("json");
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
