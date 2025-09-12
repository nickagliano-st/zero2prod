//! main.rs
use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = 8000; // TODO: Implement port configuration, probably in a configuration file
    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Failed to bind to port");

    run(listener)?.await
}
