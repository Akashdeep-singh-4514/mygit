use std::fs;
use std::io;

pub fn log() -> io::Result<()> {
    let mut current = match fs::read_to_string(".mygit/HEAD_COMMIT") {
        Ok(h) => h.trim().to_string(),
        Err(_) => {
            println!("No commits yet.");
            return Ok(());
        }
    };

    loop {
        // 1. Locate commit object
        let (dir, file) = current.split_at(2);
        let path = format!(".mygit/objects/{}/{}", dir, file);

        let data = fs::read(&path)?;

        // 2. Remove header (before \0)
        let null_index = data.iter().position(|&b| b == 0).unwrap();
        let body = &data[null_index + 1..];

        let body_str = String::from_utf8_lossy(body);

        // 3. Parse fields
        let mut parent: Option<String> = None;
        let mut message = "";

        for line in body_str.lines() {
            if line.starts_with("parent ") {
                parent = Some(line[7..].to_string());
            } else if line.starts_with("message ") {
                message = &line[8..];
            }
        }

        // 4. Print commit
        println!("commit {}", current);
        println!("    {}\n", message);

        // 5. Move to parent
        match parent {
            Some(p) => current = p,
            None => break,
        }
    }

    Ok(())
}
