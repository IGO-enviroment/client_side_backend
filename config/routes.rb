Rails.application.routes.draw do
  get "/health", to: proc { [200, {}, ["OK"]] }

  constraints format: :json do
    namespace :museum do
      resources :search, only: :show
    end
  end
end
