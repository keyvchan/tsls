use lsp_types::{TextDocumentItem, Url};
use queries::{
    highlight::{self, update_identifiers_kind},
    locals::build_definitions_and_scopes,
};
use tree_sitter::Tree;

use super::{docuemnt::Document, parsed::ParsedInfo, propertie::Properties};
use crate::global_state::GlobalState;

impl GlobalState {
    pub fn get_snapshot(&self) -> GlobalState {
        self.clone()
    }
    pub fn build_cache(&mut self, source_code: TextDocumentItem, tree: Option<&Tree>) {
        let uri = &source_code.uri;
        // if tree is None, it means we already parsed the source code
        // and we can use the cached tree
        let tree = match tree {
            Some(tree) => tree,
            None => &self.get_tree(&uri).unwrap(),
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

        let document = Document::new(
            *tree,
            source_code.version,
            source_code.text.as_bytes().to_vec(),
        );
        self.update_document(&uri, &document);

        let parsed_info = ParsedInfo::new(&ordered_scopes, &definitions_lookup_map, &identifiers);
        self.update_parsed_info(&uri, &parsed_info);

        let properties_cache = Properties::new(&source_code.language_id, &keywords);
        self.update_properties(&uri, &properties_cache);

        self.update_diagnostics(&uri);
    }

    // WARN: Not used for now
    pub fn update_cache(&mut self, uri: &Url) -> Result<(), String> {
        // check if the cache needed to be updated by source_code.version
        // insert the cache
        let tree = match self.get_tree(uri) {
            Some(tree) => tree,
            None => {
                return Err(format!("No tree found for {}", uri));
            }
        };
        let source_code = match self.get_source_code(uri) {
            Some(source_code) => source_code,
            None => {
                return Err(format!("No source code found for {}", uri));
            }
        };
        let language_id = match self.get_language_id(uri) {
            Some(language_id) => language_id,
            None => {
                return Err(format!("No language id found for {}", uri));
            }
        };

        // TODO: partial update
        let (definitions_lookup_map, ordered_scopes, mut identifiers) =
            build_definitions_and_scopes(&source_code, &tree.root_node(), &language_id);

        update_identifiers_kind(
            &mut identifiers,
            &ordered_scopes,
            &source_code,
            &tree,
            &language_id,
        );

        let parsed_info = ParsedInfo::new(&ordered_scopes, &definitions_lookup_map, &identifiers);
        self.update_parsed_info(&uri, &parsed_info);

        self.update_diagnostics(&uri)?;
        Ok(())
    }
}
