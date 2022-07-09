defmodule ChessServer.Impl.Game do
  
  @type loc :: { integer, integer }
  @type loc_pair :: { loc, loc }
  @type side :: :white | :black
  @type game_state :: :initializing | :in_progress | :done

  @type t :: %__MODULE__{
    side_to_move: side,
  }

  defstruct(
    side_to_move: :white
  ) 

  @spec new_game() :: t
  def new_game() do
    %__MODULE__{}
  end

  @spec make_move(t, loc_pair) :: t
  def make_move(game = %{ side_to_move: side_to_move }, loc_pair) when side_to_move == :white do
    inspect(loc_pair)
    %{game | side_to_move: :black }
  end

  def make_move(game, loc_pair) do
    inspect(loc_pair)
    %{game | side_to_move: :white }
  end
  
end
