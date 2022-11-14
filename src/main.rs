#[macro_use]
extern crate lazy_static;

mod config;
mod scan;
use config::ARGS;
use scan::open;

#[tokio::main]
async fn main() {
    open().await;
}
