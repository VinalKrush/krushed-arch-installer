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
use crossterm::{ Command, execute };

pub fn new_tui_text(msg: String) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    clear_terminal();

    let paragraph = Paragraph::new(msg)
        .block(Block::bordered().title("Krushed Arch Linux Installer").style(Style::default()))
        .bold();

    terminal.draw(|frame| {
        let size = frame.area();
        frame.render_widget(paragraph, size);
    })?;

    Ok(())
}

// pub fn clear_terminal() {
//     let backend = CrosstermBackend::new(stdout());
//     let mut terminal = Terminal::new(backend);
//     terminal.clear();
// }

pub fn clear_terminal() -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    let _ = execute!(std::io::stdout(), EnterAlternateScreen, disable_raw_mode());
    println!("\x1B[2J\x1B[1;1H"); // clear screen and move cursor to top left
    Ok(())
}
