# frozen_string_literal: true

module Web
  module Tickets
    #
    # Заказ билетов.
    #
    class OrderController < ApplicationController
      before_action :load_event, only: %i[create show]
      before_action :load_order, only: :show

      #
      # Данные страницы заказа.
      #
      def show
        render json: {}, status: :ok
      end

      def create
        operation = Tickets::OrderCreateOperation.new(input: params, event: @event).call
        return render json: operation.errors, status: :unprocessable_entity if operation.failure?

        record_activity("Заказ билетов", operation)
        render json: {}, status: :ok
      end

      private
        def load_event
          @event = Event.find(params[:event_id])
        end

        def load_order
          @order = Order.find(params[:id])
        end
    end
  end
end
