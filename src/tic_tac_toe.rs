use std::fmt::Display;

use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    X,
    O,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if matches!(self, Self::X) { "X" } else { "O" })
    }
}

/// A board value is either claimed by a player, or unclaimed
#[derive(Clone, Copy)]
enum BoardValue {
    None,
    Some(Player),
}

#[derive(Clone, Copy)]
pub enum GameStatus {
    Tie,
    Victory(Player),
    Incomplete,
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
    pub status: GameStatus,
    board: [[BoardValue; 3]; 3],
    selected: (u8, u8),
    turn: Player,
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        Self {
            status: GameStatus::Incomplete,
            board: [[BoardValue::None; 3]; 3],
            selected: (0, 0),
            turn: Player::X,
        }
    }
    pub fn restart(&mut self) {
        self.status = GameStatus::Incomplete;
        self.board = [[BoardValue::None; 3]; 3];
        self.selected = (0, 0);
        self.turn = Player::X;
    }
    pub fn move_left(&mut self) {
        let (x, y) = self.selected;
        self.selected = (if x > 0 { x - 1 } else { 0 }, y);
    }
    pub fn move_right(&mut self) {
        let (x, y) = self.selected;
        self.selected = ((x + 1).min(2), y);
    }
    pub fn move_up(&mut self) {
        let (x, y) = self.selected;
        self.selected = (x, if y > 0 { y - 1 } else { 0 });
    }
    pub fn move_down(&mut self) {
        let (x, y) = self.selected;
        self.selected = (x, (y + 1).min(2));
    }
    pub fn select_square(&mut self) {
        if !matches!(self.status, GameStatus::Incomplete) {
            return;
        }
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
        };
        self.status = self.check_game_status();
    }
    fn check_game_status(&mut self) -> GameStatus {
        let victory_lines = [
            [(0, 0), (1, 0), (2, 0)], // horizontal top
            [(0, 1), (1, 1), (2, 1)], // horizontal middle
            [(0, 2), (1, 2), (2, 2)], // horizontal bottom
            [(0, 0), (0, 1), (0, 2)], // vertical left
            [(1, 0), (1, 1), (1, 2)], // vertical middle
            [(2, 0), (2, 1), (2, 2)], // vertical right
            [(0, 0), (1, 1), (2, 2)], // diagonal top-left to bottom-right
            [(2, 0), (1, 1), (0, 2)], // diagonal top-right to bottom-left
        ];

        // Check for player victory
        for victory_line in victory_lines {
            let point1 = self.board[victory_line[0].1][victory_line[0].0];
            let point2 = self.board[victory_line[1].1][victory_line[1].0];
            let point3 = self.board[victory_line[2].1][victory_line[2].0];
            if let (BoardValue::Some(point1), BoardValue::Some(point2), BoardValue::Some(point3)) =
                (point1, point2, point3)
            {
                if point1 == point2 && point2 == point3 {
                    return GameStatus::Victory(point1);
                }
            }
        }

        // Check if all spaces have been claimed (tie)
        if self
            .board
            .iter()
            .all(|row| row.iter().all(|cell| matches!(cell, BoardValue::Some(_))))
        {
            return GameStatus::Tie;
        }

        return GameStatus::Incomplete;
    }
}

impl Widget for TicTacToe {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Display board
        let [[a1, a2, a3], [b1, b2, b3], [c1, c2, c3]] = self.board;
        let status_string = match self.status {
            GameStatus::Incomplete => format!("{}'s turn", self.turn),
            GameStatus::Tie => String::from("It's a tie!"),
            GameStatus::Victory(p) => format!("{} has won!", p),
        };
        let lines = [
            format!(" {} | {} | {}", a1, a2, a3),
            format!("———|———|———"),
            format!(" {} | {} | {}", b1, b2, b3),
            format!("———|———|———"),
            format!(" {} | {} | {}", c1, c2, c3),
            status_string,
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
