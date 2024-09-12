#!/bin/bash

# Install Rust and Cargo if not already installed
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Clone the OSpect repository if the directory does not exist
if [ ! -d "OSpect" ]; then
    echo "Cloning OSpect repository..."
    git clone https://github.com/Coder-Harshit/OSpect.git
else
    echo "OSpect directory already exists. Skipping clone."
fi

cd OSpect

# Build and install OSpect
echo "Building and installing OSpect..."
cargo build --release
cargo install --path .

# Add OSpect to PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Run OSpect help command
# echo "Running OSpect help command..."
# ospect --help

echo "Installation complete! You can now use the 'ospect' command."
