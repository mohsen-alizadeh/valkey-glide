use magnus::{function, method, prelude::*, Error, Ruby};

use tokio::runtime::Builder;
use tokio::runtime::Runtime;
use redis::{FromRedisValue, RedisResult};

use logger_core::log_debug;
use glide_core::client;
use glide_core::client::Client as GlideClient;
use std::cell::RefCell;
use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
};

#[magnus::wrap(class = "Valkey")]
pub struct Valkey {
    // client: GlideClient,
    // runtime: Runtime,
    number: isize
}

// struct MutPoint(RefCell<Point>);

// pub struct Client {
//     client: GlideClient,
//     runtime: Runtime,
// }
//
// fn hello(subject: String) -> String {
//     format!("Hello from Rust, {subject}!")
// }

impl Valkey {
    fn new(ruby: &Ruby, x: isize) -> Self {
        // let request = create_connection_request("17.0.0.1".to_string(), 1739, false);
        // let runtime = Builder::new_multi_thread()
        //     .enable_all()
        //     .thread_name("GLIDE for Redis C# thread")
        //     .build().unwrap();
        // let _runtime_handle = runtime.enter();
        // let client = runtime.block_on(GlideClient::new(request, None)).unwrap(); // TODO - handle errors.
        //                                                                          //
        //                                                                          //
        //                                                                          //
        //                                                                          //
        // Self { client, runtime, number: 88 }
        Self { number: 1 }
    }

    fn create_client(&self) -> String {
        // let request = create_connection_request("17.0.0.1".to_string(), 1739, false);
        // self.runtime = Builder::new_multi_thread()
        //     .enable_all()
        //     .thread_name("GLIDE for Redis C# thread")
        //     .build().unwrap();
        // let _runtime_handle = self.runtime.enter();
        // let client = self.runtime.block_on(GlideClient::new(request, None)).unwrap(); // TODO - handle errors.


        // rb_self.x = 1;
        // rb_self.number

        "sss".to_string()
        // self.client = client;
    }
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
//
// fn create_client() -> RedisResult<Client> {
//
//     // let host_cstring = unsafe { CStr::from_ptr(host as *mut c_char) };
//     // let host_string = host_cstring.to_str()?.to_string();
//     let request = create_connection_request("17.0.0.1".to_string(), 1739, false);
//     let runtime = Builder::new_multi_thread()
//         .enable_all()
//         .thread_name("GLIDE for Redis C# thread")
//         .build()?;
//     let _runtime_handle = runtime.enter();
//     let client = runtime.block_on(GlideClient::new(request, None)).unwrap(); // TODO - handle errors.
//     Ok(Client {
//         client,
//         runtime,
//     })
// }

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let class = ruby.define_class("Valkey", ruby.class_object())?;
    class.define_method("create_client", method!(Valkey::create_client, 0))?;
    Ok(())
}
