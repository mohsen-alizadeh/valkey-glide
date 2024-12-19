# frozen_string_literal: true

RSpec.describe Valkey do
  it "has a version number" do
    expect(Valkey::VERSION).not_to be nil
  end

  it "felan" do
      client = Valkey.new

      client.set("foo", "bar")

      # pp client.methods.grep /client/
      # puts client.create_client



      # expect(client.get("foo")).to eq("bar")
  end
end
