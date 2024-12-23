use std::fmt::Display;

use colored::Colorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerToken {
    Player1,
    Player2,
}

impl Display for PlayerToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerToken::Player1 => write!(f, "{}", "●".red()),
            PlayerToken::Player2 => write!(f, "{}", "●".blue()),
        }
    }
}
