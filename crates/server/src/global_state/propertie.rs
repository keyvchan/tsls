/// Properties contains the properties which are not changed during the lifetime of a
/// document. These properties are immutable and are shared between all the stages of a document.
#[derive(Debug, Clone)]
pub struct Properties {
    // pub source_code: TextDocumentItem,
    language_id: String,

    keywords: Vec<String>,
}

impl Properties {
    pub fn new(language_id: &String, keywords: &Vec<String>) -> Self {
        Self {
            language_id: *language_id,
            keywords: *keywords,
        }
    }

    /// Get a reference to the properties's language id.
    #[must_use]
    pub fn language_id(&self) -> &str {
        self.language_id.as_ref()
    }

    /// Get a reference to the properties's keywords.
    #[must_use]
    pub fn keywords(&self) -> &Vec<String> {
        self.keywords.as_ref()
    }
}
