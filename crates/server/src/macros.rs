#[macro_export]
macro_rules! req_match {
    ($req:expr, $conn:expr, $snapshot:expr) => {
        use lsp_types::request::{
            Completion, DocumentSymbolRequest, GotoDefinition, References, Rename,
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
