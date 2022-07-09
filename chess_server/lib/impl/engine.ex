defmodule ChessServer.Impl.Engine do
  use Rustler, otp_app: :chess_server, crate: "chess_engine"

  def new_board(), do: error()

  defp error(), do: :erlang.nif_error(:nif_not_loaded)
  
end
