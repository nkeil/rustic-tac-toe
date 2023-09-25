use std::{io, time::Duration};
mod terminal;
mod tic_tac_toe;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};
use terminal::{start_terminal, stop_terminal};
use tic_tac_toe::TicTacToe;

fn main() -> io::Result<()> {
    let mut terminal = start_terminal()?;
    let mut tic_tac_toe = TicTacToe::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Tic-Tac-Toe").borders(Borders::ALL);
            let instructions = Paragraph::new(Vec::from(tic_tac_toe::INSTRUCTIONS.map(Line::from)));
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
            f.render_widget(instructions, Rect::new(3, 2, 30, 5));
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

    stop_terminal(terminal)?;
    Ok(())
}
