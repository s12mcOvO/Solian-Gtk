use anyhow::Result;
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

use crate::core::config::{AppSettings, DEFAULT_SERVER_URL};
use crate::core::models::{SnTokenPair, SnUserInfo};

pub struct ApiClient {
    client: Client,
    server_url: RwLock<String>,
    token_pair: RwLock<Option<SnTokenPair>>,
    settings: Arc<RwLock<AppSettings>>,
}

impl ApiClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            server_url: RwLock::new(DEFAULT_SERVER_URL.to_string()),
            token_pair: RwLock::new(None),
            settings: Arc::new(RwLock::new(AppSettings::default())),
        }
    }

    pub async fn set_server_url(&self, url: String) {
        let mut server_url = self.server_url.write().await;
        *server_url = url;
    }

    pub async fn get_server_url(&self) -> String {
        self.server_url.read().await.clone()
    }

    pub async fn set_token(&self, token_pair: SnTokenPair) {
        let mut token = self.token_pair.write().await;
        *token = Some(token_pair);
    }

    pub async fn clear_token(&self) {
        let mut token = self.token_pair.write().await;
        *token = None;
    }

    pub async fn get_token(&self) -> Option<String> {
        self.token_pair.read().await.as_ref().map(|t| t.token.clone())
    }

    pub async fn is_authenticated(&self) -> bool {
        self.token_pair.read().await.is_some()
    }

    pub async fn get(&self, path: &str) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.server_url.read().await, path);
        let token = self.get_token().await;

        debug!("GET {}", url);

        let mut request = self.client.get(&url);

        if let Some(token) = token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Request failed: {} - {}", status, error_text);
            Err(anyhow::anyhow!("Request failed: {} - {}", status, error_text))
        }
    }

    pub async fn post<T: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R> {
        let url = format!("{}{}", self.server_url.read().await, path);
        let token = self.get_token().await;

        debug!("POST {}", url);

        let mut request = self.client.post(&url).json(body);

        if let Some(token) = token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;

        if response.status().is_success() {
            let parsed = response.json::<R>().await?;
            Ok(parsed)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Request failed: {} - {}", status, error_text);
            Err(anyhow::anyhow!("Request failed: {} - {}", status, error_text))
        }
    }

    pub async fn put<T: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R> {
        let url = format!("{}{}", self.server_url.read().await, path);
        let token = self.get_token().await;

        debug!("PUT {}", url);

        let mut request = self.client.put(&url).json(body);

        if let Some(token) = token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;

        if response.status().is_success() {
            let parsed = response.json::<R>().await?;
            Ok(parsed)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Request failed: {} - {}", status, error_text);
            Err(anyhow::anyhow!("Request failed: {} - {}", status, error_text))
        }
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", self.server_url.read().await, path);
        let token = self.get_token().await;

        debug!("DELETE {}", url);

        let mut request = self.client.delete(&url);

        if let Some(token) = token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Request failed: {} - {}", status, error_text);
            Err(anyhow::anyhow!("Request failed: {} - {}", status, error_text))
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<SnUserInfo> {
        let payload = serde_json::json!({
            "username": username,
            "password": password,
            "grant_type": "password",
        });

        let url = format!("{}/padlock/auth/token", self.server_url.read().await);
        info!("Attempting login to {}", url);

        #[derive(serde::Deserialize)]
        struct LoginResponse {
            token: String,
            refresh_token: Option<String>,
            expires_in: Option<i64>,
            refresh_expires_in: Option<i64>,
            user: Option<SnUserInfo>,
        }

        let response: LoginResponse = self.post("/padlock/auth/token", &payload).await?;

        let token_pair = SnTokenPair {
            token: response.token,
            refresh_token: response.refresh_token,
            expires_at: response.expires_in.map(|e| {
                chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::seconds(e))
                    .unwrap()
                    .to_rfc3339()
            }),
            refresh_expires_at: response.refresh_expires_in.map(|e| {
                chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::seconds(e))
                    .unwrap()
                    .to_rfc3339()
            }),
        };

        self.set_token(token_pair).await;

        response.user.ok_or_else(|| anyhow::anyhow!("No user info returned"))
    }

    pub async fn logout(&self) -> Result<()> {
        self.clear_token().await;
        Ok(())
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}
