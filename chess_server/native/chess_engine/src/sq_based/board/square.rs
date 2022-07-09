use rustler::{Encoder,Env,Term};

pub struct Location {
    pub r: usize,
    pub c: usize
}

#[derive(Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(PartialEq, Clone, Copy)]
pub enum Side {
    White,
    Black
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub type_: PieceType,
    pub side: Side,
}

impl Piece {
    fn value(&self) -> char {
        let ch = match self.type_ {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };
        return match self.side {
            Side::White => ch.to_ascii_uppercase(),
            Side::Black => ch,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Square(pub Option<Piece>);

impl Square {
    pub fn value(&self) -> char {
        return match self.0 {
            Some(piece) => piece.value(),
            None => 'x'
        }
    }
}

impl Encoder for Square {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self.0 {
            Some(piece) => {
                let piece_val = match piece.type_ {
                    PieceType::Pawn => 1,
                    PieceType::Knight => 2,
                    PieceType::Bishop => 3,
                    PieceType::Rook => 4,
                    PieceType::Queen => 5,
                    PieceType::King => 6,
                };
                piece_val * match piece.side {
                    Side::White => 1,
                    Side::Black => -1
                }
            } 
            None => 0
        }.encode(env)
    }
}
