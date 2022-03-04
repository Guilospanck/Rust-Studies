# Rust Studies
First steps with Rust: February, 26 2022.

## Installation
### Rust
Go to [Rust](https://www.rust-lang.org/learn/get-started) and install it with one of the options.
### Rust Toolchain (Rustup)
Go to [Rustup](https://www.rust-lang.org/tools/install) and install it with one of the options.

## Cargo
Cargo is the package manager for Rust (something like NPM and YARN for Node).

In order to create a new project in Rust, you can run:
```bash
cargo new project_name
```
And then it will create a folder called `project_name` with a `Cargo.lock` and `Cargo.toml` files AND a `src` folder containing a `main.rs` file.

### Cargo.toml
It's similar to `package.json`.

### Compiling and Executing a project
```bash
cargo run
```
The executable file will be at `target/debug/project_name`.

### Running tests
```bash
cargo test
```

## Rust version
```bash
rustc --version
```

## VSCode extensions for Rust
- Rust