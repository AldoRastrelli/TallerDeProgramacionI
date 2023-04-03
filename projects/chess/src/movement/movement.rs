use crate::color::Color;

pub struct Movement<'a> {
    pub piece_color: &'a Color,
    pub did_win: bool,
}

impl Movement<'_> {
    pub fn white_wins(&self) -> bool {
        self.piece_color == &Color::White && self.did_win
    }

    pub fn black_wins(&self) -> bool {
        self.piece_color == &Color::Black && self.did_win
    }
}
