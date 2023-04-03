use crate::piece_mod::piece::Piece;
use crate::position_mod::position::Position;
use std::ops::Range;

/// Represents a Piece Type. This is what gives a piece its behavior.
pub trait PieceType {
    fn string(&self) -> &str;
    fn can_capture(&self, main_piece: &Piece, p2: &Position) -> bool;
    fn is_king(&self) -> bool;

    // ******* Diagonal squares
    /// It returns true if the piece can capture in the given position in the diagonal squares from the main piece.
    fn get_result_for_diagonal_squares(
        &self,
        result: bool,
        attacker_pos: &Position,
        victim_pos: &Position,
    ) -> bool {
        let (x, y, u, v) = self.get_position_variables(attacker_pos, victim_pos);
        let n = self.get_range();

        let mut result = result || self.get_result_for_diagonal_squares_up(result, x, y, u, v, &n);
        result = result || self.get_result_for_diagonal_squares_down(result, x, y, u, v, &n);

        result
    }

    /// It returns true if the piece can capture in the given position in the diagonal down squares from the main piece.
    fn get_result_for_diagonal_squares_down(
        &self,
        result: bool,
        x: i32,
        y: i32,
        u: i32,
        v: i32,
        n: &Range<i32>,
    ) -> bool {
        let mut result = result || (u - x == y - v && n.contains(&(u - x))); // diagonal -> right & down
        result = result || (x - u == y - v && n.contains(&(x - u))); // diagonal -> left & down

        result
    }

    /// It returns true if the piece can capture in the given position in the diagonal up squares from the main piece.
    fn get_result_for_diagonal_squares_up(
        &self,
        result: bool,
        x: i32,
        y: i32,
        u: i32,
        v: i32,
        n: &Range<i32>,
    ) -> bool {
        let mut result = result || (u - x == v - y && n.contains(&(u - x))); // diagonal -> right & up
        result = result || (x - u == v - y && n.contains(&(x - u))); // diagonal -> left & up

        result
    }

    // ******* Cross Squares
    /// It returns true if the piece can capture in the given position in the cross swuares from the main piece .
    fn get_result_for_cross_squares(
        &self,
        result: bool,
        attacker_pos: &Position,
        victim_pos: &Position,
    ) -> bool {
        let (x, y, u, v) = self.get_position_variables(attacker_pos, victim_pos);
        let n = self.get_range();

        let mut result = result || y == v && n.contains(&(u - x)); // right
        result = result || y == v && n.contains(&(x - u)); // left
        result = result || x == u && n.contains(&(v - y)); // up
        result = result || x == u && n.contains(&(y - v)); // down

        result
    }

    // ******* Auxiliar Funcs
    /// Recieves two positions and returns them in a Tuple format
    fn get_position_variables(&self, p1: &Position, p2: &Position) -> (i32, i32, i32, i32) {
        (p1.x, p1.y, p2.x, p2.y)
    }

    /// Creates a range with the piece's behavior so it can know how many squares it can move in total.
    fn get_range(&self) -> Range<i32> {
        let max = if self.is_king() { 2 } else { 8 };
        1..max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece_type_mod::bishop_type::bishop::Bishop;
    use crate::piece_type_mod::king_type::king::King;
    use crate::piece_type_mod::knight_type::knight::Knight;
    use crate::piece_type_mod::pawn_type::pawn::Pawn;
    use crate::piece_type_mod::piece_type::PieceType;
    use crate::piece_type_mod::queen_type::queen::Queen;
    use crate::piece_type_mod::rook_type::rook::Rook;
    use crate::position_mod::position::Position;

    #[test]
    fn test_get_result_for_diagonal_squares() {
        let bishop = Bishop {};
        let king = King {};
        let pawn = Pawn {};
        let queen = Queen {};

        let p1 = Position { x: 0, y: 0 };
        let p2 = Position { x: 1, y: 1 };

        assert!(bishop.get_result_for_diagonal_squares(false, &p1, &p2));
        assert!(king.get_result_for_diagonal_squares(false, &p1, &p2));
        assert!(pawn.get_result_for_diagonal_squares(false, &p1, &p2));
        assert!(queen.get_result_for_diagonal_squares(false, &p1, &p2));
    }

    #[test]
    fn test_get_result_for_cross_squares() {
        let king = King {};
        let queen = Queen {};
        let rook = Rook {};

        let p1 = Position { x: 0, y: 0 };
        let p2 = Position { x: 1, y: 0 };

        assert!(king.get_result_for_cross_squares(false, &p1, &p2));
        assert!(queen.get_result_for_cross_squares(false, &p1, &p2));
        assert!(rook.get_result_for_cross_squares(false, &p1, &p2));
    }

    #[test]
    fn test_get_result_for_diagonal_squares_down() {
        let pawn = Pawn {};

        let n = 1..2;

        assert!(pawn.get_result_for_diagonal_squares_down(false, 1, 1, 0, 0, &n));
        assert!(pawn.get_result_for_diagonal_squares_down(false, 1, 1, 2, 0, &n));
    }

    #[test]
    fn test_get_result_for_diagonal_squares_up() {
        let pawn = Pawn {};

        let n = 1..2;

        assert!(pawn.get_result_for_diagonal_squares_up(false, 1, 1, 0, 2, &n));
        assert!(pawn.get_result_for_diagonal_squares_up(false, 1, 1, 2, 2, &n));
    }

    #[test]
    fn test_get_position_variables() {
        let pawn = Pawn {};
        let bishop = Bishop {};
        let king = King {};
        let knight = Knight {};
        let queen = Queen {};
        let rook = Rook {};

        let p1 = Position { x: 0, y: 0 };
        let p2 = Position { x: 1, y: 1 };

        assert_eq!(pawn.get_position_variables(&p1, &p2), (0, 0, 1, 1));
        assert_eq!(bishop.get_position_variables(&p1, &p2), (0, 0, 1, 1));
        assert_eq!(king.get_position_variables(&p1, &p2), (0, 0, 1, 1));
        assert_eq!(knight.get_position_variables(&p1, &p2), (0, 0, 1, 1));
        assert_eq!(queen.get_position_variables(&p1, &p2), (0, 0, 1, 1));
        assert_eq!(rook.get_position_variables(&p1, &p2), (0, 0, 1, 1));
    }

    #[test]
    fn test_get_range() {
        let bishop = Bishop {};
        let king = King {};
        let knight = Knight {};
        let pawn = Pawn {};
        let queen = Queen {};
        let rook = Rook {};

        assert_eq!(bishop.get_range(), 1..8);
        assert_eq!(king.get_range(), 1..2);
        assert_eq!(knight.get_range(), 1..8);
        assert_eq!(pawn.get_range(), 1..8);
        assert_eq!(queen.get_range(), 1..8);
        assert_eq!(rook.get_range(), 1..8);
    }
}
