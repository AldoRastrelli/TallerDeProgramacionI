use crate::fight_result::fight_result::FightResult;
use crate::piece::piece::Piece;

pub struct PieceList {
    list: Vec<Piece>,
    pub len: i32,
}

impl PieceList {
    pub fn new() -> PieceList {
        PieceList {
            list: Vec::new(),
            len: 0,
        }
    }

    pub fn push(&mut self, piece: Piece) {
        self.list.push(piece);
        self.len += 1;
    }

    pub fn first(&self) -> &Piece {
        &self.list[0]
    }

    pub fn second(&self) -> &Piece {
        &self.list[1]
    }

    pub fn fight(&self) {
        let first_movement = self.first().can_capture(self.second());
        let second_movement = self.second().can_capture(self.first());

        let white_wins = first_movement.white_wins() || second_movement.white_wins();
        let black_wins = first_movement.black_wins() || second_movement.black_wins();

        if white_wins && black_wins {
            FightResult::BOTH_WIN.print();
        } else if white_wins {
            FightResult::WHITE_WINS.print();
        } else if black_wins {
            FightResult::BLACK_WINS.print();
        } else {
            FightResult::NEITHER_WIN.print();
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_fight() {

//     }
// }
