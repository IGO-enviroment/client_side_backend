# frozen_string_literal: true

#
# Мероприятия.
#
class Event < ApplicationRecord
  include ElastiSeattings

  belong_to :area
  belong_to :event_type

  has_many :tags, dependent: :destroy
  has_many :visits, dependent: :destroy
  has_many :weights, dependent: :destroy

  has_one_attached :preview
end
