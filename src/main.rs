/* 
PLEASE READ!

BEFORE JUDGING THIS CODE PLEASE KNOW I AM NEW TO RUST 
THIS WILL NOT BE THE BEST AND I KNOW IT CAN BE BETTER ;-;

Also I'm using ListItem because idk how to use paragraph in ratatui

FILES:
/etc/krushed/arch-installer/
/etc/krushed/arch-installer/etc/pacman.conf



*/

mod profiles;
mod ucode;
mod drivers;
mod bell;
mod tui;

use tui::{ new_tui_text, clear_terminal };
use profiles::{ InstallProfile, install_profile };
use ucode::{ InstallUcode, install_ucode };
use drivers::{ InstallDriver, install_driver };
use bell::ring_bell;
use ratatui::{
    buffer::Buffer,
    backend::CrosstermBackend,
    prelude::Alignment,
    crossterm::event::{ self, Event, KeyCode, KeyEventKind },
    layout::{ Constraint, Layout, Rect, Position },
    style::{ Color, Modifier, Stylize, Style },
    text::{ Line, Masked, Span, Text },
    widgets::{ Block, Paragraph, Widget, Wrap, List, ListItem },
    Frame,
    DefaultTerminal,
    Terminal,
};
use dialoguer::{ Password, Input, Confirm };
use std::{ io, self, stdout, thread::sleep, time::Duration };
struct InstallerState {
    selected_profile: i32,
    selected_ucode: i32,
    selected_driver: i32,
    root_pass: String,
    hostname: String,
    swap_size: i32,
}

fn main() -> Result<(), io::Error> {
    let mut state = InstallerState {
        selected_profile: 0,
        selected_ucode: 0,
        selected_driver: 0,
        root_pass: "".to_string(),
        hostname: "".to_string(),
        swap_size: 4,
    };

    let text = Text::from(
        vec![
            Line::from("Welcome To The Krushed Arch Linux Installer"),
            Line::from(
                "Please Ensure That You Have Mounted Your Partitions To \"/mnt\" Before Continuing This Install"
            ),
            Line::from(""),
            Line::from("(PRESS ENTER TO CONTINUE)")
        ]
    )
        .magenta()
        .centered();
    new_tui_text(text);

    let driveconfirmation = Confirm::new().default(true).interact().unwrap();

    if !driveconfirmation {
        let text = Text::from(vec![Line::from("Install Cancelled...")]).centered();
        new_text(text);
        ring_bell();
        return Ok(());
    } else {
        profile_selector(&mut state)?;
    }
    Ok(())
}

fn profile_selector(state: &mut InstallerState) -> Result<(), io::Error> {
    let text = Text::from(
        vec![
            Line::from("Please Type A Number To Select A Profile"),
            Line::from(""),
            Line::from("1. Base"),
            Line::from("(Basic Arch Linux Install)"),
            Line::from(""),
            Line::from("2. Minimal"),
            Line::from("(Best For Servers)"),
            Line::from(""),
            Line::from("3. Desktop"),
            Line::from("(Basic Desktop Install | KDE Only"),
            Line::from(""),
            Line::from("4. Full Desktop"),
            Line::from("(Has Pre-Installed Apps Like Spotify, VLC, GParted, Libreoffice, Etc...)"),
            Line::from(""),
            Line::from("5. Gaming"),
            Line::from(
                "(Has Pre-Installed Gaming Dependencies Like Wine Staging, VKD3d, Steam, Lutris, And More)"
            )
        ]
    )
        .magenta()
        .centered();
    new_tui_text(text);
    let selected_profile = Input::<i32>::new().interact_text().unwrap();

    if selected_profile >= 1 && selected_profile <= 5 {
        state.selected_profile = selected_profile;
        ucode_selector(state)?;

        Ok(())
    } else {
        clear_terminal();
        println!("Invalid Choice.");
        ring_bell();
        return Ok(());
    }
}

