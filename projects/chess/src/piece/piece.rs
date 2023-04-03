use crate::bishop::bishop::Bishop;
use crate::color::Color;
use crate::king::king::King;
use crate::knight::knight::Knight;
use crate::movement::movement::Movement;
use crate::pawn::pawn::Pawn;
use crate::piece_type::piece_type::PieceType;
use crate::position::position::Position;
use crate::queen::queen::Queen;
use crate::rook::rook::Rook;

pub struct Piece {
    pub color: Color,
    position: Position,
    piece_type: Box<dyn PieceType>,
}

impl Piece {
    pub fn new(name: &str, row: i32, col: i32) -> Option<Piece> {
        let piece_type_given = Self::get_piece_type(name);

        match piece_type_given {
            Some(p) => Some(Piece {
                color: Self::get_piece_color(name),
                position: Position { x: row, y: col },
                piece_type: p,
            }),
            None => None,
        }
    }

    pub fn print_info(&self) {
        println!(
            "Type: {}, Color: {:?}, Posicion: {:?}",
            self.piece_type.string(),
            self.color,
            self.get_position()
        );
    }

    pub fn get_position(&self) -> [i32; 2] {
        self.position.get_pair()
    }

    fn get_piece_color(name: &str) -> Color {
        if name.to_lowercase() == name {
            Color::White
        } else {
            Color::Black
        }
    }

    fn get_piece_type(name: &str) -> Option<Box<dyn PieceType>> {
        let name_up = name.to_uppercase();
        match &name_up[..] {
            "R" => Some(Box::new(King {})),
            "D" => Some(Box::new(Queen {})),
            "A" => Some(Box::new(Bishop {})),
            "C" => Some(Box::new(Knight {})),
            "T" => Some(Box::new(Rook {})),
            "P" => Some(Box::new(Pawn {})),
            _ => None,
        }
    }

    pub fn can_capture(&self, other_piece: &Piece) -> Movement {
        let capture =
            &self
                .piece_type
                .can_capture(&self.position, &self.color, &other_piece.position);

        Movement {
            piece_color: &self.color,
            did_win: *capture,
        }
    }
}
