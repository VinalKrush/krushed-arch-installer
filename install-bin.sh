#!/bin/bash

# Check if running as root
if [ "$(id -u)" != "0" ]; then
  echo "Error: This script must be run as root"
  exit 1
fi

clear

echo "NOTE: THIS SCRIPT IS UNTESTED. USE AT YOUR OWN RISK"
echo "PRESS ENTER TO CONTINUE"
read

# Compile your Rust program
mkdir -p /etc/krushed/arch-installer
cp -r src/etc/* /etc/krushed/arch-installer/
cp /etc/krushed/arch-installer/etc/pacman.conf /etc/pacman.conf

# Install the compiled binary
install -Dm 755 bin/krushed-arch-installer /usr/bin/krushed-arch-installer