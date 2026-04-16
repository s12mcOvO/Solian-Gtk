use crate::core::models::{SnPublisher, SnPublisherSubscription};
use crate::core::network::ApiClient;
use anyhow::Result;
use std::sync::Arc;

pub struct PublisherService {
    client: Arc<ApiClient>,
}

impl PublisherService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_my_publishers(&self) -> Result<Vec<SnPublisher>> {
        let publishers: Vec<SnPublisher> = self.client.get("/sphere/publishers").await?.json().await?;
        Ok(publishers)
    }

    pub async fn get_publisher(&self, name: &str) -> Result<SnPublisher> {
        let publisher: SnPublisher = self.client.get(&format!("/sphere/publishers/{}", name)).await?.json().await?;
        Ok(publisher)
    }

    pub async fn update_publisher(&self, name: &str, display_name: Option<&str>, bio: Option<&str>, avatar_url: Option<&str>) -> Result<SnPublisher> {
        #[derive(serde::Serialize)]
        struct UpdateRequest {
            display_name: Option<String>,
            bio: Option<String>,
            avatar_url: Option<String>,
        }
        let request = UpdateRequest {
            display_name: display_name.map(|s| s.to_string()),
            bio: bio.map(|s| s.to_string()),
            avatar_url: avatar_url.map(|s| s.to_string()),
        };
        let publisher: SnPublisher = self.client.patch(&format!("/sphere/publishers/{}", name), &request).await?;
        Ok(publisher)
    }

    pub async fn get_publisher_stats(&self, name: &str) -> Result<serde_json::Value> {
        let stats: serde_json::Value = self.client.get(&format!("/sphere/publishers/{}/stats", name)).await?.json().await?;
        Ok(stats)
    }

    pub async fn subscribe(&self, name: &str) -> Result<()> {
        self.client.post::<(), ()>(&format!("/sphere/publishers/{}/subscribe", name), &()).await
    }

    pub async fn unsubscribe(&self, name: &str) -> Result<()> {
        self.client.delete(&format!("/sphere/publishers/{}/unsubscribe", name)).await
    }

    pub async fn get_subscribers(&self, name: &str, take: i32, offset: i32) -> Result<Vec<SnPublisherSubscription>> {
        let path = format!("/sphere/publishers/{}/subscribers?take={}&offset={}", name, take, offset);
        let subs: Vec<SnPublisherSubscription> = self.client.get(&path).await?.json().await?;
        Ok(subs)
    }

    pub async fn search_publishers(&self, query: &str, limit: i32) -> Result<Vec<SnPublisher>> {
        let path = format!("/sphere/publishers/search?q={}&limit={}", query, limit);
        let results: Vec<SnPublisher> = self.client.get(&path).await?.json().await?;
        Ok(results)
    }

    pub async fn get_activity_heatmap(&self, name: &str) -> Result<serde_json::Value> {
        let heatmap: serde_json::Value = self.client.get(&format!("/sphere/publishers/{}/heatmap", name)).await?.json().await?;
        Ok(heatmap)
    }
}