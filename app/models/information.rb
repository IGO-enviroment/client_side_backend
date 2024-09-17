# frozen_string_literal: true

#
# Информационные посты.
#
class Information < ApplicationRecord
  update_index("informations") { self }

  has_many :contents, dependent: :destroy
  has_one_attached :preview
end
