use crate::index::manage::update_index;
use crate::shas::file_content::hash_object;
use std::io;

pub fn add(file: &str) -> io::Result<()> {
    if !std::path::Path::new(".mygit").exists() {
        println!("Not a mygit repository.");
        return Ok(());
    }

    let hash = hash_object(file)?;
    update_index(file, &hash)?;

    println!("Added {} ({})", file, hash);
    Ok(())
}
