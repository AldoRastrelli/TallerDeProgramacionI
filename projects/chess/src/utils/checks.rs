use crate::color::Color;
use crate::errors::chess_error::ChessError;
use crate::piece_list_mod::piece_list::PieceList;

/// Returns true if the square has a piece. False otherwise.
pub fn square_has_piece(sq: &str) -> bool {
    sq != "_"
}

/// Returns Ok if the number of rows iterated was 8 and the Pieces found are valid. Err otherwise. 
pub fn result_is_valid(rows: i32, pieces: &PieceList) -> Result<&str, &ChessError> {
    if !row_size_is_valid(rows) {
        return Err(&ChessError::TABLE_SIZE);
    }

    match pieces_pass_checks(pieces) {
        Ok(_) => Ok(""),
        Err(e) => Err(e),
    }
}

/// Returns true if the number of iterated rows was 8.
fn row_size_is_valid(curr_rows: i32) -> bool {
    curr_rows == -1
}

/// Returns true if the Pieces in the PieceList pass all needed checks: total count and color.
fn pieces_pass_checks(pieces: &PieceList) -> Result<&str, &ChessError> {
    if pieces.len != 2 {
        return Err(&ChessError::NUMBER_PIECES);
    }

    if !pieces_checked_color(pieces) {
        return Err(&ChessError::COLOR);
    }

    Ok("")
}

/// Returns true if the Pieces in the PieceList have different colors: black and white.
fn pieces_checked_color(pieces: &PieceList) -> bool {
    let first_piece = &pieces.first();
    let second_piece = &pieces.second();

    let black_and_white = first_piece.color == Color::Black && second_piece.color == Color::White;
    let white_and_black = first_piece.color == Color::White && second_piece.color == Color::Black;

    (black_and_white) || (white_and_black)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece_mod::piece::Piece;

    #[test]
    fn test_square_has_piece_true() {
        square_has_piece(&"k");
    }

    #[test]
    fn test_square_has_piece_false() {
        square_has_piece(&"_");
    }

    #[test]
    fn test_result_is_valid_true() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"R", 1, 2).unwrap();
        let piece2 = Piece::new(&"r", 2, 1).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = result_is_valid(-1, &piece_list);

        assert!(res.is_ok());
    }

    #[test]
    fn test_result_is_valid_false() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"R", 1, 2).unwrap();
        let piece2 = Piece::new(&"r", 2, 1).unwrap();
        let piece3 = Piece::new(&"t", 1, 1).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);
        piece_list.push(piece3);

        let res = result_is_valid(-1, &piece_list);

        assert!(res.is_err());
    }

    #[test]
    fn test_row_size_is_valid_true() {
        let res = row_size_is_valid(-1);

        assert!(res);
    }

    #[test]
    fn test_row_size_is_valid_false() {
        let res = row_size_is_valid(-10);

        assert!(!res);
    }

    #[test]
    fn test_pieces_checked_color_true() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"R", 1, 2).unwrap();
        let piece2 = Piece::new(&"r", 2, 1).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = pieces_checked_color(&piece_list);

        assert!(res);
    }

    #[test]
    fn test_pieces_checked_color_false() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"R", 1, 2).unwrap();
        let piece2 = Piece::new(&"D", 2, 1).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = pieces_checked_color(&piece_list);
        assert!(!res);
    }

    #[test]
    fn test_pieces_pass_checks_true() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"R", 1, 2).unwrap();
        let piece2 = Piece::new(&"d", 2, 1).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = pieces_pass_checks(&piece_list);

        assert!(res.is_ok());
    }

    #[test]
    fn test_pieces_pass_checks_false_color() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"R", 1, 2).unwrap();
        let piece2 = Piece::new(&"D", 2, 1).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = pieces_pass_checks(&piece_list);
        assert!(res.is_err());
    }

    #[test]
    fn test_pieces_pass_checks_false_number() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"R", 1, 2).unwrap();
        let piece2 = Piece::new(&"d", 2, 1).unwrap();
        let piece3 = Piece::new(&"t", 4, 1).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);
        piece_list.push(piece3);

        let res = pieces_pass_checks(&piece_list);
        assert!(res.is_err());
    }
}
