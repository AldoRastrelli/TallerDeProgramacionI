use crate::color::Color;
use crate::piece_type::piece_type::PieceType;
use crate::position::position::Position;

pub struct Pawn {}
impl PieceType for Pawn {
    fn string(&self) -> &str {
        "Pawn"
    }

    fn is_king(&self) -> bool {
        false
    }

    fn can_capture(&self, p1: &Position, c: &Color, p2: &Position) -> bool {
        let mut result = false;
        let (x, y, u, v) = self.get_position_variables(p1, p2);
        let n = 1..2;

        // Posibilities
        match c {
            // Diagonals Up
            Color::White => {
                return self.get_result_for_diagonal_squares_up(result, x, y, u, v, &n);
            }
            Color::Black => {
                // Diagonals Down
                return self.get_result_for_diagonal_squares_down(result, x, y, u, v, &n);
            }
        }
    }
}
