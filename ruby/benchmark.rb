require 'bundler/inline'
require 'securerandom'

implementation = ARGV[0]
puts "Using implementation: #{implementation}"

gemfile do
  source 'https://rubygems.org'

  gem 'benchmark-ips', '2.14.0'
  gem 'valkey-glide', path: implementation
  gem 'pry'
  gem 'redis'
end


random = SecureRandom.hex(10)

valkey = ValkeyGlide.new
redis = Redis.new

redis.set(random, random)

if redis.get(random) != random
  raise "Redis is not working correctly"
end

if valkey.get(random) != random
  raise "Valkey is not working correctly"
end

Benchmark.ips do |x|
  x.report(implementation) { valkey.get("foo") }

  x.report("redis") { redis.get("foo") }

  x.compare!
end
