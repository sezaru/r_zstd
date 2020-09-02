use crate::{decode, encode};

use crate::compressor_handle::CompressorHandle;
use crate::decompressor_handle::DecompressorHandle;

use rustler::resource::ResourceArc;
use rustler::types::binary::Binary;
use rustler::{Atom, Env, Term};

rustler_atoms! {
    atom compression_failed;
    atom decompression_failed;
}

pub fn new_compressor<'a>(_env: Env<'a>, _args: &[Term<'a>]) -> ResourceArc<CompressorHandle> {
    CompressorHandle::new(zstd::block::Compressor::new())
}

pub fn new_compressor_with_dict<'a>(
    _env: Env<'a>,
    args: &[Term<'a>],
) -> Result<ResourceArc<CompressorHandle>, Atom> {
    let dict = decode::binary(args[0])?;

    Ok(CompressorHandle::new(zstd::block::Compressor::with_dict(
        dict.to_vec(),
    )))
}

pub fn new_decompressor<'a>(_env: Env<'a>, _args: &[Term<'a>]) -> ResourceArc<DecompressorHandle> {
    DecompressorHandle::new(zstd::block::Decompressor::new())
}

pub fn new_decompressor_with_dict<'a>(
    _env: Env<'a>,
    args: &[Term<'a>],
) -> Result<ResourceArc<DecompressorHandle>, Atom> {
    let dict = decode::binary(args[0])?;

    Ok(DecompressorHandle::new(
        zstd::block::Decompressor::with_dict(dict.to_vec()),
    ))
}

pub fn compress<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Binary<'a>, Atom> {
    let data = decode::binary(args[0])?;
    let level = decode::integer(args[1])?;

    let compressed_data = zstd::block::compress(&data, level).map_err(|_| compression_failed())?;

    encode::binary(&compressed_data, env)
}

pub fn decompress<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Binary<'a>, Atom> {
    let data = decode::binary(args[0])?;
    let capacity = zstd_safe::get_frame_content_size(&data) as usize;

    let decompressed_data =
        zstd::block::decompress(&data, capacity).map_err(|_| decompression_failed())?;

    encode::binary(&decompressed_data, env)
}

pub fn compress_with_compressor<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Binary<'a>, Atom> {
    let data = decode::binary(args[0])?;
    let level = decode::integer(args[1])?;
    let compressor_arc = decode::compressor(args[2])?;

    let mut compressor = CompressorHandle::for_write(&compressor_arc);

    let compressed_data = compressor
        .compress(&data, level)
        .map_err(|_| compression_failed())?;

    encode::binary(&compressed_data, env)
}

pub fn decompress_with_decompressor<'a>(
    env: Env<'a>,
    args: &[Term<'a>],
) -> Result<Binary<'a>, Atom> {
    let compressed_data = decode::binary(args[0])?;
    let capacity = zstd_safe::get_frame_content_size(&compressed_data) as usize;

    let decompressor_arc = decode::decompressor(args[1])?;

    let mut decompressor = DecompressorHandle::for_write(&decompressor_arc);

    let data = decompressor
        .decompress(&compressed_data, capacity)
        .map_err(|_| decompression_failed())?;

    encode::binary(&data, env)
}
