use crate::piece_mod::piece::Piece;
use crate::piece_type_mod::piece_type::PieceType;
use crate::position_mod::position::Position;

/// Represents a Bishop Piece
pub struct Bishop {}
impl PieceType for Bishop {
    /// Returns the piece's name
    fn string(&self) -> &str {
        "Bishop"
    }

    /// Returns true if the piece has a King behaviour
    fn is_king(&self) -> bool {
        false
    }

    /// Returns true if the piece can capture in the given position.
    fn can_capture(&self, main_piece: &Piece, p2: &Position) -> bool {
        let mut result = false;
        let p1 = &main_piece.get_position();
        // Diagonals
        result = self.get_result_for_diagonal_squares(result, p1, p2);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let bishop = Bishop {};
        assert_eq!(bishop.string(), "Bishop");
    }

    #[test]
    fn test_is_king() {
        let bishop = Bishop {};
        assert_eq!(bishop.is_king(), false);
    }

    // can_capture coverage is assumed to be included in piece_type tests
}
