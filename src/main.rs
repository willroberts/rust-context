mod context;
mod server;

use env_logger;
use log;

use server::{Database, Request, Server};

fn main() {
    // Initialize dependencies.
    env_logger::init();
    let db = Database::new();
    let srv = Server::new(db);

    // Blocking server would go here.
    let resp = srv.handle_req(Request {
        path: "/get/example".to_string(),
    });
    log::info!("Response: {:?}", resp);
}
