defmodule ChessServer do
  alias ChessServer.Impl.Game
  alias ChessServer.Runtime.Server

  @opaque game_pid :: Server.t

  @spec new_game() :: game_pid
  def new_game() do
    { :ok, pid } = ChessServer.Runtime.Application.start_game()
    pid
  end

  @spec make_move(game_pid, Game.loc_pair) :: Game.t
  def make_move(game_pid, loc_pair) do
    GenServer.call(game_pid, { :make_move, loc_pair })
  end

end
