use log::{debug, warn};

use lsp_types::{notification::Notification, request::Request as rr, InitializeParams};

use lsp_server::{Connection, Message, RequestId};
use std::error::Error;

use crate::{global_state, handler};

pub fn main_loop(
    connection: Connection,
    _params: InitializeParams,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    warn!("starting main loop");

    // define a AST,
    // parse a simple program make rust happy
    let mut global_state = global_state::GlobalState::new();

    for msg in &connection.receiver {
        // debug!("got msg: {:#?}", msg);
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                debug!("got request: {:#?}", req);

                match req.method.as_str() {
                    lsp_types::request::GotoDefinition::METHOD => {
                        let res: Result<
                            (RequestId, lsp_types::GotoDefinitionParams),
                            lsp_server::Request,
                        > = req.extract(lsp_types::request::GotoDefinition::METHOD);
                        match res {
                            Ok((id, params)) => {
                                let resp = handler::goto_definition(
                                    id,
                                    params,
                                    global_state.get_snapshot(),
                                );
                                connection.sender.send(Message::Response(resp))?;
                                continue;
                            }
                            Err(req) => req,
                        };
                    }
                    lsp_types::request::Completion::METHOD => {
                        let res: Result<
                            (RequestId, lsp_types::CompletionParams),
                            lsp_server::Request,
                        > = req.extract(lsp_types::request::Completion::METHOD);
                        match res {
                            Ok((id, params)) => {
                                let resp =
                                    handler::completion(id, params, global_state.get_snapshot());
                                connection.sender.send(Message::Response(resp))?;
                                continue;
                            }
                            Err(req) => req,
                        };
                    }
                    lsp_types::request::References::METHOD => {
                        let res: Result<
                            (RequestId, lsp_types::ReferenceParams),
                            lsp_server::Request,
                        > = req.extract(lsp_types::request::References::METHOD);
                        match res {
                            Ok((id, params)) => {
                                let resp =
                                    handler::references(id, params, global_state.get_snapshot());
                                connection.sender.send(Message::Response(resp))?;
                                continue;
                            }
                            Err(req) => req,
                        };
                    }

                    _ => {
                        warn!("unhandled request: {:#?}", req);
                    }
                }
            }
            Message::Response(resp) => {
                debug!("got response: {:#?}", resp);
            }
            Message::Notification(not) => {
                debug!("got notification: {:#?}", not);

                match not.method.as_str() {
                    lsp_types::notification::DidOpenTextDocument::METHOD => {
                        let not_res: Result<
                            lsp_types::DidOpenTextDocumentParams,
                            lsp_server::Notification,
                        > = not.extract(lsp_types::notification::DidOpenTextDocument::METHOD);
                        match not_res {
                            Ok(params) => {
                                handler::did_open(params.clone(), &mut global_state);
                                // publish diagnostics here.
                                let not: lsp_server::Notification = handler::publish_diagnostics(
                                    params.text_document.uri,
                                    global_state.get_snapshot(),
                                );
                                connection.sender.send(Message::Notification(not))?;

                                continue;
                            }
                            Err(not) => not,
                        };
                    }
                    lsp_types::notification::DidChangeTextDocument::METHOD => {
                        let not_res: Result<
                            lsp_types::DidChangeTextDocumentParams,
                            lsp_server::Notification,
                        > = not.extract(lsp_types::notification::DidChangeTextDocument::METHOD);
                        match not_res {
                            Ok(params) => {
                                handler::did_change(params.clone(), &mut global_state);

                                continue;
                            }
                            Err(not) => not,
                        };
                    }
                    lsp_types::notification::DidCloseTextDocument::METHOD => {
                        let not_res: Result<
                            lsp_types::DidCloseTextDocumentParams,
                            lsp_server::Notification,
                        > = not.extract(lsp_types::notification::DidCloseTextDocument::METHOD);
                        match not_res {
                            Ok(params) => {
                                handler::did_close(params);
                                continue;
                            }
                            Err(not) => not,
                        };
                    }

                    _ => {}
                }
            }
        }
    }
    Ok(())
}
