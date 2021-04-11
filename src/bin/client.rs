use kv_store::{client::send_request, Request};

#[tokio::main]
pub async fn main() {
    let response = send_request(Request::Put(
        String::from("Name"),
        String::from("Andrei"),
        None,
    ))
    .await;

    println!("Server responded: {:?}", response);
}
