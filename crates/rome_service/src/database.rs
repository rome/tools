use rome_fs::RomePath;
use salsa::{database, query_group, Durability, ParallelDatabase, Snapshot, Storage};

mod analyzer;
mod formatter;
mod parser;

use crate::{
    database::{analyzer::AnalyzerStorage, formatter::FormatterStorage, parser::ParserStorage},
    file_handlers::Features,
    settings::WorkspaceSettings,
};

pub(crate) use self::{
    analyzer::Analyzer,
    formatter::Formatter,
    parser::{AnyParse, Parser},
};

#[derive(Clone, Debug)]
pub(crate) struct Document {
    pub(crate) content: String,
    pub(crate) version: i32,
}

#[query_group(InputsStorage)]
pub(crate) trait Inputs {
    #[salsa::input]
    fn document(&self, name: RomePath) -> Document;

    #[salsa::input]
    fn language_features(&self, key: ()) -> Features;

    #[salsa::input]
    fn settings(&self, key: ()) -> WorkspaceSettings;
}

#[database(InputsStorage, ParserStorage, AnalyzerStorage, FormatterStorage)]
pub(crate) struct WorkspaceDatabase {
    storage: Storage<Self>,
}

impl Default for WorkspaceDatabase {
    fn default() -> Self {
        let mut db = Self {
            storage: Storage::default(),
        };

        db.set_language_features_with_durability((), Features::new(), Durability::HIGH);
        db.set_settings_with_durability((), WorkspaceSettings::default(), Durability::MEDIUM);
        db
    }
}

impl salsa::Database for WorkspaceDatabase {}

impl ParallelDatabase for WorkspaceDatabase {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(WorkspaceDatabase {
            storage: self.storage.snapshot(),
        })
    }
}
