use itertools::Itertools;
use crate::entities::coordinates::Coordinates;
use crate::entities::ticktype::TickType;
use crate::errors::TicTacToeError;
use crate::entities::cell::Cell;

#[derive(Debug)]
pub struct Line {
    cells: Vec<Cell>
}

impl Line {
    pub fn new(size: usize) -> Line {
        let cells : Vec<Cell> = (0..size)
            .map(|_| Cell::new())
            .collect();

        Line{cells}
    }

    pub fn from(cells: Vec<&Cell>) -> Line {
        let vec = cells.into_iter().copied().collect();

        Line {cells: vec }
    }

    pub fn compute_representation(&self) -> String {
        let cells_representations : Vec<String> = self.cells.iter()
            .map(|cell| cell.compute_representation())
            .collect();

        cells_representations.join("")
    }

    pub fn tick(&mut self, coordinates: &Coordinates, tick_type: &TickType) -> Result<(), TicTacToeError> {
        if !self.is_cell_empty(coordinates.x)? {
            return Err(TicTacToeError::CellNotEmpty)
        }

        self.cells.remove(coordinates.x);
        self.cells.insert(coordinates.x, Cell::from(tick_type));

        Ok(())
    }

    fn is_cell_empty(&self, index: usize) -> Result<bool, TicTacToeError> {
        Ok(self.retrieve_cell_at(index)?.is_empty())
    }

    pub fn retrieve_cell_at(&self, index: usize) -> Result<&Cell, TicTacToeError> {
        self.cells.get(index).ok_or(TicTacToeError::CoordinateOutOfScope)
    }

    pub fn is_won(&self) -> bool {
        if !self.is_full() {
            return false;
        }

        let unique_cells = self.cells.iter()
            .unique()
            .count();

        return unique_cells == 1
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().all(Cell::is_full)
    }
}