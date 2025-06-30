# frozen_string_literal: true

class ValkeyGlide
  #
  # Valkey Commands
  #
  module Commands
    #
    # String commands
    #
    module Strings
      # Get the value of a key.
      #
      # @param [String] key
      # @return [String]
      def get(key)
        execute_command("GET", key)
      end
    end
  end
end
