defmodule RZstd.Native do
  @moduledoc false

  use Rustler, otp_app: :r_zstd, crate: "r_zstd_nif"

  def new_compressor, do: nif_error()
  def new_compressor_with_dict(_dict), do: nif_error()
  def new_decompressor, do: nif_error()
  def new_decompressor_with_dict(_dict), do: nif_error()
  def compress(_data, _level), do: nif_error()
  def compress_dirty(_data, _level), do: nif_error()
  def decompress(_data), do: nif_error()
  def decompress_dirty(_data), do: nif_error()
  def compress_with_compressor(_data, _level, _compressor), do: nif_error()
  def compress_with_compressor_dirty(_data, _level, _compressor), do: nif_error()
  def decompress_with_decompressor(_compressed_data, _decompressor), do: nif_error()
  def decompress_with_decompressor_dirty(_compressed_data, _decompressor), do: nif_error()

  defp nif_error, do: :erlang.nif_error(:nif_not_loaded)
end
