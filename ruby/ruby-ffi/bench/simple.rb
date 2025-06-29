
require_relative '../lib/valkey-glide'

require 'benchmark/ips'

require 'redis'

valkey = ValkeyGlide.new

redis = Redis.new

Benchmark.ips do |x|
  x.report("valkey") { valkey.get("foo") }

  x.report("redis") { redis.get("foo") }


  x.compare!
end
