# frozen_string_literal: true

class Valkey
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
      def get_v1(key)
        request = CommandRequest::Command.new(
          request_type: CommandRequest::RequestType::Get,
          args_array: CommandRequest::Command::ArgsArray.new(args: [key.b])
        )

        @client.send_command(CommandRequest::Command.encode(request))
      end

      # Get the value of a key.
      #
      # @param [String] key
      # @return [String]
      def get_v2(key)
        @client.get_v2(key)
      end
    end

    # Set the string value of a key.
    def set(key, value)
      request = CommandRequest::Command.new(
        request_type: CommandRequest::RequestType::Set,
        args_array: CommandRequest::Command::ArgsArray.new(args: [key, value])
      )

      @client.send_command(CommandRequest::Command.encode(request))
    end
  end
end
