#!/bin/bash

# Clean up previous coverage artifacts
rm -rf ./target *.prof* ./coverage

# Export the necessary instrumentation flag
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="Secure-Container-%p-%m.profraw"


# Build the program
cargo build

# Run your binary or tests
cargo test
sudo ./tests/negative_testing.sh
sudo ./tests/positive_testing.sh

grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/

