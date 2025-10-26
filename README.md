# monar

A privacy-focused, high-performance monorepo. This project is structured for scalability, testability, and maintainability.

## Tech Stack

- Rust
- Cargo workspaces
- cargo-make (automation)
- GitHub Actions (CI/CD)
- Optionally: WASM/npm (for packages)

## Structure

- `cli/`: CLI tools
- `crates/`: Core libraries and utilities
- `packages/`: Feature packages (npm/WASM)
- `examples/`: Example projects
- `tests/`: Integration tests
- `scripts/`: Helper scripts
- `docs/`: Documentation
- `.github/`: CI/CD workflows

See each directory's README for more details.

## Development Setup

This project uses [cargo-make](https://sagiegurari.github.io/cargo-make/) for automation (build, test, lint, etc.).

### Install cargo-make

```sh
cargo install cargo-make
```

### Common Tasks

```sh
cargo make build      # Build all workspace members
cargo make test       # Run all tests
cargo make lint       # Run formatting and clippy
cargo make run-cli    # Run the main CLI
cargo make clean      # Clean build artifacts
```
