extern crate itertools;
use itertools::Itertools;
use crate::entities::ticktype::TickType;
use crate::entities::coordinates::Coordinates;
use crate::entities::gamestate::GameState;
use crate::errors::TicTacToeError;
use crate::entities::line::Line;

#[derive(Debug)]
pub struct Grid {
    lines: Vec<Line>
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        let lines: Vec<Line> = (0..size)
            .map(|_| Line::new(size))
            .collect();

        Grid { lines }
    }

    pub fn compute_representation(&self) -> String {
        let lines_representation: Vec<String> = self.lines.iter()
            .map(|line| line.compute_representation())
            .collect();

        lines_representation.join("\n")
    }

    pub fn game_state(&self) -> GameState {
        if self.is_grid_full() {
            return GameState::Finished
        }

        if self.check_vertical_win() {
            return GameState::Won
        }

        GameState::InProgress
    }

    fn check_vertical_win(&self) -> bool {
        self.lines.iter().any(Line::is_won)
    }

    pub fn tick(&mut self, coordinates: &Coordinates, tick_type: &TickType) -> Result<(), TicTacToeError> {
        self.fetch_line_at(coordinates.y as usize)
            .ok_or(TicTacToeError::CoordinateOutOfScope)?
            .tick(coordinates, tick_type)
    }

    fn fetch_horizontal_winner(&self) -> bool {
        self.lines.iter().any(Line::is_won)
    }

    fn is_grid_full(&self) -> bool {
        self.lines.iter().all(Line::is_full)
    }

    fn fetch_line_at(&mut self, index: usize) -> Option<&mut Line> {
        self.lines.get_mut(index)
    }
}



#[derive(Debug)]
pub struct Cell {
    pub state: Option<TickType>
}

impl Cell {
    pub fn new() -> Cell{
        Cell{
            state: Option::None
        }
    }

    pub fn from(tick_type: &TickType) -> Cell {
        Cell {
            state: Option::Some(tick_type.clone())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.state.is_none()
    }

    pub fn is_full(&self) -> bool {
        self.state.is_some()
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
    use crate::entities::player::Player;
    use crate::entities::ticktype::TickType;
    use crate::entities::coordinates::Coordinates;
    use crate::errors::TicTacToeError;

    #[test]
    fn new_game_should_be_represented_as_an_empty_grid() {
        let morpion_size = 4;
        let expected_representation = "    \n    \n    \n    ";

        let game = Grid::new(morpion_size);

        assert_eq!(game.compute_representation(), expected_representation)
    }

    #[test]
    fn should_not_play_somewhere_already_played() {
        let morpion_size = 3;
        let mut game = Grid::new(morpion_size);

        let player1 = Player::new("Dimitri", TickType::Nought);
        let player2 = Player::new("Alphonse", TickType::Cross);

        let _ = player1.play(&Coordinates::from(0, 0), &mut game);
        let result = player2.play(&Coordinates::from(0, 0), &mut game);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TicTacToeError::CellNotEmpty)
    }
}