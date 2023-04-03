use crate::piece_mod::piece::Piece;
use crate::results::fight_result::FightResult;

/// PieceList is a list of pieces that can fight each other. Pieces are given by input. 

pub struct PieceList {
    list: Vec<Piece>,
    pub len: i32,
}

impl Default for PieceList {
    fn default() -> Self {
        Self::init()
    }
}

impl PieceList {
    /// Initializes the struct
    pub fn init() -> PieceList {
        PieceList {
            list: Vec::new(),
            len: 0,
        }
    }

    /// Pushes a piece to the end of the list
    pub fn push(&mut self, piece: Piece) {
        self.list.push(piece);
        self.len += 1;
    }

    /// Returns the first piece.
    pub fn first(&self) -> &Piece {
        &self.list[0]
    }

    /// Returns the second piece
    pub fn second(&self) -> &Piece {
        &self.list[1]
    }

    /// Makes the first two pieces in the list fight each other and returns the result. This result can be:
    /// - E: both wins
    /// - B: white wins
    /// - N: black wins
    /// - P: neither wins
    pub fn fight(&self) -> FightResult {
        let first_movement = self.first().can_capture(self.second());
        let second_movement = self.second().can_capture(self.first());

        let white_wins = first_movement.white_wins() || second_movement.white_wins();
        let black_wins = first_movement.black_wins() || second_movement.black_wins();

        if white_wins && black_wins {
            FightResult::BOTH_WIN
        } else if white_wins {
            FightResult::WHITE_WINS
        } else if black_wins {
            FightResult::BLACK_WINS
        } else {
            FightResult::NEITHER_WIN
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece_mod::piece::Piece;

    #[test]
    fn test_create_piece_list() {
        let piece_list = PieceList::default();

        assert_eq!(piece_list.len, 0);
    }

    #[test]
    fn test_push_piece() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"R", 1, 2).unwrap();
        let piece2 = Piece::new(&"r", 2, 1).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        assert_eq!(piece_list.len, 2);
    }

    #[test]
    fn test_first_piece() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"R", 1, 2).unwrap();
        let piece2 = Piece::new(&"r", 2, 1).unwrap();

        let expected_info = piece1.get_info();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let first_info = piece_list.first().get_info();

        assert_eq!(&first_info, &expected_info);
    }

    #[test]
    fn test_second_piece() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"R", 1, 2).unwrap();
        let piece2 = Piece::new(&"r", 2, 1).unwrap();

        let expected_info = piece2.get_info();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let first_info = piece_list.second().get_info();

        assert_eq!(&first_info, &expected_info);
    }

    // King and Queen
    #[test]
    fn test_fight_black_king_white_queen_both_win() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"R", 3, 3).unwrap();
        let piece2 = Piece::new(&"d", 3, 4).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::BOTH_WIN.output);
    }

    // King and Bishop
    #[test]
    fn test_fight_black_king_white_bishop_white_wins() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"R", 3, 3).unwrap();
        let piece2 = Piece::new(&"a", 1, 1).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::WHITE_WINS.output);
    }

    // King and Knight
    #[test]
    fn test_fight_black_king_white_knight_neither_win() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"R", 3, 3).unwrap();
        let piece2 = Piece::new(&"c", 1, 1).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::NEITHER_WIN.output);
    }

    // King and Rook
    #[test]
    fn test_fight_black_rook_white_king_black_wins() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"T", 3, 5).unwrap();
        let piece2 = Piece::new(&"r", 3, 3).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::BLACK_WINS.output);
    }

    // King and Pawn
    #[test]
    fn test_fight_black_king_white_pawn_both_win() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"R", 3, 3).unwrap();
        let piece2 = Piece::new(&"p", 4, 2).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::BOTH_WIN.output);
    }

    // Queen and Bishop
    #[test]
    fn test_fight_black_queen_white_bishop_both_win() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"D", 5, 6).unwrap();
        let piece2 = Piece::new(&"a", 1, 2).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::BOTH_WIN.output);
    }

    // Queen and Knight
    #[test]
    fn test_fight_black_queen_white_knight_white_wins() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"D", 5, 6).unwrap();
        let piece2 = Piece::new(&"c", 4, 4).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::WHITE_WINS.output);
    }

    // Queen and Rook
    #[test]
    fn test_fight_black_queen_white_rook_neither_win() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"D", 5, 6).unwrap();
        let piece2 = Piece::new(&"t", 3, 0).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::NEITHER_WIN.output);
    }

    // Queen and Pawn
    #[test]
    fn test_fight_black_queen_white_pawn_black_wins() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"D", 5, 6).unwrap();
        let piece2 = Piece::new(&"p", 3, 4).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::BLACK_WINS.output);
    }

    // Bishop and Knight
    #[test]
    fn test_fight_black_knight_white_bishop_white_wins() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"a", 3, 3).unwrap();
        let piece2 = Piece::new(&"C", 1, 1).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::WHITE_WINS.output);
    }

    // Bishop and Rook
    #[test]
    fn test_fight_black_bishop_white_rook_white_wins() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"A", 3, 3).unwrap();
        let piece2 = Piece::new(&"t", 3, 0).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::WHITE_WINS.output);
    }

    // Bishop and Pawn
    #[test]
    fn test_fight_black_bishop_white_pawn_both_win() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"A", 3, 3).unwrap();
        let piece2 = Piece::new(&"p", 2, 2).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::BOTH_WIN.output);
    }

    // Knight and Rook
    #[test]
    fn test_fight_black_knight_white_rook_neither_win() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"C", 0, 0).unwrap();
        let piece2 = Piece::new(&"t", 7, 7).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::NEITHER_WIN.output);
    }

    // Knight and Pawn
    #[test]
    fn test_fight_black_pawn_white_knight_black_wins() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"c", 0, 0).unwrap();
        let piece2 = Piece::new(&"P", 1, 1).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::BLACK_WINS.output);
    }

    // Rook and Pawn
    #[test]
    fn test_fight_black_pawn_white_rook_white_wins() {
        let mut piece_list = PieceList::init();
        let piece1 = Piece::new(&"t", 1, 2).unwrap();
        let piece2 = Piece::new(&"P", 1, 1).unwrap();

        piece_list.push(piece1);
        piece_list.push(piece2);

        let res = piece_list.fight();

        assert_eq!(res.output, FightResult::WHITE_WINS.output);
    }
}
