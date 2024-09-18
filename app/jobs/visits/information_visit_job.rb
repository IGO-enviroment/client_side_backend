# frozen_string_literal: true

module Visits
  #
  # <Description>
  #
  class InformationVisitsJob < ApplicationJob
    def perform(ip:, record_id:)
      info = Information.find_by(id: record_id)
      return if info.blank?

      Visits::Repository.update_entry_ip_by_record(ip: ip, record: info)
    end
  end
end
