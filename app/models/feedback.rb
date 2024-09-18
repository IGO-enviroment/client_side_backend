# frozen_string_literal: true

#
# Обратная связь после мероприятия.
#
class Feedback < ApplicationRecord
  belongs_to :event
end
