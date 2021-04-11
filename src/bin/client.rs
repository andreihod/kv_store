use clap::{App, Arg};
use kv_store::{client::send_request, Request};

#[tokio::main]
pub async fn main() {
    let matches = App::new("kv_store_client")
        .subcommand(
            App::new("put")
                .arg(Arg::new("KEY").required(true))
                .arg(Arg::new("VALUE").required(true))
                .arg(Arg::new("SECONDS_TO_EXPIRE")),
        )
        .subcommand(App::new("get").arg(Arg::new("KEY").required(true)))
        .subcommand(App::new("delete").arg(Arg::new("KEY").required(true)))
        .get_matches();

    let maybe_request = match matches.subcommand() {
        Some(("put", sub_m)) => {
            let key = sub_m.value_of("KEY").unwrap();
            let value = sub_m.value_of("VALUE").unwrap();
            let expiration: Option<i32> = sub_m
                .value_of("SECONDS_TO_EXPIRE")
                .map(|seconds| seconds.parse().expect("Invalid number"));

            Some(Request::Put(key.to_string(), value.to_string(), expiration))
        }
        Some(("get", sub_m)) => {
            let key = sub_m.value_of("KEY").unwrap();
            Some(Request::Get(key.to_string()))
        }
        Some(("delete", sub_m)) => {
            let key = sub_m.value_of("KEY").unwrap();
            Some(Request::Delete(key.to_string()))
        }
        _ => None,
    };

    if let Some(request) = maybe_request {
        println!("Server responded: {:?}", send_request(request).await);
    };
}
