defmodule Test.RZstdTest do
  use ExUnit.Case

  @data "bla bla"

  test "compress/2 compress some data" do
    {:ok, compressed_data} = RZstd.compress(@data, 1)

    assert RZstd.decompress!(compressed_data) == @data
  end

  test "compress_dirty/2 compress some data" do
    {:ok, compressed_data} = RZstd.compress_dirty(@data, 1)

    assert RZstd.decompress!(compressed_data) == @data
  end

  test "compress/3 compress some data with compressor" do
    {:ok, compressor} = RZstd.Compressor.new()

    {:ok, compressed_data} = RZstd.compress(@data, 1, compressor)

    assert RZstd.decompress!(compressed_data) == @data
  end

  test "compress_dirty/3 compress some data with compressor" do
    {:ok, compressor} = RZstd.Compressor.new()

    {:ok, compressed_data} = RZstd.compress_dirty(@data, 1, compressor)

    assert RZstd.decompress!(compressed_data) == @data
  end

  test "decompress/2 decompress some data" do
    {:ok, compressed_data} = RZstd.compress(@data, 1)

    assert RZstd.decompress(compressed_data) == {:ok, @data}
  end

  test "decompress_dirty/2 decompress some data" do
    {:ok, compressed_data} = RZstd.compress(@data, 1)

    assert RZstd.decompress_dirty(compressed_data) == {:ok, @data}
  end

  test "decompress/3 decompress some compressed data with decompressor" do
    {:ok, decompressor} = RZstd.Decompressor.new()

    {:ok, compressed_data} = RZstd.compress(@data, 1)

    assert RZstd.decompress(compressed_data, decompressor) == {:ok, @data}
  end

  test "decompress_dirty/3 decompress some compressed data with decompressor" do
    {:ok, decompressor} = RZstd.Decompressor.new()

    {:ok, compressed_data} = RZstd.compress_dirty(@data, 1)

    assert RZstd.decompress_dirty(compressed_data, decompressor) == {:ok, @data}
  end
end
