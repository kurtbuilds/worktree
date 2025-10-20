set positional-arguments
set export
set dotenv-load

# Run the project
run *ARGS:
    cargo run -- "$@"

# Check the project for errors
check:
    cargo check

# Install the binary
install:
    cargo install --path .

# Run tests
test *ARGS:
    cargo test "$@"
