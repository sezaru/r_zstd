defmodule Test.RZstd.CompressorTest do
  use ExUnit.Case

  test "new/0 creates a new compressor" do
    assert {:ok, compressor} = RZstd.Compressor.new()
    assert is_reference(compressor)
  end

  test "with_dict/0 creates a new compressor using a dict" do
    dict = File.read!(Path.join(__DIR__, "dict.zst"))

    assert {:ok, compressor} = RZstd.Compressor.with_dict(dict)
    assert is_reference(compressor)
  end
end
