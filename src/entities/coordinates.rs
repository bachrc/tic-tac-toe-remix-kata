pub struct Coordinates {
    pub x: usize,
    pub y: usize
}

impl Coordinates {
    pub fn from(x: usize, y: usize) -> Coordinates {
        Coordinates {x, y}
    }
}
