defmodule Test.RZstd.DecompressorTest do
  use ExUnit.Case

  test "new/0 creates a new decompressor" do
    assert {:ok, decompressor} = RZstd.Decompressor.new()
    assert is_reference(decompressor)
  end

  test "with_dict/0 creates a new decompressor using a dict" do
    dict = File.read!(Path.join(__DIR__, "dict.zst"))

    assert {:ok, decompressor} = RZstd.Decompressor.with_dict(dict)
    assert is_reference(decompressor)
  end
end
