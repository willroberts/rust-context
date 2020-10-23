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
            duration += s.duration;
        }
        log::trace!("Trace duration: {:?}", duration);
    }
}

pub struct Span {
    name: String,
    start_time: SystemTime,
    duration: Duration,
}
impl Span {
    pub fn new(name: String) -> Span {
        Span {
            name: name,
            start_time: SystemTime::now(),
            duration: Duration::from_millis(0),
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
        log::trace!("Span '{}' had duration {:?}", self.name, self.duration);
        trace.spans.push(self);
    }
}
