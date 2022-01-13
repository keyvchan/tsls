use lsp_types::{Diagnostic, Position, Url};
use queries::utils::Symbol;
use std::collections::HashMap;
use tree_sitter::{Node, Range, Tree};

type Byte = u8;
type ScopeID = usize;

#[derive(Debug, Clone)]
pub struct Properties {
    pub ast: Tree,
    // pub source_code: TextDocumentItem,
    pub language_id: String,
    pub version: i32,

    // use Vec<line<column>> store the content also preserve the order
    pub source_code: Vec<Byte>,
    pub keywords: Vec<String>,
    pub ordered_scopes: Vec<Range>,
    pub definitions_lookup_map: HashMap<String, Vec<Symbol>>,
    pub identifiers: HashMap<ScopeID, Vec<Symbol>>,
}

#[derive(Debug, Clone)]
pub struct GlobalState {
    pub sources: HashMap<Url, Properties>,
    pub diagnostics: HashMap<Url, Vec<Diagnostic>>,
}

impl GlobalState {
    pub fn new() -> Self {
        GlobalState {
            sources: HashMap::new(),
            diagnostics: HashMap::new(),
        }
    }

    pub fn get_mutable_tree(&mut self, url: &Url) -> Option<&mut Tree> {
        match self.sources.get_mut(url) {
            Some(properties) => Some(&mut properties.ast),
            None => None,
        }
    }

    pub fn get_source_code(&self, url: &Url) -> Option<Vec<Byte>> {
        self.sources
            .get(url)
            .map(|properties| properties.source_code.clone())
    }

    pub fn get_language_id(&self, url: &Url) -> Option<String> {
        self.sources
            .get(url)
            .map(|properties| properties.language_id.to_string())
    }

    pub fn update_source_code(&mut self, url: &Url, new_source_code: Vec<Byte>) {
        if let Some(properties) = self.sources.get_mut(url) {
            properties.source_code = new_source_code;
        }
    }

    pub fn get_node_at_position(&self, url: &Url, position: Position) -> Option<Node> {
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
}
