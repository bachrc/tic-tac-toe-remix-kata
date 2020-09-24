use crate::entities::ticktype::TickType;

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
}

pub struct Players {
    player1: Player,
    player2: Player
}

impl Players {
    pub fn from_couple(player1: Player, player2: Player) -> Players {
        return Players {
            player1, player2
        }
    }
}