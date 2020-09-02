use zstd::block::Decompressor;

use rustler::resource::ResourceArc;

use std::ops::Deref;
use std::sync::RwLockWriteGuard;
use std::sync::{Arc, RwLock};

pub struct DecompressorHandle {
    decompressor: Arc<RwLock<Decompressor>>,
}

unsafe impl Sync for DecompressorHandle {}
unsafe impl Send for DecompressorHandle {}

impl Deref for DecompressorHandle {
    type Target = Arc<RwLock<Decompressor>>;

    fn deref(&self) -> &Self::Target {
        &self.decompressor
    }
}

impl DecompressorHandle {
    pub fn new(decompressor: Decompressor) -> ResourceArc<Self> {
        ResourceArc::new(DecompressorHandle {
            decompressor: Arc::new(RwLock::new(decompressor)),
        })
    }

    pub fn for_write<'a>(
        decompressor_arc: &'a ResourceArc<DecompressorHandle>,
    ) -> RwLockWriteGuard<'a, Decompressor> {
        decompressor_arc.deref().decompressor.write().unwrap()
    }
}
