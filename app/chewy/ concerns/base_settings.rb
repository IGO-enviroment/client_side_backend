# frozen_string_literal: true

module BaseSettings
  extend ActiveSupport::Concern

  class_methods do
    def settings_attributes
      {
        analysis: {
          analyzer: {
            clear_text: {
              tokenizer: :standard,
              type: :custom,
              filter: %i[lowercase russian_stop russian_stemmer english_stop english_stemmer]
            }
          },
          filter: {
            russian_stop: {
              type: "stop",
              stopwords: "_russian_"
            },
            russian_stemmer: {
              type: "stemmer",
              language: "russian"
            },
            english_stop: {
              type: "stop",
              stopwords: "_english_"
            },
            english_stemmer: {
              type: "stemmer",
              language: "english"
            }
          }
        }
      }
    end
  end
end
