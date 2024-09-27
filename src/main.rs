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

use gtk::ffi::gtk_recent_info_get_added;
use profiles::{ InstallProfile, install_profile };
use ucode::{ InstallUcode, install_ucode };
use drivers::{ InstallDriver, install_driver };
use bell::ring_bell;
use ratatui::{ backend::CrosstermBackend, Terminal };
use ratatui::widgets::{ List, ListItem, Block, Borders };
use dialoguer::{ Password, Input, Confirm };
use std::io::{ self, stdout };
struct InstallerState {
    selected_profile: i32,
    selected_ucode: i32,
    selected_driver: i32,
    root_pass: String,
    hostname: String,
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

fn chroot_command(_command: &str) {
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

fn main() -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut state = InstallerState {
        selected_profile: 0,
        selected_ucode: 0,
        selected_driver: 0,
        root_pass: "".to_string(),
        hostname: "".to_string(),
    };
    terminal.clear()?;

    println!("Hello, Welcome To The Krushed Arch Linux Installer");
    println!("");
    println!("");
    let driveconfirmation = Confirm::new()
        .with_prompt("Have You Set Up Your Drives And Mounted Them To \"/mnt\"?")
        .default(true)
        .interact()
        .unwrap();

    if !driveconfirmation {
        terminal.clear()?;
        println!("Cancelled Install... \nPlease set up your drives...");
        ring_bell();
        return Ok(());
    } else {
        profile_selector(&mut state)?;
    }
    Ok(())
}

fn profile_selector(state: &mut InstallerState) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    // PROFILE SELECT

    let profile_choices_msg = vec![
        ListItem::new("Please Type A Number To Select A Profile\n\n "),
        ListItem::new("1. Base"),
        ListItem::new("(Pretty Much Nothing Is Installed)"),
        ListItem::new(""),
        ListItem::new("2. Minimal"),
        ListItem::new("(Best For Servers)"),
        ListItem::new(""),
        ListItem::new("3. Desktop"),
        ListItem::new("(Minimal Desktop Env)"),
        ListItem::new(""),
        ListItem::new("4. Full Desktop"),
        ListItem::new("(Has Apps Like VLC, GPARTED, LIBREOFFICE, and CODE.)"),
        ListItem::new(""),
        ListItem::new("5. Gaming"),
        ListItem::new("(Pre Installed Wine Staging , Steam, Lutris, And Other Gaming Packages.)"),
        ListItem::new("")
    ];

    let profile_list = List::new(profile_choices_msg).block(Block::default().borders(Borders::ALL));

    terminal.draw(|frame| {
        let size = frame.area();
        frame.render_widget(profile_list, size);
    })?;

    let selected_profile = Input::<i32>::new().interact_text().unwrap();
    terminal.clear()?;

    if selected_profile >= 1 && selected_profile <= 5 {
        println!("{selected_profile}");
        state.selected_profile = selected_profile;
        ucode_selector(state)?;

        Ok(())
    } else {
        println!("Invalid Profile Choice, Please Enter A Number Between 1 - 5");
        ring_bell();
        return Ok(());
    }
}

fn ucode_selector(state: &mut InstallerState) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // UCODE SELECT
    let ucode_choices_msg = vec![
        ListItem::new("Please Type A Number To Select A CPU Platform\n\n "),
        ListItem::new("1. INTEL"),
        ListItem::new("2. AMD")
    ];

    let ucode_list = List::new(ucode_choices_msg).block(Block::default().borders(Borders::ALL));

    terminal.draw(|frame| {
        let size = frame.area();
        frame.render_widget(ucode_list, size);
    })?;

    let selected_ucode = Input::<i32>::new().interact_text().unwrap();
    terminal.clear()?;

    if selected_ucode >= 1 && selected_ucode <= 2 {
        state.selected_ucode = selected_ucode;
        driver_selector(state)?;

        Ok(())
    } else {
        println!("Invalid CPU Choice.");
        ring_bell();
        return Ok(());
    }
}

fn driver_selector(state: &mut InstallerState) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // UCODE SELECT
    let driver_choices_msg = vec![
        ListItem::new("Please Type A Number To Select A GPU Driver\n\n "),
        ListItem::new("1. AMD"),
        ListItem::new("2. NVIDIA"),
        ListItem::new("3. INTEL"),
        ListItem::new("4. VMWARE"),
        ListItem::new("5. No Driver")
    ];

    let driver_list = List::new(driver_choices_msg).block(Block::default().borders(Borders::ALL));

    terminal.draw(|frame| {
        let size = frame.area();
        frame.render_widget(driver_list, size);
    })?;

    let selected_driver = Input::<i32>::new().interact_text().unwrap();
    terminal.clear()?;

    if selected_driver >= 1 && selected_driver <= 5 {
        state.selected_driver = selected_driver;
        root_password(state)
        // root_password(state)
    } else {
        println!("Invalid Driver Choice.");
        ring_bell();
        return Ok(());
    }
}

