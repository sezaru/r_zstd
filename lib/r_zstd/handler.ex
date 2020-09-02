defmodule RZstd.Handler do
  @moduledoc false

  alias RZstd.Exception

  def unwrap({:ok, value}), do: value
  def unwrap({:error, error}), do: raise(Exception, error)
end
