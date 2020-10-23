mod context;
mod server;

use env_logger;
use log;

use server::{Database, Request, Server};

/// Example code.
fn main() {
    // Initialize dependencies.
    env_logger::init();
    let db = Database::new();
    let srv = Server::new(db);

    // Blocking server would go here.
    let resp = srv.handle_req(get_request());
    log::info!("Response: {:?}", resp);
}

/// Faked utility functions.
fn get_request() -> Request {
    Request {
        path: "/get/example".to_string(),
    }
}
