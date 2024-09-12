#!/bin/bash

# Install Rust and Cargo if not already installed
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

curl -O https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/releases/ospect

# move OSpect to /usr/local/bin for system wide access
mv ospect $HOME/bin
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
    # echo "OSpect added to PATH in ~/.bashrc"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bash_profile
    # echo "OSpect added to PATH in ~/.bash_profile"
else
    echo "Unsupported operating system: $OSTYPE"
    exit 1
fi
# Run OSpect help command
# echo "Running OSpect help command..."
# ospect --help

echo "Installation complete! You can now use the 'ospect' command."
