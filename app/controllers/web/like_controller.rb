# frozen_string_literal: true

module Web
  #
  # Взаимодейстиве с мероприятиями.
  #
  class LikeController < ApplicationController
    before_action :load_record, only: :create

    def create
      operation = Likes::TriggerOperation.new(input: params, record: @record).call
      return render json: operation.errors, status: :internal_server_error if operation.failure?

      render json: operation.result, status: :ok
    end

    private
      def load_record
        @record = Likes::Repository.fetch_record(params[:id], params[:type])
      end
  end
end
