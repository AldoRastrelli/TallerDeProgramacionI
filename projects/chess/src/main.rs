use chess::chess_error::chess_error::ChessError;
use chess::parsing_checks::parsing_checks::parse_config;
use chess::parsing_checks::parsing_checks::parse_pieces;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_config(&args);

    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(c) => {
            let pieces = parse_pieces(&c);
            match pieces {
                Some(c) => {
                    c.fight();
                }
                None => {}
            }
        }
        Err(e) => ChessError::FILE_READING.print(),
    }
}
