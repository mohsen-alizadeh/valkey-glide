# frozen_string_literal: true

require "test_helper"

class TestValkey < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Valkey::VERSION
  end

  def test_it_does_something_useful
    assert true
  end

  def test_get_v1
    client = Valkey.new

    client.set("foo", "bar")

    assert_equal client.get_v1("foo"), "bar"
  end

  def test_get_v2
    client = Valkey.new

    client.set("foo", "bar")

    assert_equal client.get_v2("foo"), "bar"
  end
end
