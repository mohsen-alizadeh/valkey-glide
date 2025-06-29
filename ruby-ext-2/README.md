# Valkey

## Installation

```
$ gem install redis
```

## Usage

```ruby
require "valkey"

client = Valkey.new

client.set("mykey", "hello world")
# => "OK"


# using protobuf messaging method
client.get_v1("mykey")
# => "hello world"

# using direct extension method call
client.get_v2("mykey")
# => "hello world"
```

## Development

compile

```
$ bundle exec rake compile
```

run tests

```
$ bundle exec rake test
```

run benchmarks

```
$ ruby single.rb

=== GET key ===
ruby 3.1.4p223 (2023-03-30 revision 957bb7cb81) [x86_64-linux]
Warming up --------------------------------------
           valkey-v1     7.034k i/100ms
           valkey-v2     9.916k i/100ms
               redis     9.203k i/100ms
        redis-client     9.180k i/100ms
Calculating -------------------------------------
           valkey-v1     74.403k (± 5.0%) i/s   (13.44 μs/i) -    372.802k in   5.023350s
           valkey-v2     96.720k (± 6.3%) i/s   (10.34 μs/i) -    485.884k in   5.045234s
               redis     90.146k (± 3.7%) i/s   (11.09 μs/i) -    450.947k in   5.009374s
        redis-client     91.332k (± 3.8%) i/s   (10.95 μs/i) -    459.000k in   5.033211s

Comparison:
           valkey-v1:    74402.9 i/s
           valkey-v2:    96719.5 i/s - 1.30x  faster
        redis-client:    91331.8 i/s - 1.23x  faster
               redis:    90145.7 i/s - 1.21x  faster
```
## Contributing
