use sha1::{Digest, Sha1};
use std::fs;
use std::io::{self, Read};

pub fn hash_object(path: &str) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let header = format!("blob {}\0", content.len());

    let mut store = Vec::new();
    store.extend(header.as_bytes());
    store.extend(&content);

    let mut hasher = Sha1::new();
    hasher.update(&store);
    let hash = hex::encode(hasher.finalize());

    let (dir, file) = hash.split_at(2);
    let object_dir = format!(".mygit/objects/{}", dir);
    let object_path = format!("{}/{}", object_dir, file);

    fs::create_dir_all(&object_dir)?;

    if !std::path::Path::new(&object_path).exists() {
        fs::write(&object_path, store)?;
    }

    Ok(hash)
}
