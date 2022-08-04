mod caps;
mod global_state;
mod handler;
mod macros;
mod main_loop;

use std::error::Error;

use log::warn;
use lsp_server::Connection;
use lsp_types::InitializeParams;
use main_loop::main_loop;

pub fn server_mode() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that  we must have our logging only write out to stderr.
    warn!("starting generic LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    // let (connection, io_threads) = Connection::stdio();
    let (connection, io_threads) = Connection::listen("localhost:12345")?;

    let (connection_id, initialization_params) = connection.initialize_start()?;

    let init_params: InitializeParams = serde_json::from_value(initialization_params).unwrap();
    let client_capabilities: lsp_types::ClientCapabilities = init_params.capabilities.clone();
    // debug!("Client has capabilities: {:?}", client_capabilities);
    let caps = caps::new(client_capabilities);

    let initialize_data = serde_json::json!({
        "capabilities": caps,
        "serverInfo": {
            "name": "tsls",
            "version": "0.1"
        }
    });

    connection.initialize_finish(connection_id, initialize_data)?;

    main_loop(connection, init_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    warn!("shutting down server");
    Ok(())
}
