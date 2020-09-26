mod entities;
mod errors;

fn main() {
    println!("Bonjour ! Tu t'apprêtes à pénétrer l'une des plus grandes aventures jamais conçues à ce jour. Le morpion. C'est parti.");
}

#[cfg(test)]
mod tests {
    use crate::entities::ticktype::TickType;
    use crate::entities::player::Player;
    use crate::entities::coordinates::Coordinates;
    use crate::entities::grid::Grid;
    use crate::entities::gamestate::GameState;

    #[test]
    fn should_display_the_board_correctly_after_few_plays() {
        let expected_representation = String::from("OX  \n    \n    \n    ");

        let player1 = Player::new("Dimitri", TickType::Nought);
        let player2 = Player::new("Alphonse", TickType::Cross);

        let mut grid = Grid::new(4);

        player1.play(&Coordinates::from(0, 0), &mut grid).unwrap();
        player2.play(&Coordinates::from(1, 0), &mut grid).unwrap();

        let board_representation: String = grid.compute_representation();

        assert_eq!(board_representation, expected_representation)
    }

    #[test]
    fn an_empty_grid_should_not_be_considered_as_finished() {
        let grid = Grid::new(3);

        assert_eq!(grid.game_state(), GameState::InProgress)
    }

    #[test]
    fn a_filled_grid_without_any_winners_should_indicate_the_finished_game_state() {
        let mut game = Grid::new(3);

        let player1 = Player::new("Dimitri", TickType::Nought);
        let player2 = Player::new("Alphonse", TickType::Cross);

        player1.play(&Coordinates::from(0, 0), &mut game).unwrap();
        player1.play(&Coordinates::from(0, 2), &mut game).unwrap();
        player1.play(&Coordinates::from(1, 1), &mut game).unwrap();
        player1.play(&Coordinates::from(2, 1), &mut game).unwrap();

        player1.play(&Coordinates::from(0, 1), &mut game).unwrap();
        player2.play(&Coordinates::from(1, 0), &mut game).unwrap();
        player2.play(&Coordinates::from(1, 2), &mut game).unwrap();
        player2.play(&Coordinates::from(2, 0), &mut game).unwrap();
        player2.play(&Coordinates::from(2, 2), &mut game).unwrap();

        assert_eq!(game.game_state(), GameState::Finished)
    }
}
