# frozen_string_literal: true

module Events
  class FilterRecordsOperation
    extend Dry::Initializer
    option :input
    option :contract, default: -> { Events::FilterRecordsContract }

    def call
      validate_params
      hits = EventsSearch.new(params: input).call
      @result = HitsSerialize.from_response(hits.result_response)
      self
    end

    def success?
      @errors.blank? 
    end

    def failure?
      @errors.present?
    end

    def result
      success? ? @result : @errors
    end

    def errors
      @errors
    end

    private
      def validate_params
        params_validator = contract.call(input)
        @input = params_validator.to_h.with_indifferent_access
      end

      def setup_errors(values)
        @errors = ErrorsSerializer.from_contract(values)
      end
  end
end
