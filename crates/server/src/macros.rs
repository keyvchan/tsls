pub mod request {
    /// Match a request against a list of patterns.
    #[macro_export]
    macro_rules! req_match {
        ($req:expr, $conn:expr, $snapshot:expr) => {
            use lsp_types::request::{
                Completion, DocumentSymbolRequest, GotoDefinition, References, Rename, Request,
            };
            match $req.method.as_str() {
                GotoDefinition::METHOD => req!(goto_definition, $req, $conn, $snapshot),
                Rename::METHOD => req!(rename, $req, $conn, $snapshot),
                Completion::METHOD => req!(completion, $req, $conn, $snapshot),
                DocumentSymbolRequest::METHOD => req!(document_symbol, $req, $conn, $snapshot),
                References::METHOD => req!(references, $req, $conn, $snapshot),
                _ => {
                    error!("unhandled request: {:?}", $req);
                    continue;
                }
            }
        };
    }

    /// Match a request
    #[macro_export]
    macro_rules! req {
        ($method:ident, $req:expr, $conn:expr, $snapshot:expr) => {{
            let res = $req.clone().extract(&$req.method);
            match res {
                Ok((id, params)) => {
                    let resp = handler::$method(id, params, $snapshot);
                    $conn.sender.send(Message::Response(resp))?;
                    continue;
                }
                Err(req) => req,
            };
        }};
    }
}

pub mod notification {

    /// received a notification
    #[macro_export]
    macro_rules! not {
        ($method:ident, $params_type:ident, $not:expr, $conn:expr, $state:ident) => {{
            let not_res = $not.clone().extract::<$params_type>(&$not.method);
            match not_res {
                Ok(params) => {
                    handler::$method(params.clone(), &mut $state);
                    let not = handler::publish_diagnostics(
                        params.text_document.uri,
                        $state.get_snapshot(),
                    );
                    $conn.sender.send(Message::Notification(not))?;
                    continue;
                }
                Err(not) => not,
            };
        }};
    }

    /// Match a notification against a list of patterns.
    #[macro_export]
    macro_rules! not_match {
        ($not:expr, $conn:expr, $state:ident) => {
            use lsp_types::{
                notification::{
                    DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Notification,
                },
                DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
            };
            match $not.method.as_str() {
                DidOpenTextDocument::METHOD => {
                    not!(did_open, DidOpenTextDocumentParams, $not, $conn, $state)
                }
                DidChangeTextDocument::METHOD => {
                    not!(did_change, DidChangeTextDocumentParams, $not, $conn, $state)
                }
                DidCloseTextDocument::METHOD => {
                    not!(did_close, DidCloseTextDocumentParams, $not, $conn, $state)
                }

                _ => {
                    error!("unhandled notification: {:?}", $not);
                    continue;
                }
            }
        };
    }
}
