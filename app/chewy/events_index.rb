# frozen_string_literal: true

#
# <Description>
#
class EventsIndex < Chewy::Index
  include BaseSettings

  settings settings_attributes

  index_scope Event

  field :id, type: :keyword
  field :title, type: :search_as_you_type, analyzer: :clear_text
  field :description, type: :text, analyzer: :clear_text

  field :preview_url, type: :text, value: -> { preview&.url }
  field :publish, type: :boolean
  field :start_at, type: :date

  field :tickets, type: :integer

  field :visits, type: :integer, value: -> { visits.count }

  field :area do
    field :id, type: :keyword
    field :title, type: :search_as_you_type, analyzer: :clear_text
    field :description, type: :text, analyzer: :clear_text

    field :publish, type: :boolean
    field :address_value, type: :text
  end

  field :event_type do
    field :id, type: :keyword
    field :title, type: :search_as_you_type, analyzer: :clear_text
    field :description, type: :text
    field :publish, type: :boolean
  end
end
