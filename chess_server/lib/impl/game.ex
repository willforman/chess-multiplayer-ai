defmodule ChessServer.Impl.Game do
  
  @type loc :: { integer, integer }
  @type loc_pair :: { loc, loc }
  @type player_pair :: { String.t, String.t }
  @type side :: :white | :black
  @type game_state :: :initializing | :in_progress | :done

  @type t :: %__MODULE__{
    player_white: String,
    player_black: String,
    side_to_move: side,
  }

  defstruct(
    player_white: "",
    player_black: "",
    side_to_move: :white
  ) 

  @spec new_game(Type.player_pair) :: t
  def new_game(player_pair) do
    %__MODULE__{
      player_white: elem(player_pair, 0),
      player_black: elem(player_pair, 1),
    }  
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
