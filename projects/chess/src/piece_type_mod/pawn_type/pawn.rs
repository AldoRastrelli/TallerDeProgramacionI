use crate::color::Color;
use crate::piece_mod::piece::Piece;
use crate::piece_type_mod::piece_type::PieceType;
use crate::position_mod::position::Position;

/// Represents a Pawn Piece
pub struct Pawn {}
impl PieceType for Pawn {
    /// Returns the piece's name
    fn string(&self) -> &str {
        "Pawn"
    }

    /// Returns true if the piece has a King behaviour 
    fn is_king(&self) -> bool {
        false
    }
    
    /// Returns true if the piece can capture in the given position.
    fn can_capture(&self, main_piece: &Piece, p2: &Position) -> bool {
        let result = false;
        let p1 = &main_piece.get_position();
        let color = &main_piece.color;
        let (px, py, pu, pv) = self.get_position_variables(p1, p2);
        let n = 1..2;

        // Posibilities
        match color {
            // Diagonals Up
            Color::White => self.get_result_for_diagonal_squares_up(result, px, py, pu, pv, &n),
            Color::Black => {
                // Diagonals Down
                self.get_result_for_diagonal_squares_down(result, px, py, pu, pv, &n)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let pawn = Pawn {};
        assert_eq!(pawn.string(), "Pawn");
    }

    #[test]
    fn test_is_king() {
        let pawn = Pawn {};
        assert_eq!(pawn.is_king(), false);
    }

    // can_capture coverage is assumed to be included in piece_type tests
}
