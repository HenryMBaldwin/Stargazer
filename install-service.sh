#!/bin/bash

# Run the command to uninstall the daemon
sudo cargo run --bin uninstall-daemon

# Build the stargazer-daemon
cargo build --bin stargazer-daemon

# Run the command to install the daemon
sudo cargo run --bin install-daemon
