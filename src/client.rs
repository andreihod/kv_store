use futures::prelude::*;
use tokio::net::TcpStream;
use tokio_serde::{formats::SymmetricalBincode, SymmetricallyFramed};

use crate::{read_write_streams, Request, Response};

pub async fn send_request(request: Request) -> Response {
    let socket = TcpStream::connect("127.0.0.1:17653").await.unwrap();
    let (read, write) = read_write_streams(socket);

    let mut receiver = SymmetricallyFramed::new(read, SymmetricalBincode::<Response>::default());
    let mut sender = SymmetricallyFramed::new(write, SymmetricalBincode::default());

    sender.send(request).await.unwrap();

    if let Some(msg) = receiver.try_next().await.unwrap() {
        return msg;
    }

    Response::Failure(String::from("No response from server"))
}
