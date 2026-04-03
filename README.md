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

## Installation

### 1. Prerequisites

Install a **Rust** toolchain if you do not already have one. The usual approach is [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Open a new shell (or run `source "$HOME/.cargo/env"`) so `cargo` and `rustc` are on your `PATH`. This project targets the **2024 edition** (see `Cargo.toml`); use a current stable toolchain from rustup.

### 2. Get the source

Clone or copy this repository, then go to its root:

```bash
cd /path/to/mygit
```

### 3. Build a release binary

```bash
cargo build --release
```

The binary is `target/release/mygit`. You can run it directly:

```bash
./target/release/mygit --help
```

### 4. Put `mygit` on your PATH (pick one)

**User install via Cargo** (recommended: installs to `~/.cargo/bin`, which rustup usually adds to `PATH`):

```bash
cargo install --path .
```

After that, `mygit` should work from any directory if `~/.cargo/bin` is on your `PATH`.

**Copy to your user bin** (no `sudo`; create `~/.local/bin` if it does not exist):

```bash
mkdir -p "$HOME/.local/bin"
cp target/release/mygit "$HOME/.local/bin/"
```

Ensure `"$HOME/.local/bin"` is listed in your `PATH` in your shell profile.

**System-wide** (Linux/macOS; requires appropriate permissions):

```bash
sudo cp target/release/mygit /usr/local/bin/
```

### Run without installing

From the project root, use Cargo to build and invoke the binary in one step:

```bash
cargo run --release -- init
cargo run --release -- add somefile.txt
cargo run --release -- commit "message"
cargo run --release -- log
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
