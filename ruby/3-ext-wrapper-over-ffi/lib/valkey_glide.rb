# frozen_string_literal: true

require_relative "valkey_glide/version"
require_relative "valkey_glide/valkey_glide"

require_relative "valkey_glide/native"
require_relative "valkey_glide/protobuf/command_request_pb"
require_relative "valkey_glide/protobuf/connection_request_pb"
require_relative "valkey_glide/protobuf/response_pb"

class ValkeyGlide
  def initialize
    request = ConnectionRequest::ConnectionRequest.new(
      addresses: [ConnectionRequest::NodeAddress.new(host: "127.0.0.1", port: 6379)]
    )

    @client = Client.new(ConnectionRequest::ConnectionRequest.encode(request))
  end
end
