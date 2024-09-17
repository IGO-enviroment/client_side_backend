# frozen_string_literal: true

#
# Мейлер для пользователей.
#
class UserMailer < ApplicationMailer
  default from: Rails.env.production? ? Settings.mailer.from : "<dev@museum.ru>"
  layout "mailer"
  helper "adminland/mailer"

  def code_send(email:, code:)
    @subject = ""
    headers = {
      to: email,
      subject: @subject
    }
    mail(headers)
  end
end
