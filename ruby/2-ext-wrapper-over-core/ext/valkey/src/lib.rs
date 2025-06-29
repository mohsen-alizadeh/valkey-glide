use magnus::{function, method, prelude::*, Error, Ruby};
use glide_core::connection_request;
use glide_core::command_request;
use glide_core::command_request::Command;
use glide_core::command_request::command;

use glide_core::client::Client as GlideClient;
use glide_core::request_type::RequestType;
use glide_core::request_type;
use glide_core::ConnectionRequest;
use protobuf::Message;
use tokio::runtime;
use core::str;
use bytes::Bytes;
use redis::Cmd;

#[magnus::wrap(class = "Client")]
struct ValkeyClient {
    runtime: runtime::Runtime,
    client: GlideClient,
}

impl ValkeyClient {
    fn new(connection_request: magnus::RString) -> Self {
        let connection_request_bytes = unsafe { connection_request.as_slice() };

        let request = connection_request::ConnectionRequest::parse_from_bytes(connection_request_bytes)
            .map_err(|err| err.to_string()).unwrap();

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .worker_threads(1)
            .build()
            .unwrap();

        let client = runtime
            .block_on(GlideClient::new(ConnectionRequest::from(request), None))
            .map_err(|err| err.to_string()).unwrap();

        ValkeyClient{client, runtime}
    }

    fn send_command(ruby: &Ruby, rb_self: &Self, request_string: magnus::RString) -> String {
        let request_bytes = unsafe { request_string.as_slice() };

        let mut client = rb_self.client.clone();

        let request = command_request::Command::parse_from_bytes(request_bytes).expect("felan");

        let cmd = get_redis_command(&request).unwrap();

        let result = rb_self.runtime.block_on(async {
            client.send_command(&cmd, None).await.unwrap()
        });

        return convert(result);
    }

    fn get_v2(rb_self: &Self, key: String) -> String {
        let mut client = rb_self.client.clone();

        let mut cmd = redis::cmd("GET");
        cmd.arg(key);

        let result = rb_self.runtime.block_on(async {
            client.send_command(&cmd, None).await.unwrap()
        });

        return convert(result);
    }
}

fn convert(value: redis::Value) -> String {
    match value {
        redis::Value::BulkString(v) => String::from_utf8_lossy(&v).to_string(),
        redis::Value::Okay => "OK".to_string(),
        _ => todo!()
    }
}

fn get_command(request: &Command) -> Option<Cmd> {
    let request_type: crate::request_type::RequestType = request.request_type.into();
    request_type.get_command()
}

fn get_redis_command(command: &Command) -> Result<Cmd, Error> {
    let mut cmd = get_command(command).unwrap();

    match &command.args {
        Some(command::Args::ArgsArray(args_vec)) => {
            for arg in args_vec.args.iter() {
                cmd.arg(arg.as_ref());
            }
        }
        Some(command::Args::ArgsVecPointer(pointer)) => {
            let res = *unsafe { Box::from_raw(*pointer as *mut Vec<Bytes>) };
            for arg in res {
                cmd.arg(arg.as_ref());
            }
        }
        &None => todo!(),
        &Some(_) => todo!()
    };


    Ok(cmd)
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let class = ruby.define_class("Client", ruby.class_object())?;
    class.define_singleton_method("new", function!(ValkeyClient::new, 1))?;

    class.define_method("send_command", method!(ValkeyClient::send_command, 1))?;
    class.define_method("get_v2", method!(ValkeyClient::get_v2, 1))?;

    Ok(())
}
