use futures::prelude::*;
use kv_store::{read_write_streams, Request, Response};
use tokio::net::TcpStream;
use tokio_serde::{formats::*, SymmetricallyFramed};

#[tokio::main]
pub async fn main() {
    // Bind a server socket
    let socket = TcpStream::connect("127.0.0.1:17653").await.unwrap();
    let (read, write) = read_write_streams(socket);

    let mut receiver = SymmetricallyFramed::new(read, SymmetricalBincode::<Response>::default());
    let mut sender = SymmetricallyFramed::new(write, SymmetricalBincode::default());

    sender
        .send(Request::Put(
            String::from("Name"),
            String::from("Andrei"),
            None,
        ))
        .await
        .unwrap();

    while let Some(msg) = receiver.try_next().await.unwrap() {
        println!("Server response: {:?}", msg);
        break;
    }
}
