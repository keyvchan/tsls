use lsp_types::{
    ClientCapabilities, CompletionOptions, DeclarationCapability, OneOf, SaveOptions,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    TextDocumentSyncSaveOptions, WorkDoneProgressOptions,
};

/// The capabilities provided by the client (editor)
/// use config to enable/disable capabilities
pub fn new(client_caps: ClientCapabilities) -> ServerCapabilities {
    let server_caps = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::INCREMENTAL),
                will_save: None,
                will_save_wait_until: None,
                // we don't need text here since we already processed in didchange
                save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                    include_text: Some(false),
                })),
            },
        )),
        completion_provider: Some(CompletionOptions {
            resolve_provider: None,
            trigger_characters: Some(vec![".".to_string()]),
            all_commit_characters: None,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        }),
        declaration_provider: Some(DeclarationCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        rename_provider: None,
        document_symbol_provider: None,
        hover_provider: None,
        signature_help_provider: None,
        type_definition_provider: None,
        implementation_provider: None,
        document_highlight_provider: None,
        workspace_symbol_provider: None,
        code_action_provider: None,
        code_lens_provider: None,
        document_formatting_provider: Some(OneOf::Left(true)),
        document_range_formatting_provider: None,
        document_on_type_formatting_provider: None,
        selection_range_provider: None,
        folding_range_provider: None,
        linked_editing_range_provider: None,
        document_link_provider: None,
        color_provider: None,
        execute_command_provider: None,
        workspace: None,
        call_hierarchy_provider: None,
        semantic_tokens_provider: None,
        moniker_provider: None,
        experimental: None,
    };

    merge_capabilities(client_caps, server_caps)
}

fn merge_capabilities(
    _client_caps: ClientCapabilities,
    server_caps: ServerCapabilities,
) -> ServerCapabilities {
    server_caps
}
