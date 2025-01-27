use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

pub fn load_file_content(input_path: &PathBuf) -> Result<String, io::Error> {
    // Open the file and read the content into a String
    let mut file = fs::File::open(input_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_load_file_content() {
        let test_path = PathBuf::from("test_file.txt");
        let mut file = File::create(&test_path).expect("Failed to create test file");
        writeln!(file, "Test content").expect("Failed to write test content");

        let content = load_file_content(&test_path).expect("Failed to load file content");
        assert_eq!(content.trim(), "Test content");
    }
}