fn ucode_selector(state: &mut InstallerState) -> Result<(), io::Error> {
    let text = Text::from(
        vec![
            Line::from("Please Type A Number To Select A CPU Platform"),
            Line::from(""),
            Line::from("1. INTEL"),
            Line::from("2. AMD")
        ]
    )
        .magenta()
        .centered();
    new_tui_text(text);

    let selected_ucode = Input::<i32>::new().interact_text().unwrap();

    if selected_ucode >= 1 && selected_ucode <= 2 {
        state.selected_ucode = selected_ucode;
        driver_selector(state)?;

        Ok(())
    } else {
        clear_terminal();
        println!("Invalid Choice.");
        ring_bell();
        return Ok(());
    }
}

fn driver_selector(state: &mut InstallerState) -> Result<(), io::Error> {
    let text = Text::from(
        vec![
            Line::from("Please Type A Number To Select A GPU Driver"),
            Line::from(""),
            Line::from("1. AMD"),
            Line::from("2. INTEL"),
            Line::from("3. INTEL"),
            Line::from("4. VMWARE"),
            Line::from("5. No Driver")
        ]
    )
        .magenta()
        .centered();
    new_tui_text(text);

    let selected_driver = Input::<i32>::new().interact_text().unwrap();

    if selected_driver >= 1 && selected_driver <= 5 {
        state.selected_driver = selected_driver;
        root_password(state)
    } else {
        clear_terminal();
        println!("Invalid Choice.");
        ring_bell();
        return Ok(());
    }
}

fn root_password(state: &mut InstallerState) -> Result<(), io::Error> {
    let text = Text::from(
        vec![Line::from("Please Set A Root Password"), Line::from("(Leave Empty To Disable Root)")]
    )
        .magenta()
        .centered();
    new_tui_text(text);

    let root_pass = Password::new()
        .with_confirmation("Confirm Password", "Passwords Do Not Match")
        .interact()
        .unwrap();

    state.root_pass = root_pass;
    host_name(state)?;

    Ok(())
}

fn host_name(state: &mut InstallerState) -> Result<(), io::Error> {
    let text = Text::from(vec![Line::from("Please Type Set A Hostname")])
        .magenta()
        .centered();
    new_tui_text(text);

    let host_name = Input::new().interact().unwrap();
    terminal.clear()?;

    state.hostname = host_name;
    swap_creation(state)?;

    Ok(())
}

fn swap_creation(state: &mut InstallerState) -> Result<(), io::Error> {
    let text = Text::from(
        vec![Line::from("Would You Like To Create A Swap File?"), Line::from("(Y/n)")]
    )
        .magenta()
        .centered();
    new_tui_text(text);

    let confirm = Confirm::new().default(true).interact().unwrap();

    if confirm {
        let text = Text::from(
            vec![
                Line::from("How Big Do You Want The Swap File?"),
                Line::from(""),
                Line::from("Example:"),
                Line::from("8"),
                Line::from("(This Would Be 8GB)")
            ]
        )
            .magenta()
            .centered();
        new_tui_text(text);

        let swap_size = Input::<i32>::new().interact_text().unwrap();

        state.swap_size = swap_size;
        install_confirm(state);
    } else {
        state.swap_size = -1;
        install_confirm(state);
    }
    Ok(())
}

fn install_confirm(state: &mut InstallerState) -> Result<(), io::Error> {
    let text = Text::from(
        vec![
            Line::from("Are You Sure You Want To Continue With This Install?"),
            Line::from("(PRESS ENTER TO CONTINUE)")
        ]
    )
        .magenta()
        .centered();
    new_tui_text(text);

    let install_confirmation = Confirm::new().default(true).interact().unwrap();

    if !install_confirmation {
        clear_terminal();
        println!(" Install Cancelled...");
        ring_bell();
        return Ok(());
    } else {
        start_install(state)?;
    }

    Ok(())
}

