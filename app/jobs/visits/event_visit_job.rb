# frozen_string_literal: true

module Visits
  #
  # <Description>
  #
  class EventVisitsJob < ApplicationJob
    def perform(ip:, record_id:)
      event = Event.find_by(id: record_id)
      return if event.blank?

      Visits::Repository.update_entry_ip_by_record(ip: ip, record: event)
    end
  end
end
