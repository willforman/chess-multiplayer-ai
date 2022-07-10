defmodule ChessServer.Impl.Engine do
  use Rustler, otp_app: :chess_server, crate: "chess_engine"

  def new_board(), do: error()
  def get_square(_board, _loc), do: error()
  def make_move(_board, _src, _dst), do: error()

  defp error(), do: :erlang.nif_error(:nif_not_loaded)
  
end
