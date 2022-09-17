use std::{collections::HashMap, sync::Arc};

use lsp_types::{Diagnostic, Url};
use parking_lot::Mutex;
use queries::errors::build_diagnostics;
use salsa::ParallelDatabase;
use tree_sitter::Tree;
use types::SourceFile;

use crate::db::{RootDatabase, SourceDatabase};

// Here is our global state
#[derive(Default, Debug)]
pub struct GlobalState {
    pub database: RootDatabase,

    // store all the asts into a hashmap
    pub asts: Arc<Mutex<HashMap<Url, Tree>>>,

    // We store the diagnostics in a hashmap for fast lookup
    pub diagnostics: Arc<Mutex<HashMap<Url, Vec<Diagnostic>>>>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self::default()
    }

    // store the content of the file in the database
    pub fn set_source_inputs(&mut self, url: Url, content: SourceFile) {
        self.database.set_source(url, content);
    }

    pub fn update_diagnostics(&mut self, url: Url) {
        let binding = self.asts.lock();
        let root_node = binding.get(&url).unwrap().root_node();
        self.diagnostics.lock().insert(
            url.clone(),
            build_diagnostics(self.database.source(url).text.into_bytes(), &root_node),
        );
    }

    pub fn snapshot(&self) -> GlobalStateSnapshot {
        GlobalStateSnapshot {
            db: self.database.snapshot(),
            asts: Arc::clone(&self.asts),
            diagnostics: Arc::clone(&self.diagnostics),
        }
    }
}

pub struct GlobalStateSnapshot {
    pub db: salsa::Snapshot<RootDatabase>,
    pub asts: Arc<Mutex<HashMap<Url, Tree>>>,
    pub diagnostics: Arc<Mutex<HashMap<Url, Vec<Diagnostic>>>>,
}
