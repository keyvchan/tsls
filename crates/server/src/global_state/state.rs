/// There are five stages of a document, represented three states:
/// 1. `closed`, document is not open
/// 2. `opened`, document is opened, but not edited
/// 3. `edited`, document is edited, but not saved
/// 4. `opened`, document is saved, state back to opened
/// 5. `closed`, document is closed
use std::collections::HashMap;

use helper::types::Symbol;
use lsp_types::{Diagnostic, Position, Url};
use queries::errors::build_diagnostics;
use tree_sitter::{Node, Range, Tree};

use super::{docuemnt::Document, parsed::ParsedInfo, propertie::Properties};

#[derive(Debug, Clone)]
pub struct GlobalState {
    /// CAUSION: Don't out of sync with all these states, after saved
    properties: HashMap<Url, Properties>,
    documents: HashMap<Url, Document>,
    parsed_info: HashMap<Url, ParsedInfo>,
    diagnostics: HashMap<Url, Vec<Diagnostic>>,
}

impl GlobalState {
    /// Create a new GlobalState
    pub fn new() -> Self {
        GlobalState {
            properties: HashMap::new(),
            documents: HashMap::new(),
            parsed_info: HashMap::new(),
            diagnostics: HashMap::new(),
        }
    }

    /// Get node at a given position
    pub fn _get_node_at_position(&self, url: &Url, position: Position) -> Option<Node> {
        let properties = self.documents.get(url)?;
        let node = properties.tree().root_node();
        node.descendant_for_point_range(
            tree_sitter::Point {
                row: position.line as usize,
                column: position.character as usize,
            },
            tree_sitter::Point {
                row: position.line as usize,
                column: position.character as usize,
            },
        )
    }

    pub fn clear(&mut self, uri: &lsp_types::Url) {
        self.documents.remove(uri);
        self.diagnostics.remove(uri);
        self.properties.remove(uri);
        self.parsed_info.remove(uri);
    }

    /// Get a reference to the global state's parsed info.
    #[must_use]
    pub fn parsed_info(&self) -> &HashMap<Url, ParsedInfo> {
        &self.parsed_info
    }

    /// Get a mutable reference to the global state's parsed info.
    #[must_use]
    pub fn parsed_info_mut(&mut self) -> &mut HashMap<Url, ParsedInfo> {
        &mut self.parsed_info
    }

    /// Set the global state's parsed info.
    pub fn set_parsed_info(&mut self, parsed_info: HashMap<Url, ParsedInfo>) {
        self.parsed_info = parsed_info;
    }

    /// Get a reference to the global state's properties.
    #[must_use]
    pub fn properties(&self) -> &HashMap<Url, Properties> {
        &self.properties
    }

    /// Get a mutable reference to the global state's properties.
    #[must_use]
    pub fn properties_mut(&mut self) -> &mut HashMap<Url, Properties> {
        &mut self.properties
    }

    /// Set the global state's properties.
    pub fn set_properties(&mut self, properties: HashMap<Url, Properties>) {
        self.properties = properties;
    }

    /// Get a reference to the global state's document.
    #[must_use]
    pub fn documents(&self) -> &HashMap<Url, Document> {
        &self.documents
    }

    /// Get a mutable reference to the global state's document.
    #[must_use]
    pub fn documents_mut(&mut self) -> &mut HashMap<Url, Document> {
        &mut self.documents
    }

    /// Set the global state's document.
    pub fn set_documents(&mut self, document: HashMap<Url, Document>) {
        self.documents = document;
    }

    /// Get a reference to the global state's diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &HashMap<Url, Vec<Diagnostic>> {
        &self.diagnostics
    }

    /// Get a mutable reference to the global state's diagnostics.
    #[must_use]
    pub fn diagnostics_mut(&mut self) -> &mut HashMap<Url, Vec<Diagnostic>> {
        &mut self.diagnostics
    }

    /// Set the global state's diagnostics.
    pub fn set_diagnostics(&mut self, diagnostics: HashMap<Url, Vec<Diagnostic>>) {
        self.diagnostics = diagnostics;
    }
}

