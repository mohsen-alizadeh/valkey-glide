use magnus::{function, method, prelude::*, Error, Ruby};
use tokio::runtime::Builder;
use tokio::runtime::Runtime;
use glide_core::client;
use redis::{FromRedisValue, RedisResult};
use glide_core::client::Client as GlideClient;
use glide_core::request_type::RequestType;

#[magnus::wrap(class = "Valkey")]
struct Valkey {
    runtime: Runtime,
    client: GlideClient,
}

impl Valkey {
    fn new() -> Self {
        let request = create_connection_request("127.0.0.1".to_string(), 6379, false);
        let runtime = Builder::new_multi_thread()
            .enable_all()
            .thread_name("GLIDE for Redis C# thread")
            .build().unwrap();
        let _runtime_handle = runtime.enter();
        let client = runtime.block_on(GlideClient::new(request, None)).unwrap(); // TODO - handle errors.


        Self { client, runtime }
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

    fn set(&mut self, key: String, value: String) -> bool {
        let mut cmd = RequestType::Set.get_command().unwrap();
        cmd.arg(key).arg(value);

        self.runtime.block_on(async {
            self.client.send_command(&cmd, None).await;
        });

        true
    }
}

fn create_connection_request(host: String, port: u32, use_tls: bool) -> client::ConnectionRequest {
    let address_info = client::NodeAddress {
        host,
        port: port as u16,
    };
    let addresses = vec![address_info];
    client::ConnectionRequest {
        addresses,
        tls_mode: if use_tls {
            Some(client::TlsMode::SecureTls)
        } else {
            Some(client::TlsMode::NoTls)
        },
        ..Default::default()
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let class = ruby.define_class("Valkey", ruby.class_object())?;
    class.define_singleton_method("new", function!(Valkey::new, 0))?;
    class.define_method("create_client", method!(Valkey::create_client, 0))?;
    class.define_method("set", method!(Valkey::set, 2))?;
    Ok(())
}
