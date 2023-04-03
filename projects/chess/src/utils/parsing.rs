use crate::errors::chess_error::ChessError;
use crate::piece_list_mod::piece_list::PieceList;
use crate::piece_mod::piece::Piece;
use crate::utils::checks::result_is_valid;
use crate::utils::checks::square_has_piece;
use std::str::SplitWhitespace;

/// Returns the first argument needed
pub fn parse_config(args: &[String]) -> &str {
    &args[1]
}

/// Parses the table and returns a PieceList if it is valid. None otherwise.
/// It also prints the error if the table is invalid.
/// It iterates through the rows and calls parse_row for each row.
/// It also calls result_is_valid to check if the number of rows iterated was 8 and the Pieces found are valid.
pub fn parse_pieces(table: &str) -> Option<PieceList> {
    let mut pieces = PieceList::default();
    let rows = table.split('\n');
    let mut curr_row = 7;
    let mut curr_col;

    for r in rows {
        curr_col = 0;
        match parse_row(r, curr_row, &mut curr_col, &mut pieces) {
            Ok(_) => {
                curr_row -= 1;
            }
            Err(e) => {
                e.print();
                return None;
            }
        }
    }

    match result_is_valid(curr_row, &pieces) {
        Ok(_) => Some(pieces),
        Err(e) => {
            e.print();
            None
        }
    }
}

/// Parses the row given and returns Ok if the row is valid. Err otherwise.
/// If valid, it adds the Pieces found to the PieceList.
/// It also updates the current row and column.
/// The current row is updated by decrementing it.
/// The current column is updated by incrementing it.
fn parse_row<'a>(
    row: &'a str,
    curr_row: i32,
    curr_col: &'a mut i32,
    pieces: &'a mut PieceList,
) -> Result<&'a str, &'a ChessError<'a>> {
    let squares: SplitWhitespace = row.split_whitespace();

    for s in squares {
        if *curr_col > 7 {
            return Err(&ChessError::TABLE_SIZE);
        }

        if square_has_piece(s) {
            let piece = Piece::new(s, *curr_col, curr_row);
            match piece {
                Some(p) => {
                    pieces.push(p);
                }
                None => {
                    return Err(&ChessError::UKNOWN);
                }
            }
        }
        *curr_col += 1;
    }
    Ok("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::piece_list_mod::piece_list::PieceList;
    use crate::piece_mod::piece::Piece;

    #[test]
    fn test_parse_config() {
        let args = vec![String::from("chess"), String::from("config")];
        assert_eq!(parse_config(&args), "config");
    }

    #[test]
    fn test_parse_pieces() {
        let table = "_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ A _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ _ _ _ _ _\n_ _ _ t _ _ _ _";

        let pieces = parse_pieces(table).unwrap();

        assert_eq!(pieces.len, 2);
    }
}