// properties
impl GlobalState {
    /// Get the language_id of a given url, return None if not found, language_id otherwise
    pub fn get_language_id(&self, url: &Url) -> Option<&str> {
        self.properties().get(url).map(|p| p.language_id())
    }

    /// Get keywords of a given url, return None if not found, keywords otherwise
    pub fn get_keywords(&self, url: &Url) -> Option<&Vec<String>> {
        self.properties().get(url).map(|p| p.keywords())
    }

    pub fn update_properties(&mut self, url: &Url, properties: &Properties) {
        self.properties_mut().insert(url.clone(), *properties);
    }
}

// document
impl GlobalState {
    /// Get document version of a given url, return 0 if not found version otherwise
    pub fn get_version(&self, url: &lsp_types::Url) -> i32 {
        self.documents()
            .get(url)
            .map(|d| d.version())
            .unwrap_or_else(|| 0)
    }
    /// Get a inmutable reference to the ast
    pub fn get_tree(&self, url: &Url) -> Option<&Tree> {
        self.documents().get(url).map(|d| d.tree())
    }
    /// Get a mutable reference to the ast
    pub fn get_tree_mut(&mut self, url: &Url) -> Option<&mut Tree> {
        self.documents_mut().get_mut(url).map(|d| d.tree_mut())
    }

    pub fn get_document_mut(&mut self, url: &Url) -> Option<&mut Document> {
        self.documents_mut().get_mut(url)
    }

    /// Get the source code of a given url, return None if not found, byte vector otherwise
    pub fn get_source_code(&self, url: &Url) -> Option<&Vec<u8>> {
        self.documents()
            .get(url)
            .map(|properties| properties.source_code())
    }
    /// Get the source code of a given url, return None if not found, byte vector otherwise
    pub fn get_source_code_mut(&mut self, url: &Url) -> Option<&mut Vec<u8>> {
        self.documents_mut()
            .get_mut(url)
            .map(|properties| properties.source_code_mut())
    }

    /// Update the source code of a given url
    pub fn update_source_code(&mut self, url: &Url, new_source_code: &Vec<u8>) {
        self.documents_mut().get_mut(url).map(|properties| {
            properties.set_source_code(new_source_code);
        });
    }

    pub fn update_tree(&mut self, url: &Url, new_tree: Tree) {
        self.documents_mut()
            .get_mut(url)
            .map(|document| document.set_tree(new_tree));
    }

    pub fn update_document(&mut self, url: &Url, document: &Document) {
        self.documents_mut().insert(*url, *document);
    }
}

// diagnostics
impl GlobalState {
    /// Get a reference to the global state's diagnostics.
    #[must_use]
    pub fn get_diagnostics(&self, url: &Url) -> &Vec<Diagnostic> {
        &self.diagnostics.get(url).unwrap_or_else(|| vec![].as_ref())
    }

    // update the diagnostics of a given url
    pub fn update_diagnostics(&mut self, uri: &lsp_types::Url) -> Result<(), String> {
        let source_code = match self.get_source_code(uri) {
            Some(source_code) => source_code,
            None => return Err("source code not found".to_string()),
        };
        let tree = match self.get_tree(uri) {
            Some(tree) => tree,
            None => return Err("tree not found".to_string()),
        };
        let diagnostics = build_diagnostics(&source_code, &tree.root_node());
        self.diagnostics_mut().insert(*uri, diagnostics);
        Ok(())
    }
}

// parsed_info
impl GlobalState {
    pub fn get_identifiers(&self, url: &Url) -> Option<&HashMap<usize, Vec<Symbol>>> {
        self.parsed_info().get(url).map(|p| p.identifiers())
    }

    pub fn get_ordered_scopes(&self, url: &Url) -> Option<&Vec<Range>> {
        self.parsed_info().get(url).map(|p| p.ordered_scopes())
    }

    pub fn update_parsed_info(&mut self, url: &Url, parsed_info: &ParsedInfo) {
        self.parsed_info_mut().insert(*url, *parsed_info);
    }
}
