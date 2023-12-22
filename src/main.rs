use std::net::TcpListener;

use paperboy::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Couldn't bind to port 8080");
    run(listener)?.await
}
