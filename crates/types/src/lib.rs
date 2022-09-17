// Internal representation of all types in this project

use lsp_types::Url;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    pub url: Url,
    pub text: String,
    pub language_id: String,
    pub version: i32,
}
