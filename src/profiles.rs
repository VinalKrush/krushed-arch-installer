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
    bluez
    spotify-launcher


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

use std::process::Command;
pub enum InstallProfile {
    Base,
    Minimal,
    Desktop,
    FullDesktop,
    Gaming,
}

fn run_command(command: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        println!("Command failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn chroot_command(_command: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("arch-chroot /mnt {}", _command))
        .output()
        .expect("Failed to execute chroot command");

    if !output.status.success() {
        println!("Command failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn install_profile(profile: InstallProfile) {
    match profile {
        InstallProfile::Base => {
            println!("INSTALLING BASE PROFILE");
            println!("");
            println!("(This May Take Long Depending On Your Internet Speed...)");
            println!("");
            println!("");
            println!("");
            println!("");
            println!("");
            base_profile();
        }
        InstallProfile::Minimal => {
            println!("INSTALLING MINIMAL PROFILE");
            println!("");
            println!("(This May Take Long Depending On Your Internet Speed...)");
            println!("");
            println!("");
            println!("");
            println!("");
            println!("");
            base_profile();
            minimal_profile();
        }
        InstallProfile::Desktop => {
            println!("INSTALLING DESKTOP PROFILE");
            println!("");
            println!("(This May Take Long Depending On Your Internet Speed...)");
            println!("");
            println!("");
            println!("");
            println!("");
            println!("");
            base_profile();
            minimal_profile();
            desktop_profile();
        }
        InstallProfile::FullDesktop => {
            println!("INSTALLING FULL DESKTOP PROFILE");
            println!("");
            println!("(This May Take Long Depending On Your Internet Speed...)");
            println!("");
            println!("");
            println!("");
            println!("");
            println!("");
            base_profile();
            minimal_profile();
            desktop_profile();
            full_desktop_profile();
        }
        InstallProfile::Gaming => {
            println!("INSTALLING GAMING PROFILE");
            println!("");
            println!("(This May Take Long Depending On Your Internet Speed...)");
            println!("");
            println!("");
            println!("");
            println!("");
            println!("");
            base_profile();
            minimal_profile();
            desktop_profile();
            full_desktop_profile();
            gaming_profile();
        }
    }
}

fn base_profile() {
    //Base Install
    println!("Downloading Base Packages...");
    run_command(
        "pacstrap -K -P /mnt base base-devel linux linux-firmware linux-headers grub efibootmgr openssh networkmanager vim git"
    );
    chroot_command("systemctl enable NetworkManager.service");
}

fn minimal_profile() {
    //Minimal Install
    println!("Downloading Minimal Packages...");
    run_command(
        "pacstrap -K -P /mnt os-prober fastfetch btop ly reflector ldns wget curl xclip unzip unrar btrfs-progs exfat-utils ntfs-3g"
    );
    run_command("cp -r /etc/xdg/reflector/reflector.conf /mnt/etc/xdg/reflector/reflector.conf ");
    chroot_command("systemctl enable ly.service reflector.service");
}

fn desktop_profile() {
    //Desktop Install
    println!("Downloading Desktop Packages...");
    run_command(
        "pacstrap -K -P /mnt xorg wayland plasma firefox pipewire lib32-pipewire wireplumber pipewire-audio pipewire-alsa pipewire-pulse noto-fonts konsole dolphin"
    );
}

fn full_desktop_profile() {
    //Full Desktop Install
    println!("Downloading What Some People May Consider Bloat Packages...");
    run_command(
        "pacstrap -K -P /mnt zsh noto-fonts-cjk noto-fonts-extra noto-fonts-emoji ttf-hack-nerd gparted gvfs gvfs-afc grub-customizer flatpak dpkg less qpwgraph gnome-calculator fzf fuse2 fuse3 alsa-utils ufw vlc libreoffice-fresh code kvantum bluez spotify-launcher"
    );
    chroot_command("systemctl enable bluetooth.service");
}

fn gaming_profile() {
    //Gaming Install
    println!("Downloading Gaming Dependencies...");
    run_command(
        "pacstrap -K -P /mnt steam discord lutris wine-staging giflib lib32-giflib gnutls lib32-gnutls v4l-utils lib32-v4l-utils libpulse lib32-libpulse alsa-plugins lib32-alsa-plugins alsa-lib lib32-alsa-lib sqlite lib32-sqlite libxcomposite lib32-libxcomposite ocl-icd lib32-ocl-icd libva lib32-libva gtk3 lib32-gtk3 gst-plugins-base-libs lib32-gst-plugins-base-libs vulkan-icd-loader lib32-vulkan-icd-loader sdl2 lib32-sdl2 jre-openjdk jre8-openjdk jre11-openjdk jre17-openjdk jre21-openjdk"
    );
}
