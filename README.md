# rust-context

Example repo demonstrating a request-scoped Context object which flows down the
stack. Allows request tracing without explicitly passing Trace or Span objects.

This is largely based on the premise of Context in Go, but is used for tracing
instead of request cancellation.

## contents

`main.rs`: initializes a fake server and database, and sends a request
`context.rs`: contains the types for Context, Trace, and Span
`server.rs` contains methods to simulate work, using Context to measure duration
