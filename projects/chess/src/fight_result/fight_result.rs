pub struct FightResult<'a> {
    pub output: &'a str,
}

impl FightResult<'_> {
    pub const WHITE_WINS: Self = Self { output: "B" };
    pub const BLACK_WINS: Self = Self { output: "N" };
    pub const BOTH_WIN: Self = Self { output: "E" };
    pub const NEITHER_WIN: Self = Self { output: "P" };

    pub fn print(&self) {
        println!("{}", &self.output);
    }
}
