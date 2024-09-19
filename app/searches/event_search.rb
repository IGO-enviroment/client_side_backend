# frozen_string_literal: true

class EventSearch
  extend Dry::Initializer
  option :params

  def call
    EventIndex.search(
      query: {
        bool: {
          filter: {}
        }
      }
    )
  end

  private
    def start_filter
      return if params[:start_at]
    end
end