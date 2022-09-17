use std::error::Error;

use database::GlobalState;
use log::{debug, error, warn};
use lsp_server::{Connection, Message};
use lsp_types::InitializeParams;

use crate::{handler, not, not_match, req, req_match};

pub fn main_loop(
    connection: Connection,
    _params: InitializeParams,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    warn!("starting main loop");

    let mut global_state = GlobalState::new();

    for msg in &connection.receiver {
        // debug!("got msg: {:#?}", msg);
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                debug!("got request: {:?}", req);

                req_match!(req, connection, global_state.snapshot());
            }
            Message::Response(resp) => {
                debug!("got response: {:?}", resp);
            }
            Message::Notification(not) => {
                debug!("got notification: {:?}", not);
                not_match!(not, connection, global_state);
            }
        }
    }
    Ok(())
}
