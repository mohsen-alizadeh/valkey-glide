# frozen_string_literal: true

require "valkey/commands"

class Valkey
  class Client
    include Commands
  end
end
