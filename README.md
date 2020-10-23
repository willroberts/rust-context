# rust-context

Example repo demonstrating a request-scoped Context object which flows down the
stack. Allows request tracing without explicitly passing Trace or Span objects.

This is largely based on the premise of Context in Go, but is used for tracing
instead of request cancellation.

## contents

`main.rs`: initializes a fake server and database, and sends a request
`context.rs`: contains the types for Context, Trace, and Span
`server.rs` contains methods to simulate work, using Context to measure duration

## example output

```
[2020-10-23T19:54:33Z DEBUG rust_context::server] quering database for key 'key'
[2020-10-23T19:54:33Z TRACE rust_context::context] Span 'database' with ID 20 had duration 42.22ms
[2020-10-23T19:54:33Z TRACE rust_context::context] Span 'handler' with ID 10 had duration 85.115ms
[2020-10-23T19:54:33Z TRACE rust_context::context] Trace duration: 85.115ms
[2020-10-23T19:54:33Z INFO  rust_context] Response: Response { body: "value" }
```
