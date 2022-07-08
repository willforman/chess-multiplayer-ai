defmodule ChessWeb.Live.Index.New_Game do

  use ChessWeb, :live_component

  def mount(socket) do
    { :ok, socket }
  end

  def render(assigns) do
    ~H"""
      <div class="new_game">
        <h2>New Game</h2>
        <button>Multiplayer</button>
        <button>Computer</button>
      </div>
    """
  end
  
end
