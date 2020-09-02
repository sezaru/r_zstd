defmodule RZstd.Compressor do
  @moduledoc "Compressor used to optimize zstd compression speed and memory usage."

  alias RZstd.Native

  @type t :: reference

  @spec new :: {:ok, t}
  def new, do: Native.new_compressor()

  @spec with_dict(binary) :: {:ok, t} | {:error, :binary_decode_failed}
  def with_dict(dict), do: Native.new_compressor_with_dict(dict)
end
