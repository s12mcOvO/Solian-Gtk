use crate::core::network::ApiClient;
use anyhow::Result;
use std::sync::Arc;

pub struct CheckInService {
    client: Arc<ApiClient>,
}

impl CheckInService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn checkin(&self, location: &str, status: Option<&str>) -> Result<serde_json::Value> {
        #[derive(serde::Serialize)]
        struct CheckInRequest<'a> {
            location: &'a str,
            status: Option<&'a str>,
        }
        
        let request = CheckInRequest {
            location,
            status,
        };
        
        let response: serde_json::Value = self.client.post("/sphere/checkin", &request).await?;
        Ok(response)
    }

    pub async fn get_checkins(&self, take: i32, offset: i32) -> Result<Vec<serde_json::Value>> {
        let path = format!("/sphere/checkins?take={}&offset={}", take, offset);
        let checkins: Vec<serde_json::Value> = self.client.get(&path).await?.json().await?;
        Ok(checkins)
    }

    pub async fn get_nearby_checkins(&self, lat: f64, lng: f64, radius: f64) -> Result<Vec<serde_json::Value>> {
        let path = format!("/sphere/checkins/nearby?lat={}&lng={}&radius={}", lat, lng, radius);
        let checkins: Vec<serde_json::Value> = self.client.get(&path).await?.json().await?;
        Ok(checkins)
    }

    pub async fn delete_checkin(&self, checkin_id: &str) -> Result<()> {
        let path = format!("/sphere/checkins/{}", checkin_id);
        self.client.delete(&path).await
    }
}