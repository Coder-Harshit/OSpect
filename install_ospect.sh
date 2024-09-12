#!/bin/bash

# Script name for clarity (optional)
SCRIPT_NAME="ospect_install.sh"

# User confirmation before making system-wide changes
echo "This script will install OSpect for your user only."
echo "It will not modify system-wide settings."
echo "Continue? (y/N)"

read -r confirmation

if [[ ! "$confirmation" =~ ^[Yy]$ ]]; then
  echo "Aborting installation."
  exit 1
fi

# Check for Rust and Cargo
if ! command -v rustc &> /dev/null; then
  echo "Rust is not installed. Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

# Download OSpect pre-built binary (if available) or build from source
if [[ -f "https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/releases/ospect" ]]; then
  # Download pre-built binary (adjust URL if necessary)
  echo "Downloading pre-built OSpect..."
  curl -O https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/releases/ospect
else
  echo "Pre-built binary not found. Building OSpect from source..."
  # Assuming your OSpect source code is in the current directory
  git clone https://github.com/Coder-Harshit/OSpect.git
  cd OSpect
  cargo build --release
  if [[ ! -f "target/release/ospect" ]]; then
    echo "OSpect build failed."
    exit 1
  fi
  mv target/release/ospect ospect
fi

# Check for executable permissions (optional, adjust if needed)
if [[ ! -x ospect ]]; then
  chmod +x ospect
fi

# Install OSpect for the user
echo "Adding OSpect to your user's PATH..."
user_bin_dir="$HOME/bin"
if [[ ! -d "$user_bin_dir" ]]; then
  echo "Creating $user_bin_dir directory..."
  mkdir -p "$user_bin_dir"
fi
mv ospect "$user_bin_dir"

# Update shell configuration file (handle different shells gracefully)
shell_config_file=""
if [[ -f ~/.bashrc ]]; then
  shell_config_file=~/.bashrc
elif [[ -f ~/.zshrc ]]; then
  shell_config_file=~/.zshrc
  # Add line for zsh (adjust if necessary)
  echo 'export PATH="$HOME/bin:$PATH"' >> "$shell_config_file"
else
  echo "Warning: Could not find a compatible shell configuration file."
  echo "You may need to manually add 'export PATH=\"$HOME/bin:$PATH\"' to your shell configuration file (e.g., ~/.bashrc or ~/.zshrc)."
fi

# Source the shell configuration file (if applicable)
if [[ ! -z "$shell_config_file" ]]; then
  echo "Sourcing $shell_config_file..."
  source "$shell_config_file"
fi

echo "OSpect installation complete for your user!"
echo "Restart your terminal or run 'source ~/.bashrc' (or your shell's equivalent) for the changes to take effect."
