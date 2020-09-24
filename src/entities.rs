
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

struct Players {
    player1: Player,
    player2: Player
}

pub struct Game {
    players : Players,
}

impl Game {
    pub fn new(size: u32, player1: Player, player2: Player) -> Game {
        Game {
            players: Players {
                player1, player2
            }
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

pub struct Coordinates {
    x: u32,
    y: u32
}

impl Coordinates {
    pub fn from(x: u32, y: u32) -> Coordinates {
        Coordinates {x, y}
    }
}

pub enum TickType {
    NOUGHT, CROSS
}