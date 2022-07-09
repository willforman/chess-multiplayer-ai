defmodule ChessServer.Runtime.Server do

  alias ChessServer.Impl.Game

  @type t :: pid()

  use GenServer

  # Client

  def start_link(player_pair) do
    GenServer.start_link(__MODULE__, player_pair)
  end

  # Server

  def init(player_pair) do
    { :ok, Game.new_game(player_pair) }
  end
    
  def handle_call({ :make_move, loc_pair }, _from, game) do
    game = Game.make_move(game, loc_pair)
    { :reply, game, game }
  end
end
