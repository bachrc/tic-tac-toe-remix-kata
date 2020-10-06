use crate::entities::ticktype::TickType;

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