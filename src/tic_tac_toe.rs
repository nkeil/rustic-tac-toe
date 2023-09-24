use std::fmt::Display;

use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

#[derive(Clone, Copy)]
enum Player {
    X,
    O,
}

/// A board value is either claimed by a player, or unclaimed
#[derive(Clone, Copy)]
enum BoardValue {
    None,
    Some(Player),
}

/// Defines how a board value is displayed in the terminal UI
impl Display for BoardValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value_str = match self {
            Self::None => " ",
            Self::Some(Player::X) => "X",
            Self::Some(Player::O) => "O",
        };
        write!(f, "{}", value_str)
    }
}

#[derive(Clone, Copy)]
pub struct TicTacToe {
    board: [[BoardValue; 3]; 3],
    selected: (u8, u8),
    turn: Player,
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        Self {
            board: [[BoardValue::None; 3]; 3],
            selected: (0, 0),
            turn: Player::X,
        }
    }
    pub fn move_left(&mut self) {
        let (x, y) = self.selected;
        self.selected = (if x > 0 { x - 1 } else { 0 }, y)
    }
    pub fn move_right(&mut self) {
        let (x, y) = self.selected;
        self.selected = ((x + 1).min(2), y)
    }
    pub fn move_up(&mut self) {
        let (x, y) = self.selected;
        self.selected = (x, if y > 0 { y - 1 } else { 0 })
    }
    pub fn move_down(&mut self) {
        let (x, y) = self.selected;
        self.selected = (x, (y + 1).min(2))
    }
    pub fn select_square(&mut self) {
        let (x, y) = self.selected;
        let x = x as usize;
        let y = y as usize;
        if let BoardValue::Some(_) = self.board[y][x] {
            // Can't select a square that has already been selected
            return;
        }
        self.board[y][x] = BoardValue::Some(self.turn);
        self.turn = match self.turn {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl Widget for TicTacToe {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Display board
        #[rustfmt::skip]
        let [
          [a1, a2, a3], 
          [b1, b2, b3], 
          [c1, c2, c3]
        ] = self.board;
        let lines = [
            format!(" {} | {} | {}", a1, a2, a3),
            format!("———|———|———"),
            format!(" {} | {} | {}", b1, b2, b3),
            format!("———|———|———"),
            format!(" {} | {} | {}", c1, c2, c3),
        ];
        for (i, line) in lines.into_iter().enumerate() {
            buf.set_line(
                area.x,
                area.y + i as u16,
                &Line::styled(line, Style::default()),
                area.width,
            );
        }

        // Highlight selected square
        let selected_x = area.x + 1 + (self.selected.0 as u16) * 4;
        let selected_y = area.y + (self.selected.1 as u16) * 2;
        let val = buf.get(selected_x, selected_y).symbol.clone();
        buf.set_string(
            selected_x - 1,
            selected_y,
            format!(" {} ", val),
            Style::default().bg(Color::Gray).fg(Color::Black),
        );
    }
}
