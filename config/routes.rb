Rails.application.routes.draw do
  get "/health", to: proc { [200, {}, ["OK"]] }

  constraints format: :json do
    namespace :web do
      resources :session, only: %i[create show]

      scope :tickets do
        resources :order, only: %i[create show index]
      end

      resources :information, only: %i[index show]
      resources :event, only: %i[index show]

      resources :area, only: %i[index show]

      resources :like, only: %i[create]

      resources :billbaord, only: %i[show]

      resources :search, only: :show
    end
  end
end
