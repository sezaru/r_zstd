use zstd::block::Compressor;

use rustler::resource::ResourceArc;

use std::ops::Deref;
use std::sync::RwLockWriteGuard;
use std::sync::{Arc, RwLock};

pub struct CompressorHandle {
    compressor: Arc<RwLock<Compressor>>,
}

unsafe impl Sync for CompressorHandle {}
unsafe impl Send for CompressorHandle {}

impl Deref for CompressorHandle {
    type Target = Arc<RwLock<Compressor>>;

    fn deref(&self) -> &Self::Target {
        &self.compressor
    }
}

impl CompressorHandle {
    pub fn new(compressor: Compressor) -> ResourceArc<Self> {
        ResourceArc::new(CompressorHandle {
            compressor: Arc::new(RwLock::new(compressor)),
        })
    }

    pub fn for_write<'a>(
        compressor_arc: &'a ResourceArc<CompressorHandle>,
    ) -> RwLockWriteGuard<'a, Compressor> {
        compressor_arc.deref().compressor.write().unwrap()
    }
}
