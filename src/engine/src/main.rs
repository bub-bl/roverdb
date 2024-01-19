use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::server::Server;

pub mod server;

fn main() {
    println!("Hello World!");

    // Create new tcp server
    let mut server = Server::new();
    server.start();
}