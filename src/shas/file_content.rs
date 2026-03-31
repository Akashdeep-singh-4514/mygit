use sha1::{Digest, Sha1};
use std::fs;
use std::io::{self, Read};

pub fn hash_object(path: &str) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let mut hasher = Sha1::new();
    hasher.update(&content);
    let hash = hex::encode(hasher.finalize());

    let object_path = format!(".mygit/objects/{}", hash);

    if !std::path::Path::new(&object_path).exists() {
        fs::write(&object_path, content)?;
    }

    Ok(hash)
}
