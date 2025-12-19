// cargo run --example m5_builder
//
// Demonstrates the Builder Pattern - a common Rust idiom for constructing
// complex objects step by step with optional configuration.

// The final object we want to build
struct Server {
    host: String,
    port: u16,
    max_connections: u32,
    timeout_seconds: u32,
}

// Builder struct - holds configuration before building
struct ServerBuilder {
    host: String,
    port: u16,
    max_connections: u32,
    timeout_seconds: u32,
}

impl ServerBuilder {
    // Start with sensible defaults
    fn new() -> Self {
        ServerBuilder {
            host: String::from("localhost"),
            port: 8080,
            max_connections: 100,
            timeout_seconds: 30,
        }
    }

    // Each setter takes self by value and returns Self
    // This allows method chaining: builder.host(...).port(...)
    fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    fn timeout(mut self, seconds: u32) -> Self {
        self.timeout_seconds = seconds;
        self
    }

    // Consume the builder and create the final object
    fn build(self) -> Server {
        Server {
            host: self.host,
            port: self.port,
            max_connections: self.max_connections,
            timeout_seconds: self.timeout_seconds,
        }
    }
}

fn main() {
    // Use all defaults
    let server1 = ServerBuilder::new().build();
    println!("Server 1: {}:{}", server1.host, server1.port);

    // Customize everything with method chaining
    let server2 = ServerBuilder::new()
        .host("0.0.0.0")
        .port(3000)
        .max_connections(1000)
        .timeout(60)
        .build();
    println!("Server 2: {}:{} (max: {}, timeout: {}s)",
        server2.host, server2.port,
        server2.max_connections, server2.timeout_seconds);

    // Partially customize - order doesn't matter
    let server3 = ServerBuilder::new()
        .timeout(120)
        .port(9000)
        .build();
    println!("Server 3: {}:{} (timeout: {}s)",
        server3.host, server3.port, server3.timeout_seconds);
}
