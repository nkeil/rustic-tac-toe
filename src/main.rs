use std::{io, time::Duration};
mod tic_tac_toe;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Rect},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use tic_tac_toe::TicTacToe;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut tic_tac_toe = TicTacToe::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Tic-Tac-Toe").borders(Borders::ALL);
            let instructions = Paragraph::new("Press q to quit\nPress r to restart");
            f.render_widget(block, size);
            f.render_widget(
                tic_tac_toe,
                Rect::new(
                    size.x + size.width / 2 - 6,
                    size.y + size.height / 2 - 3,
                    11,
                    5,
                ),
            );
            f.render_widget(instructions, Rect::new(3, 2, 20, 5));
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => tic_tac_toe.restart(),
                    KeyCode::Left => tic_tac_toe.move_left(),
                    KeyCode::Right => tic_tac_toe.move_right(),
                    KeyCode::Up => tic_tac_toe.move_up(),
                    KeyCode::Down => tic_tac_toe.move_down(),
                    KeyCode::Char(' ') => tic_tac_toe.select_square(),
                    _ => {}
                }
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
