#[derive(Debug, PartialEq)]
/// Represents a position in the table. Axis are thought as in x-y: lower left corner is (0,0).
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    /// Returns the [x,y] pair
    pub fn get_pair(&self) -> [i32; 2] {
        [self.x, self.y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_position() {
        let position = Position { x: 1, y: 2 };
        assert_eq!(position.x, 1);
        assert_eq!(position.y, 2);
    }

    #[test]
    fn test_get_pair() {
        let position = Position { x: 1, y: 2 };
        assert_eq!(position.get_pair(), [1, 2]);
    }
}
