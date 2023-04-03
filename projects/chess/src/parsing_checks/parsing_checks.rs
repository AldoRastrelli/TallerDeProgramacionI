use crate::chess_error::chess_error::ChessError;
use crate::color::Color;
use crate::piece::piece::Piece;
use crate::piece_list::piece_list::PieceList;
use std::str::SplitWhitespace;

pub fn parse_config(args: &[String]) -> &str {
    &args[1]
}

// TODO: len > 30
pub fn parse_pieces(table: &str) -> Option<PieceList> {
    let mut pieces = PieceList::new();
    let rows = table.split('\n');
    let mut curr_row = 7;
    let mut curr_col;

    for r in rows {
        let cols: SplitWhitespace = r.split_whitespace();
        curr_col = 0;
        for c in cols {
            if curr_col > 7 {
                ChessError::TABLE_SIZE.print();
                return None;
            }

            if c != "_" {
                // TODO this should catch piece type
                let piece = Piece::new(c, curr_col, curr_row);
                match piece {
                    Some(p) => {
                        pieces.push(p);
                    }
                    None => {
                        ChessError::UKNOWN.print();
                        return None;
                    }
                }
            }
            curr_col += 1;
        }
        curr_row -= 1;
    }

    if !pieces_pass_checks(&pieces) {
        return None;
    }

    Some(pieces)
}

fn pieces_pass_checks(pieces: &PieceList) -> bool {
    if pieces.len != 2 {
        ChessError::NUMBER_PIECES.print();
        return false;
    }

    if !pieces_checked_color(&pieces) {
        ChessError::COLOR.print();
        return false;
    }

    return true;
}

fn pieces_checked_color(pieces: &PieceList) -> bool {
    let first_piece = &pieces.first();
    let second_piece = &pieces.second();

    let black_and_white = first_piece.color == Color::Black && second_piece.color == Color::White;
    let white_and_black = first_piece.color == Color::White && second_piece.color == Color::Black;

    (black_and_white) || (white_and_black)
}
