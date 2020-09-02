use crate::compressor_handle::CompressorHandle;
use crate::decompressor_handle::DecompressorHandle;

use rustler::resource::ResourceArc;
use rustler::types::binary::Binary;
use rustler::{Atom, Term};

rustler_atoms! {
    atom integer_decode_failed;
    atom binary_decode_failed;
    atom compressor_decode_failed;
    atom decompressor_decode_failed;
}

pub fn integer<'a>(term: Term<'a>) -> Result<i32, Atom> {
    term.decode::<i32>().map_err(|_| integer_decode_failed())
}

pub fn binary<'a>(term: Term<'a>) -> Result<Binary, Atom> {
    term.decode::<Binary>().map_err(|_| binary_decode_failed())
}

pub fn compressor<'a>(term: Term<'a>) -> Result<ResourceArc<CompressorHandle>, Atom> {
    term.decode::<ResourceArc<CompressorHandle>>()
        .map_err(|_| compressor_decode_failed())
}

pub fn decompressor<'a>(term: Term<'a>) -> Result<ResourceArc<DecompressorHandle>, Atom> {
    term.decode::<ResourceArc<DecompressorHandle>>()
        .map_err(|_| decompressor_decode_failed())
}
