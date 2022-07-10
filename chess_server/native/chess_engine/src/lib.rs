mod sq_based;
use rustler::Atom;
use sq_based::board::Board;
use sq_based::board::square::{Location,Square};

mod atoms {
    rustler::atoms! {
        ok,
        error
    }
}

#[rustler::nif]
fn new_board() -> Board {
    return Board::new();
}

#[rustler::nif]
fn get_square(board: Board, loc: Location) -> Square {
    board.get_square(loc)
}

#[rustler::nif]
fn make_move(board: Board, src: Location, dst: Location) -> (Atom, Option<Board>) {
    let mut board: Board = board.clone().into();
    match board.make_move(src, dst) {
        Ok(()) => (atoms::ok(), Some(board)),
        Err(_) => (atoms::error(), None)
    }
}

rustler::init!("Elixir.ChessServer.Impl.Engine", [new_board, get_square, make_move]);
