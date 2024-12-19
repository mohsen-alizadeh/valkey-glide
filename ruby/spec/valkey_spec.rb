# frozen_string_literal: true

RSpec.describe Valkey do
  it "has a version number" do
    expect(Valkey::VERSION).not_to be nil
  end

  it "set a simple string" do
      client = Valkey.new

      client.set("foo", "bar")

      expect(client.get("foo")).to eq("bar")
  end
end
