# frozen_string_literal: true

module Visits
  #
  # <Description>
  #
  class Repository
    class << self
      def update_entry_ip_by_record(ip:, record:)
        visit = Visit.find_or_initialize_by(ip: ip, model: record)
        return visit.save! if visit.new_record?

        visit.increment!(:entered)
      end
    end
  end
end
