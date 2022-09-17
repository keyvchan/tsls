// Here we have our database

use std::fmt::Debug;

use lsp_types::Url;
use types::SourceFile;

// input query group
#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase: salsa::Database {
    // query
    #[salsa::input]
    fn source(&self, name: Url) -> SourceFile;
}

#[salsa::database(SourceDatabaseStorage)]
#[derive(Default)]
pub struct RootDatabase {
    pub storage: salsa::Storage<Self>,
}

impl salsa::Database for RootDatabase {}

impl salsa::ParallelDatabase for RootDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(RootDatabase {
            storage: self.storage.snapshot(),
        })
    }
}

impl Debug for RootDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RootDatabase").finish()
    }
}
