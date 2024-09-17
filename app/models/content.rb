# frozen_string_literal: true

#
# Данные поста.
#
class Content < ApplicationRecord
  TYPES = %i[text media]

  belongs_to :information, polymorphic: true

  has_one_attached :file

  validates :file, attached: true, content_type: {
    content_type: %w[
      application/msword
      text/csv
      image/gif
      image/jpeg
      audio/mpeg
      video/mp4
      video/webm
      image/png
      audio/mp3
    ],
    less_than: 50.gigabyte,
  }, if: { kind.present? && kind.media? }

  enum :kind, {
    text: 0, media: 1
  }, default: :text

  update_index("informations") { information }
end
