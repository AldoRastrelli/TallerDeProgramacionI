use crate::piece_type_mod::piece_type::PieceType;
// Piece Types
use crate::piece_type_mod::bishop_type::bishop::Bishop;
use crate::piece_type_mod::king_type::king::King;
use crate::piece_type_mod::knight_type::knight::Knight;
use crate::piece_type_mod::pawn_type::pawn::Pawn;
use crate::piece_type_mod::queen_type::queen::Queen;
use crate::piece_type_mod::rook_type::rook::Rook;
// Other
use crate::color::Color;
use crate::piece_movement::movement::Movement;
use crate::position_mod::position::Position;

/// Represents a piece of chess. It has a color, a position and a piece_type that gives it a behavior.
pub struct Piece {
    pub color: Color,
    position: Position,
    piece_type: Box<dyn PieceType>,
}

impl Piece {
    /// Tries to create a Piece. If it was not possible, it returns None.
    pub fn new(name: &str, row: i32, col: i32) -> Option<Piece> {
        let piece_type_given = Self::get_piece_type(name);

        piece_type_given.map(|p| Piece {
            color: Self::get_piece_color(name),
            position: Position { x: row, y: col },
            piece_type: p,
        })
    }

    /// Prints piece info. This is used for testing.
    pub fn print_info(&self) {
        println!("{}", self.get_info());
    }

    /// Formats the piece's info.
    pub fn get_info(&self) -> String {
        format!(
            "Type: {},Color: {:?}, Posicion: {:?}",
            self.piece_type.string(),
            self.color,
            self.get_position().get_pair()
        )
    }

    /// Position getter
    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// Color getter
    fn get_piece_color(name: &str) -> Color {
        if name.to_lowercase() == name {
            Color::White
        } else {
            Color::Black
        }
    }

    /// Type getter
    pub fn get_type(&self) -> &Box<dyn PieceType> {
        &self.piece_type
    }

    /// Parses the piece's name into a type for behavior.
    fn get_piece_type(name: &str) -> Option<Box<dyn PieceType>> {
        let name_up = name.to_uppercase();
        match &name_up[..] {
            "R" => Some(Box::new(King {})),
            "D" => Some(Box::new(Queen {})),
            "A" => Some(Box::new(Bishop {})),
            "C" => Some(Box::new(Knight {})),
            "T" => Some(Box::new(Rook {})),
            "P" => Some(Box::new(Pawn {})),
            _ => None,
        }
    }

