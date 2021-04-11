use criterion::{criterion_group, criterion_main, Criterion};
use kv_store::{client::send_request, server::serve, storage::Storage, Request};
use rand::{thread_rng, Rng};
use std::sync::Arc;

async fn retriever() {
    let mut rng = thread_rng();
    let key = format!("key_{}", rng.gen::<u32>());
    send_request(Request::Get(key)).await;
}

async fn populate_storage() {
    for n in 0..500_000 {
        let key = format!("key_{}", n);
        let value = String::from("some data");
        send_request(Request::Put(key, value, None)).await;
    }
}

fn retrieve(c: &mut Criterion) {
    // Start server
    let rt = tokio::runtime::Runtime::new().unwrap();
    let storage = Arc::new(Storage::new());
    rt.spawn(serve(storage));
    println!("Populating 500_000 key-values, it may take a while...");
    rt.block_on(populate_storage());

    c.bench_function("retrieve", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| retriever())
    });
}

criterion_group!(benches, retrieve);
criterion_main!(benches);
