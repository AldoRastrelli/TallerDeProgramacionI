use crate::piece_mod::piece::Piece;
use crate::piece_type_mod::piece_type::PieceType;
use crate::position_mod::position::Position;

/// Represents a King Piece
pub struct King {}
impl PieceType for King {
    /// Returns the piece's name
    fn string(&self) -> &str {
        "King"
    }

    /// Returns true if the piece has a King behaviour
    fn is_king(&self) -> bool {
        true
    }

    /// Returns true if the piece can capture in the given position.
    fn can_capture(&self, main_piece: &Piece, p2: &Position) -> bool {
        let mut result = false;
        let p1 = &main_piece.get_position();
        // Cross Moves
        result = result || self.get_result_for_cross_squares(result, p1, p2);
        // Diagonals
        result = result || self.get_result_for_diagonal_squares(result, p1, p2);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let king = King {};
        assert_eq!(king.string(), "King");
    }

    #[test]
    fn test_is_king() {
        let king = King {};
        assert_eq!(king.is_king(), true);
    }

    // can_capture coverage is assumed to be included in piece_type tests
}
