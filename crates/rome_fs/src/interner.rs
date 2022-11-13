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
use rome_diagnostics::file::FileId;
use std::sync::RwLock;

pub trait PathInterner {
    /// Insert the path and get its [FileId]
    ///
    /// If an equivalent path already exists, it returns
    /// the [FileId] of the existing path and `false`.
    /// Otherwise, it inserts the new path and returns the [FileId]
    /// of the inserted path and `true`
    fn intern_path(&self, path: PathBuf) -> (FileId, bool);
}

/// Indexed implementation of [PathInterner] that caches the path in advance
pub struct IndexSetInterner {
    storage: RwLock<IndexSet<PathBuf>>,
    handler: Sender<(FileId, PathBuf)>,
}

impl IndexSetInterner {
    pub fn new() -> (Self, Receiver<(FileId, PathBuf)>) {
        let (send, recv) = unbounded();
        let interner = Self {
            storage: RwLock::new(IndexSet::new()),
            handler: send,
        };

        (interner, recv)
    }
}

impl PathInterner for IndexSetInterner {
    fn intern_path(&self, path: PathBuf) -> (FileId, bool) {
        let (index, result) = self.storage.write().unwrap().insert_full(path.clone());
        let field_id = FileId::from(index);
        if result {
            self.handler.send((field_id, path)).ok();
        }
        (field_id, result)
    }
}

/// Fast implementation of [PathInterner] that is entirely lock-free, at the
/// expense of not performing any de-duplication
///
/// It's intended to be used in situations where it is known in advance the
/// interner will never see the same path twice (eg. directory traversals
/// without symlinks)
pub struct AtomicInterner {
    storage: AtomicUsize,
    handler: Sender<(FileId, PathBuf)>,
}

impl AtomicInterner {
    pub fn new() -> (Self, Receiver<(FileId, PathBuf)>) {
        let (send, recv) = unbounded();
        let interner = Self {
            storage: AtomicUsize::new(0),
            handler: send,
        };

        (interner, recv)
    }
}

impl PathInterner for AtomicInterner {
    fn intern_path(&self, path: PathBuf) -> (FileId, bool) {
        let id = FileId::from(self.storage.fetch_add(1, Ordering::Relaxed));
        self.handler.send((id, path)).ok();
        // This is always set to true, as links are not de-duplicated by default
        (id, true)
    }
}
