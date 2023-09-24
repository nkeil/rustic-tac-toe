use std::{io, time::Duration};
mod tic_tac_toe;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Rect},
    widgets::{Block, Borders},
    Terminal,
};
use tic_tac_toe::*;

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
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }

                if KeyCode::Left == key.code {
                    tic_tac_toe.selected = match tic_tac_toe.selected {
                        None => Some((0, 0)),
                        Some((x, y)) => Some((if x > 0 { x - 1 } else { 0 }, y)),
                    }
                } else if KeyCode::Right == key.code {
                    tic_tac_toe.selected = match tic_tac_toe.selected {
                        None => Some((1, 0)),
                        Some((x, y)) => Some(((x + 1).min(2), y)),
                    }
                } else if KeyCode::Up == key.code {
                    tic_tac_toe.selected = match tic_tac_toe.selected {
                        None => Some((0, 0)),
                        Some((x, y)) => Some((x, if y > 0 { y - 1 } else { 0 })),
                    }
                } else if KeyCode::Down == key.code {
                    tic_tac_toe.selected = match tic_tac_toe.selected {
                        None => Some((0, 1)),
                        Some((x, y)) => Some((x, (y + 1).min(2))),
                    }
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
