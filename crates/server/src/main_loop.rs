use std::error::Error;

use log::{debug, error, warn};
use lsp_server::{Connection, Message};
use lsp_types::{
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Notification,
    },
    request::Request,
    DidOpenTextDocumentParams, InitializeParams,
};

use crate::{global_state, handler, req, req_match};

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

                req_match!(req, connection, global_state.get_snapshot());
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
