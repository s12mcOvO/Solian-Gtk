use crate::core::network::ApiClient;
use anyhow::Result;
use std::sync::Arc;

pub struct CountdownService {
    client: Arc<ApiClient>,
}

impl CountdownService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_countdowns(&self) -> Result<Vec<serde_json::Value>> {
        let countdowns: Vec<serde_json::Value> = self.client.get("/sphere/countdowns").await?.json().await?;
        Ok(countdowns)
    }

    pub async fn create_countdown(&self, title: &str, description: &str, target_time: &str) -> Result<serde_json::Value> {
        #[derive(serde::Serialize)]
        struct CreateRequest<'a> {
            title: &'a str,
            description: &'a str,
            target_time: &'a str,
        }
        
        let request = CreateRequest {
            title,
            description,
            target_time,
        };
        
        let countdown: serde_json::Value = self.client.post("/sphere/countdowns", &request).await?;
        Ok(countdown)
    }

    pub async fn delete_countdown(&self, countdown_id: &str) -> Result<()> {
        let path = format!("/sphere/countdowns/{}", countdown_id);
        self.client.delete(&path).await
    }
}

pub struct RssService {
    client: Arc<ApiClient>,
}

impl RssService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_feeds(&self) -> Result<Vec<serde_json::Value>> {
        let feeds: Vec<serde_json::Value> = self.client.get("/sphere/rss/feeds").await?.json().await?;
        Ok(feeds)
    }

    pub async fn add_feed(&self, url: &str, title: Option<&str>) -> Result<serde_json::Value> {
        #[derive(serde::Serialize)]
        struct AddFeedRequest<'a> {
            url: &'a str,
            title: Option<&'a str>,
        }
        
        let request = AddFeedRequest {
            url,
            title,
        };
        
        let feed: serde_json::Value = self.client.post("/sphere/rss/feeds", &request).await?;
        Ok(feed)
    }

    pub async fn remove_feed(&self, feed_id: &str) -> Result<()> {
        let path = format!("/sphere/rss/feeds/{}", feed_id);
        self.client.delete(&path).await
    }

    pub async fn get_feed_items(&self, feed_id: &str, take: i32) -> Result<Vec<serde_json::Value>> {
        let path = format!("/sphere/rss/feeds/{}/items?take={}", feed_id, take);
        let items: Vec<serde_json::Value> = self.client.get(&path).await?.json().await?;
        Ok(items)
    }
}

pub struct FriendsService {
    client: Arc<ApiClient>,
}

impl FriendsService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_friends(&self, take: i32, offset: i32) -> Result<Vec<serde_json::Value>> {
        let path = format!("/sphere/friends?take={}&offset={}", take, offset);
        let friends: Vec<serde_json::Value> = self.client.get(&path).await?.json().await?;
        Ok(friends)
    }

    pub async fn add_friend(&self, account_id: &str) -> Result<()> {
        let path = format!("/sphere/friends/{}", account_id);
        self.client.post::<(), ()>(&path, &()).await
    }

    pub async fn remove_friend(&self, account_id: &str) -> Result<()> {
        let path = format!("/sphere/friends/{}", account_id);
        self.client.delete(&path).await
    }

    pub async fn get_blocklist(&self) -> Result<Vec<serde_json::Value>> {
        let blocked: Vec<serde_json::Value> = self.client.get("/sphere/blocks").await?.json().await?;
        Ok(blocked)
    }

    pub async fn block_account(&self, account_id: &str) -> Result<()> {
        let path = format!("/sphere/blocks/{}", account_id);
        self.client.post::<(), ()>(&path, &()).await
    }

    pub async fn unblock_account(&self, account_id: &str) -> Result<()> {
        let path = format!("/sphere/blocks/{}", account_id);
        self.client.delete(&path).await
    }
}

pub struct ProgressionService {
    client: Arc<ApiClient>,
}

impl ProgressionService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_achievements(&self) -> Result<Vec<serde_json::Value>> {
        let achievements: Vec<serde_json::Value> = self.client.get("/sphere/progressions").await?.json().await?;
        Ok(achievements)
    }

    pub async fn get_user_progress(&self) -> Result<serde_json::Value> {
        let progress: serde_json::Value = self.client.get("/sphere/progressions/me").await?.json().await?;
        Ok(progress)
    }
}