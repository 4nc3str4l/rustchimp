pub mod lib;

use rustchimp::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
