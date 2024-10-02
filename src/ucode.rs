use main::run_command;
use main::chroot_command;
use tui::{ new_tui_text, clear_terminal };

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
use std::io::{ self, stdout };

pub enum InstallUcode {
    Intel,
    AMD,
}

// fn run_command(command: &str) {
//     use std::process::Command;
//     let output = Command::new("sh")
//         .arg("-c")
//         .arg(command)
//         .output()
//         .expect("Failed to execute command");

//     if !output.status.success() {
//         println!("Command failed: {}", String::from_utf8_lossy(&output.stderr));
//     }
// }

pub fn install_ucode(ucode: InstallUcode) {
    match ucode {
        InstallUcode::Intel => {
            // Intel Ucode Install
            let text = Text::from(vec![Line::from("Downloading INTEL UCODE...")])
                .blue()
                .centered();
            new_tui_text(text);
            run_command("pacstrap -K -P /mnt intel-ucode");
        }
        InstallUcode::AMD => {
            // AMD Ucode Install
            let text = Text::from(vec![Line::from("Downloading AMD UCODE...")])
                .red()
                .centered();
            new_tui_text(text);
            run_command("pacstrap -K -P /mnt amd-ucode");
        }
    }
}
