pub mod api;
pub mod compressor_handle;
pub mod decompressor_handle;
pub mod encode;

extern crate rustler;

extern crate zstd;
extern crate zstd_safe;

use compressor_handle::CompressorHandle;
use decompressor_handle::DecompressorHandle;

use rustler::{Env, Term};

rustler::init!(
    "Elixir.RZstd.Native",
    [
        api::new_compressor,
        api::new_compressor_with_dict,
        api::new_decompressor,
        api::new_decompressor_with_dict,
        api::compress,
        api::compress_dirty,
        api::decompress,
        api::decompress_dirty,
        api::compress_with_compressor,
        api::compress_with_compressor_dirty,
        api::decompress_with_decompressor,
        api::decompress_with_decompressor_dirty,
    ],
    load = load
);

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(CompressorHandle, env);
    rustler::resource!(DecompressorHandle, env);

    true
}
