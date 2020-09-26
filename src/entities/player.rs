use crate::entities::ticktype::TickType;
use crate::entities::coordinates::Coordinates;
use crate::entities::grid::Grid;

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

    pub fn play(&self, coordinates: &Coordinates, grid: &mut Grid) {
        grid.tick(&coordinates, &self.tick_type)
    }
}