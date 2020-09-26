mod entities;

fn main() {
    println!("Bonjour ! Tu t'apprêtes à pénétrer l'une des plus grandes aventures jamais conçues à ce jour. Le morpion. C'est parti.");
}

#[cfg(test)]
mod tests {
    use crate::entities::ticktype::TickType;
    use crate::entities::player::Player;
    use crate::entities::coordinates::Coordinates;
    use crate::entities::grid::Grid;

    #[test]
    fn should_display_the_board_correctly_after_few_plays() {
        let expected_representation = String::from("OX  \n    \n    \n    ");

        let player1 = Player::new("Dimitri", TickType::NOUGHT);
        let player2 = Player::new("Alphonse", TickType::CROSS);

        let mut grid = Grid::new(4);

        player1.play(&Coordinates::from(0, 0), &mut grid);
        player2.play(&Coordinates::from(1, 0), &mut grid);

        let board_representation: String = grid.compute_representation();

        assert_eq!(board_representation, expected_representation)
    }
}
