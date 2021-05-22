use rustler::types::binary::{Binary, OwnedBinary};
use rustler::{Atom, Env};

use std::io::Write;

mod atoms {
    rustler::atoms! {
        binary_alloc_failed,
        binary_write_failed,
    }
}

pub fn binary<'a>(value: &[u8], env: Env<'a>) -> Result<Binary<'a>, Atom> {
    let mut binary = OwnedBinary::new(value.len()).ok_or(atoms::binary_alloc_failed())?;

    binary
        .as_mut_slice()
        .write(&value)
        .map_err(|_| atoms::binary_write_failed())?;

    Ok(binary.release(env))
}
