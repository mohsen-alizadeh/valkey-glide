# frozen_string_literal: true

RSpec.describe ValkeyGlide2 do
  it "has a version number" do
    expect(ValkeyGlide2::VERSION).not_to be nil
  end

  it "set a simple string" do
      client = ValkeyGlide2.new

      pp client
      client.test()

      # client.get("foo")

      # client.set("foo", "bar")
      #
      # expect(client.get("foo")).to eq("bar")
  end
end
