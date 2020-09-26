pub enum TickType {
    NOUGHT,
    CROSS
}

impl TickType {
    pub fn compute_representation(&self) -> String {
        match &self {
            TickType::NOUGHT => String::from("O"),
            TickType::CROSS => String::from("X")
        }
    }
}
