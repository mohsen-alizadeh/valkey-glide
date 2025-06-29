use glide_core::command_request::command;
use magnus::{function, method, prelude::*, Error, Ruby};
use redis::Cmd;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;
use glide_core::client;
use redis::{FromRedisValue, RedisResult, Value};
use glide_core::client::Client as GlideClient;
use glide_core::request_type::RequestType;
use logger_core::log_debug;
use core::str;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::iter::once;
use std::rc::Rc;
use glide_core::connection_request;
use glide_core::command_request;
use glide_core::ConnectionRequest;
use magnus::TryConvert;
use protobuf::Message;

#[magnus::wrap(class = "Valkey")]
struct Valkey {
    // runtime: Runtime,
    // client: GlideClient,
}

// struct Valkey {
//     runtime: RefCell<Runtime>,
//     client: RefCell<GlideClient>,
// }

// #[magnus::wrap(class = "Valkey")]
// struct MutValkey(RefCell<Valkey>);

impl Valkey {
    fn new() -> Self {
        Self{}
    }
    //
    //
    //
    // fn new() -> Self {
    //     let request = create_connection_request("127.0.0.1".to_string(), 6379, false);
    //
    //     let runtime = Builder::new_current_thread()
    //         .enable_all()
    //         .build().
    //         unwrap();
    //
    //     let client = runtime.block_on(GlideClient::new(request, None)).unwrap();
    //
    //     Valkey{client: RefCell::new(client), runtime: RefCell::new(runtime)}
    // }

    // fn create_client(connection_request: magnus::RString) -> Self {
    //     // let request_bytes = unsafe { request.char_bytes().collect::<Vec<_>>() };
    //     let connection_request_bytes = unsafe { connection_request.as_slice() };
    //
    //     let request = connection_request::ConnectionRequest::parse_from_bytes(connection_request_bytes)
    //         .map_err(|err| err.to_string()).unwrap();
    //
    //     let runtime = Builder::new_current_thread()
    //         .enable_all()
    //         .worker_threads(1)
    //         .build()
    //         .unwrap();
    //
    //     let client = runtime
    //         .block_on(GlideClient::new(ConnectionRequest::from(request), None))
    //         .map_err(|err| err.to_string()).unwrap();
    //
    //     // Valkey{client: RefCell::new(client), runtime: RefCell::new(runtime)}
    //     Valkey{client: client, runtime: runtime}
    // }

    fn test(&self) -> isize {
        return 22;
    }

    fn execute(ruby: &Ruby, rb_self: &Self, request_string: magnus::Value) -> isize {
        return 22;
        // return "string".to_string();

        // let request_bytes = unsafe { request_string.as_slice() };
        //
        // let mut client = rb_self.client.clone();
        //
        // let request = command_request::Command::parse_from_bytes(request_bytes).expect("felan");
        // // let request = command_request::Command::parse_from_bytes(request_bytes).unwrap();
        //
        // let request_type: RequestType = request.request_type.into();
        // let cmd = request_type.get_command().expect("failed here 3");
        //

        // rb_self.runtime.block_on(async {
        //     Self::convert(client.send_command(&cmd, None).await.expect("failed 1"))
        // })
    }

    // fn convert(value: redis::Value) -> impl magnus::method::ReturnValue {
    //     match value {
    //         redis::Value::BulkString(v) => v,
    //         _ => todo!()
    //     }
    // }


    // fn set(ruby: &Ruby, rb_self: &Self, key: String, value: String) -> bool {
    //     log_debug("ID", "message");
    //     true
    // }

    // fn get(ruby: &Ruby, rb_self: &Self, key: String) -> String {
    //     log_debug("IDAAAAAAAAAAAAAAAAAAAA", "message");
    //
    //     return "felan".to_string();
    //
    //     let mut cmd = RequestType::Get.get_command().unwrap();
    //     cmd.arg(key);
    //
    //     // let mut borrow = self.0.borrow_mut();
    //
    //     let result = rb_self.runtime.borrow().block_on(async {
    //         rb_self.client.borrow_mut().send_command(&cmd, None).await.unwrap()
    //     });
    //
    //     match result {
    //         Value::SimpleString(text) => return text,
    //         Value::Nil => return "Nil".to_string(),
    //         Value::BulkString(text) => return str::from_utf8(&text).unwrap().to_string(),
    //         _ => todo!()
    //     }
    // }
}

// fn create_connection_request(host: String, port: u32, use_tls: bool) -> client::ConnectionRequest {
//     let address_info = client::NodeAddress {
//         host,
//         port: port as u16,
//     };
//     let addresses = vec![address_info];
//     client::ConnectionRequest {
//         addresses,
//         tls_mode: if use_tls {
//             Some(client::TlsMode::SecureTls)
//         } else {
//             Some(client::TlsMode::NoTls)
//         },
//         ..Default::default()
//     }
// }

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let class = ruby.define_class("Valkey", ruby.class_object())?;
    // class.define_singleton_method("new", function!(Valkey::new, 0))?;
    // class.define_singleton_method("create_client", function!(Valkey::create_client, 1))?;
    // class.define_method("create_client", function!(Valkey::create_client, 1))?;
    class.define_method("test", method!(Valkey::test, 0))?;
    // class.define_method("create_client", method!(MutValkey::create_client, 0))?;
    // class.define_method("set", method!(Valkey::set, 2))?;
    // class.define_method("get", method!(Valkey::get, 1))?;
    // class.define_method("execute", method!(Valkey::execute, 1))?;
    Ok(())
}
