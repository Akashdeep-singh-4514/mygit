use sha1::{Digest, Sha1};
use std::fs;
use std::io::{self};
use std::path::Path;

pub fn commit(message: &str) -> io::Result<()> {
    // 1. Read index
    let index_path = ".mygit/index";
    if !Path::new(index_path).exists() {
        println!("Nothing to commit (no index found)");
        return Ok(());
    }

    let index_content = fs::read_to_string(index_path)?;
    if index_content.trim().is_empty() {
        println!("Nothing to commit (index is empty)");
        return Ok(());
    }

    // 2. Build TREE content
    let mut tree_body = String::new();

    for line in index_content.lines() {
        let mut parts = line.split_whitespace();
        let hash = parts.next().unwrap();
        let file = parts.next().unwrap();

        tree_body.push_str(&format!("blob {} {}\n", hash, file));
    }

    // 3. Create TREE object
    let tree_header = format!("tree {}\0", tree_body.len());

    let mut tree_store = Vec::new();
    tree_store.extend(tree_header.as_bytes());
    tree_store.extend(tree_body.as_bytes());

    let mut hasher = Sha1::new();
    hasher.update(&tree_store);
    let tree_hash = hex::encode(hasher.finalize());

    if let Some(last_tree) = get_last_commit_tree() {
        if last_tree == tree_hash {
            println!("Nothing to commit (no changes)");
            return Ok(());
        }
    }

    let (dir, file) = tree_hash.split_at(2);
    let tree_dir = format!(".mygit/objects/{}", dir);
    let tree_path = format!("{}/{}", tree_dir, file);

    fs::create_dir_all(&tree_dir)?;
    if !Path::new(&tree_path).exists() {
        fs::write(&tree_path, &tree_store)?;
    }

    // 4. Build COMMIT content
    let mut commit_body = format!("tree {}\n", tree_hash);

    // Add parent if exists
    let head_path = ".mygit/HEAD_COMMIT";
    if Path::new(head_path).exists() {
        let parent = fs::read_to_string(head_path)?;
        let parent = parent.trim();
        if !parent.is_empty() {
            commit_body.push_str(&format!("parent {}\n", parent));
        }
    }

    commit_body.push_str(&format!("message {}\n", message));

    // 5. Create COMMIT object
    let commit_header = format!("commit {}\0", commit_body.len());

    let mut commit_store = Vec::new();
    commit_store.extend(commit_header.as_bytes());
    commit_store.extend(commit_body.as_bytes());

    let mut hasher = Sha1::new();
    hasher.update(&commit_store);
    let commit_hash = hex::encode(hasher.finalize());

    let (dir, file) = commit_hash.split_at(2);
    let commit_dir = format!(".mygit/objects/{}", dir);
    let commit_path = format!("{}/{}", commit_dir, file);

    fs::create_dir_all(&commit_dir)?;
    if !Path::new(&commit_path).exists() {
        fs::write(&commit_path, &commit_store)?;
    }

    // 6. Update HEAD
    fs::write(head_path, &commit_hash)?;

    // 7. Output
    println!("Committed as {}", commit_hash);

    Ok(())
}

fn get_last_commit_tree() -> Option<String> {
    let head_path = ".mygit/HEAD_COMMIT";

    if !std::path::Path::new(head_path).exists() {
        return None;
    }

    let commit_hash = std::fs::read_to_string(head_path).ok()?;
    let commit_hash = commit_hash.trim();

    let (dir, file) = commit_hash.split_at(2);
    let path = format!(".mygit/objects/{}/{}", dir, file);

    let data = std::fs::read(path).ok()?;

    // find null byte
    let null_index = data.iter().position(|&b| b == 0)?;
    let body = &data[null_index + 1..];

    let body_str = String::from_utf8_lossy(body);

    for line in body_str.lines() {
        if line.starts_with("tree ") {
            return Some(line[5..].to_string());
        }
    }

    None
}
