use glide_core::connection_request;
use magnus::{define_class, function, prelude::*, Error, RString, Ruby, Value, class};
use glide_ffi;
use glide_core;

fn hello(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}


    // connection_request_bytes: *const u8,
    // connection_request_len: usize,
    // client_type: *const ClientType,
    // pubsub_callback: PubSubCallback,

fn create_client(connection_request: RString) -> Result<Value, Error> {
    return Ok(connection_request.as_value());

}


#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let valkey_glide = define_class("ValkeyGlide", class::object())?;
    let module = valkey_glide.define_module("Native")?;
    module.define_singleton_method("create_client", function!(create_client, 1))?;

    // module.define_singleton_method("hello", function!(hello, 1))?;
    Ok(())
}
