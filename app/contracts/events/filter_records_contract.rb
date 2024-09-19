# frozen_string_literal: true

module Events
  class FilterRecordsContract < ApplicationContract
    params do
      optional(:start_at).array(:time)

      optional(:duration).array(:integer)
      optional(:tickets).array(:integer)

      optional(:q).maybe(:string)

      optional(:price).array(:decimal)

      optional(:tag_ids).array(:integer)
      optional(:area_ids).array(:integer)
      optional(:event_type_ids).array(:integer)
    end
  end
end
