pub mod lib;

use std::net::TcpListener;

use rustchimp::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run(TcpListener::bind("127.0.0.1:8000").unwrap())?.await
}
