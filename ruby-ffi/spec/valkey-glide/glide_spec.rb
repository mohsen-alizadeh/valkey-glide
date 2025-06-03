# frozen_string_literal: true

RSpec.describe ValkeyGlide do
  let(:c) { ValkeyGlide.new }

  it "test_set_and_get" do
    expect(c.set("foo", "s1")).to eq("OK")

    expect(c.get("foo")).to eq("s1")
  end

  it "test_set_and_get_with_newline_characters" do
    c.set("foo", "1\n")

    expect("1\n").to eq(c.get("foo"))
  end
end
