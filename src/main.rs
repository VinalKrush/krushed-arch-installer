/* 
PLEASE READ!

BEFORE JUDGING THIS CODE PLEASE KNOW I AM NEW TO RUST 
THIS WILL NOT BE THE BEST AND I KNOW IT CAN BE BETTER ;-;

Also I'm using ListItem because idk how to use paragraph in ratatui
*/


//mod profiles;

//use profiles::{InstallProfile, install_profile};
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::widgets::{List, ListItem, Block, Borders};
use dialoguer::{Password, Input, Confirm};
use std::io::{self, stdout};
//use std::thread;

struct InstallerState {
    selected_profile: i32,
    selected_ucode: i32,
    selected_driver: i32,
    root_pass: String,
    username: String,
    user_pass: String,
}

/* fn run_command(command: &str) {
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
    */

fn main() -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut state = InstallerState { selected_profile: 0, selected_ucode: 0, selected_driver: 0, root_pass: "".to_string(), username: "".to_string(), user_pass: "".to_string(), };
    terminal.clear()?;

    println!("Hello, Welcome To The Krushed Arch Linux Installer (Made In Rust)");
    println!("");
    println!("");
    let driveconfirmation = Confirm::new()
        .with_prompt("Have You Set Up Your Drives And Mounted Them To \"/mnt\"?")
        .default(true)
        .interact()
        .unwrap();

    if !driveconfirmation {
        terminal.clear()?;
        println!(" Install... \nPlease set up your drives...");
        return Ok(())
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
            ListItem::new("1. Base          \n(Pretty Much Nothing Is Installed.)\n "),
            ListItem::new("2. Minimal       \n(Best For Servers.)\n "),
            ListItem::new("3. Desktop       \n(Minimal Desktop Env.)\n "),
            ListItem::new("4. Full Desktop  \n(Has Apps Like VLC, GPARTED, LIBREOFFICE, and CODE.)\n "),
            ListItem::new("5. Gaming        \n(Pre Installed Wine Staging , Steam, Lutris, And Other Gaming Packages.)\n "),
        ];

        let profile_list = List::new(profile_choices_msg)
            .block(Block::default().borders(Borders::ALL));

        terminal.draw(|frame| {
            let size = frame.area();
            frame.render_widget(profile_list, size);
        })?;

        let selected_profile = Input::<i32>::new()
            .interact_text()
            .unwrap();
        terminal.clear()?;

        if selected_profile >= 1 && selected_profile <= 5 {
            println!("{selected_profile}");
            state.selected_profile = selected_profile;
            ucode_selector(state)?;

            Ok(())
        } else {
            println!("Invalid Profile Choice, Please Enter A Number Between 1 - 5");
            return Ok(())
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
        ListItem::new("2. AMD"),
    ];

    let ucode_list = List::new(ucode_choices_msg)
        .block(Block::default().borders(Borders::ALL));

    terminal.draw(|frame| {
        let size = frame.area();
        frame.render_widget(ucode_list, size);
    })?;

    let selected_ucode = Input::<i32>::new()
        .interact_text()
        .unwrap();
    terminal.clear()?;

    if selected_ucode >= 1 && selected_ucode <= 2 {
        state.selected_ucode = selected_ucode;
        driver_selector(state)?;

        Ok(())
    } else {
        println!("Invalid CPU Choice.");
        return Ok(())
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
    ];

    let driver_list = List::new(driver_choices_msg)
        .block(Block::default().borders(Borders::ALL));

    terminal.draw(|frame| {
        let size = frame.area();
        frame.render_widget(driver_list, size);
    })?;

    let selected_driver = Input::<i32>::new()
        .interact_text()
        .unwrap();
    terminal.clear()?;

    if selected_driver >= 1 && selected_driver <= 4 {
        state.selected_driver = selected_driver;
        root_password(state)
    } else {
        println!("Invalid CPU Choice.");
        return Ok(())
    }
}

fn root_password(state: &mut InstallerState) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // UCODE SELECT
    let root_choices_msg = vec![
        ListItem::new("Please Set A Root Password\n\n "),
    ];

    let root_list = List::new(root_choices_msg)
        .block(Block::default().borders(Borders::ALL));

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
    user_name(state)?;

    Ok(())
}

fn user_name(state: &mut InstallerState) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // UCODE SELECT
    let user_choices_msg = vec![
        ListItem::new("Please Type A Username\n\n "),
        ListItem::new("Note This User Will Be Added To The \"Wheel\" Group"),
    ];

    let user_list = List::new(user_choices_msg)
        .block(Block::default().borders(Borders::ALL));

    terminal.draw(|frame| {
        let size = frame.area();
        frame.render_widget(user_list, size);
    })?;

    let user_na = Input::new()
        .interact()
        .unwrap();
    terminal.clear()?;

    state.username = user_na;
    user_password(state)?;

    Ok(())
}

fn user_password(state: &mut InstallerState) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // UCODE SELECT
    let user_choices_msg = vec![
        ListItem::new("Please Set A User Password"),
    ];

    let user_list = List::new(user_choices_msg)
        .block(Block::default().borders(Borders::ALL));

    terminal.draw(|frame| {
        let size = frame.area();
        frame.render_widget(user_list, size);
    })?;

    let user_pass = Password::new()
        .with_confirmation("Confirm Password", "Passwords Do Not Match")
        .interact()
        .unwrap();
    terminal.clear()?;

    state.user_pass = user_pass;
    println!("UCODE: {0}", state.selected_ucode);
    println!("PROFILE: {0}", state.selected_profile);
    println!("DRIVER: {0}", state.selected_driver);
    println!("ROOT PASSWORD: {}", state.root_pass);
    println!("USERNAME: {}", state.username);
    println!("USER PASS: {}", state.user_pass);

    Ok(())
}