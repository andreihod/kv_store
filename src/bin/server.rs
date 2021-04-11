use std::sync::Arc;

use kv_store::{server::serve, storage::Storage};
pub fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let storage = Arc::new(Storage::new());

    rt.block_on(serve(storage));
}
