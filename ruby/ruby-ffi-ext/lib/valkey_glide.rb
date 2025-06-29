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

    # client_type = Native::ClientType.new
    # client_type[:tag] = 1 # AsyncClient

    request = ConnectionRequest::ConnectionRequest.encode(request)
    # request_buf = FFI::MemoryPointer.new(:char, request_str.bytesize)
    # request_buf.put_bytes(0, request_str)

    # request_len = request_str.bytesize

    response_ptr = Native.create_client(
      request
    )

    pp response_ptr
  end
end
