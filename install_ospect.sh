#!/bin/bash

# Install Rust and Cargo if not already installed
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

curl -O https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/releases/ospect

# move OSpect to /usr/local/bin for system wide access
sudo mv ospect /usr/local/bin

# Run OSpect help command
# echo "Running OSpect help command..."
# ospect --help

echo "Installation complete! You can now use the 'ospect' command."
