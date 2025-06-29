
require 'bundler/inline'

implementation = ARGV[0]
puts "Using implementation: #{implementation}"

gemfile do
  source 'https://rubygems.org'

  gem 'benchmark-ips', '2.14.0'
  gem 'valkey-glide', path: implementation
  gem 'pry'
  gem 'redis'
end



valkey = ValkeyGlide.new
redis = Redis.new

Benchmark.ips do |x|
  x.report(implementation) { valkey.get("foo") }

  x.report("redis") { redis.get("foo") }

  x.compare!
end
