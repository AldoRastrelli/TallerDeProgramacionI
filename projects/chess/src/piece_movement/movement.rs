use crate::color::Color;

/// Represents a Move in the table. It has a color and a result.
pub struct Movement<'a> {
    pub piece_color: &'a Color,
    pub did_win: bool,
}

impl Movement<'_> {
    /// Returns true if white wins. False otherwise.
    pub fn white_wins(&self) -> bool {
        self.piece_color == &Color::White && self.did_win
    }
    /// Returns true if black wins. False otherwise.
    pub fn black_wins(&self) -> bool {
        self.piece_color == &Color::Black && self.did_win
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_wins() {
        let movement = Movement {
            piece_color: &Color::White,
            did_win: true,
        };
        assert!(movement.white_wins());
    }

    #[test]
    fn test_black_wins() {
        let movement = Movement {
            piece_color: &Color::Black,
            did_win: true,
        };
        assert!(movement.black_wins());
    }

    #[test]
    fn test_white_does_not_win() {
        let movement = Movement {
            piece_color: &Color::White,
            did_win: false,
        };
        assert!(!movement.white_wins());
    }

    #[test]
    fn test_black_does_not_win() {
        let movement = Movement {
            piece_color: &Color::Black,
            did_win: false,
        };
        assert!(!movement.black_wins());
    }
}
