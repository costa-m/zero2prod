use std::net::TcpListener;

use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let address = listener.local_addr().unwrap();
    println!("{}", address);
    run(listener)?.await
}
