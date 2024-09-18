# frozen_string_literal: true

module Web
  #
  # Взаимодейстиве с мероприятиями.
  #
  class EventController < ApplicationController
    before_action :load_event, only: :show
    before_action :visits, only: :show

    def index
      operation = Events::FilterRecordsOperation.new(input: params).call
      return render json: operation.errors, status: :internal_server_error if operation.failure?

      render json: operation.result, status: :ok
    end

    def show
      render json: {}, status: :ok
    end

    private
      # Считает посещения мероприятия
      def visits
        Visits::EventVisitsJob.perform_later(ip: request.ip, record_id: @event.id)
      end

      def load_event
        @event = Event.find(params[:id])
      end
  end
end
