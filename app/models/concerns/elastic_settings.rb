# frozen_string_literal: true

#
# Настройки еластика для моделей
#
module Elastic
  module BaseSettings
    extend ActiveSupport::Concern

    included do
      include Elasticsearch::Model
      include Elasticsearch::Model::Callbacks
    end

    # rubocop:disable Metrics/BlockLength
    class_methods do
      def settings_attributes
        {
          index: {
            analysis: {
              analyzer: {
                clear_text: {
                  type: :custom,
                  tokenizer: :standard,
                  char_filter: [
                    :html_strip
                  ],
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
        }
      end
    end
    # rubocop:enable Metrics/BlockLength
  end
end