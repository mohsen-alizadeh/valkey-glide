# frozen_string_literal: true

RSpec.describe Valkey do
  it "has a version number" do
    expect(Valkey::VERSION).not_to be nil
  end
end
