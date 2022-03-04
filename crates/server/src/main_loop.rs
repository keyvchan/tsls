use log::{debug, error, warn};

use lsp_types::{
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Notification,
    },
    request::{
        Completion, DocumentSymbolRequest, GotoDefinition, References, Rename, Request as rr,
    },
    DidOpenTextDocumentParams, InitializeParams,
};

use lsp_server::{Connection, Message};
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
                debug!("got request: {:?}", req);

                match req.method.as_str() {
                    GotoDefinition::METHOD => {
                        let res = req.clone().extract(&req.method);
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
                    Completion::METHOD => {
                        let res = req.clone().extract(&req.method);
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
                    References::METHOD => {
                        let res = req.clone().extract(&req.method);
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

                    Rename::METHOD => {
                        let res = req.clone().extract(&req.method);
                        match res {
                            Ok((id, params)) => {
                                let resp = handler::rename(id, params, global_state.get_snapshot());
                                connection.sender.send(Message::Response(resp))?;
                                continue;
                            }
                            Err(req) => req,
                        };
                    }
                    DocumentSymbolRequest::METHOD => {
                        let res = req.clone().extract(&req.method);
                        match res {
                            Ok((id, params)) => {
                                let resp = handler::document_symbol(
                                    id,
                                    params,
                                    global_state.get_snapshot(),
                                );
                                connection.sender.send(Message::Response(resp))?;
                                continue;
                            }
                            Err(req) => {
                                error!("error: {:#?}", req);
                                req
                            }
                        };
                    }
                    _ => {
                        warn!("unhandled request: {:?}", req);
                    }
                }
            }
            Message::Response(resp) => {
                debug!("got response: {:?}", resp);
            }
            Message::Notification(not) => {
                debug!("got notification: {:?}", not);

                match not.method.as_str() {
                    DidOpenTextDocument::METHOD => {
                        let not_res = not
                            .clone()
                            .extract::<DidOpenTextDocumentParams>(&not.method);
                        match not_res {
                            Ok(params) => {
                                handler::did_open(params.clone(), &mut global_state);

                                // publish diagnostics here.
                                let not = handler::publish_diagnostics(
                                    params.text_document.uri,
                                    global_state.get_snapshot(),
                                );
                                connection.sender.send(Message::Notification(not))?;

                                continue;
                            }
                            Err(not) => not,
                        };
                    }
                    DidChangeTextDocument::METHOD => {
                        match not.clone().extract(&not.method.clone()) {
                            Ok(params) => {
                                handler::did_change(params, &mut global_state);

                                continue;
                            }
                            Err(not) => not,
                        };
                    }
                    DidCloseTextDocument::METHOD => {
                        let not_res = not.clone().extract(&not.method.clone());
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
