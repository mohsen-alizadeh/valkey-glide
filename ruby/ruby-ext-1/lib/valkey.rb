# frozen_string_literal: true

require_relative "valkey/version"
require_relative "valkey/valkey"
require_relative "valkey/protobuf/command_request_pb"
require_relative "valkey/protobuf/connection_request_pb"
require_relative "valkey/protobuf/response_pb"

class Valkey
  # def initialize(options = {})
  #     puts "initialize in Ruby"
  #
  #     request = ConnectionRequest::ConnectionRequest.new(
  #         addresses: [ConnectionRequest::NodeAddress.new(host: '127.0.0.1', port: 6379)]
  #     )
  #
  #     create_client(ConnectionRequest::ConnectionRequest.encode(request))
  # end
#     # SERVER_URL_OPTIONS = %i(url host port path).freeze
#
# #  def initialize(options = {})
# #     # @options = options.dup
# #     # @options[:reconnect_attempts] = 1 unless @options.key?(:reconnect_attempts)
# #     # if ENV["REDIS_URL"] && SERVER_URL_OPTIONS.none? { |o| @options.key?(o) }
# #     #   @options[:url] = ENV["REDIS_URL"]
# #     # end
# #     # inherit_socket = @options.delete(:inherit_socket)
# #     # @subscription_client = nil
# #
# #     # @client = initialize_client(@options)
# #     # @client.inherit_socket! if inherit_socket
# #   end
#
#  def set(key, value)
#     CommandRequest::Command.new(request_type: CommandRequest::RequestType::Set)
#
#     pp
#  end

# def test()
#     puts "TEST --- "
# end

 def get(key)
     request = CommandRequest::Command.new(
        request_type: CommandRequest::RequestType::Get
     )

     pp (CommandRequest::Command.encode(request))

     execute(CommandRequest::Command.encode(request))
 end
end
