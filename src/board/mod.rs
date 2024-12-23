use std::fmt::Display;

use crate::board_cell::BoardCell;

pub struct Board(pub [[BoardCell; 7]; 6]);

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for item in row {
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
