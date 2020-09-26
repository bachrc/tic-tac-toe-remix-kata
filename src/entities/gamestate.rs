use crate::entities::player::Player;

#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Finished,
    WonBy(Player)
}