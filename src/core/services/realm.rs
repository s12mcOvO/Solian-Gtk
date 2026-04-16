use crate::core::models::SnRealm;
use crate::core::network::ApiClient;
use anyhow::Result;
use std::sync::Arc;

pub struct RealmService {
    client: Arc<ApiClient>,
}

impl RealmService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_realms(&self, take: i32, offset: i32) -> Result<Vec<SnRealm>> {
        let path = format!("/sphere/realms?take={}&offset={}", take, offset);
        let realms: Vec<SnRealm> = self.client.get(&path).await?.json().await?;
        Ok(realms)
    }

    pub async fn get_realm(&self, slug: &str) -> Result<SnRealm> {
        let path = format!("/sphere/realms/{}", slug);
        let realm: SnRealm = self.client.get(&path).await?.json().await?;
        Ok(realm)
    }

    pub async fn get_realm_posts(&self, slug: &str, take: i32, offset: i32) -> Result<Vec<crate::core::models::SnPost>> {
        let path = format!("/sphere/realms/{}/posts?take={}&offset={}", slug, take, offset);
        let posts: Vec<crate::core::models::SnPost> = self.client.get(&path).await?.json().await?;
        Ok(posts)
    }

    pub async fn join_realm(&self, slug: &str) -> Result<()> {
        let path = format!("/sphere/realms/{}/join", slug);
        self.client.post::<(), ()>(&path, &()).await
    }

    pub async fn leave_realm(&self, slug: &str) -> Result<()> {
        let path = format!("/sphere/realms/{}/leave", slug);
        self.client.delete(&path).await
    }

    pub async fn create_realm(&self, name: &str, slug: &str, description: &str) -> Result<SnRealm> {
        #[derive(serde::Serialize)]
        struct CreateRealmRequest<'a> {
            name: &'a str,
            slug: &'a str,
            description: &'a str,
        }
        
        let request = CreateRealmRequest {
            name,
            slug,
            description,
        };
        
        let realm: SnRealm = self.client.post("/sphere/realms", &request).await?;
        Ok(realm)
    }

    pub async fn search_realms(&self, query: &str, take: i32) -> Result<Vec<SnRealm>> {
        let path = format!("/sphere/realms/search?q={}&take={}", query, take);
        let realms: Vec<SnRealm> = self.client.get(&path).await?.json().await?;
        Ok(realms)
    }

    pub async fn get_trending_realms(&self, take: i32) -> Result<Vec<SnRealm>> {
        let path = format!("/sphere/realms/trending?take={}", take);
        let realms: Vec<SnRealm> = self.client.get(&path).await?.json().await?;
        Ok(realms)
    }
}