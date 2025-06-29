# frozen_string_literal: true

require_relative "valkey/version"
require_relative "valkey/valkey"
require_relative "valkey/protobuf/command_request_pb"
require_relative "valkey/protobuf/connection_request_pb"
require_relative "valkey/protobuf/response_pb"
require_relative "valkey/commands"

#
# Valkey Client
#
class Valkey
  include Commands

  def initialize
    request = ConnectionRequest::ConnectionRequest.new(
      addresses: [ConnectionRequest::NodeAddress.new(host: "127.0.0.1", port: 6379)]
    )

    @client = Client.new(ConnectionRequest::ConnectionRequest.encode(request))
  end
end
