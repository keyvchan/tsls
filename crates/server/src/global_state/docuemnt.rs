use tree_sitter::Tree;

/// Document contains the ast and the source code, it is updated when the document is edited
#[derive(Debug, Clone)]
pub struct Document {
    tree: Tree,
    // pub source_code: TextDocumentItem,
    version: i32,

    // use byte vector store the source code
    source_code: Vec<u8>,
}

impl Document {
    pub fn new(tree: Tree, version: i32, source_code: Vec<u8>) -> Self {
        Self {
            tree,
            version,
            source_code,
        }
    }

    /// Get a reference to the document's ast.
    #[must_use]
    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    /// Get a mutable reference to the document's ast.
    #[must_use]
    pub fn tree_mut(&mut self) -> &mut Tree {
        &mut self.tree
    }

    /// Get the document's version.
    #[must_use]
    pub fn version(&self) -> i32 {
        self.version
    }

    /// Set the document's version.
    pub fn set_version(&mut self, version: i32) {
        self.version = version;
    }

    /// Set the document's ast.
    pub fn set_tree(&mut self, tree: Tree) {
        self.tree = tree;
    }

    /// Get a reference to the document's source code.
    #[must_use]
    pub fn source_code(&self) -> &Vec<u8> {
        self.source_code.as_ref()
    }

    /// Get a mutable reference to the document's source code.
    #[must_use]
    pub fn source_code_mut(&mut self) -> &mut Vec<u8> {
        &mut self.source_code
    }

    /// Set the document's source code.
    pub fn set_source_code(&mut self, source_code: &Vec<u8>) {
        self.source_code = *source_code;
    }
}
