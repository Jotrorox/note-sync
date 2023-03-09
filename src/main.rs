use std::{io, thread, time::Duration};
use crossterm::{
    terminal::{
        enable_raw_mode, 
        EnterAlternateScreen, 
        LeaveAlternateScreen, 
        disable_raw_mode
    }, 
    execute, 
    event::{
        EnableMouseCapture, 
        DisableMouseCapture
    }
};
use tui::{
    backend::CrosstermBackend, 
    Terminal, 
    widgets::{
        Block, 
        Borders
    }
};

fn main() -> Result<(), io::Error>{
    // Setup the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Draw the UI
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Note Sync")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_secs(5));

    // restore the terminal
    terminal.show_cursor()?;
    execute!(
        terminal.backend_mut(), 
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;

    Ok(())
}
