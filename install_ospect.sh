#!/bin/bash

# Install Rust and Cargo if not already installed
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

## Clone the OSpect repository if the directory does not exist
#if [ ! -d "OSpect" ]; then
#    echo "Cloning OSpect repository..."
#    git clone https://github.com/Coder-Harshit/OSpect.git
#else
#    echo "OSpect directory already exists. Skipping clone."
#fi

#cd OSpect

# Build and install OSpect
#echo "Building and installing OSpect..."
#cargo build --release

curl -O https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/releases/ospect
#cargo install --path .

chmod +x ospect



# Add OSpect to PATH
# Get the current working directory
CURRENT_DIR=$(pwd)

export PATH="$CURRENT_DIR:$PATH"
# Determine the OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    echo 'export PATH="$CURRENT_DIR:$PATH"' >> ~/.bashrc
    echo "OSpect added to PATH in ~/.bashrc"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    echo 'export PATH="$CURRENT_DIR:$PATH"' >> ~/.bash_profile
    echo "OSpect added to PATH in ~/.bash_profile"
else
    echo "Unsupported operating system: $OSTYPE"
    exit 1
fi

# Run OSpect help command
# echo "Running OSpect help command..."
# ospect --help

echo "Installation complete! You can now use the 'ospect' command."
