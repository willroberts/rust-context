use log;
use std::time::{Duration, SystemTime};

use crate::server::Request;

/// Context flows down the stack and is request-scoped.
pub struct Context {
    pub resource: String,
    pub trace: Trace,
}
impl Context {
    pub fn from_request(req: &Request) -> Context {
        Context {
            resource: req.path.clone(),
            trace: Trace { spans: Vec::new() },
        }
    }
}

/// Traces are request-scoped and stored in a Context.
pub struct Trace {
    spans: Vec<Span>,
}
impl Trace {
    pub fn stop(self) {
        let mut duration = Duration::from_millis(0);
        for s in self.spans {
            if s.parent == None {
                duration += s.duration;
            }
        }
        log::trace!("Trace duration: {:?}", duration);
    }
}

pub struct Span {
    id: u64,
    name: String,
    start_time: SystemTime,
    duration: Duration,
    parent: Option<u64>,
}
impl Span {
    pub fn new(name: String, id: u64, parent: Option<u64>) -> Span {
        Span {
            name: name,
            id: id,
            start_time: SystemTime::now(),
            duration: Duration::from_millis(0),
            parent: parent,
        }
    }
    pub fn stop(mut self, trace: &mut Trace) {
        let mut d = self.duration;
        match self.start_time.elapsed() {
            Ok(dur) => {
                d = dur;
            }
            Err(_) => {}
        };
        self.duration = d;
        log::trace!("Span '{}' with ID {} had duration {:?}", self.name, self.id, self.duration);
        trace.spans.push(self);
    }
}
