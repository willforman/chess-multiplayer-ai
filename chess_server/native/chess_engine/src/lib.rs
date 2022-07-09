mod board;
use board::Board;

#[rustler::nif]
fn new_board() -> Board {
    return Board::new();
}

rustler::init!("Elixir.ChessServer.Impl.Engine", [new_board]);
