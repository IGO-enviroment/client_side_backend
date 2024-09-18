# frozen_string_literal: true

module Web
  module Callbacks
    #
    # Колбыэки.
    #
    class YookasssaController < ApplicationController
      before_action :load_order, only: :show

      def show
        operation = Orders::Yookasssa::CallbackOperation.new(input: params, order: @order).call
        record_activity("Колбэк от юкассы", operation)
        render json: {}, status: :ok
      end

      private
        def load_order
          @order = Order.find(params[:id])
        end
    end
  end
end
