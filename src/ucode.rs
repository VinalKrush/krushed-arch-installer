use crate::tui::{ new_tui_text };

use ratatui::{
    style::{ Stylize },
    text::{ Line, Text },
};

pub enum InstallUcode {
    Intel,
    AMD,
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

pub fn install_ucode(ucode: InstallUcode) {
    match ucode {
        InstallUcode::Intel => {
            // Intel Ucode Install
            let text = Text::from(vec![Line::from("Downloading INTEL UCODE...")])
                .blue()
                .centered();
            let _ = new_tui_text(text.to_string());
            run_command("pacstrap -K -P /mnt intel-ucode");
        }
        InstallUcode::AMD => {
            // AMD Ucode Install
            let text = Text::from(vec![Line::from("Downloading AMD UCODE...")])
                .red()
                .centered();
            let _ = new_tui_text(text.to_string());
            run_command("pacstrap -K -P /mnt amd-ucode");
        }
    }
}
