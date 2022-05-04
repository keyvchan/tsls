use std::collections::HashMap;

use helper::types::Symbol;
use tree_sitter::Range;

/// ParsedProperties contains the properties should be updated after edited, but it can be posponed
/// after saved.
#[derive(Debug, Clone)]
pub struct ParsedInfo {
    ordered_scopes: Vec<Range>,
    definitions_lookup_map: HashMap<String, Vec<Symbol>>,
    identifiers: HashMap<usize, Vec<Symbol>>,
}

impl ParsedInfo {
    pub fn new(
        ordered_scopes: &Vec<Range>,
        definitions_lookup_map: &HashMap<String, Vec<Symbol>>,
        identifiers: &HashMap<usize, Vec<Symbol>>,
    ) -> Self {
        Self {
            ordered_scopes: *ordered_scopes,
            definitions_lookup_map: *definitions_lookup_map,
            identifiers: *identifiers,
        }
    }

    /// Get a reference to the parsed info's ordered scopes.
    #[must_use]
    pub fn ordered_scopes(&self) -> &Vec<Range> {
        &self.ordered_scopes
    }

    /// Get a mutable reference to the parsed info's ordered scopes.
    #[must_use]
    pub fn ordered_scopes_mut(&mut self) -> &mut Vec<Range> {
        &mut self.ordered_scopes
    }

    /// Set the parsed info's ordered scopes.
    pub fn set_ordered_scopes(&mut self, ordered_scopes: Vec<Range>) {
        self.ordered_scopes = ordered_scopes;
    }

    /// Get a reference to the parsed info's definitions lookup map.
    #[must_use]
    pub fn definitions_lookup_map(&self) -> &HashMap<String, Vec<Symbol>> {
        &self.definitions_lookup_map
    }

    /// Get a mutable reference to the parsed info's definitions lookup map.
    #[must_use]
    pub fn definitions_lookup_map_mut(&mut self) -> &mut HashMap<String, Vec<Symbol>> {
        &mut self.definitions_lookup_map
    }

    /// Set the parsed info's definitions lookup map.
    pub fn set_definitions_lookup_map(
        &mut self,
        definitions_lookup_map: HashMap<String, Vec<Symbol>>,
    ) {
        self.definitions_lookup_map = definitions_lookup_map;
    }

    /// Get a reference to the parsed info's identifiers.
    #[must_use]
    pub fn identifiers(&self) -> &HashMap<usize, Vec<Symbol>> {
        &self.identifiers
    }

    /// Get a mutable reference to the parsed info's identifiers.
    #[must_use]
    pub fn identifiers_mut(&mut self) -> &mut HashMap<usize, Vec<Symbol>> {
        &mut self.identifiers
    }

    /// Set the parsed info's identifiers.
    pub fn set_identifiers(&mut self, identifiers: HashMap<usize, Vec<Symbol>>) {
        self.identifiers = identifiers;
    }
}
