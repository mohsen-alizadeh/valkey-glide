
require 'bundler/inline'

gemfile do
  source 'https://rubygems.org'

  gem 'benchmark-ips', '2.14.0'
  gem 'valkey-glide', path: '1-pure-ruby'
  gem 'pry'
  gem 'redis'
end

valkey = ValkeyGlide.new
redis = Redis.new

Benchmark.ips do |x|
  x.report("1-pure-ruby") { valkey.get("foo") }

  x.report("redis") { redis.get("foo") }

  x.compare!
end
