use crate::color::Color;
use crate::position::position::Position;
use std::ops::Range;

pub trait PieceType {
    fn string(&self) -> &str;
    fn can_capture(&self, p1: &Position, c: &Color, p2: &Position) -> bool;
    fn is_king(&self) -> bool;

    // ******* Diagonal squares

    fn get_result_for_diagonal_squares(
        &self,
        result: bool,
        attacker_pos: &Position,
        victim_pos: &Position,
    ) -> bool {
        let (x, y, u, v) = self.get_position_variables(attacker_pos, victim_pos);
        let n = self.get_range();

        let mut result = result || self.get_result_for_diagonal_squares_up(result, x, y, u, v, &n);
        result = result || self.get_result_for_diagonal_squares_down(result, x, y, u, v, &n);

        result
    }

    fn get_result_for_diagonal_squares_down(
        &self,
        result: bool,
        x: i32,
        y: i32,
        u: i32,
        v: i32,
        n: &Range<i32>,
    ) -> bool {
        let mut result = result || (u - x == y - v && n.contains(&(u - x))); // diagonal -> right & down
        result = result || (x - u == y - v && n.contains(&(x - u))); // diagonal -> left & down

        result
    }

    fn get_result_for_diagonal_squares_up(
        &self,
        result: bool,
        x: i32,
        y: i32,
        u: i32,
        v: i32,
        n: &Range<i32>,
    ) -> bool {
        let mut result = result || (u - x == v - y && n.contains(&(u - x))); // diagonal -> right & up
        result = result || (x - u == v - y && n.contains(&(x - u))); // diagonal -> left & up

        result
    }

    // ******* Cross Squares
    fn get_result_for_cross_squares(
        &self,
        result: bool,
        attacker_pos: &Position,
        victim_pos: &Position,
    ) -> bool {
        let (x, y, u, v) = self.get_position_variables(attacker_pos, victim_pos);
        let n = self.get_range();

        let mut result = result || y == v && n.contains(&(u - x)); // right
        result = result || y == v && n.contains(&(x - u)); // left
        result = result || x == u && n.contains(&(v - y)); // up
        result = result || x == u && n.contains(&(y - v)); // down

        result
    }

    // ******* Auxiliar Funcs
    fn get_position_variables(&self, p1: &Position, p2: &Position) -> (i32, i32, i32, i32) {
        (p1.x, p1.y, p2.x, p2.y)
    }

    fn get_range(&self) -> Range<i32> {
        let max = if self.is_king() { 2 } else { 8 };
        1..max
    }
}
