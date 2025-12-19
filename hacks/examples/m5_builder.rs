// cargo run --example m5_builder

struct Server {
    host: String,
    port: u16,
}

struct ServerBuilder {
    host: String,
    port: u16,
}

impl ServerBuilder {
    fn new() -> Self {
        ServerBuilder { host: String::from("localhost"), port: 8080 }
    }

    fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn build(self) -> Server {
        Server { host: self.host, port: self.port }
    }
}

fn main() {
    let server = ServerBuilder::new()
        .host("0.0.0.0")
        .port(3000)
        .build();

    println!("Server: {}:{}", server.host, server.port);
}
