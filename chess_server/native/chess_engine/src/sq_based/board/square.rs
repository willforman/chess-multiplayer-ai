use rustler::{ Decoder, Encoder, Env, Term, NifResult };

#[derive(Debug)]
pub struct Location {
    pub r: usize,
    pub c: usize
}

impl Location {
    fn from_tuple((r, c): (usize, usize)) -> Location {
        Location { r, c }
    }
}

impl<'a> Decoder<'a> for Location {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let loc_tuple = term.decode::<(usize, usize)>()?;
        Ok(Location::from_tuple(loc_tuple))
    }
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

impl PieceType {
    fn char_value(&self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        }
    }

    fn int_value(&self) -> i32 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Knight => 2,
            PieceType::Bishop => 3,
            PieceType::Rook => 4,
            PieceType::Queen => 5,
            PieceType::King => 6,
        }
    }

    fn from_int(int_val: i32) -> PieceType {
        match int_val {
            1 => PieceType::Pawn,
            2 => PieceType::Knight,
            3 => PieceType::Bishop,
            4 => PieceType::Rook,
            5 => PieceType::Queen,
            6 => PieceType::King,
            _ => panic!("Invalid int given for piecetype")
        }
    }
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
    fn char_value(&self) -> char {
        let ch = self.type_.char_value();
        match self.side {
            Side::White => ch.to_ascii_uppercase(),
            Side::Black => ch,
        }
    }

    fn int_value(&self) -> i32 {
        let int_val = self.type_.int_value(); 
        match self.side {
            Side::White => int_val,
            Side::Black => int_val * -1
        }
    }

    fn from_int(int_val: i32) -> Piece {
        if int_val < 0 {
            Piece { type_: PieceType::from_int(int_val * -1), side: Side::Black }
        } else {
            Piece { type_: PieceType::from_int(int_val), side: Side::White }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Square(pub Option<Piece>);

impl Square {
    pub fn char_value(&self) -> char {
        match self.0 {
            Some(piece) => piece.char_value(),
            None => 'x'
        }
    }

    pub fn int_value(&self) -> i32 {
        match self.0 {
            Some(piece) => piece.int_value(),
            None => 0
        }
    }

    pub fn from_int(int_val: i32) -> Square {
        if int_val != 0 {
            Square(Some(Piece::from_int(int_val)))
        } else {
            Square(None)
        }
    }
}

impl Encoder for Square {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        self.int_value().encode(env)
    }
}

impl<'a> Decoder<'a> for Square {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let int_val = term.decode()?;
        Ok(Square::from_int(int_val))
    }
}