fn user_creation(state: &mut InstallerState) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Ask If Want To Make A New User?

    // Ask for username
    terminal.clear()?;
    let username = Input::new().with_prompt("Enter Username:").interact().unwrap();

    // Ask for password
    terminal.clear()?;
    let password = Password::new()
        .with_prompt("Enter Password:")
        .with_confirmation("Confirm Password", "Passwords Do Not Match.")
        .interact()
        .unwrap();

    // If user should be an admin
    terminal.clear()?;
    let user_admin = Confirm::new()
        .with_prompt(format!("Should {} be an admin?", username).as_str())
        .default(true)
        .interact()
        .unwrap();

    fn create_user(username: String, password: String, user_admin: bool) -> Result<(), io::Error> {
        // Create user and add to wheel group and set password
        chroot_command(format!("mkdir /home/{0}", username).as_str());
        chroot_command(format!("useradd -m -G wheel {0}", username).as_str());
        chroot_command(format!("chown -R {0}:{0} /home/{0}", username).as_str());
        chroot_command(format!("sudo chpasswd <<< \"{0}:{1}\"", username, password).as_str());
        Ok(())
    }

    fn create_user_no_admin(
        username: String,
        password: String,
        user_admin: bool
    ) -> Result<(), io::Error> {
        // Create user and set password
        chroot_command(format!("mkdir /home/{0}", username).as_str());
        chroot_command(format!("useradd -m {0}", username).as_str());
        chroot_command(format!("chown -R {0}:{0} /home/{0}", username).as_str());
        chroot_command(format!("sudo chpasswd <<< \"{0}:{1}\"", username, password).as_str());
        Ok(())
    }

    fn other_installers(state: &mut InstallerState) -> Result<(), io::Error> {
        if state.selected_profile >= 4 {
            // Install yay installer
            run_command(format!("touch /mnt/usr/bin/install-yay").as_str());
            run_command(
                format!(
                    "cp -r /etc/krushed/arch-installer/usr-config/install-yay.sh /usr/bin/install-yay"
                ).as_str()
            );
            chroot_command(format!("chmod +x /usr/bin/install-yay").as_str());

            // Install krushed zsh config installer
            run_command(format!("touch /usr/bin/install-krushed-zsh").as_str());
            run_command(
                format!(
                    "cp -r /etc/krushed/arch-installer/usr-config/install-krushed-zsh.sh /usr/bin/install-krushed-zsh"
                ).as_str()
            );
            chroot_command(format!("chmod +x /usr/bin/install-krushed-zsh").as_str());
        }
        Ok(())
    }

    if user_admin {
        create_user(username, password, user_admin)?;
    } else {
        create_user_no_admin(username, password, user_admin)?;
    }

    other_installers(state)?;

    // Ask if they want to make another user
    terminal.clear()?;
    let another_user = Confirm::new()
        .with_prompt("Would you like to make another user?")
        .default(false)
        .interact()
        .unwrap();
    if another_user {
        user_creation(state);
    }

    Ok(())
}

