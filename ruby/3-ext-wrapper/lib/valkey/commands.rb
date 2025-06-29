# frozen_string_literal: true

require_relative "commands/strings"

class Valkey
  module Commands
    include Strings
  end
end
