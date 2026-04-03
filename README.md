# mygit

A small [Rust](https://www.rust-lang.org/) learning project: a miniature version-control tool with a Git-like workflow. It stores objects under `.mygit/` and supports initializing a repo, staging files, committing with a message, and walking the commit history.

This is **not** a full Git implementation. It is meant for experimentation and understanding how blobs, trees, commits, and a simple index fit together.

## Features

| Command | What it does |
|--------|----------------|
| `init` | Creates `.mygit/` with `objects/`, `refs/heads/`, `HEAD` (points at `refs/heads/main`), and prints a confirmation. |
| `add <file>` | Reads the file, stores a `blob` object (content-addressed with SHA-1), and records `hash path` in `.mygit/index`. |
| `commit <message>` | Builds a `tree` from the index, creates a `commit` object (with optional `parent` from the previous commit), updates `.mygit/HEAD_COMMIT` to the new commit hash. |
| `log` | Prints commits newest-first by following `parent` links, showing hash and message. |

HEAD in `.mygit/HEAD` is initialized for compatibility with a Git-like layout; the tool tracks the current tip via `.mygit/HEAD_COMMIT`.

## Requirements

- A recent **Rust** toolchain (`cargo`, `rustc`) with support for the **2024 edition** (as set in `Cargo.toml`).

## Build

Release binary:

```bash
cargo build --release
```

The executable is at `target/release/mygit`.

### Optional: install on your PATH

```bash
sudo cp target/release/mygit /usr/local/bin/
```

Or run without installing:

```bash
cargo run --release -- <subcommand> ...
```

## Usage

From a directory where you want a repository:

```bash
mygit init
echo "hello" > notes.txt
mygit add notes.txt
mygit commit "first commit"
mygit log
```

- Run commands from the **repository root** (where `.mygit` lives).
- `add` expects a path relative to the current working directory (typically a single file name in the root).

## Repository layout (inside `.mygit`)

| Path | Role |
|------|------|
| `objects/XX/…` | Stored objects: `blob`, `tree`, and `commit` payloads with type headers and null-separated metadata (similar in spirit to Git loose objects). |
| `index` | Staging area: lines of `hex_hash relative_path`. |
| `HEAD_COMMIT` | Current commit hash (what `commit` updates and `log` follows). |
| `HEAD` | Text file `ref: refs/heads/main` after `init`. |
| `refs/heads/` | Created at init; branch updates are not fully wired like Git. |

## Limitations

- No branching, merging, remote, packs, or garbage collection.
- Single-user, local-only workflow; commit identity, dates, and rich metadata are not stored.
- Filenames in the index and tree are simple strings; edge cases (spaces in paths, directories) are not the focus of this demo.

## License

This project did not ship with a license file in the repository; add one if you distribute or reuse the code.
