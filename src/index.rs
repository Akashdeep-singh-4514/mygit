use std::fs;
use std::io::{self, Write};

pub fn update_index(file: &str, hash: &str) -> io::Result<()> {
    let index_path = ".mygit/index";

    let mut entries = Vec::new();

    if let Ok(content) = fs::read_to_string(index_path) {
        for line in content.lines() {
            let mut parts = line.split_whitespace();
            let existing_hash = parts.next().unwrap();
            let existing_file = parts.next().unwrap();

            if existing_file != file {
                entries.push(format!("{} {}", existing_hash, existing_file));
            }
        }
    }

    entries.push(format!("{} {}", hash, file));

    let mut file_handle = fs::File::create(index_path)?;
    for entry in entries {
        writeln!(file_handle, "{}", entry)?;
    }

    Ok(())
}
