data = """
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
"""

dict = File.read!(Path.join(__DIR__, "dict.zst"))

load_compressor = fn _ -> RZstd.Compressor.new() end
load_compressor_with_dict = fn _ -> RZstd.Compressor.with_dict(dict) end

Benchee.run(%{
  "compress/2" => fn -> RZstd.compress(data, 5) end,
  "compress_dirty/2" => fn -> RZstd.compress(data, 5) end,
  "compress/3" =>
    {fn compressor -> RZstd.compress(data, 5, compressor) end, before_scenario: load_compressor},
  "compress_dirty/3" =>
    {fn compressor -> RZstd.compress(data, 5, compressor) end, before_scenario: load_compressor},
  "compress/3 with dict" =>
    {fn compressor -> RZstd.compress(data, 5, compressor) end,
     before_scenario: load_compressor_with_dict},
  "compress_dirty/3 with dict" =>
    {fn compressor -> RZstd.compress(data, 5, compressor) end,
     before_scenario: load_compressor_with_dict}
})
