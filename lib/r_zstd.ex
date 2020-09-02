defmodule RZstd do
  @moduledoc """
  `RZstd` API.
  """

  alias RZstd.{Compressor, Decompressor, Handler, Native}

  use Unsafe.Generator, docs: true, handler: {Handler, :unwrap}

  @type compression_error ::
          :binary_decode_failed
          | :integer_decode_failed
          | :compression_failed
          | :binary_alloc_failed
          | :binary_write_failed

  @type compression_with_compressor_error :: compression_error | :compressor_decode_failed

  @type decompression_error ::
          :binary_decode_failed
          | :decompression_failed
          | :binary_alloc_failed
          | :binary_write_failed

  @type decompression_with_decompressor_error :: decompression_error | :decompressor_decode_failed

  @unsafe [
    {:compress, [2, 3]},
    {:compress_dirty, [2, 3]},
    {:decompress, [1, 2]},
    {:decompress_dirty, [1, 2]}
  ]

  @spec compress(binary, integer) :: {:ok, binary} | {:error, compression_error}
  def compress(data, level), do: Native.compress(data, level)

  @spec compress(binary, integer, Compressor.t()) ::
          {:ok, binary} | {:error, compression_with_compressor_error}
  def compress(data, level, compressor),
    do: Native.compress_with_compressor(data, level, compressor)

  @spec compress_dirty(binary, integer) :: {:ok, binary} | {:error, compression_error}
  def compress_dirty(data, level), do: Native.compress_dirty(data, level)

  @spec compress_dirty(binary, integer, Compressor.t()) ::
          {:ok, binary} | {:error, compression_with_compressor_error}
  def compress_dirty(data, level, compressor),
    do: Native.compress_with_compressor_dirty(data, level, compressor)

  @spec decompress(binary) :: {:ok, binary} | {:error, compression_error}
  def decompress(data), do: Native.decompress(data)

  @spec decompress(binary, Decompressor.t()) ::
          {:ok, binary} | {:error, decompression_with_decompressor_error}
  def decompress(data, decompressor), do: Native.decompress_with_decompressor(data, decompressor)

  @spec decompress_dirty(binary) :: {:ok, binary} | {:error, decompression_error}
  def decompress_dirty(data), do: Native.decompress_dirty(data)

  @spec decompress_dirty(binary, Decompressor.t()) ::
          {:ok, binary} | {:error, decompression_with_decompressor_error}
  def decompress_dirty(data, decompressor),
    do: Native.decompress_with_decompressor_dirty(data, decompressor)
end
