#[derive(Clone, Debug, PartialEq)]
pub enum TickType {
    Nought,
    Cross
}

impl TickType {
    pub fn compute_representation(&self) -> String {
        match &self {
            TickType::Nought => String::from("O"),
            TickType::Cross => String::from("X")
        }
    }
}
