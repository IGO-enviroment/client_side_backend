# frozen_string_literal: true

#
# Веса для постов.
#
class Weight < ApplicationRecord
  TYPE_AND_VALUE = {
    "views" => 1,
    "likes" => 2,
    "admin" => 100
  }.freeze

  belongs_to :event, polymorphic: true
  belongs_to :information, polymorphic: true

  class << self
    #
    # Подсчет веса для просмотров.
    #
    # @param [Integer] uniq_visits Количество уникальный просмотров
    #
    # @return [Integer]
    #
    def calculate_weight_by_views(uniq_visits:)
      raise ArgumentError unless uniq_visits.is_a?(Integer)

      uniq_visits * TYPE_AND_VALUE["views"]
    end
  end
end
