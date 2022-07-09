mod sq_based;
use sq_based::board::Board;

#[rustler::nif]
fn new_board() -> Board {
    return Board::new();
}

// #[rustler::nif]
// fn make_move(board: Vec<)

rustler::init!("Elixir.ChessServer.Impl.Engine", [new_board]);
