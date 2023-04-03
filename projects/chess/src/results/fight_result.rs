#[derive(Debug)]

/// Represents the result of a fight between two pieces.
/// Possible Values:
/// - B: White wins
/// - N: Black wins
/// - E: Both win
/// - P: Neither win
pub struct FightResult<'a> {
    pub output: &'a str,
}

impl FightResult<'_> {
    pub const WHITE_WINS: Self = Self { output: "B" };
    pub const BLACK_WINS: Self = Self { output: "N" };
    pub const BOTH_WIN: Self = Self { output: "E" };
    pub const NEITHER_WIN: Self = Self { output: "P" };

    /// Prints the result of the fight
    pub fn print(&self) {
        println!("{}", &self.output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_wins() {
        assert_eq!(FightResult::WHITE_WINS.output, "B");
    }

    #[test]
    fn test_black_wins() {
        assert_eq!(FightResult::BLACK_WINS.output, "N");
    }

    #[test]
    fn test_both_win() {
        assert_eq!(FightResult::BOTH_WIN.output, "E");
    }

    #[test]
    fn test_neither_win() {
        assert_eq!(FightResult::NEITHER_WIN.output, "P");
    }
}
