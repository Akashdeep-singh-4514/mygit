use std::fs::OpenOptions;
use std::io;
use std::io::Write;

pub fn update_index(file: &str, hash: &str) -> io::Result<()> {
    let index_path = ".mygit/index";

    let mut file_handle = OpenOptions::new()
        .create(true)
        .append(true)
        .open(index_path)?;

    writeln!(file_handle, "{} {}", hash, file)?;
    Ok(())
}
