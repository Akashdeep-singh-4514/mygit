use crate::index;
use crate::object::blob;
use std::io;

pub fn add(file: &str) -> io::Result<()> {
    if !std::path::Path::new(".mygit").exists() {
        println!("Not a mygit repository.");
        return Ok(());
    }

    let hash = blob::hash_object(file)?;
    index::update_index(file, &hash)?;

    println!("Added {} ({})", file, hash);
    Ok(())
}
