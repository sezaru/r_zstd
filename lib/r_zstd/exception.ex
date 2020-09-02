defmodule RZstd.Exception do
  @moduledoc false

  defexception [:message]

  @impl true
  def exception(value), do: struct!(__MODULE__, message: "Failed with error: #{value}")
end
