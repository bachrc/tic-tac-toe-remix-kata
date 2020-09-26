use crate::entities::ticktype::TickType;
use crate::entities::coordinates::Coordinates;

pub struct Grid {
    size: u32,
    lines: Vec<Line>
}

impl Grid {
    pub fn new(size: u32) -> Grid {
        let lines : Vec<Line> = (0..size)
            .map(|_| Line::new(size))
            .collect();

        Grid { size, lines }
    }

    pub fn compute_representation(&self) -> String {
        let lines_representation : Vec<String> = self.lines.iter()    
            .map(|line| line.compute_representation())
            .collect();

        lines_representation.join("\n")
    }

    pub fn tick(&mut self, coordinates: &Coordinates, tick_type: &TickType) {
        self.lines.get(coordinates.y as usize)
            .expect("Invalid coordinate !")
            .tick(coordinates, tick_type);
    }
}
pub struct Line {
    cells: Vec<Cell>
}

impl Line {
    pub fn new(size: u32) -> Line {
        let cells : Vec<Cell> = (0..size)
            .map(|_| Cell::new())
            .collect();

        Line{cells}
    }

    pub fn compute_representation(&self) -> String {
        let cells_representations : Vec<String> = self.cells.iter()
            .map(|cell| cell.compute_representation())
            .collect();

        cells_representations.join("")
    }

    fn tick(&self, coordinates: &Coordinates, tick_type: &TickType) {
        
    }
}

pub struct Cell {
    state: Option<TickType>
}

impl Cell {
    pub fn new() -> Cell{
        Cell{
            state: Option::None
        }
    }

    pub fn compute_representation(&self) -> String {
        match &self.state {
            Some(state) => state.compute_representation(),
            _ => String::from(" ")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::grid::Grid;

    #[test]
    fn new_game_should_be_represented_as_an_empty_grid() {
        let morpion_size = 4;
        let expected_representation = "    \n    \n    \n    ";

        let game = Grid::new(morpion_size);

        assert_eq!(game.compute_representation(), expected_representation)
    }
}