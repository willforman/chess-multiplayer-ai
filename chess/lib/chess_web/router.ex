defmodule ChessWeb.Router do
  use ChessWeb, :router

  import ChessWeb.UserAuth

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, {ChessWeb.LayoutView, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
    plug :fetch_current_user
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", ChessWeb do
    pipe_through :browser

    live "/", Live.Index
  end

  # Other scopes may use custom stacks.
  # scope "/api", ChessWeb do
  #   pipe_through :api
  # end

  # Enables the Swoosh mailbox preview in development.
  #
  # Note that preview only shows emails that were sent by the same
  # node running the Phoenix server.
  if Mix.env() == :dev do
    scope "/dev" do
      pipe_through :browser

      forward "/mailbox", Plug.Swoosh.MailboxPreview
    end
  end

  ## Authentication routes

  scope "/", ChessWeb do
    pipe_through [:browser, :redirect_if_user_is_authenticated]

    get "/user/register", UserRegistrationController, :new
    post "/user/register", UserRegistrationController, :create
    get "/user/log_in", UserSessionController, :new
    post "/user/log_in", UserSessionController, :create
    get "/user/reset_password", UserResetPasswordController, :new
    post "/user/reset_password", UserResetPasswordController, :create
    get "/user/reset_password/:token", UserResetPasswordController, :edit
    put "/user/reset_password/:token", UserResetPasswordController, :update
  end

  scope "/", ChessWeb do
    pipe_through [:browser, :require_authenticated_user]

    get "/user/settings", UserSettingsController, :edit
    put "/user/settings", UserSettingsController, :update
    get "/user/settings/confirm_email/:token", UserSettingsController, :confirm_email
  end

  scope "/", ChessWeb do
    pipe_through [:browser]

    delete "/user/log_out", UserSessionController, :delete
    get "/user/confirm", UserConfirmationController, :new
    post "/user/confirm", UserConfirmationController, :create
    get "/user/confirm/:token", UserConfirmationController, :edit
    post "/user/confirm/:token", UserConfirmationController, :update
  end
end
