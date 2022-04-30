use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::core::core_manager;
use crate::inode::inode::Inode;
use crate::util::lru_cache;

pub struct InodeManager {
    pub size: usize,
    pub capacity: usize,
    // pub cache: lru_cache::LRUCache<InodeLink>,
    pub core_manager: core_manager::CoreManager,
    pub inode_buffer: Vec<InodeLink>,
    pub lock: Mutex<bool>,
}

pub type InodeLink = Arc<RefCell<Inode>>;

impl InodeManager {
    pub fn new() -> InodeManager {
        let mut buf = vec![];
        for _ in 0..30 {
            buf.push(Arc::new(RefCell::new(Inode::new())));
        }
        let capacity = 4096;
        InodeManager {
            size: 0,
            capacity: capacity as usize,
            core_manager: core_manager::CoreManager::new(),
            inode_buffer: vec![],
            lock: Mutex::new(false),
            // cache: lru_cache::LRUCache::new(capacity as usize),
        }
    }

    // Allocate an inode on device dev.
    // Mark it as allocated by giving it type type.
    // Returns an unlocked but allocated and referenced inode.
    pub fn i_alloc(&mut self) -> Option<InodeLink> {
        todo!()
    }

    // Find the inode with number ino on device dev
    // and return the in-memory copy.
    pub fn i_get(&mut self, ino: u32) -> Option<InodeLink> {
        let mut empty_index = -1;
        let _ = self.lock.lock();
        for (index, ip) in self.inode_buffer.iter().enumerate() {
            if ip.borrow().ref_cnt > 0 && ip.borrow().ino == ino {
                ip.borrow_mut().ref_cnt += 1;
                return Some(Arc::clone(ip));
            }
            if empty_index == -1 && ip.borrow().ref_cnt == 0 {
                empty_index = index as i32;
            }
        }
        if empty_index == -1 {
            return None;
        }
        let inode = self.core_manager.get_inode(ino);
        let link = Arc::new(RefCell::new(inode));
        self.inode_buffer[empty_index as usize] = link;
        Some(Arc::clone(&self.inode_buffer[empty_index as usize]))
    }

    // Increment reference count for ip.
    pub fn i_dup(&mut self, inode: InodeLink) -> InodeLink {
        let _ = self.lock.lock().unwrap();
        inode.borrow_mut().ref_cnt += 1;
        Arc::clone(&inode)
    }

    // Drop a reference to an in-memory inode.
    // If that was the last reference, the inode cache entry can
    // be recycled.
    // If that was the last reference and the inode has no links
    // to it, free the inode (and its content) on disk.
    pub fn i_put(&mut self, inode: InodeLink) -> bool {
        let _ = self.lock.lock();
        if inode.borrow().ref_cnt == 1 && inode.borrow().valid && inode.borrow().n_link == 0 {
            inode.borrow_mut().valid = false;
            return true;
        }
        inode.borrow_mut().ref_cnt -= 1;
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        
    }
}