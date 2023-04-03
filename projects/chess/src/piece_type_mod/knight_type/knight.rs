use crate::piece_mod::piece::Piece;
use crate::piece_type_mod::piece_type::PieceType;
use crate::position_mod::position::Position;

/// Represents a Knight Piece
pub struct Knight {}
impl PieceType for Knight {
    /// Returns the piece's name
    fn string(&self) -> &str {
        "Knight"
    }

    /// Returns true if the piece has a King behaviour 
    fn is_king(&self) -> bool {
        false
    }

    /// Returns true if the piece can capture in the given position.
    fn can_capture(&self, main_piece: &Piece, p2: &Position) -> bool {
        let mut result = false;
        let p1 = &main_piece.get_position();
        let (px, py, pu, pv) = self.get_position_variables(p1, p2);

        result = result || (pu - px == 2 && pv - py == 1); // right & up
        result = result || (pu - px == 2 && py - pv == 1); // right & down
        result = result || (px - pu == 2 && pv - py == 1); // left & up
        result = result || (px - pu == 2 && py - pv == 1); // left & down
        result = result || (pv - py == 2 && pu - px == 1); // up & right
        result = result || (pv - py == 2 && px - pu == 1); // up & left
        result = result || (py - pv == 2 && pu - px == 1); // down & right
        result = result || (py - pv == 2 && px - pu == 1); // down & left

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let knight = Knight {};
        assert_eq!(knight.string(), "Knight");
    }

    #[test]
    fn test_is_king() {
        let knight = Knight {};
        assert_eq!(knight.is_king(), false);
    }

    // can_capture coverage is assumed to be included in piece_type tests
}
