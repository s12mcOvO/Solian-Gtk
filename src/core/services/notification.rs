use crate::core::models::{SnNotification, SnPublisher, SnPublisherSubscription};
use crate::core::network::ApiClient;
use anyhow::Result;
use std::sync::Arc;

pub struct NotificationService {
    client: Arc<ApiClient>,
}

impl NotificationService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_notifications(&self, take: i32, offset: i32) -> Result<Vec<SnNotification>> {
        let path = format!("/sphere/notifications?take={}&offset={}", take, offset);
        let notifications: Vec<SnNotification> = self.client.get(&path).await?.json().await?;
        Ok(notifications)
    }

    pub async fn mark_as_read(&self, notification_id: &str) -> Result<()> {
        let path = format!("/sphere/notifications/{}/read", notification_id);
        self.client.post::<(), ()>(&path, &()).await
    }

    pub async fn mark_all_as_read(&self) -> Result<()> {
        self.client.post::<(), ()>("/sphere/notifications/read-all", &()).await
    }

    pub async fn get_unread_count(&self) -> Result<i32> {
        #[derive(serde::Deserialize)]
        struct UnreadResponse {
            count: i32,
        }
        let response: UnreadResponse = self.client.get("/sphere/notifications/unread-count").await?.json().await?;
        Ok(response.count)
    }
}