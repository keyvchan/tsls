use std::collections::HashMap;

use helper::types::Symbol;
use log::warn;
use lsp_types::{Diagnostic, Position, Url};
use tree_sitter::{Node, Range, Tree};

type Byte = u8;
type ScopeID = usize;

#[derive(Debug, Clone)]
pub struct Properties {
    pub ast: Tree,
    // pub source_code: TextDocumentItem,
    pub language_id: String,
    pub version: i32,

    // use byte vector store the source code
    pub source_code: Vec<Byte>,
    pub keywords: Vec<String>,
    pub ordered_scopes: Vec<Range>,
    pub definitions_lookup_map: HashMap<String, Vec<Symbol>>,
    pub identifiers: HashMap<ScopeID, Vec<Symbol>>,
}

impl Properties {
    pub fn clear(&mut self) {
        self.identifiers.clear();
        self.definitions_lookup_map.clear();
        self.ordered_scopes.clear();
        self.keywords.clear();
        self.source_code.clear();
        self.version = 0;
        self.language_id.clear();

        // Ast not changed, in case fast parse when it opened again
    }
}

#[derive(Debug, Clone)]
pub struct GlobalState {
    pub sources: HashMap<Url, Properties>,
    pub diagnostics: HashMap<Url, Vec<Diagnostic>>,
}

impl GlobalState {
    /// Create a new GlobalState
    pub fn new() -> Self {
        GlobalState {
            sources: HashMap::new(),
            diagnostics: HashMap::new(),
        }
    }

    /// Get the diagnostics of a given url
    pub fn get_diagnostics(&self, uri: &lsp_types::Url) -> Option<Vec<Diagnostic>> {
        self.diagnostics.get(uri).cloned()
    }

    /// Get a mutable reference to the ast
    pub fn get_tree(&self, url: &Url) -> Option<&Tree> {
        match self.sources.get(url) {
            Some(properties) => Some(&properties.ast),
            None => None,
        }
    }

    /// Get the source code of a given url, return None if not found, byte vector otherwise
    pub fn get_source_code(&self, url: &Url) -> Option<Vec<Byte>> {
        self.sources
            .get(url)
            .map(|properties| properties.source_code.clone())
    }

    /// Get the language_id of a given url, return None if not found, language_id otherwise
    pub fn get_language_id(&self, url: &Url) -> Option<String> {
        self.sources
            .get(url)
            .map(|properties| properties.language_id.to_string())
    }

    /// Update the source code of a given url
    pub fn update_source_code(&mut self, url: &Url, new_source_code: Vec<Byte>) {
        if let Some(properties) = self.sources.get_mut(url) {
            properties.source_code = new_source_code;
        }
    }

    /// Get node at a given position
    pub fn _get_node_at_position(&self, url: &Url, position: Position) -> Option<Node> {
        let properties = self.sources.get(url)?;
        let node = properties.ast.root_node();
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

    /// Get document version of a given url, return 0 if not found, version otherwise
    pub fn get_version(&self, uri: &lsp_types::Url) -> Option<i32> {
        let source = self.sources.get(uri);
        match source {
            Some(source) => Some(source.version),
            // we don't have a version, return 0
            None => Some(0),
        }
    }

    pub fn clear(&mut self, uri: &lsp_types::Url) {
        match self.sources.get_mut(uri) {
            Some(properties) => {
                properties.clear();
            }
            None => {
                warn!("clear: no properties found for uri: {:?}", uri);
            }
        };
        self.diagnostics.clear();
    }
}
