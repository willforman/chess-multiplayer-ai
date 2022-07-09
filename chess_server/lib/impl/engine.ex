defmodule ChessServer.Impl.Engine do
  use Rustler, otp_app: :chess_server, crate: "chess_engine"

  @spec add(integer, integer) :: integer
  def add(_a, _b), do: error()

  def new_arr(), do: error()

  def double(_a), do: error()

  defp error(), do: :erlang.nif_error(:nif_not_loaded)
  
end
