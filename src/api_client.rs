use crate::api_types;
use std::sync::RwLock;

pub struct CachedFetch<T> {
    fetch: Box<dyn Fn() -> T + Send + Sync>,
    time_to_live: chrono::Duration,
    cache: RwLock<Option<(T, chrono::DateTime<chrono::Utc>)>>,
}
impl<T: Clone> CachedFetch<T> {
    pub fn new(fetch: impl Fn() -> T + Send + Sync + 'static) -> Self {
        Self {
            fetch: Box::new(fetch),
            time_to_live: chrono::Duration::seconds(5),
            cache: RwLock::new(None),
        }
    }

    pub fn set_cache_ttl(mut self, time_to_live: chrono::TimeDelta) -> Self {
        self.time_to_live = time_to_live;
        self
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

fn fetch<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, String> {
    println!("Fetching {url} ...");
    ureq::get(url)
        .call()
        .and_then(|mut res| res.body_mut().read_json::<T>())
        .map_err(|err| format!("{err:?}"))
}

pub fn fetch_schedule() -> Result<api_types::Schedule, String> {
    let today = chrono::Utc::now();
    let start_date = today
        .checked_sub_days(chrono::Days::new(2))
        .unwrap()
        .format("%Y-%m-%d");
    fetch(&format!(
        "https://api-web.nhle.com/v1/schedule/{start_date}"
    ))
}

pub fn fetch_standings() -> Result<api_types::Standings, String> {
    fetch("https://api-web.nhle.com/v1/standings/now")
}
