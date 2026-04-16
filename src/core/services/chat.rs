use crate::core::models::{SnChatInvite, SnChatMessage, SnChatMember, SnChatRoom, SnChatSummary, SnCreateChatRequest};
use crate::core::network::ApiClient;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub struct ChatService {
    client: Arc<ApiClient>,
    joined_rooms: RwLock<Vec<SnChatRoom>>,
}

impl ChatService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self {
            client,
            joined_rooms: RwLock::new(Vec::new()),
        }
    }

    pub async fn get_joined_rooms(&self) -> Result<Vec<SnChatRoom>> {
        let rooms: Vec<SnChatRoom> = self.client.get("/messager/chat/rooms").await?.json().await?;
        *self.joined_rooms.write().await = rooms.clone();
        Ok(rooms)
    }

    pub async fn get_room_messages(&self, room_id: &str, limit: i32, before: Option<String>) -> Result<Vec<SnChatMessage>> {
        let path = if let Some(before) = before {
            format!("/messager/chat/{}/messages?limit={}&before={}", room_id, limit, before)
        } else {
            format!("/messager/chat/{}/messages?limit={}", room_id, limit)
        };
        let messages: Vec<SnChatMessage> = self.client.get(&path).await?.json().await?;
        Ok(messages)
    }

    pub async fn send_message(&self, room_id: &str, content: &str, reply_to: Option<String>) -> Result<SnChatMessage> {
        #[derive(serde::Serialize)]
        struct SendMessageRequest<'a> {
            content: &'a str,
            reply_to_id: Option<&'a str>,
        }
        
        let request = SendMessageRequest {
            content,
            reply_to_id: reply_to.as_deref(),
        };
        
        let path = format!("/messager/chat/{}/messages", room_id);
        let message: SnChatMessage = self.client.post(&path, &request).await?;
        Ok(message)
    }

    pub async fn create_room(&self, request: SnCreateChatRequest) -> Result<SnChatRoom> {
        let room: SnChatRoom = self.client.post("/messager/chat/rooms", &request).await?;
        Ok(room)
    }

    pub async fn create_direct_chat(&self, related_user_id: &str) -> Result<SnChatRoom> {
        #[derive(serde::Serialize)]
        struct DirectChatRequest {
            related_user_id: String,
        }
        
        let request = DirectChatRequest {
            related_user_id: related_user_id.to_string(),
        };
        
        let room: SnChatRoom = self.client.post("/messager/chat/direct", &request).await?;
        Ok(room)
    }

    pub async fn get_chat_summaries(&self) -> Result<Vec<SnChatSummary>> {
        let summaries: Vec<SnChatSummary> = self.client.get("/messager/chat/summaries").await?.json().await?;
        Ok(summaries)
    }

    pub async fn get_invites(&self) -> Result<Vec<SnChatInvite>> {
        let invites: Vec<SnChatInvite> = self.client.get("/messager/chat/invites").await?.json().await?;
        Ok(invites)
    }

    pub async fn accept_invite(&self, invite_id: &str) -> Result<SnChatRoom> {
        #[derive(serde::Serialize)]
        struct AcceptRequest {}
        let room: SnChatRoom = self.client.post(&format!("/messager/chat/invites/{}/accept", invite_id), &AcceptRequest {}).await?;
        Ok(room)
    }

    pub async fn decline_invite(&self, invite_id: &str) -> Result<()> {
        self.client.delete(&format!("/messager/chat/invites/{}", invite_id)).await
    }

    pub async fn get_room_members(&self, room_id: &str) -> Result<Vec<SnChatMember>> {
        let members: Vec<SnChatMember> = self.client.get(&format!("/messager/chat/{}/members", room_id)).await?.json().await?;
        Ok(members)
    }

    pub async fn add_reaction(&self, room_id: &str, message_id: &str, emoji: &str) -> Result<()> {
        #[derive(serde::Serialize)]
        struct ReactionRequest {
            emoji: String,
        }
        
        let path = format!("/messager/chat/{}/messages/{}/reactions", room_id, message_id);
        let _ = self.client.post::<ReactionRequest, ()>(&path, &ReactionRequest { emoji: emoji.to_string() }).await;
        Ok(())
    }

    pub async fn remove_reaction(&self, room_id: &str, message_id: &str, emoji: &str) -> Result<()> {
        let path = format!("/messager/chat/{}/messages/{}/reactions/{}", room_id, message_id, emoji);
        self.client.delete(&path).await
    }

    pub async fn leave_room(&self, room_id: &str) -> Result<()> {
        self.client.delete(&format!("/messager/chat/{}", room_id)).await
    }
}