use std::io;
use tui::{backend::TermionBackend, Terminal};
use termion::raw::IntoRawMode;

pub fn run() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    Ok(())
}