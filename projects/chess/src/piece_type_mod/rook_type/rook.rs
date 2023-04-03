use crate::piece_mod::piece::Piece;
use crate::piece_type_mod::piece_type::PieceType;
use crate::position_mod::position::Position;

/// Represents a Rook Piece
pub struct Rook {}
impl PieceType for Rook {
    /// Returns the piece's name
    fn string(&self) -> &str {
        "Rook"
    }

    /// Returns true if the piece has a King behaviour 
    fn is_king(&self) -> bool {
        false
    }

    /// Returns true if the piece can capture in the given position.
    fn can_capture(&self, main_piece: &Piece, p2: &Position) -> bool {
        let mut result = false;
        let p1 = &main_piece.get_position();

        // Cross Moves
        result = self.get_result_for_cross_squares(result, p1, p2);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let rook = Rook {};
        assert_eq!(rook.string(), "Rook");
    }

    #[test]
    fn test_is_king() {
        let rook = Rook {};
        assert_eq!(rook.is_king(), false);
    }

    // can_capture coverage is assumed to be included in piece_type tests
}
