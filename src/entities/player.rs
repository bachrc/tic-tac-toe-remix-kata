use crate::entities::ticktype::TickType;
use crate::entities::coordinates::Coordinates;
use crate::entities::grid::Grid;
use crate::errors::TicTacToeError;

#[derive(Debug, PartialEq)]
pub struct Player {
    name: String,
    tick_type: TickType
}

impl Player {
    pub fn new(name: &str, tick_type: TickType) -> Player {
        Player {
            name: String::from(name), tick_type
        }
    }

    pub fn play(&self, coordinates: &Coordinates, grid: &mut Grid) -> Result<(), TicTacToeError> {
        grid.tick(&coordinates, &self.tick_type)
    }
}