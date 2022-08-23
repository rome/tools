//! This module declares the [PathInterner] trait, a utility for creating
//! de-duplicated [FileId]s from instances of [PathBuf]. It also provides a few
//! implementations of that trait tailored for different use-cases, namely
//! [IndexSetInterner] and [AtomicInterner]
use std::{
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

use crossbeam::channel::{unbounded, Receiver, Sender};
use indexmap::IndexSet;
use std::sync::RwLock;

pub type FileId = usize;

pub trait PathInterner {
    fn intern_path(&self, path: PathBuf) -> FileId;
}

pub struct IndexSetInterner {
    storage: RwLock<IndexSet<PathBuf>>,
}

impl PathInterner for IndexSetInterner {
    fn intern_path(&self, path: PathBuf) -> FileId {
        let (index, _) = self.storage.write().unwrap().insert_full(path);
        index
    }
}

/// Fast implementation of [PathInterner] that is entirely lock-free, at the
/// expense of not performing any de-duplication
///
/// It's intended to be used in situations where it is known in advance the
/// interner will never see the same path twice (eg. directory traversals
/// without symlinks)
pub struct AtomicInterner {
    counter: AtomicUsize,
    storage: Sender<(FileId, PathBuf)>,
}

impl AtomicInterner {
    pub fn new() -> (Self, Receiver<(FileId, PathBuf)>) {
        let (send, recv) = unbounded();
        let interner = Self {
            counter: AtomicUsize::new(0),
            storage: send,
        };

        (interner, recv)
    }
}

impl PathInterner for AtomicInterner {
    fn intern_path(&self, path: PathBuf) -> FileId {
        let id = self.counter.fetch_add(1, Ordering::Relaxed);
        self.storage.send((id, path)).ok();
        id
    }
}
