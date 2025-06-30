use glide_core::connection_request;
use magnus::{class, define_class, function, prelude::*, value::qnil, Error, IntoValue, RString, Ruby, Value};
use glide_ffi;
use glide_core;

fn hello(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}

extern "C-unwind" fn pubsub_callback(
    client_ptr: usize,
    kind: glide_ffi::PushKind,
    message: *const u8,
    message_len: i64,
    channel: *const u8,
    channel_len: i64,
    pattern: *const u8,
    pattern_len: i64,
) -> () {

}

    // connection_request_bytes: *const u8,
    // connection_request_len: usize,
    // client_type: *const ClientType,
    // pubsub_callback: PubSubCallback,

fn create_client(connection_request: RString) -> Result<Value, Error> {
    let connection_request_bytes = connection_request.to_string().unwrap().as_bytes().to_vec();
    let client_type = glide_ffi::ClientType::SyncClient;

    let connection_response = unsafe {
        glide_ffi::create_client(
            connection_request_bytes.as_ptr(),
            connection_request_bytes.len(),
            &client_type as *const glide_ffi::ClientType, pubsub_callback
            )
    };

    dbg!(connection_response);

    Ok(qnil().into_value())
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let valkey_glide = define_class("ValkeyGlide", class::object())?;
    let module = valkey_glide.define_module("Native")?;
    module.define_singleton_method("create_client", function!(create_client, 1))?;

    // module.define_singleton_method("hello", function!(hello, 1))?;
    Ok(())
}

