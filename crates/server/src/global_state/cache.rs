use log::warn;
use lsp_types::TextDocumentItem;
use queries::{
    errors,
    highlight::{self, update_identifiers_kind},
    locals::build_definitions_and_scopes,
};
use tree_sitter::Tree;

use crate::global_state::{GlobalState, Properties};

impl GlobalState {
    pub fn get_snapshot(&self) -> GlobalState {
        self.clone()
    }
    pub fn build_cache(&mut self, source_code: TextDocumentItem, tree: Option<&Tree>) {
        // if tree is None, it means we already parsed the source code
        // and we can use the cached tree
        let tree = match tree {
            Some(tree) => tree.clone(),
            None => self.get_snapshot_tree(&source_code.uri).unwrap(),
        };
        let (definitions_lookup_map, ordered_scopes, mut identifiers) =
            build_definitions_and_scopes(
                &source_code.text.as_bytes().to_vec(),
                &tree.root_node(),
                &source_code.language_id,
            );

        update_identifiers_kind(
            &mut identifiers,
            &ordered_scopes,
            &source_code.text.as_bytes().to_vec(),
            &tree,
            &source_code.language_id,
        );

        let keywords = highlight::build_keywords_cache(source_code.language_id.clone());

        // Save it to the global state
        let properties = Properties {
            ast: tree.to_owned(),
            source_code: source_code.text.as_bytes().to_vec(),
            language_id: source_code.language_id.to_owned(),
            version: source_code.version,
            keywords,
            ordered_scopes,
            definitions_lookup_map,
            identifiers,
        };

        // insert update the value in hashmap
        self.sources.insert(source_code.uri.clone(), properties);

        let diagnostics =
            errors::build_diagnostics(source_code.text.as_bytes().to_vec(), &tree.root_node());

        self.diagnostics.insert(source_code.uri, diagnostics);
    }

    // WARN: Not used for now
    pub fn update_cache(&mut self, source_code: TextDocumentItem, tree: Option<&Tree>) {
        // check if the cache needed to be updated by source_code.version
        if source_code.version >= self.get_version(&source_code.uri).unwrap_or_default() {
            // insert the cache
            self.build_cache(source_code, tree);
        } else {
            warn!("Cache already up to date");
        }
    }
}
