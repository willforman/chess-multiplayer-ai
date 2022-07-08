defmodule ChessWeb.Live.Index do

  use ChessWeb, :live_view


  def mount(_params, _session, socket) do
    games = [
      %{
        host_name: "wf8581@gmail.com"
      },
      %{
        host_name: "player2@gmail.com"
      }
    ]
    { :ok, assign(socket, :games, games) }
  end

  def render(assigns) do
    ~H"""
      <div class="index-container">
        <%= live_component __MODULE__.Available_Games, games: assigns.games, id: 1 %>
        <%= live_component __MODULE__.New_Game, id: 2 %>
      </div>
    """
  end
  
end
