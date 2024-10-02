mod main;

use main::run_command;
use main::chroot_command;

use ratatui::{
    buffer::Buffer,
    backend::CrosstermBackend,
    prelude::Alignment,
    crossterm::event::{ self, Event, KeyCode, KeyEventKind },
    layout::{ Constraint, Layout, Rect, Position },
    style::{ Color, Modifier, Stylize, Style },
    text::{ Line, Masked, Span },
    widgets::{ Block, Paragraph, Widget, Wrap, List, ListItem },
    Frame,
    DefaultTerminal,
    Terminal,
};
use std::io::{ self, stdout };

pub fn new_text(msg: Sting, title: String, fg_color: String) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let paragraph = Paragraph::new(msg)
        .block(Block::bordered().style(Style::default().fg(fg_color)))
        .bold()
        .centered();

    terminal.draw(|frame| {
        let size = frame.area();
        frame.render_widget(paragraph, size);
    })?;

    Ok(())
}
