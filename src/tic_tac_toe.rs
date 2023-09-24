use std::fmt::Display;

use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

#[derive(Clone, Copy)]
enum BoardValue {
    None,
    X,
    O,
}

impl Display for BoardValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value_str = match self {
            Self::None => " ",
            Self::X => "X",
            Self::O => "O",
        };
        write!(f, "{}", value_str)
    }
}

#[derive(Clone, Copy)]
pub struct TicTacToe {
    board: [[BoardValue; 3]; 3],
    selected: Option<(u8, u8)>,
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        Self {
            board: [[BoardValue::None; 3]; 3],
            selected: None,
        }
    }
    pub fn move_left(&mut self) {
        self.selected = match self.selected {
            None => Some((0, 0)),
            Some((x, y)) => Some((if x > 0 { x - 1 } else { 0 }, y)),
        }
    }
    pub fn move_right(&mut self) {
        self.selected = match self.selected {
            None => Some((1, 0)),
            Some((x, y)) => Some(((x + 1).min(2), y)),
        }
    }
    pub fn move_up(&mut self) {
        self.selected = match self.selected {
            None => Some((0, 0)),
            Some((x, y)) => Some((x, if y > 0 { y - 1 } else { 0 })),
        }
    }
    pub fn move_down(&mut self) {
        self.selected = match self.selected {
            None => Some((0, 1)),
            Some((x, y)) => Some((x, (y + 1).min(2))),
        }
    }
}

impl Widget for TicTacToe {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [[a1, a2, a3], [b1, b2, b3], [c1, c2, c3]] = self.board;
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
        if let Some((x, y)) = self.selected {
            let selected_x = area.x + 1 + (x as u16) * 4;
            let selected_y = area.y + (y as u16) * 2;
            let val = buf.get(selected_x, selected_y).symbol.clone();
            buf.set_string(
                selected_x - 1,
                selected_y,
                format!(" {} ", val),
                Style::default().bg(Color::Gray).fg(Color::Black),
            );
        }
    }
}
