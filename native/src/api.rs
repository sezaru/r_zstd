use crate::encode;

use crate::compressor_handle::CompressorHandle;
use crate::decompressor_handle::DecompressorHandle;

use rustler::resource::ResourceArc;
use rustler::types::binary::Binary;
use rustler::{Atom, Env};

mod atoms {
    rustler::atoms! {
        compression_failed,
        decompression_failed,
        binary_creation_failed,
        binary_population_failed,
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn new_compressor<'a>() -> Result<ResourceArc<CompressorHandle>, Atom> {
    Ok(CompressorHandle::new(zstd::block::Compressor::new()))
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn new_compressor_with_dict(dict: Binary) -> Result<ResourceArc<CompressorHandle>, Atom> {
    Ok(CompressorHandle::new(zstd::block::Compressor::with_dict(
        dict.to_vec(),
    )))
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn new_decompressor() -> Result<ResourceArc<DecompressorHandle>, Atom> {
    Ok(DecompressorHandle::new(zstd::block::Decompressor::new()))
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn new_decompressor_with_dict(dict: Binary) -> Result<ResourceArc<DecompressorHandle>, Atom> {
    Ok(DecompressorHandle::new(
        zstd::block::Decompressor::with_dict(dict.to_vec()),
    ))
}

#[rustler::nif]
pub fn compress<'a>(env: Env<'a>, data: Binary, level: i32) -> Result<Binary<'a>, Atom> {
    do_compress(env, data, level)
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn compress_dirty<'a>(env: Env<'a>, data: Binary, level: i32) -> Result<Binary<'a>, Atom> {
    do_compress(env, data, level)
}

#[rustler::nif]
pub fn decompress<'a>(env: Env<'a>, compressed_data: Binary) -> Result<Binary<'a>, Atom> {
    do_decompress(env, compressed_data)
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn decompress_dirty<'a>(env: Env<'a>, compressed_data: Binary) -> Result<Binary<'a>, Atom> {
    do_decompress(env, compressed_data)
}

#[rustler::nif]
pub fn compress_with_compressor<'a>(
    env: Env<'a>,
    data: Binary,
    level: i32,
    compressor_arc: ResourceArc<CompressorHandle>,
) -> Result<Binary<'a>, Atom> {
    do_compress_with_compressor(env, data, level, compressor_arc)
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn compress_with_compressor_dirty<'a>(
    env: Env<'a>,
    data: Binary,
    level: i32,
    compressor_arc: ResourceArc<CompressorHandle>,
) -> Result<Binary<'a>, Atom> {
    do_compress_with_compressor(env, data, level, compressor_arc)
}

#[rustler::nif]
pub fn decompress_with_decompressor<'a>(
    env: Env<'a>,
    compressed_data: Binary,
    decompressor_arc: ResourceArc<DecompressorHandle>,
) -> Result<Binary<'a>, Atom> {
    do_decompress_with_decompressor(env, compressed_data, decompressor_arc)
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn decompress_with_decompressor_dirty<'a>(
    env: Env<'a>,
    compressed_data: Binary,
    decompressor_arc: ResourceArc<DecompressorHandle>,
) -> Result<Binary<'a>, Atom> {
    do_decompress_with_decompressor(env, compressed_data, decompressor_arc)
}

fn do_compress<'a>(env: Env<'a>, data: Binary, level: i32) -> Result<Binary<'a>, Atom> {
    let compressed_data =
        zstd::block::compress(&data, level).map_err(|_| atoms::compression_failed())?;

    encode::binary(&compressed_data, env)
}

fn do_decompress<'a>(env: Env<'a>, compressed_data: Binary) -> Result<Binary<'a>, Atom> {
    let capacity = zstd_safe::get_frame_content_size(&compressed_data) as usize;
    let data = zstd::block::decompress(&compressed_data, capacity)
        .map_err(|_| atoms::decompression_failed())?;

    encode::binary(&data, env)
}

fn do_compress_with_compressor<'a>(
    env: Env<'a>,
    data: Binary,
    level: i32,
    compressor_arc: ResourceArc<CompressorHandle>,
) -> Result<Binary<'a>, Atom> {
    let mut compressor = CompressorHandle::for_write(&compressor_arc);

    let compressed_data = compressor
        .compress(&data, level)
        .map_err(|_| atoms::compression_failed())?;

    encode::binary(&compressed_data, env)
}

fn do_decompress_with_decompressor<'a>(
    env: Env<'a>,
    compressed_data: Binary,
    decompressor_arc: ResourceArc<DecompressorHandle>,
) -> Result<Binary<'a>, Atom> {
    let capacity = zstd_safe::get_frame_content_size(&compressed_data) as usize;
    let mut decompressor = DecompressorHandle::for_write(&decompressor_arc);

    let data = decompressor
        .decompress(&compressed_data, capacity)
        .map_err(|_| atoms::decompression_failed())?;

    encode::binary(&data, env)
}
