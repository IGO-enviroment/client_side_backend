# frozen_string_literal: true

module Visits
  #
  # <Description>
  #
  class EventRecalculateJob < ApplicationJob
    def perform(record_id:)
      event = Event.find_by(id: record_id)
      return if event.blank?

      visits = event.visits.count
      event.weight_by_views.update!(value: Weight.calculate_weight_by_views(visits: visits))
    end
  end
end
