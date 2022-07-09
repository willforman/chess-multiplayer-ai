pub mod square;
use square::{Location, Piece, PieceType, Side, Square};
use rustler::{Encoder, Env, Term};

pub struct Board([[Square; 8]; 8]);

impl Board {
    pub fn make_move(&mut self, src: Location, dst: Location) -> bool {
        return match &self.0[src.r][src.c].0 {
            None => false,
            Some(src_piece) => match &self.0[src.r][src.c].0 {
                None => false,
                Some(dst_piece) => {
                    if src_piece.side == dst_piece.side {
                        return false;
                    }
                    self.0[dst.r][dst.c] = self.0[src.r][src.c];
                    self.0[src.r][src.c] = Square(None);
                    return true;
                }
            }
        }
    }

    pub fn print(&self) {
        for r in 0..8 {
            for c in 0..8 {
                print!("{}", self.0[r][c].value());
            }
            println!("");
        }
    }

    pub fn new() -> Board {
        Board([
              [Square(Some(Piece{type_: PieceType::Rook, side: Side::Black})), Square(Some(Piece{type_: PieceType::Knight, side: Side::Black})), Square(Some(Piece{type_: PieceType::Bishop, side: Side::Black})), Square(Some(Piece{type_: PieceType::Queen, side: Side::Black})), Square(Some(Piece{type_: PieceType::King, side: Side::Black})), Square(Some(Piece{type_: PieceType::Bishop, side: Side::Black})), Square(Some(Piece{type_: PieceType::Knight, side: Side::Black})), Square(Some(Piece{type_: PieceType::Rook, side: Side::Black}))],
              [Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black}))], 
              [Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None)], 
              [Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None)], 
              [Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None)], 
              [Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None)],
              [Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::White}))],
              [Square(Some(Piece{type_: PieceType::Rook, side: Side::White})), Square(Some(Piece{type_: PieceType::Knight, side: Side::White})), Square(Some(Piece{type_: PieceType::Bishop, side: Side::White})), Square(Some(Piece{type_: PieceType::Queen, side: Side::White})), Square(Some(Piece{type_: PieceType::King, side: Side::White})), Square(Some(Piece{type_: PieceType::Bishop, side: Side::White})), Square(Some(Piece{type_: PieceType::Knight, side: Side::White})), Square(Some(Piece{type_: PieceType::Rook, side: Side::White}))]
        ])
    }
}

impl Encoder for Board {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        for row in self.0 {
            row.encode(env)
        }
    }
}
