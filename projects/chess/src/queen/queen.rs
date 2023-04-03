use crate::color::Color;
use crate::piece_type::piece_type::PieceType;
use crate::position::position::Position;

pub struct Queen {}
impl PieceType for Queen {
    fn string(&self) -> &str {
        "Queen"
    }

    fn is_king(&self) -> bool {
        false
    }

    fn can_capture(&self, p1: &Position, c: &Color, p2: &Position) -> bool {
        let mut result = false;

        // Cross Moves
        result = self.get_result_for_cross_squares(result, p1, p2);
        // Diagonals
        result = self.get_result_for_diagonal_squares(result, p1, p2);

        result
    }
}
