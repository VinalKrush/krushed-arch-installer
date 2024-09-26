#!/bin/bash

# Check if running as root
if [ "$(id -u)" != "0" ]; then
  echo "Error: This script must be run as root"
  exit 1
fi

clear

echo "NOTE: CURRENTLY KNOWN BUGS:"
echo "User Creation Does Not Change The Sudoers File So It Does Not Make Admin Accounts"
echo ""
echo "PRESS ENTER TO CONTINUE"
read

# Compile your Rust program
mkdir -p /etc/krushed/arch-installer/etc
mkdir -p /etc/krushed/arch-installer/usr-config
touch /etc/krushed/arch-installer/usr-config/.zshrc

cp -r etc/* /etc/krushed/arch-installer/etc/
cp -r usr-config/.zshrc /etc/krushed/arch-installer/usr-config/.zshrc
cp /etc/krushed/arch-installer/etc/pacman.conf /etc/pacman.conf

# Install the compiled binary
install -Dm 755 bin/krushed-arch-installer /usr/bin/krushed-arch-installer

echo "INSTALLED!"
krushed-arch-installer
