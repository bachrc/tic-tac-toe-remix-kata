use crate::entities::ticktype::TickType;

#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Finished,
    Won
}