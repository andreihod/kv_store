use std::sync::Arc;

use futures::prelude::*;
use tokio::net::TcpListener;
use tokio_serde::{formats::SymmetricalBincode, SymmetricallyFramed};

use crate::{
    read_write_streams,
    storage::{Expiration, Storage},
    Request, Response,
};

pub async fn serve(storage: Arc<Storage>) {
    let listener = TcpListener::bind("127.0.0.1:17653").await.unwrap();
    loop {
        let storage = storage.clone();

        let (socket, _) = listener.accept().await.unwrap();
        let (read, write) = read_write_streams(socket);

        let mut receiver = SymmetricallyFramed::new(read, SymmetricalBincode::<Request>::default());
        let mut sender = SymmetricallyFramed::new(write, SymmetricalBincode::default());

        tokio::spawn(async move {
            match receiver.try_next().await {
                Ok(Some(request)) => {
                    let response = handle_request(storage.clone(), request).await;
                    sender.send(response).await.unwrap();
                }
                _ => {
                    sender
                        .send(Response::Failure(String::from(
                            "Invalid or malformed command",
                        )))
                        .await
                        .unwrap();
                }
            }
        });
    }
}

async fn handle_request(storage: Arc<Storage>, request: Request) -> Response {
    let now = chrono::Utc::now().naive_utc();

    match request {
        Request::Put(key, value, Some(expiration)) => {
            storage
                .put(
                    key,
                    value,
                    Some(Expiration {
                        now,
                        seconds_to_expire: expiration,
                    }),
                )
                .await;
            Response::Success(None)
        }
        Request::Put(key, value, None) => {
            storage.put(key, value, None).await;
            Response::Success(None)
        }
        Request::Get(key) => Response::Success(storage.get(key, now).await),
        Request::Delete(key) => {
            storage.delete(key).await;
            Response::Success(None)
        }
    }
}
