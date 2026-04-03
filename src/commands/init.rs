use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn init_repo() -> io::Result<()> {
    let base = Path::new(".mygit");

    if base.exists() {
        println!("Repository already initialized.");
        return Ok(());
    }

    fs::create_dir(base)?;
    fs::create_dir(base.join("objects"))?;
    fs::create_dir_all(base.join("refs").join("heads"))?;

    let mut head_file = fs::File::create(base.join("HEAD"))?;
    head_file.write_all(b"ref: refs/heads/main\n")?;

    println!("Initialized empty mygit repository in .mygit/");
    Ok(())
}
