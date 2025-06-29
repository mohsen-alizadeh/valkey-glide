# frozen_string_literal: true

class ValkeyGlide
  module Commands
    module Strings
      # Set the string value of a key.
      #
      # @param [String] key
      # @param [String] value
      # @param [Hash] options
      #   - `:ex => Integer`: Set the specified expire time, in seconds.
      #   - `:px => Integer`: Set the specified expire time, in milliseconds.
      #   - `:exat => Integer` : Set the specified Unix time at which the key will expire, in seconds.
      #   - `:pxat => Integer` : Set the specified Unix time at which the key will expire, in milliseconds.
      #   - `:nx => true`: Only set the key if it does not already exist.
      #   - `:xx => true`: Only set the key if it already exist.
      #   - `:keepttl => true`: Retain the time to live associated with the key.
      #   - `:get => true`: Return the old string stored at key, or nil if key did not exist.
      # @return [String, Boolean] `"OK"` or true, false if `:nx => true` or `:xx => true`
      def set(key, value, ex: nil, px: nil, exat: nil, pxat: nil, nx: nil, xx: nil, keepttl: nil, get: nil)
        args = [key, value.to_s]
        args << "EX" << Integer(ex) if ex
        args << "PX" << Integer(px) if px
        args << "EXAT" << Integer(exat) if exat
        args << "PXAT" << Integer(pxat) if pxat
        args << "NX" if nx
        args << "XX" if xx
        args << "KEEPTTL" if keepttl
        args << "GET" if get

        if nx || xx
          send_command(RequestType::SET args, &BoolifySet)
        else
          send_command(RequestType::SET, args)
        end
      end

      # Get the value of a key.
      #
      # @param [String] key
      # @return [String]
      def get(key)
        send_command(RequestType::GET, [key])
      end
    end
  end
end
