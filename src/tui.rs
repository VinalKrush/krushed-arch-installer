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

pub fn new_tui_text(msg: Sting) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    clear_terminal();

    let paragraph = Paragraph::new(msg)
        .block(
            Block::bordered()
                .title("Krushed Arch Linux Installer")
                .style(Style::default().title_alignment(Alignment::Center))
        )
        .bold();

    terminal.draw(|frame| {
        let size = frame.area();
        frame.render_widget(paragraph, size);
    })?;

    Ok(())
}

pub fn clear_terminal() -> io::Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(())
}
