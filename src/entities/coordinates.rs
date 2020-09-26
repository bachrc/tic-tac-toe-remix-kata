pub struct Coordinates {
    pub x: u32,
    pub y: u32
}

impl Coordinates {
    pub fn from(x: u32, y: u32) -> Coordinates {
        Coordinates {x, y}
    }
}
