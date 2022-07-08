defmodule ChessWeb.Live.Index.Available_Games do

  use ChessWeb, :live_component

  def mount(socket) do
    { :ok, socket }
  end

  def render(assigns) do
    ~H"""
      <div class="available_games">
        <h2>Available Games</h2>
        <ul>
          <%= for game <- @games do %>
          <li>
            <%= game.host_name %>
          </li>
          <% end %>
        </ul>
      </div>
    """
  end
  
end
