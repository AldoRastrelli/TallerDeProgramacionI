use chess::errors::chess_error::ChessError;
use chess::utils::parsing::parse_config;
use chess::utils::parsing::parse_pieces;
use std::env;
use std::fs;

/// Recieves a file to be read.
/// Then, it parses it to obtain a PieceList. Pieces have chess piece's behavior.
/// It prints the result of the fight between the pieces.
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_config(&args);

    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(c) => {
            let pieces = parse_pieces(&c);
            if let Some(c) = pieces {
                let result = c.fight();
                result.print();
            }
        }
        Err(_e) => ChessError::FILE_READING.print(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::color::Color;
    use chess::piece_list_mod::piece_list::PieceList;
    use chess::piece_mod::piece::Piece;
}
