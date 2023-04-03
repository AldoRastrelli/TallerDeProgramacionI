use crate::color::Color;
use crate::piece_type::piece_type::PieceType;
use crate::position::position::Position;

pub struct King {}
impl PieceType for King {
    fn string(&self) -> &str {
        "King"
    }

    fn is_king(&self) -> bool {
        true
    }

    fn can_capture(&self, p1: &Position, c: &Color, p2: &Position) -> bool {
        let mut result = false;
        // Cross Moves
        result = result || self.get_result_for_cross_squares(result, p1, p2);
        // Diagonals
        result = result || self.get_result_for_diagonal_squares(result, p1, p2);

        result
    }
}
