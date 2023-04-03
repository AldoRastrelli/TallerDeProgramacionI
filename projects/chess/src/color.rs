use core::fmt::Debug;

/// Represents a color in the chess table: black or white
#[derive(Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}
