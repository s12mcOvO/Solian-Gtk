use crate::core::models::{SnPost, SnTimelinePage};
use crate::core::network::ApiClient;
use anyhow::Result;
use std::sync::Arc;

pub struct PostsService {
    client: Arc<ApiClient>,
}

impl PostsService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_timeline(&self, cursor: Option<String>, take: i32) -> Result<SnTimelinePage> {
        let path = match cursor {
            Some(c) => format!("/sphere/timeline?take={}&cursor={}", take, c),
            None => format!("/sphere/timeline?take={}", take),
        };
        
        let timeline: SnTimelinePage = self.client.get(&path).await?.json().await?;
        Ok(timeline)
    }

    pub async fn get_home_timeline(&self, take: i32) -> Result<SnTimelinePage> {
        let path = format!("/sphere/timeline?take={}&filter=home", take);
        let timeline: SnTimelinePage = self.client.get(&path).await?.json().await?;
        Ok(timeline)
    }

    pub async fn get_local_timeline(&self, take: i32) -> Result<SnTimelinePage> {
        let path = format!("/sphere/timeline?take={}&filter=local", take);
        let timeline: SnTimelinePage = self.client.get(&path).await?.json().await?;
        Ok(timeline)
    }

    pub async fn get_federated_timeline(&self, take: i32) -> Result<SnTimelinePage> {
        let path = format!("/sphere/timeline?take={}&filter=federated", take);
        let timeline: SnTimelinePage = self.client.get(&path).await?.json().await?;
        Ok(timeline)
    }

    pub async fn get_posts(&self, take: i32, offset: i32) -> Result<Vec<SnPost>> {
        let path = format!("/sphere/posts?take={}&offset={}", take, offset);
        let posts: Vec<SnPost> = self.client.get(&path).await?.json().await?;
        Ok(posts)
    }

    pub async fn get_post(&self, post_id: &str) -> Result<SnPost> {
        let path = format!("/sphere/posts/{}", post_id);
        let post: SnPost = self.client.get(&path).await?.json().await?;
        Ok(post)
    }

    pub async fn get_post_replies(&self, post_id: &str, take: i32) -> Result<Vec<SnPost>> {
        let path = format!("/sphere/posts/{}/replies?take={}", post_id, take);
        let replies: Vec<SnPost> = self.client.get(&path).await?.json().await?;
        Ok(replies)
    }

    pub async fn create_post(&self, content: &str, visibility: Option<&str>, reply_to: Option<&str>) -> Result<SnPost> {
        #[derive(serde::Serialize)]
        struct CreatePostRequest<'a> {
            content: &'a str,
            visibility: Option<&'a str>,
            reply_to_id: Option<&'a str>,
        }
        
        let request = CreatePostRequest {
            content,
            visibility,
            reply_to_id: reply_to,
        };
        
        let post: SnPost = self.client.post("/sphere/posts", &request).await?;
        Ok(post)
    }

    pub async fn delete_post(&self, post_id: &str) -> Result<()> {
        let path = format!("/sphere/posts/{}", post_id);
        self.client.delete(&path).await
    }

    pub async fn like_post(&self, post_id: &str) -> Result<()> {
        let path = format!("/sphere/posts/{}/favourite", post_id);
        self.client.post::<(), ()>(&path, &()).await
    }

    pub async fn unlike_post(&self, post_id: &str) -> Result<()> {
        let path = format!("/sphere/posts/{}/favourite", post_id);
        self.client.delete(&path).await
    }

    pub async fn reblog_post(&self, post_id: &str) -> Result<SnPost> {
        let path = format!("/sphere/posts/{}/boost", post_id);
        let post: SnPost = self.client.post(&path, &()).await?;
        Ok(post)
    }

    pub async fn unreblog_post(&self, post_id: &str) -> Result<()> {
        let path = format!("/sphere/posts/{}/boost", post_id);
        self.client.delete(&path).await
    }

    pub async fn add_reaction(&self, post_id: &str, emoji: &str) -> Result<()> {
        #[derive(serde::Serialize)]
        struct ReactionRequest {
            emoji: String,
        }
        
        let path = format!("/sphere/posts/{}/reactions", post_id);
        let _ = self.client.post::<ReactionRequest, ()>(&path, &ReactionRequest { emoji: emoji.to_string() }).await;
        Ok(())
    }

    pub async fn bookmark_post(&self, post_id: &str) -> Result<()> {
        let path = format!("/sphere/posts/{}/bookmark", post_id);
        self.client.post::<(), ()>(&path, &()).await
    }

    pub async fn unbookmark_post(&self, post_id: &str) -> Result<()> {
        let path = format!("/sphere/posts/{}/bookmark", post_id);
        self.client.delete(&path).await
    }

    pub async fn get_bookmarks(&self, take: i32, offset: i32) -> Result<Vec<SnPost>> {
        let path = format!("/sphere/posts/bookmarks?take={}&offset={}", take, offset);
        let posts: Vec<SnPost> = self.client.get(&path).await?.json().await?;
        Ok(posts)
    }

    pub async fn get_drafts(&self) -> Result<Vec<SnPost>> {
        let posts: Vec<SnPost> = self.client.get("/sphere/posts/drafts").await?.json().await?;
        Ok(posts)
    }
}