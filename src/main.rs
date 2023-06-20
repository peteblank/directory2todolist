extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct FileInfo {
    path: String,
    red: bool,
    comment: Option<String>,
}

fn main() -> io::Result<()> {
    // Read the current directory
    let current_dir = env::current_dir()?;
    let files = walk_directory(&current_dir)?;

    // Save file information as JSON
    let json = serde_json::to_string(&files)?;
    fs::write("file_info.json", json)?;

    // Load the JSON file
    let json_content = fs::read_to_string("file_info.json")?;
    let mut loaded_files: Vec<FileInfo> = serde_json::from_str(&json_content)?;

    // Display the file information
    for (index, file) in loaded_files.iter().enumerate() {
        let marked = if file.red { "✓" } else { " " };
        let comment = if let Some(ref c) = file.comment {
            format!("({})", c)
        } else {
            "".to_string()
        };

        println!("{:<4}{} {} {}", index + 1, marked, file.path, comment);
    }

    // Interactive interface to mark rows as red and add comments
    loop {
        print!("Enter the row number to mark as red (or 'q' to quit): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input == "q" {
            break;
        }

        if let Ok(row) = input.parse::<usize>() {
            if let Some(file) = loaded_files.get_mut(row - 1) {
                file.red = true;

                print!("Enter a comment: ");
                io::stdout().flush()?;
                let mut comment = String::new();
                io::stdin().read_line(&mut comment)?;
                file.comment = Some(comment.trim().to_string());
            }
        }
    }

    // Save the updated file information as JSON
    let updated_json = serde_json::to_string(&loaded_files)?;
    fs::write("updated_file_info.json", updated_json)?;

    Ok(())
}

fn walk_directory(dir: &std::path::PathBuf) -> io::Result<Vec<FileInfo>> {
    let mut file_info = vec![];

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            let sub_files = walk_directory(&path)?;
            file_info.extend(sub_files);
        } else {
            let path_str = path.to_string_lossy().into_owned();
            let file = FileInfo {
                path: path_str,
                red: false,
                comment: None,
            };
            file_info.push(file);
        }
    }

    Ok(file_info)
}

