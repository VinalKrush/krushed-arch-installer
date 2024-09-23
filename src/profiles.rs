/*
DEFAULT PROFILE PACKAGES

Base (Base Arch Linux. Almost Nothing Installed):
    base 
    base-devel 
    linux 
    linux-firmware 
    linux-headers 
    grub 
    efibootmgr 
    openssh 
    networkmanager
    vim
    git 
    UCODE


Minimal (Minimal Stuff Installed):
    os-prober
    fastfetch
    btop
    ly
    reflector
    ldns
    wget
    curl
    xclip
    unzip
    unrar
    btrfs-progs
    exfat-utils
    ntfs-3g

Desktop (Minimal KDE Desktop Environment):
    GPU-Drivers
    xorg
    wayland
    plasma
    firefox
    pipewire
    lib32-pipewire
    wireplumber
    pipewire-audio
    pipewire-alsa
    pipewire-pulse
    noto-fonts
    konsole
    dolphin


Full Desktop (Full KDE Desktop Environment With Everything You'll Ever Need):
    zsh
    noto-fonts-cjk
    noto-fonts-extra
    noto-fonts-emoji
    ttf-hack-nerd
    gparted
    gvfs
    gvfs-afc
    grub-customizer
    flatpak
    dpkg
    less
    qpwgraph
    gnome-calculator
    fzf
    fuse2
    fuse3
    alsa-utils
    ufw
    vlc
    libreoffice-fresh
    code
    kvantum
    ocs-url



Gaming (Full KDE Desktop Gaming Environment With Preinstalled Wine-Staging And Other Gaming Packages):
    steam
    discord
    lutris
    wine-staging
    giflib 
    lib32-giflib 
    gnutls 
    lib32-gnutls 
    v4l-utils 
    lib32-v4l-utils 
    libpulse
    lib32-libpulse 
    alsa-plugins 
    lib32-alsa-plugins 
    alsa-lib 
    lib32-alsa-lib 
    sqlite 
    lib32-sqlite 
    libxcomposite
    lib32-libxcomposite 
    ocl-icd 
    lib32-ocl-icd 
    libva 
    lib32-libva 
    gtk3 
    lib32-gtk3 
    gst-plugins-base-libs
    lib32-gst-plugins-base-libs 
    vulkan-icd-loader 
    lib32-vulkan-icd-loader 
    sdl2 
    lib32-sdl2
    jre-openjdk
    jre8-openjdk
    jre11-openjdk
    jre17-openjdk
    jre21-openjdk
*/

use indicatif::{ProgressBar, ProgressStyle};
use std::process::{Command, Stdio};

pub enum InstallProfile {
    Base,
    Minimal,
    Desktop,
    FullDesktop,
    Gaming,
}

fn run_command(command: &str) {
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        println!("Command failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn install_profile(profile: InstallProfile) {
    match profile {
        InstallProfile::Base => {
            // Base Install
            println!("Installing Base Profile...");
            //run_command("pacstrap -K -P /mnt base base-devel linux linux-firmware linux-headers grub efibootmgr openssh networkmanager vim git");
        }
        InstallProfile::Minimal => {
            // Base Install
            println!("Installing Base Profile...");
            //run_command("pacstrap -K -P /mnt base base-devel linux linux-firmware linux-headers grub efibootmgr openssh networkmanager vim git");

            // Minimal Install
            println!("Installing Minimal Profile...");
            //run_command("pacstrap -K -P /mnt os-prober fastfetch btop ly reflector ldns wget curl xclip unzip unrar btrfs-progs exfat-utils ntfs-3g");
        }
        InstallProfile::Desktop => {
            // Base Install
            println!("Installing Base Profile...");
            //run_command("pacstrap -K -P /mnt base base-devel linux linux-firmware linux-headers grub efibootmgr openssh networkmanager vim git");

            // Minimal Install
            println!("Installing Minimal Profile...");
            //run_command("pacstrap -K -P /mnt os-prober fastfetch btop ly reflector ldns wget curl xclip unzip unrar btrfs-progs exfat-utils ntfs-3g");

            //Desktop Install
            println!("Installing Desktop Profile...");
            //run_command("pacstrap -K -P /mnt xorg wayland plasma firefox pipewire lib32-pipewire wireplumber pipewire-audio pipewire-alsa pipewire-pulse noto-fonts konsole dolphin");
        }
        InstallProfile::FullDesktop  => {
            // Base Install
            println!("Installing Base Profile...");
            //run_command("pacstrap -K -P /mnt base base-devel linux linux-firmware linux-headers grub efibootmgr openssh networkmanager vim git");

            // Minimal Install
            println!("Installing Minimal Profile...");
            //run_command("pacstrap -K -P /mnt os-prober fastfetch btop ly reflector ldns wget curl xclip unzip unrar btrfs-progs exfat-utils ntfs-3g");

            //Desktop Install
            println!("Installing Desktop Profile...");
            //run_command("pacstrap -K -P /mnt xorg wayland plasma firefox pipewire lib32-pipewire wireplumber pipewire-audio pipewire-alsa pipewire-pulse noto-fonts konsole dolphin");

            //Full Desktop Install
            println!("Installing Full Desktop Profile...");
            //run_command("pacstrap -K -P /mnt zsh noto-fonts-cjk noto-fonts-extra noto-fonts-emoji ttf-hack-nerd gparted gvfs gvfs-afc grub-customizer flatpak dpkg less qpwgraph gnome-calculator fzf fuse2 fuse3 alsa-utils ufw vlc libreoffice-fresh code kvantum ocs-url");
        }
        InstallProfile::Gaming => {
            // Base Install
            println!("Installing Base Profile...");
            //run_command("pacstrap -K -P /mnt base base-devel linux linux-firmware linux-headers grub efibootmgr openssh networkmanager vim git");

            // Minimal Install
            println!("Installing Minimal Profile...");
            //run_command("pacstrap -K -P /mnt os-prober fastfetch btop ly reflector ldns wget curl xclip unzip unrar btrfs-progs exfat-utils ntfs-3g");

            //Desktop Install
            println!("Installing Desktop Profile...");
            //run_command("pacstrap -K -P /mnt xorg wayland plasma firefox pipewire lib32-pipewire wireplumber pipewire-audio pipewire-alsa pipewire-pulse noto-fonts konsole dolphin");

            //Full Desktop Install
            println!("Installing Full Desktop Profile...");
            //run_command("pacstrap -K -P /mnt zsh noto-fonts-cjk noto-fonts-extra noto-fonts-emoji ttf-hack-nerd gparted gvfs gvfs-afc grub-customizer flatpak dpkg less qpwgraph gnome-calculator fzf fuse2 fuse3 alsa-utils ufw vlc libreoffice-fresh code kvantum ocs-url");

            //Gaming Install
            println!("Installing Gaming Profile...");
            //run_command("pacstrap -K -P /mnt steam discord lutris wine-staging giflib lib32-giflib gnutls lib32-gnutls v4l-utils lib32-v4l-utils libpulse lib32-libpulse alsa-plugins lib32-alsa-plugins alsa-lib lib32-alsa-lib sqlite lib32-sqlite libxcomposite lib32-libxcomposite ocl-icd lib32-ocl-icd libva lib32-lbva gtk3 lib32-gtk3 gst-plugins-base-libs lib32-gst-plugins-base-libs vulkan-icd-loader lib32-vulkan-icd-loader sdl2 lib32-sdl2 jre-openjdk jre8-openjdk jre11-openjdk jre17-openjdk jre21-openjdk");
        }

    }
}