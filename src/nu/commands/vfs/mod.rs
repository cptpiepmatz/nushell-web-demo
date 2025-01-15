use std::{ops::Deref, sync::OnceLock};
use vfs::{FileSystem, MemoryFS};

pub struct VirtualFileSystem(OnceLock<MemoryFS>);
static VFS: VirtualFileSystem = VirtualFileSystem(OnceLock::new());

impl VirtualFileSystem {
    fn init() -> MemoryFS {
        vfs_shadow::load_into_vfs!("vfs", MemoryFS::new()).expect("MemoryFS is fresh")
    }
}

impl Deref for VirtualFileSystem {
    type Target = MemoryFS;

    fn deref(&self) -> &Self::Target {
        self.0.get_or_init(VirtualFileSystem::init)
    }
}
