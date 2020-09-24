mod entities;

fn main() {
    println!("Bonjour ! Tu t'apprêtes à pénétrer l'une des plus grandes aventures jamais conçues à ce jour. Le morpion. C'est parti.");
}

#[cfg(test)]
mod tests {
    use crate::entities::ticktype::TickType;
    use crate::entities::player::Player;
    use crate::entities::game::Game;
    use crate::entities::coordinates::Coordinates;

    #[test]
    fn should_display_the_board_correctly_after_few_plays() {
        let expected_representation = String::from("OX  \n    \n    \n    ");

        let player1 = Player::new("Dimitri", TickType::NOUGHT);
        let player2 = Player::new("Alphonse", TickType::CROSS);

        let mut game = Game::new(4, player1, player2);
        game.first_player_play(&Coordinates::from(0, 0));
        game.second_player_play(&Coordinates::from(1, 0));

        let board_representation: String = game.compute_board_representation();

        assert_eq!(board_representation, expected_representation)
    }
}
