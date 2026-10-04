# Displays help message
default:
    @just --list

## Development commands

# Launch the GUI package
[working-directory: 'gui']
run: (clippy "cyrano-gui")
    dx serve

# Cleans the build artifacts
clean:
    cargo clean

# Run clippy on a specific package
clippy PKG:
    cargo clippy -p {{PKG}} --all-targets -- -D warnings

# Run clippy on all packages
clippy-all:
    cargo clippy --workspace --all-targets -- -D warnings

# Run unit tests on a specific package
test PKG: (clippy PKG)
    cargo test -p {{PKG}}

# Run benches on a specific package
bench PKG: (clippy PKG)
    cargo bench -p {{PKG}}

## Production

# Builds the full project
build:
    cargo build --release --workspace