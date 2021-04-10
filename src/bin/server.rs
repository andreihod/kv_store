use futures::prelude::*;
use kv_store::{read_write_streams, Request, Response};
use tokio::net::TcpListener;
use tokio_serde::{formats::SymmetricalBincode, SymmetricallyFramed};

#[tokio::main]
pub async fn main() {
    let listener = TcpListener::bind("127.0.0.1:17653").await.unwrap();

    println!("listening on {:?}", listener.local_addr());

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let (read, write) = read_write_streams(socket);

        let mut receiver = SymmetricallyFramed::new(read, SymmetricalBincode::<Request>::default());
        let mut sender = SymmetricallyFramed::new(write, SymmetricalBincode::default());

        tokio::spawn(async move {
            while let Some(msg) = receiver.try_next().await.unwrap() {
                println!("Server requested: {:?}", msg);
                sender.send(Response::Success(None)).await.unwrap();
            }
        });
    }
}
