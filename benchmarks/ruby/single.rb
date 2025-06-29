# frozen_string_literal: true

require_relative "setup"

redis_client = RedisClient.new
redis = Redis.new
valkey = Valkey.new


redis_client.call("SET", "key", "value")

benchmark("GET key") do |x|
  x.report("valkey-v1") { valkey.get_v1("key") }
  x.report("valkey-v2") { valkey.get_v2("key") }
  x.report("redis") { redis.get("key") }
  x.report("redis-client") { redis_client.call("GET", "key") }
end
