use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};
use log::{info, warn, error};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub uid: String,
    pub name: String,
    pub short_name: String,
    pub price: i32,
    pub avg_24h_price: i32,
    pub avg_7days_price: i32,
    pub trader_name: String,
    pub trader_price: i32,
    pub icon: String,
    pub img: String,
    pub img_big: String,
    pub bsg_id: String,
    pub is_functional: bool,
    pub tags: Vec<String>,
    pub updated: String,
}

#[derive(Debug)]
struct CacheEntry<T> {
    data: T,
    timestamp: Instant,
}

impl<T> CacheEntry<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            timestamp: Instant::now(),
        }
    }

    fn is_expired(&self, ttl: Duration) -> bool {
        self.timestamp.elapsed() > ttl
    }
}

pub struct TarkovMarketAPI {
    api_key: String,
    client: reqwest::Client,
    item_cache: Arc<RwLock<HashMap<String, CacheEntry<Item>>>>,
    search_cache: Arc<RwLock<HashMap<String, CacheEntry<Vec<Item>>>>>,
    last_request: Arc<RwLock<Instant>>,
    min_request_interval: Duration,
}

impl TarkovMarketAPI {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            item_cache: Arc::new(RwLock::new(HashMap::new())),
            search_cache: Arc::new(RwLock::new(HashMap::new())),
            last_request: Arc::new(RwLock::new(Instant::now())),
            min_request_interval: Duration::from_millis(200), // 5 requests per second max
        }
    }

    async fn wait_for_rate_limit(&self) {
        let mut last_request = self.last_request.write().await;
        let elapsed = last_request.elapsed();
        if elapsed < self.min_request_interval {
            tokio::time::sleep(self.min_request_interval - elapsed).await;
        }
        *last_request = Instant::now();
    }

    async fn make_request<T: for<'de> Deserialize<'de>>(&self, endpoint: &str, query: &str) -> Result<T> {
        self.wait_for_rate_limit().await;

        let url = format!("https://api.tarkov-market.app/api/v1{}?{}", endpoint, query);
        let response = self.client
            .get(&url)
            .header("x-api-key", &self.api_key)
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("API request failed with status {}: {}", status, body));
        }

        response.json().await.context("Failed to parse response")
    }

    pub async fn search_item(&self, query: &str) -> Result<Vec<Item>> {
        let cache_key = query.to_lowercase();
        {
            let cache = self.search_cache.read().await;
            if let Some(entry) = cache.get(&cache_key) {
                if !entry.is_expired(Duration::from_secs(300)) { // 5 minute cache
                    info!("Cache hit for search: {}", query);
                    return Ok(entry.data.clone());
                }
            }
        }

        let items: Vec<Item> = self.make_request("/item", &format!("q={}", query)).await?;
        
        let mut cache = self.search_cache.write().await;
        cache.insert(cache_key, CacheEntry::new(items.clone()));
        
        Ok(items)
    }

    pub async fn get_item_by_uid(&self, uid: &str) -> Result<Item> {
        {
            let cache = self.item_cache.read().await;
            if let Some(entry) = cache.get(uid) {
                if !entry.is_expired(Duration::from_secs(300)) { // 5 minute cache
                    info!("Cache hit for item: {}", uid);
                    return Ok(entry.data.clone());
                }
            }
        }

        let mut items: Vec<Item> = self.make_request("/item", &format!("uid={}", uid)).await?;
        let item = items.pop().ok_or_else(|| anyhow::anyhow!("Item not found"))?;
        
        let mut cache = self.item_cache.write().await;
        cache.insert(uid.to_string(), CacheEntry::new(item.clone()));
        
        Ok(item)
    }

    pub async fn get_all_items(&self) -> Result<Vec<Item>> {
        const CACHE_KEY: &str = "all_items";
        {
            let cache = self.search_cache.read().await;
            if let Some(entry) = cache.get(CACHE_KEY) {
                if !entry.is_expired(Duration::from_secs(3600)) { // 1 hour cache
                    info!("Cache hit for all items");
                    return Ok(entry.data.clone());
                }
            }
        }

        let items: Vec<Item> = self.make_request("/items/all", "").await?;
        
        let mut cache = self.search_cache.write().await;
        cache.insert(CACHE_KEY.to_string(), CacheEntry::new(items.clone()));
        
        Ok(items)
    }

    pub async fn clear_cache(&self) {
        let mut item_cache = self.item_cache.write().await;
        let mut search_cache = self.search_cache.write().await;
        item_cache.clear();
        search_cache.clear();
        info!("Cache cleared");
    }
} 