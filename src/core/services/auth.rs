use crate::core::models::SnUserInfo;
use crate::core::network::ApiClient;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub struct AuthService {
    client: Arc<ApiClient>,
    current_user: RwLock<Option<SnUserInfo>>,
}

impl AuthService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            client,
            current_user: RwLock::new(None),
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<SnUserInfo> {
        let user = self.client.login(username, password).await?;
        *self.current_user.write().await = Some(user.clone());
        info!("User logged in: {}", user.account.name);
        Ok(user)
    }

    pub async fn logout(&self) -> Result<()> {
        self.client.logout().await?;
        *self.current_user.write().await = None;
        info!("User logged out");
        Ok(())
    }

    pub async fn get_current_user(&self) -> Option<SnUserInfo> {
        self.current_user.read().await.clone()
    }

    pub async fn is_authenticated(&self) -> bool {
        self.current_user.read().await.is_some()
    }
}
