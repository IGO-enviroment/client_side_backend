# frozen_string_literal: true

#
# <Description>
#
class InformationsIndex < Chewy::Index
  include BaseSettings

  settings settings_attributes

  index_scope Information

  field :id, type: :keyword
  field :title, type: :search_as_you_type, analyzer: :clear_text
  field :description, type: :text, analyzer: :clear_text

  field :preview_url, type: :text, value: -> { preview&.url }
  field :publish, type: :boolean
  field :published_at, type: :date
  field :visits, type: :integer, value: -> { visits.count }

  field :contents do
    field :kind, type: :integer
    field :order_value, value: :integer
    field :value, type: :text, analyzer: :clear_text
  end
end
