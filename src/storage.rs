use chrono::{Duration, NaiveDateTime};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct Storage {
    hashmap: RwLock<HashMap<String, (String, Option<NaiveDateTime>)>>,
}

pub struct Expiration {
    pub now: NaiveDateTime,
    pub seconds_to_expire: i32,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            hashmap: RwLock::new(HashMap::new()),
        }
    }

    pub async fn put(&self, key: String, value: String, expiration: Option<Expiration>) {
        let end_time = expiration
            .map(|config| config.now + Duration::seconds(config.seconds_to_expire as i64));
        self.hashmap.write().await.insert(key, (value, end_time));
    }

    pub async fn get(&self, key: String, now: NaiveDateTime) -> Option<String> {
        if let Some((value, expiration)) = self.hashmap.read().await.get(&key) {
            if is_valid(&now, expiration) {
                return Some(value.clone());
            }
        }

        None
    }

    pub async fn delete(&self, key: String) {
        self.hashmap.write().await.remove(&key);
    }
}

fn is_valid(now: &NaiveDateTime, expiration: &Option<NaiveDateTime>) -> bool {
    if let Some(end) = expiration {
        return end >= now;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn put_adds_a_new_key_value_successfully() {
        let storage = Storage::new();
        let now = chrono::Utc::now().naive_utc();

        storage
            .put(String::from("Key"), String::from("Value"), None)
            .await;

        assert_eq!(
            storage.get(String::from("Key"), now).await,
            Some(String::from("Value"))
        );
    }

    #[tokio::test]
    async fn get_fetches_some_value_successfully_before_expiration() {
        let storage = Storage::new();
        let now = chrono::Utc::now().naive_utc();

        let expiration = Expiration {
            now,
            seconds_to_expire: 20,
        };

        let now_plus_10_sec = now + Duration::seconds(10);

        storage
            .put(String::from("Key"), String::from("Value"), Some(expiration))
            .await;

        assert_eq!(
            storage.get(String::from("Key"), now_plus_10_sec).await,
            Some(String::from("Value"))
        );
    }

    #[tokio::test]
    async fn get_fetches_none_successfully_after_expiration() {
        let storage = Storage::new();
        let now = chrono::Utc::now().naive_utc();

        let expiration = Expiration {
            now,
            seconds_to_expire: 20,
        };

        let now_plus_25_sec = now + Duration::seconds(25);

        storage
            .put(String::from("Key"), String::from("Value"), Some(expiration))
            .await;

        assert_eq!(
            storage.get(String::from("Key"), now_plus_25_sec).await,
            None
        );
    }

    #[tokio::test]
    async fn delete_removes_data_successfully() {
        let storage = Storage::new();
        let now = chrono::Utc::now().naive_utc();

        storage
            .put(String::from("Key"), String::from("Value"), None)
            .await;

        assert_eq!(
            storage.get(String::from("Key"), now).await,
            Some(String::from("Value"))
        );

        storage.delete(String::from("Key")).await;

        assert_eq!(storage.get(String::from("Key"), now).await, None);
    }
}
