use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnAccount {
    pub id: String,
    pub name: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub created_at: Option<String>,
    pub followers_count: Option<i64>,
    pub following_count: Option<i64>,
    pub posts_count: Option<i64>,
    pub verified: Option<bool>,
    pub badges: Option<Vec<SnBadge>>,
    pub level: Option<i32>,
    pub exp: Option<i64>,
    pub status: Option<String>,
    pub presence: Option<String>,
    pub locale: Option<String>,
    pub link: Option<String>,
    pub location: Option<String>,
    #[serde(rename = "acct")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnBadge {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub level: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnPost {
    pub id: String,
    pub content: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub author: Option<Box<SnAccount>>,
    pub media_attachments: Option<Vec<SnMedia>>,
    pub favourited: Option<bool>,
    pub favourites_count: Option<i64>,
    pub reblogged: Option<bool>,
    pub reblogs_count: Option<i64>,
    pub replies_count: Option<i64>,
    pub sensitive: Option<bool>,
    pub spoiler_text: Option<String>,
    pub visibility: Option<String>,
    pub application: Option<SnApplication>,
    pub mentions: Option<Vec<SnMention>>,
    pub tags: Option<Vec<SnTag>>,
    pub card: Option<SnCard>,
    pub poll: Option<SnPoll>,
    pub url: Option<String>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnMedia {
    pub id: String,
    #[serde(rename = "type")]
    pub media_type: String,
    pub url: String,
    pub preview_url: Option<String>,
    pub description: Option<String>,
    pub blurhash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnApplication {
    pub name: String,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnMention {
    pub id: String,
    pub username: String,
    pub url: String,
    pub acct: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnTag {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnCard {
    pub url: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub card_type: String,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub provider_name: Option<String>,
    pub provider_url: Option<String>,
    pub html: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub image: Option<String>,
    pub blurhash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnPoll {
    pub id: String,
    pub expires_at: Option<String>,
    pub expired: Option<bool>,
    pub multiple: Option<bool>,
    pub votes_count: Option<i64>,
    pub options: Vec<SnPollOption>,
    pub emojis: Option<Vec<SnCustomEmoji>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnPollOption {
    pub title: String,
    pub votes_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnCustomEmoji {
    pub shortcode: String,
    pub url: String,
    pub static_url: Option<String>,
    pub visible_in_picker: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnRealm {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub member_count: Option<i64>,
    pub owner: Option<Box<SnAccount>>,
    pub created_at: Option<String>,
    pub followers_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnChat {
    pub id: String,
    pub title: Option<String>,
    pub icon: Option<String>,
    pub last_message: Option<Box<SnChatMessage>>,
    pub unread_count: Option<i64>,
    pub participants: Vec<SnAccount>,
    pub is_group: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnReaction {
    pub emoji: String,
    pub count: i64,
    pub me: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnNotification {
    pub id: String,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub created_at: Option<String>,
    pub account: Option<Box<SnAccount>>,
    pub status: Option<Box<SnPost>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnWallet {
    pub id: String,
    pub name: String,
    pub balance: String,
    pub currency: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnThought {
    pub id: String,
    pub messages: Vec<SnThoughtMessage>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnThoughtMessage {
    pub id: String,
    pub role: String,
    pub content: Option<String>,
    pub function_calls: Option<Vec<SnFunctionCall>>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnFunctionCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnDriveFile {
    pub id: String,
    pub name: String,
    pub mime_type: String,
    pub size: i64,
    pub url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnSearchResult {
    pub accounts: Vec<SnAccount>,
    pub statuses: Vec<SnPost>,
    pub hashtags: Vec<SnTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnTokenPair {
    pub token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<String>,
    pub refresh_expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnUserInfo {
    pub account: SnAccount,
    pub permissions: Vec<String>,
    pub token_pair: SnTokenPair,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnChatRoom {
    pub id: String,
    pub name: Option<String>,
    pub r#type: Option<i32>,
    pub is_pinned: Option<bool>,
    pub is_public: Option<bool>,
    pub realm_id: Option<String>,
    pub realm: Option<Box<SnRealm>>,
    pub members: Option<Vec<SnChatMember>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub avatar_url: Option<String>,
    pub encryption_mode: Option<i32>,
    pub last_message: Option<Box<SnChatMessage>>,
    pub unread_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnChatMember {
    pub id: Option<String>,
    pub account_id: String,
    pub role: Option<i32>,
    pub account: SnAccount,
    pub joined_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnChatMessage {
    pub id: String,
    pub content: Option<String>,
    pub sender_id: Option<String>,
    pub sender: Option<Box<SnAccount>>,
    pub room_id: String,
    pub r#type: Option<i32>,
    pub attachments: Option<Vec<SnMedia>>,
    pub reply_to_id: Option<String>,
    pub reactions: Option<Vec<SnReaction>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub is_pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnChatSummary {
    pub room_id: String,
    pub last_message: Option<String>,
    pub last_message_at: Option<String>,
    pub unread_count: i64,
    pub mentions_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnChatInvite {
    pub id: String,
    pub room: Option<SnChatRoom>,
    pub inviter: Option<Box<SnAccount>>,
    pub invitee: Option<Box<SnAccount>>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnCreateChatRequest {
    pub name: Option<String>,
    pub r#type: i32,
    pub member_ids: Vec<String>,
    pub realm_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnTimelinePage {
    pub items: Vec<SnPost>,
    pub next_cursor: Option<String>,
    pub prev_cursor: Option<String>,
}
