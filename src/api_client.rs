use crate::api_types;
use std::sync::RwLock;

pub struct CachedFetch<T> {
    fetch: Box<dyn Fn() -> T + Send + Sync>,
    time_to_live: chrono::Duration,
    cache: RwLock<Option<(T, chrono::DateTime<chrono::Utc>)>>,
}
impl<T: Clone> CachedFetch<T> {
    pub fn new(
        cache_time_to_live: chrono::TimeDelta,
        fetch: impl Fn() -> T + Send + Sync + 'static,
    ) -> Self {
        Self {
            fetch: Box::new(fetch),
            time_to_live: cache_time_to_live,
            cache: RwLock::new(None),
        }
    }

    pub fn get(&self) -> T {
        // if the value is cached we don't need an exclusive lock
        if let Ok(cache_lock) = self.cache.read() {
            if let Some((value, expiry_date)) = &*cache_lock {
                if *expiry_date >= chrono::Utc::now() {
                    return value.clone();
                }
            }
        }
        // when the value is not cached or has expired we need to acquire write lock
        let mut cache = self.cache.write().unwrap();
        let new_value = (self.fetch)();
        let expiry_date = chrono::Utc::now() + self.time_to_live;
        *cache = Some((new_value.clone(), expiry_date));
        new_value
    }
}

pub fn fetch_schedule() -> Result<api_types::Schedule, String> {
    let today = chrono::Utc::now();
    let start_date = today
        .checked_sub_days(chrono::Days::new(2))
        .unwrap()
        .format("%Y-%m-%d");
    let nhl_api_url = format!("https://api-web.nhle.com/v1/schedule/{}", start_date);
    println!("Fetching {nhl_api_url} ...");
    ureq::get(nhl_api_url)
        .call()
        .and_then(|mut res| res.body_mut().read_json())
        .map_err(|err| format!("{err:?}"))
}
