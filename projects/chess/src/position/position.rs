pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn get_pair(&self) -> [i32; 2] {
        [self.x, self.y]
    }
}