fn root_password(state: &mut InstallerState) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // UCODE SELECT
    let root_choices_msg = vec![ListItem::new("Please Set A Root Password\n\n ")];

    let root_list = List::new(root_choices_msg).block(Block::default().borders(Borders::ALL));

    terminal.draw(|frame| {
        let size = frame.area();
        frame.render_widget(root_list, size);
    })?;

    let root_pass = Password::new()
        .with_confirmation("Confirm Password", "Passwords Do Not Match")
        .interact()
        .unwrap();
    terminal.clear()?;

    state.root_pass = root_pass;
    host_name(state)?;

    Ok(())
}

fn host_name(state: &mut InstallerState) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // UCODE SELECT
    let host_name_msg = vec![ListItem::new("Please Type A Hostname\n\n ")];

    let host_list = List::new(host_name_msg).block(Block::default().borders(Borders::ALL));

    terminal.draw(|frame| {
        let size = frame.area();
        frame.render_widget(host_list, size);
    })?;

    let host_na = Input::new().interact().unwrap();
    terminal.clear()?;

    state.hostname = host_na;
    install_confirm(state)?;

    Ok(())
}

fn install_confirm(state: &mut InstallerState) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let install_confirmation = Confirm::new()
        .with_prompt("Are you sure you want to continue with this install?")
        .default(true)
        .interact()
        .unwrap();

    if !install_confirmation {
        terminal.clear()?;
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

    fn create_user(username: String, password: String, admin: bool) -> Result<(), io::Error> {
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
        admin: bool
    ) -> Result<(), io::Error> {
        // Create user and set password
        chroot_command(format!("mkdir /home/{0}", username).as_str());
        chroot_command(format!("useradd -m {0}", username).as_str());
        chroot_command(format!("chown -R {0}:{0} /home/{0}", username).as_str());
        chroot_command(format!("sudo chpasswd <<< \"{0}:{1}\"", username, password).as_str());
        Ok(())
    }

    fn other_installers(username: String) -> Result<(), io::Error> {
        // Install yay installer
        run_command(format!("touch /mnt/home/{0}/install-yay.sh", username).as_str());
        run_command(
            format!(
                "cp -r /etc/krushed/arch-installer/usr-config/install-yay.sh /mnt/home/{0}/install-yay.sh",
                username
            ).as_str()
        );
        chroot_command(format!("chmod +x /home/{0}/install-yay.sh", username).as_str());

        // Install krushed zsh config installer
        run_command(format!("touch /mnt/home/{0}/install-krushed-zsh.sh", username).as_str());
        run_command(
            format!(
                "cp -r /etc/krushed/arch-installer/usr-config/install-krushed-zsh.sh /mnt/home/{0}/install-krushed-zsh.sh",
                username
            ).as_str()
        );
        chroot_command(format!("chmod +x /home/{0}/install-krushed-zsh.sh", username).as_str());
        Ok(())
    }

    if user_admin {
        create_user(username, password, user_admin)?;
    } else {
        create_user_no_admin(username, password, user_admin)?;
    }

    if state.selected_profile >= 4 {
        other_installers(state)?;
    }

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
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
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
            ring_bell();
            println!("A Weird Error Happened And I Didn't Remeber What Driver You Selected...");
        }
    }

    terminal.clear()?;

    println!("SETTING UP SYSTEM");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    chroot_command("ln -s /usr/bin/vim /usr/bin/vi");

    // Installing Grub So If Install Fails Beyond  This Point, You Can Still Boot Into The Install.
    println!("Setting Up Grub...");
    chroot_command(
        format!(
            "grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id={0}-Arch-Linux",
            state.hostname
        ).as_str()
    );
    chroot_command("grub-mkconfig -o /boot/grub/grub.cfg");

    println!("Setting Hostname...");
    //Using shell command because idk how to write to files in rust yet
    chroot_command(format!("echo \"{0}\" > /etc/hostname", state.hostname).as_str());

    println!("Generating Locale...");
    chroot_command("locale-gen");

    println!("Generating initramfs...");
    chroot_command("mkinitcpio -P");

    // User Creation
    terminal.clear()?;
    ring_bell();
    ring_bell();
    ring_bell();
    let new_user_msg = Confirm::new()
        .with_prompt("Would You Like To Create A New User?")
        .default(true)
        .interact()
        .unwrap();

    if new_user_msg {
        user_creation(state);
    }

    chroot_command(format!("sudo chpasswd <<< \"root:{0}\"", state.root_pass).as_str());

    terminal.clear()?;

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
