use crate::core::models::SnDriveFile;
use crate::core::network::ApiClient;
use anyhow::Result;
use std::sync::Arc;

pub struct FileService {
    client: Arc<ApiClient>,
}

impl FileService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_files(&self, take: i32, offset: i32) -> Result<Vec<SnDriveFile>> {
        let path = format!("/sphere/drive?take={}&offset={}", take, offset);
        let files: Vec<SnDriveFile> = self.client.get(&path).await?.json().await?;
        Ok(files)
    }

    pub async fn get_file(&self, file_id: &str) -> Result<SnDriveFile> {
        let path = format!("/sphere/drive/{}", file_id);
        let file: SnDriveFile = self.client.get(&path).await?.json().await?;
        Ok(file)
    }

    pub async fn delete_file(&self, file_id: &str) -> Result<()> {
        let path = format!("/sphere/drive/{}", file_id);
        self.client.delete(&path).await
    }

    pub async fn create_folder(&self, name: &str, parent_id: Option<&str>) -> Result<SnDriveFile> {
        #[derive(serde::Serialize)]
        struct CreateFolderRequest<'a> {
            name: &'a str,
            parent_id: Option<&'a str>,
        }
        
        let request = CreateFolderRequest {
            name,
            parent_id,
        };
        
        let folder: SnDriveFile = self.client.post("/sphere/drive", &request).await?;
        Ok(folder)
    }
}

pub struct FitnessService {
    client: Arc<ApiClient>,
}

impl FitnessService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_activities(&self, take: i32) -> Result<Vec<serde_json::Value>> {
        let path = format!("/sphere/fitness/activities?take={}", take);
        let activities: Vec<serde_json::Value> = self.client.get(&path).await?.json().await?;
        Ok(activities)
    }

    pub async fn log_activity(&self, activity_type: &str, duration: i32, distance: Option<f64>, calories: Option<i32>) -> Result<serde_json::Value> {
        #[derive(serde::Serialize)]
        struct ActivityRequest<'a> {
            activity_type: &'a str,
            duration: i32,
            distance: Option<f64>,
            calories: Option<i32>,
        }
        
        let request = ActivityRequest {
            activity_type,
            duration,
            distance,
            calories,
        };
        
        let activity: serde_json::Value = self.client.post("/sphere/fitness/activities", &request).await?;
        Ok(activity)
    }

    pub async fn get_goals(&self) -> Result<Vec<serde_json::Value>> {
        let goals: Vec<serde_json::Value> = self.client.get("/sphere/fitness/goals").await?.json().await?;
        Ok(goals)
    }

    pub async fn set_goal(&self, goal_type: &str, target: i32, period: &str) -> Result<serde_json::Value> {
        #[derive(serde::Serialize)]
        struct GoalRequest<'a> {
            goal_type: &'a str,
            target: i32,
            period: &'a str,
        }
        
        let request = GoalRequest {
            goal_type,
            target,
            period,
        };
        
        let goal: serde_json::Value = self.client.post("/sphere/fitness/goals", &request).await?;
        Ok(goal)
    }

    pub async fn get_stats(&self) -> Result<serde_json::Value> {
        let stats: serde_json::Value = self.client.get("/sphere/fitness/stats").await?.json().await?;
        Ok(stats)
    }
}

pub struct OauthService {
    client: Arc<ApiClient>,
}

impl OauthService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_providers(&self) -> Result<Vec<serde_json::Value>> {
        let providers: Vec<serde_json::Value> = self.client.get("/sphere/oauth/providers").await?.json().await?;
        Ok(providers)
    }

    pub async fn authorize(&self, provider: &str) -> Result<String> {
        #[derive(serde::Serialize)]
        struct AuthorizeRequest<'a> {
            provider: &'a str,
        }
        
        let response: serde_json::Value = self.client.post("/sphere/oauth/authorize", &AuthorizeRequest { provider }).await?;
        let url = response["url"].as_str().unwrap_or("").to_string();
        Ok(url)
    }

    pub async fn callback(&self, code: &str, state: &str) -> Result<serde_json::Value> {
        #[derive(serde::Serialize)]
        struct CallbackRequest<'a> {
            code: &'a str,
            state: &'a str,
        }
        
        let result: serde_json::Value = self.client.post("/sphere/oauth/callback", &CallbackRequest { code, state }).await?;
        Ok(result)
    }

    pub async fn unlink_account(&self, provider: &str) -> Result<()> {
        let path = format!("/sphere/oauth/{}", provider);
        self.client.delete(&path).await
    }
}