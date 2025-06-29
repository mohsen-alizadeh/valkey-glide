# frozen_string_literal: true

require_relative "valkey/version"
require_relative "valkey/valkey"
require_relative "valkey/protobuf/command_request_pb"
require_relative "valkey/protobuf/connection_request_pb"
require_relative "valkey/protobuf/response_pb"

class ValkeyGlide2
  def get(key)
    request = CommandRequest::Command.new(
      request_type: CommandRequest::RequestType::Get
    )

    execute(CommandRequest::Command.encode(request))
  end
end
