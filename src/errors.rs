use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TicTacToeError {
    CellNotEmpty,
    CoordinateOutOfScope
}

impl TicTacToeError {
    pub fn error_message(&self) -> &str{
        match self {
            TicTacToeError::CellNotEmpty => "La case n'est pas vide !",
            TicTacToeError::CoordinateOutOfScope => "La coordonnée est hors de portée"
        }
    }
}

impl fmt::Display for TicTacToeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TicTacToe Error : {}", self.error_message())
    }
}
