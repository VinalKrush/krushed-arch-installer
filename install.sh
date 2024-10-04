#!/bin/bash

# Check if running as root
if [ "$(id -u)" != "0" ]; then
  echo "Error: This script must be run as root"
  exit 1
fi

clear

# Compile your Rust program
mkdir -p /etc/krushed/arch-installer/etc
mkdir -p /etc/krushed/arch-installer/usr-config
touch /etc/krushed/arch-installer/usr-config/.zshrc
touch /etc/krushed/arch-installer/usr-config/install-krushed-zsh.sh
touch /etc/krushed/arch-installer/usr-config/install-yay.sh

cp -r etc/* /etc/krushed/arch-installer/etc/
cp -r usr-config/.zshrc /etc/krushed/arch-installer/usr-config/.zshrc
cp -r usr-config/install-krushed-zsh /etc/krushed/arch-installer/usr-config/install-krushed-zsh.sh
cp -r usr-config/install-yay /etc/krushed/arch-installer/usr-config/install-yay.sh
cp /etc/krushed/arch-installer/etc/pacman.conf /etc/pacman.conf

# Install the compiled binary
install -Dm 755 bin/krushed-arch-installer /usr/bin/krushed-arch-installer

echo "INSTALLED!"
krushed-arch-installer
