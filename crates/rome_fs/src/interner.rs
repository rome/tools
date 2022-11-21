use std::path::PathBuf;
use std::sync::RwLock;

use crossbeam::channel::{unbounded, Receiver, Sender};
use indexmap::IndexSet;
use rome_diagnostics::location::FileId;

/// File paths interner cache
///
/// The path interner stores an instance of [PathBuf] and
/// returns its unique [FileId]
pub struct PathInterner {
    storage: RwLock<IndexSet<PathBuf>>,
    handler: Sender<(FileId, PathBuf)>,
}

impl PathInterner {
    pub fn new() -> (Self, Receiver<(FileId, PathBuf)>) {
        let (send, recv) = unbounded();
        let interner = Self {
            storage: RwLock::new(IndexSet::new()),
            handler: send,
        };

        (interner, recv)
    }

    /// Insert the path and get its [FileId]
    ///
    /// If an equivalent path already exists, it returns
    /// the [FileId] of the existing path and `false`.
    /// Otherwise, it inserts the new path and returns the [FileId]
    /// of the inserted path and `true`
    pub fn intern_path(&self, path: PathBuf) -> (FileId, bool) {
        let (index, result) = self.storage.write().unwrap().insert_full(path.clone());
        let field_id = FileId::from(index);
        if result {
            self.handler.send((field_id, path)).ok();
        }
        (field_id, result)
    }
}
