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

use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode,
    enable_raw_mode,
    EnterAlternateScreen,
    LeaveAlternateScreen,
};

pub fn new_tui_text(msg: String) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    clear_terminal();

    let paragraph = Paragraph::new(msg)
        .block(Block::bordered().title("Krushed Arch Linux Installer").magenta().on_black())
        .alignment(Alignment::Center)
        .bold();

    terminal.draw(|frame| {
        let size = frame.area();
        frame.render_widget(paragraph, size);
    })?;

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

pub fn clear_terminal() {
    println!("\x1B[2J\x1B[1;1H")
}
