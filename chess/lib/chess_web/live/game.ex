defmodule ChessWeb.Live.Game do

  use ChessWeb, :live_view


  def mount(%{"id" => id}, _session, socket) do
    game = Chess.Game.get(id)
    { :ok, socket }
  end

  def render(assigns) do
    ~H"""
      Hello game
    """
  end
  
end
