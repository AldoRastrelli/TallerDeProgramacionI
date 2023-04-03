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

    pub fn print(&self) {
        println!("ERROR: [{}]", &self.message);
    }
}
