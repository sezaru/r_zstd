pub mod api;
pub mod atoms;
pub mod compressor_handle;
pub mod decode;
pub mod decompressor_handle;
pub mod encode;
pub mod macros;

#[macro_use]
extern crate rustler;

extern crate zstd;
extern crate zstd_safe;

use compressor_handle::CompressorHandle;
use decompressor_handle::DecompressorHandle;

use rustler::schedule::SchedulerFlags;
use rustler::{Encoder, Env, Term};

#[inline(always)]
fn new_compressor<'a>(env: Env<'a>, args: &[Term<'a>]) -> Term<'a> {
    return_ok!(api::new_compressor(env, args), env)
}

#[inline(always)]
fn new_compressor_with_dict<'a>(env: Env<'a>, args: &[Term<'a>]) -> Term<'a> {
    return_from_result!(api::new_compressor_with_dict(env, args), env)
}

#[inline(always)]
fn new_decompressor<'a>(env: Env<'a>, args: &[Term<'a>]) -> Term<'a> {
    return_ok!(api::new_decompressor(env, args), env)
}

#[inline(always)]
fn new_decompressor_with_dict<'a>(env: Env<'a>, args: &[Term<'a>]) -> Term<'a> {
    return_from_result!(api::new_decompressor_with_dict(env, args), env)
}

#[inline(always)]
fn compress<'a>(env: Env<'a>, args: &[Term<'a>]) -> Term<'a> {
    return_from_result!(api::compress(env, args), env)
}

#[inline(always)]
fn decompress<'a>(env: Env<'a>, args: &[Term<'a>]) -> Term<'a> {
    return_from_result!(api::decompress(env, args), env)
}

#[inline(always)]
fn compress_with_compressor<'a>(env: Env<'a>, args: &[Term<'a>]) -> Term<'a> {
    return_from_result!(api::compress_with_compressor(env, args), env)
}

#[inline(always)]
fn decompress_with_decompressor<'a>(env: Env<'a>, args: &[Term<'a>]) -> Term<'a> {
    return_from_result!(api::decompress_with_decompressor(env, args), env)
}

rustler_export_nifs!(
    "Elixir.RZstd.Native",
    [
        (
            "new_compressor",
            0,
            new_compressor,
            SchedulerFlags::DirtyCpu
        ),
        (
            "new_compressor_with_dict",
            1,
            new_compressor_with_dict,
            SchedulerFlags::DirtyCpu
        ),
        (
            "new_decompressor",
            0,
            new_decompressor,
            SchedulerFlags::DirtyCpu
        ),
        (
            "new_decompressor_with_dict",
            1,
            new_decompressor_with_dict,
            SchedulerFlags::DirtyCpu
        ),
        ("compress", 2, compress),
        ("compress_dirty", 2, compress, SchedulerFlags::DirtyCpu),
        ("decompress", 1, decompress),
        ("decompress_dirty", 1, decompress, SchedulerFlags::DirtyCpu),
        ("compress_with_compressor", 3, compress_with_compressor),
        (
            "compress_with_compressor_dirty",
            3,
            compress_with_compressor,
            SchedulerFlags::DirtyCpu
        ),
        (
            "decompress_with_decompressor",
            2,
            decompress_with_decompressor
        ),
        (
            "decompress_with_decompressor_dirty",
            2,
            decompress_with_decompressor,
            SchedulerFlags::DirtyCpu
        ),
    ],
    Some(on_load)
);

fn on_load<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    resource_struct_init!(CompressorHandle, env);
    resource_struct_init!(DecompressorHandle, env);

    true
}
