use crate::tui::{ new_tui_text, clear_terminal };

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

pub enum InstallDriver {
    AMD,
    NVIDIA,
    INTEL,
    VMWARE,
    NONE,
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

pub fn install_driver(drivers: InstallDriver) {
    match drivers {
        InstallDriver::AMD => {
            // AMD Drivers Install
            let text = Text::from(
                vec![
                    Line::from("Downloading AMD Drivers..."),
                    Line::from("This May Take Awhile...")
                ]
            )
                .red()
                .centered();
            new_tui_text(text.to_string());
            run_command(
                "pacstrap -K -P /mnt mesa lib32-mesa xf86-video-amdgpu vulkan-radeon lib32-vulkan-radeon libva-mesa-driver lib32-libva-mesa-driver mesa-vdpau"
            );
        }
        InstallDriver::NVIDIA => {
            // NVIDIA Drivers Install
            let text = Text::from(
                vec![
                    Line::from("Downloading NVIDIA Drivers..."),
                    Line::from("This May Take Awhile...")
                ]
            )
                .green()
                .centered();
            new_tui_text(text.to_string());
            run_command(
                "pacstrap -K -P /mnt nvidia-dkms nvidia-utils lib32-nvidia-utils libva-mesa-driver mesa-vdpau libva-nvidia-driver"
            );
            chroot_command("systemctl enable nvidia-resume.service");
        }
        InstallDriver::INTEL => {
            // INTEL Drivers Install
            let text = Text::from(
                vec![
                    Line::from("Downloading INTEL Drivers..."),
                    Line::from("This May Take Awhile...")
                ]
            )
                .blue()
                .centered();
            new_tui_text(text.to_string());
            run_command(
                "pacstrap -K -P /mnt mesa lib32-mesa xf86-video-intel vulkan-intel lib32-vulkan-intel intel-media-driver libva-intel-driver"
            );
        }
        InstallDriver::VMWARE => {
            // VMWARE Drivers Install
            let text = Text::from(
                vec![
                    Line::from("Downloading VMWARE Drivers..."),
                    Line::from("This May Take Awhile...")
                ]
            )
                .light_yellow()
                .centered();
            new_tui_text(text.to_string());
            run_command("pacstrap -K -P /mnt open-vm-tools net-tools devtools");
            chroot_command("systemctl enable vmtoolsd.service vmware-vmblock-fuse.service");
        }
        InstallDriver::NONE => {
            // No Drivers.
            let text = Text::from(
                vec![
                    Line::from("NO GPU DRIVER SELECTED..."),
                    Line::from("CONTINUING IN 5 SECONDS...")
                ]
            )
                .red()
                .centered();
            new_tui_text(text.to_string());
        }
    }
}
