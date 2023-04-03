/// ChessError is the type of error that can be returned by the program
/// It has a message that is printed when the error is returned
/// It also has a print method that prints the error in a special format
pub struct ChessError<'a> {
    pub message: &'a str,
}

impl ChessError<'_> {
    pub const COLOR: Self = Self {
        message: "It is expected for pieces to be of different colors: 1 black & 1 white",
    };
    pub const NUMBER_PIECES: Self = Self {
        message: "The number of pieces expected is 2 (two)",
    };
    pub const UKNOWN: Self = Self {
        message: "Unkown piece found",
    };
    pub const TABLE_SIZE: Self = Self {
        message: "Chess table does not respect 8x8 size",
    };
    pub const FILE_READING: Self = Self {
        message: "Could not read file: file missing or broken",
    };

    /// prints the error in special format
    pub fn print(&self) {
        println!("ERROR: [{}]", &self.message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_chess_error() {
        let chess_error = ChessError { message: "test" };

        assert_eq!(chess_error.message, "test");
    }

    #[test]
    fn test_create_color_error() {
        let chess_error = ChessError::COLOR;
        assert_eq!(
            chess_error.message,
            "It is expected for pieces to be of different colors: 1 black & 1 white"
        );
    }

    #[test]
    fn test_create_number_pieces_error() {
        let chess_error = ChessError::NUMBER_PIECES;
        assert_eq!(
            chess_error.message,
            "The number of pieces expected is 2 (two)"
        );
    }

    #[test]
    fn test_create_uknown_error() {
        let chess_error = ChessError::UKNOWN;
        assert_eq!(chess_error.message, "Unkown piece found");
    }

    #[test]
    fn test_create_table_size_error() {
        let chess_error = ChessError::TABLE_SIZE;
        assert_eq!(chess_error.message, "Chess table does not respect 8x8 size");
    }

    #[test]
    fn test_create_file_reading_error() {
        let chess_error = ChessError::FILE_READING;
        assert_eq!(
            chess_error.message,
            "Could not read file: file missing or broken"
        );
    }

    #[test]
    fn test_print_error() {
        let chess_error = ChessError::FILE_READING;
        chess_error.print();
    }
}
