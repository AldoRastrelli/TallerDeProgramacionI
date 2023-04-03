use crate::color::Color;
use crate::piece_type::piece_type::PieceType;
use crate::position::position::Position;

pub struct Knight {}
impl PieceType for Knight {
    fn string(&self) -> &str {
        "Knight"
    }

    fn is_king(&self) -> bool {
        false
    }

    fn can_capture(&self, p1: &Position, c: &Color, p2: &Position) -> bool {
        let mut result = false;
        let (x, y, u, v) = self.get_position_variables(p1, p2);

        result = result || (u - x == 2 && v - y == 1); // right & up
        result = result || (u - x == 2 && y - v == 1); // right & down
        result = result || (x - u == 2 && v - y == 1); // left & up
        result = result || (x - u == 2 && y - v == 1); // left & down
        result = result || (v - y == 2 && u - x == 1); // up & right
        result = result || (v - y == 2 && x - u == 1); // up & left
        result = result || (y - v == 2 && u - x == 1); // down & right
        result = result || (y - v == 2 && x - u == 1); // down & left

        result
    }
}
