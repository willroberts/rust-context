use log;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

use crate::context::{Context, Span};

/// Utility structs for request / response semantics.
pub struct Request {
    pub path: String,
}
#[derive(Debug)]
pub struct Response {
    pub body: String,
}

/// Server handles requests, e.g. a GRPC service.
pub struct Server {
    database: Database,
}
impl Server {
    pub fn new(db: Database) -> Server {
        Server { database: db }
    }
    pub fn handle_req(&self, req: Request) -> Response {
        let span = Span::new("handler".to_string());
        let mut ctx = Context::from_request(&req);
        do_work(10, 100);
        let val = self.database.query(&mut ctx, "key".to_string());
        span.stop(&mut ctx.trace);
        ctx.trace.stop();
        Response { body: val }
    }
}

/// Database handles queries, e.g. a Cassandra client.
pub struct Database {}
impl Database {
    pub fn new() -> Database {
        Database {}
    }
    pub fn query(&self, ctx: &mut Context, key: String) -> String {
        let span = Span::new("database".to_string());
        log::debug!("quering database for key '{}'", key);
        do_work(20, 200);
        span.stop(&mut ctx.trace);
        "value".to_string()
    }
}

/// Faked utility functions.
pub fn do_work(min_time: u64, max_time: u64) {
    let mut rng = rand::thread_rng();
    let dur = Duration::from_millis(rng.gen_range(min_time, max_time));
    sleep(dur);
}