fn start_install(state: &mut InstallerState) -> Result<(), io::Error> {
    clear_terminal();
    let chosen_profile;
    let chosen_ucode;
    let chosen_driver;
    // let mut chosen_root_password = state.root_pass;
    // let mut chosen_username = state.username;
    // let mut chosen_user_password = state.user_pass;

    match state.selected_profile {
        1 => {
            chosen_profile = InstallProfile::Base;
            install_profile(chosen_profile);
        }
        2 => {
            chosen_profile = InstallProfile::Minimal;
            install_profile(chosen_profile);
        }
        3 => {
            chosen_profile = InstallProfile::Desktop;
            install_profile(chosen_profile);
        }
        4 => {
            chosen_profile = InstallProfile::FullDesktop;
            install_profile(chosen_profile);
        }
        5 => {
            chosen_profile = InstallProfile::Gaming;
            install_profile(chosen_profile);
        }
        _ => {
            ring_bell();
            println!("A Weird Error Happened And I Didn't Remeber What Profile You Selected...");
            return Ok(());
        }
    }

    match state.selected_ucode {
        1 => {
            chosen_ucode = InstallUcode::Intel;
            install_ucode(chosen_ucode);
        }
        2 => {
            chosen_ucode = InstallUcode::AMD;
            install_ucode(chosen_ucode);
        }
        _ => {
            ring_bell();
            println!("A Weird Error Happened And I Didn't Remeber What UCODE You Selected...");
            sleep(Duration::from_secs("5"));
        }
    }

    match state.selected_driver {
        1 => {
            chosen_driver = InstallDriver::AMD;
            install_driver(chosen_driver);
        }
        2 => {
            chosen_driver = InstallDriver::NVIDIA;
            install_driver(chosen_driver);
        }
        3 => {
            chosen_driver = InstallDriver::INTEL;
            install_driver(chosen_driver);
        }
        4 => {
            chosen_driver = InstallDriver::VMWARE;
            install_driver(chosen_driver);
        }
        5 => {
            chosen_driver = InstallDriver::NONE;
            install_driver(chosen_driver);
        }
        _ => {
            chosen_driver = InstallDriver::NONE;
            install_driver(chosen_driver);
            ring_bell();
            println!("A Weird Error Happened And I Didn't Remeber What Driver You Selected...");
            sleep(Duration::from_secs("5"));
        }
    }
    println!("Generating fstab...");
    run_command("genfstab -U /mnt >> /mnt/etc/fstab");
    chroot_command("ln -s /usr/bin/vim /usr/bin/vi");
    clear_terminal();

    // Installing Grub So If Install Fails Beyond  This Point, You Can Still Boot Into The Install.
    println!("Setting Up Grub...");
    chroot_command(
        format!(
            "grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id={0}-Arch-Linux",
            state.hostname
        ).as_str()
    );
    chroot_command("grub-mkconfig -o /boot/grub/grub.cfg");

    if state.swap_size > 0 {
        println!("Creating Swap File...");
        swap_size_mb = state.swap.size * 1024;
        chroot_command(
            format!("dd if=/dev/zero of=/swapfile bs={0} count=1048576", swap_size_mb).as_str()
        );
        chroot_command("chmod 600 /swapfile");
        chroot_command("mkswap /swapfile");
        chroot_command("swapon /swapfile");
        chroot_command("echo '/swapfile swap swap defaults 0 0' >> /etc/fstab");
    }

    println!("Setting Hostname...");
    //Using shell command because idk how to write to files in rust yet
    chroot_command(format!("echo \"{0}\" > /etc/hostname", state.hostname).as_str());

    println!("Generating Locale...");
    chroot_command("locale-gen");

    println!("Generating initramfs...");
    chroot_command("mkinitcpio -P");

    // User Creation
    clear_terminal();
    ring_bell();
    let new_user_msg = Confirm::new()
        .with_prompt("Would You Like To Create A New User?")
        .default(true)
        .interact()
        .unwrap();

    if new_user_msg {
        user_creation(state);
    }

    chroot_command("echo 'wheel ALL=(ALL:ALL) ALL' | sudo EDITOR='tee -a' visudo");

    chroot_command(format!("sudo chpasswd <<< \"root:{0}\"", state.root_pass).as_str());

    clear_terminal();

    println!("KRUSHED ARCH INSTALLER IS NOW DONE");
    let restart_confirmation = Confirm::new()
        .with_prompt("DO YOU WANT TO RESTART?")
        .default(true)
        .interact()
        .unwrap();

    if !restart_confirmation {
        terminal.clear()?;
        println!("Krushed Arch Installer Complete!");
        return Ok(());
    } else {
        run_command("reboot");
    }

    Ok(())
}

pub fn run_command(command: &str) {
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

pub fn chroot_command(_command: &str) {
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("arch-chroot /mnt {}", _command))
        .output()
        .expect("Failed to execute chroot command");

    if !output.status.success() {
        println!("Command failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}
