use crate::entities::player::{Player, Players};
use crate::entities::coordinates::Coordinates;

pub struct Game {
    players : Players,
}

impl Game {
    pub fn new(size: u32, player1: Player, player2: Player) -> Game {
        Game {
            players: Players::from_couple(player1, player2)
        }
    }

    pub fn first_player_play(&self, coordinates: &Coordinates) {

    }

    pub fn second_player_play(&self, coordinates: &Coordinates) {

    }

    pub fn compute_board_representation(&self) -> String {
        String::new()
    }
}