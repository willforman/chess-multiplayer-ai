pub mod square;
pub mod error;

use error::MovePieceError;
use square::{Location, Piece, PieceType, Side, Square};
use rustler::{ Decoder, Encoder, Env, Term, NifResult, ListIterator };

#[derive(Clone, Copy)]
pub struct Board([Square; 64]);

impl std::ops::Index<usize> for Board {
    type Output = [Square];

    fn index(&self, row: usize) -> &[Square] {
        let start = 8 * row;
        &self.0[start .. start + 8]
    }
}

impl std::ops::IndexMut<usize> for Board {
    fn index_mut(&mut self, row: usize) -> &mut [Square] {
        let start = 8 * row;
        &mut self.0[start .. start + 8]
    }
}

impl Board {
    pub fn get_square(&self, loc: Location) -> Square {
        self[loc.r][loc.c]
    }

    pub fn make_move(&mut self, src: Location, dst: Location) -> Result<(), MovePieceError> {
        return match self[src.r][src.c].0 {
            None => Err(MovePieceError::NoSrcPiece(src)),
            Some(src_piece) => {
                match self[dst.r][dst.c].0 {
                    Some(dst_piece) => {
                        if dst_piece.side == src_piece.side {
                            Err(MovePieceError::SrcDstPiecesSideSame(src, dst))
                        } else {
                            self[dst.r][dst.c] = self[src.r][src.c];
                            self[src.r][src.c] = Square(None);
                            Ok(())
                        }
                    }
                    None => {
                        self[dst.r][dst.c] = self[src.r][src.c];
                        self[src.r][src.c] = Square(None);
                        Ok(())
                    }
                }
            }
            // Some(src_piece) => match &self[src.r][src.c].0 {
            //     None => Err(()),
            //     Some(dst_piece) => {
            //         if src_piece.side == dst_piece.side {
            //             return Err(());
            //         }
            //         self[dst.r][dst.c] = self[src.r][src.c];
            //         self[src.r][src.c] = Square(None);
            //         Ok(())
            //     }
            // }
        }
    }

    pub fn print(&self) {
        for r in 0..8 {
            for c in 0..8 {
                print!("{}", self[r][c].char_value());
            }
            println!("");
        }
    }

    pub fn new() -> Board {
        Board([
              Square(Some(Piece{type_: PieceType::Rook, side: Side::Black})), Square(Some(Piece{type_: PieceType::Knight, side: Side::Black})), Square(Some(Piece{type_: PieceType::Bishop, side: Side::Black})), Square(Some(Piece{type_: PieceType::Queen, side: Side::Black})), Square(Some(Piece{type_: PieceType::King, side: Side::Black})), Square(Some(Piece{type_: PieceType::Bishop, side: Side::Black})), Square(Some(Piece{type_: PieceType::Knight, side: Side::Black})), Square(Some(Piece{type_: PieceType::Rook, side: Side::Black})),
              Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::Black})),
              Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), 
              Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), 
              Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), 
              Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None), Square(None),
              Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})), Square(Some(Piece{type_: PieceType::Pawn, side: Side::White})),
              Square(Some(Piece{type_: PieceType::Rook, side: Side::White})), Square(Some(Piece{type_: PieceType::Knight, side: Side::White})), Square(Some(Piece{type_: PieceType::Bishop, side: Side::White})), Square(Some(Piece{type_: PieceType::Queen, side: Side::White})), Square(Some(Piece{type_: PieceType::King, side: Side::White})), Square(Some(Piece{type_: PieceType::Bishop, side: Side::White})), Square(Some(Piece{type_: PieceType::Knight, side: Side::White})), Square(Some(Piece{type_: PieceType::Rook, side: Side::White}))
        ])
    }
}

impl Encoder for Board {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        self.0.encode(env)
    }
}

impl<'a> Decoder<'a> for Board {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let mut arr: [Square; 64] = [Square(None); 64];
        let iter = term.decode::<ListIterator>()?;
        for (idx, item) in iter.enumerate() {
            arr[idx] = item.decode::<Square>()?;
        }
        Ok(Board(arr))
    }
}
