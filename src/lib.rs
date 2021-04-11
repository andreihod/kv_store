use serde::{Deserialize, Serialize};
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpStream,
};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

pub mod client;
pub mod server;
pub mod storage;

pub fn read_write_streams(
    stream: TcpStream,
) -> (
    FramedRead<OwnedReadHalf, LengthDelimitedCodec>,
    FramedWrite<OwnedWriteHalf, LengthDelimitedCodec>,
) {
    let (read_socket, write_socket) = stream.into_split();

    let write_length_delimited = FramedWrite::new(write_socket, LengthDelimitedCodec::new());
    let read_length_delimited = FramedRead::new(read_socket, LengthDelimitedCodec::new());

    (read_length_delimited, write_length_delimited)
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Request {
    Put(String, String, Option<i32>),
    Get(String),
    Delete(String),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Response {
    Success(Option<String>),
    Failure(String),
}
