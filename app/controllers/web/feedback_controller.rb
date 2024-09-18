# frozen_string_literal: true

module Web
  #
  # Обратная связь от посетителей.
  #
  class FeedbackController < ApplicationController
    def show
      render json: {}, status: :ok
    end

    def create
      operation = Feedbacks::CreateOperation.new(input: feedback_params)
      return render json: operation.errors, status: :unprocessable_entity if operation.failure?

      render json: {}, status: :ok
    end

    private
      def feedback_params
        params.require(:feedback).permit(:name, :email, :message)
      end
  end
end