    /// Returns true if the piece can capture the other_piece. Otherwise it returns false.
    pub fn can_capture(&self, other_piece: &Piece) -> Movement {
        let capture = &self.piece_type.can_capture(self, &other_piece.position);

        Movement {
            piece_color: &self.color,
            did_win: *capture,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_black_king_piece() {
        let piece = Piece::new(&"R", 3, 3).unwrap();
        let info_expected = "Type: King,Color: Black, Posicion: [3, 3]";

        assert_eq!(piece.get_info(), info_expected);
    }

    #[test]
    fn test_new_king_piece() {
        let piece = Piece::new(&"r", 3, 3).unwrap();
        let info_expected = "Type: King,Color: White, Posicion: [3, 3]";

        assert_eq!(piece.get_info(), info_expected);
    }

    #[test]
    fn test_new_black_queen_piece() {
        let piece = Piece::new(&"D", 3, 3).unwrap();
        let info_expected = "Type: Queen,Color: Black, Posicion: [3, 3]";

        assert_eq!(piece.get_info(), info_expected);
    }

    #[test]
    fn test_new_queen_piece() {
        let piece = Piece::new(&"d", 3, 3).unwrap();
        let info_expected = "Type: Queen,Color: White, Posicion: [3, 3]";

        assert_eq!(piece.get_info(), info_expected);
    }

    #[test]
    fn test_new_black_bishop_piece() {
        let piece = Piece::new(&"A", 3, 3).unwrap();
        let info_expected = "Type: Bishop,Color: Black, Posicion: [3, 3]";

        assert_eq!(piece.get_info(), info_expected);
    }

    #[test]
    fn test_new_bishop_piece() {
        let piece = Piece::new(&"a", 3, 3).unwrap();
        let info_expected = "Type: Bishop,Color: White, Posicion: [3, 3]";

        assert_eq!(piece.get_info(), info_expected);
    }

    #[test]
    fn test_new_black_knight_piece() {
        let piece = Piece::new(&"C", 3, 3).unwrap();
        let info_expected = "Type: Knight,Color: Black, Posicion: [3, 3]";

        assert_eq!(piece.get_info(), info_expected);
    }

    #[test]
    fn test_new_knight_piece() {
        let piece = Piece::new(&"c", 3, 3).unwrap();
        let info_expected = "Type: Knight,Color: White, Posicion: [3, 3]";

        assert_eq!(piece.get_info(), info_expected);
    }

    #[test]
    fn test_new_black_rook_piece() {
        let piece = Piece::new(&"T", 3, 3).unwrap();
        let info_expected = "Type: Rook,Color: Black, Posicion: [3, 3]";

        assert_eq!(piece.get_info(), info_expected);
    }

    #[test]
    fn test_new_rook_piece() {
        let piece = Piece::new(&"t", 3, 3).unwrap();
        let info_expected = "Type: Rook,Color: White, Posicion: [3, 3]";

        assert_eq!(piece.get_info(), info_expected);
    }

    #[test]
    fn test_new_black_pawn_piece() {
        let piece = Piece::new(&"P", 3, 3).unwrap();
        let info_expected = "Type: Pawn,Color: Black, Posicion: [3, 3]";

        assert_eq!(piece.get_info(), info_expected);
    }

    #[test]
    fn test_new_pawn_piece() {
        let piece = Piece::new(&"p", 3, 3).unwrap();
        let info_expected = "Type: Pawn,Color: White, Posicion: [3, 3]";

        assert_eq!(piece.get_info(), info_expected);
    }

    #[test]
    fn test_new_invalid_piece() {
        let piece = Piece::new(&"X", 3, 3);

        assert!(piece.is_none());
    }

    #[test]
    fn test_get_position() {
        let piece = Piece::new(&"R", 3, 3).unwrap();
        let position_expected = Position { x: 3, y: 3 };

        assert_eq!(piece.get_position(), &position_expected);
    }

    #[test]
    fn test_get_color_black() {
        let piece = Piece::new(&"R", 3, 3).unwrap();

        assert_eq!(piece.color, Color::Black);
    }

    #[test]
    fn test_get_color_white() {
        let piece = Piece::new(&"r", 3, 3).unwrap();

        assert_eq!(piece.color, Color::White);
    }

    // King
    #[test]
    fn test_king_captures_pos_1() {
        let piece = Piece::new(&"R", 3, 3).unwrap();
        let other_piece = Piece::new(&"r", 2, 4).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_king_captures_pos_2() {
        let piece = Piece::new(&"R", 3, 3).unwrap();
        let other_piece = Piece::new(&"r", 3, 4).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_king_captures_pos_3() {
        let piece = Piece::new(&"R", 3, 3).unwrap();
        let other_piece = Piece::new(&"r", 4, 4).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_king_captures_pos_4() {
        let piece = Piece::new(&"R", 3, 3).unwrap();
        let other_piece = Piece::new(&"r", 4, 3).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_king_captures_pos_5() {
        let piece = Piece::new(&"R", 3, 3).unwrap();
        let other_piece = Piece::new(&"r", 4, 2).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_king_captures_pos_6() {
        let piece = Piece::new(&"R", 3, 3).unwrap();
        let other_piece = Piece::new(&"r", 3, 2).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_king_captures_pos_7() {
        let piece = Piece::new(&"R", 3, 3).unwrap();
        let other_piece = Piece::new(&"r", 2, 2).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_king_captures_pos_8() {
        let piece = Piece::new(&"R", 3, 3).unwrap();
        let other_piece = Piece::new(&"r", 2, 3).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_king_doesnt_capture() {
        let piece = Piece::new(&"R", 3, 3).unwrap();
        let other_piece = Piece::new(&"r", 5, 5).unwrap();

        assert!(!piece.can_capture(&other_piece).did_win);
    }

    // Queen
    #[test]
    fn test_queen_captures_pos_1() {
        let piece = Piece::new(&"D", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 1, 5).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_queen_captures_pos_2() {
        let piece = Piece::new(&"D", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 3, 5).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_queen_captures_pos_3() {
        let piece = Piece::new(&"D", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 5, 5).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_queen_captures_pos_4() {
        let piece = Piece::new(&"D", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 5, 3).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]

    fn test_queen_captures_pos_5() {
        let piece = Piece::new(&"D", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 5, 1).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_queen_captures_pos_6() {
        let piece = Piece::new(&"D", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 3, 1).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_queen_captures_pos_7() {
        let piece = Piece::new(&"D", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 1, 1).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_queen_captures_pos_8() {
        let piece = Piece::new(&"D", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 1, 3).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_queen_doesnt_capture() {
        let piece = Piece::new(&"D", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 5, 7).unwrap();

        assert!(!piece.can_capture(&other_piece).did_win);
    }

    // Bishop
    #[test]
    fn test_bishop_captures_pos_1() {
        let piece = Piece::new(&"A", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 0, 6).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_bishop_captures_pos_2() {
        let piece = Piece::new(&"A", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 6, 6).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_bishop_captures_pos_3() {
        let piece = Piece::new(&"A", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 6, 0).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_bishop_captures_pos_4() {
        let piece = Piece::new(&"A", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 0, 0).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_bishop_doesnt_capture() {
        let piece = Piece::new(&"A", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 5, 7).unwrap();

        assert!(!piece.can_capture(&other_piece).did_win);
    }

    // Knight
    #[test]
    fn test_knight_captures_pos_1() {
        let piece = Piece::new(&"C", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 1, 2).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_knight_captures_pos_2() {
        let piece = Piece::new(&"C", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 1, 4).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_knight_captures_pos_3() {
        let piece = Piece::new(&"C", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 2, 1).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_knight_captures_pos_4() {
        let piece = Piece::new(&"C", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 2, 5).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_knight_captures_pos_5() {
        let piece = Piece::new(&"C", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 4, 1).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_knight_captures_pos_6() {
        let piece = Piece::new(&"C", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 4, 5).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_knight_captures_pos_7() {
        let piece = Piece::new(&"C", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 5, 2).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_knight_captures_pos_8() {
        let piece = Piece::new(&"C", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 5, 4).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_knight_doesnt_capture() {
        let piece = Piece::new(&"C", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 5, 7).unwrap();

        assert!(!piece.can_capture(&other_piece).did_win);
    }

    // Rook
    #[test]
    fn test_rook_captures_pos_1() {
        let piece = Piece::new(&"T", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 3, 0).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_rook_captures_pos_2() {
        let piece = Piece::new(&"T", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 3, 7).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_rook_captures_pos_3() {
        let piece = Piece::new(&"T", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 0, 3).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_rook_captures_pos_4() {
        let piece = Piece::new(&"T", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 7, 3).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_rook_doesnt_capture() {
        let piece = Piece::new(&"T", 3, 3).unwrap();
        let other_piece = Piece::new(&"p", 5, 7).unwrap();

        assert!(!piece.can_capture(&other_piece).did_win);
    }

    // Black Pawn
    #[test]
    fn test_black_pawn_captures_pos_1() {
        let piece = Piece::new(&"P", 3, 3).unwrap();
        let other_piece = Piece::new(&"t", 2, 2).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_black_pawn_captures_pos_2() {
        let piece = Piece::new(&"P", 3, 3).unwrap();
        let other_piece = Piece::new(&"t", 4, 2).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_black_pawn_doesnt_capture() {
        let piece = Piece::new(&"P", 3, 3).unwrap();
        let other_piece = Piece::new(&"t", 5, 7).unwrap();

        assert!(!piece.can_capture(&other_piece).did_win);
    }

    // White Pawn
    #[test]
    fn test_white_pawn_captures_pos_1() {
        let piece = Piece::new(&"p", 3, 3).unwrap();
        let other_piece = Piece::new(&"T", 2, 4).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_white_pawn_captures_pos_2() {
        let piece = Piece::new(&"p", 3, 3).unwrap();
        let other_piece = Piece::new(&"T", 4, 4).unwrap();

        assert!(piece.can_capture(&other_piece).did_win);
    }

    #[test]
    fn test_white_pawn_doesnt_capture() {
        let piece = Piece::new(&"p", 3, 3).unwrap();
        let other_piece = Piece::new(&"T", 5, 7).unwrap();

        assert!(!piece.can_capture(&other_piece).did_win);
    }
}
