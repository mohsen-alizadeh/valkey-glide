# frozen_string_literal: true

require_relative "valkey/version"
require_relative "valkey/valkey"

# class Valkey
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
#  end
#
# #  def get(key)
# #     create_client
# #  end
# end
