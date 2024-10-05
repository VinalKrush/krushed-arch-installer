use ratatui::{
    backend::CrosstermBackend,
    prelude::Alignment,
    style::{ Stylize },
    widgets::{ Block, Paragraph },
    Terminal,
};
use std::io::{ self, stdout };

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

pub fn clear_terminal() {
    println!("\x1B[2J\x1B[1;1H")
}
