use std::fmt::Display;

use colored::Colorize;

use crate::player_token::PlayerToken;

pub enum BoardCell {
    Empty,
    Placed(PlayerToken),
}

impl Display for BoardCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardCell::Empty => write!(f, "{}", ". ".bright_black()),
            BoardCell::Placed(token) => write!(f, "{token} "),
        }
    }
}
