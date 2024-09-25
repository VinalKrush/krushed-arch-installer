#!/bin/bash

# Check if running as root
if [ "$(id -u)" != "0" ]; then
  echo "Error: This script must be run as root"
  exit 1
fi

clear

echo "NOTE: THIS SCRIPT IS UNTESTED. USE AT YOUR OWN RISK"
read
echo "Installing dependencies..."
# Install Rust if not already installed
if ! command -v rustc &> /dev/null; then
  pacman -S rustup
fi

# Compile your Rust program
cargo build --release
mkdir /etc/krushed/arch-installer
cp -r src/etc/* /etc/krushed/arch-installer/

# Install the compiled binary
install -Dm 755 target/release/krushed-arch-installer /usr/bin/krushed-arch-installer