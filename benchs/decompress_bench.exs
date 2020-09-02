data = """
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
"""

dict = File.read!(Path.join(__DIR__, "dict.zst"))

compressed_data = RZstd.compress(data, 5)

load_decompressor = fn _ -> RZstd.Decompressor.new() end
load_decompressor_with_dict = fn _ -> RZstd.Decompressor.with_dict(dict) end

Benchee.run(%{
  "decompress/1" => fn -> RZstd.decompress(compressed_data) end,
  "decompress_dirty/1" => fn -> RZstd.decompress(compressed_data) end,
  "decompress/2" =>
    {fn decompressor -> RZstd.decompress(compressed_data, decompressor) end,
     before_scenario: load_decompressor},
  "decompress_dirty/2" =>
    {fn decompressor -> RZstd.decompress(compressed_data, decompressor) end,
     before_scenario: load_decompressor},
  "decompress/2 with dict" =>
    {fn decompressor -> RZstd.decompress(compressed_data, decompressor) end,
     before_scenario: load_decompressor_with_dict},
  "decompress_dirty/2 with dict" =>
    {fn decompressor -> RZstd.decompress(compressed_data, decompressor) end,
     before_scenario: load_decompressor_with_dict}
})
