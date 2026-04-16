use serde::{Deserialize, Serialize};

pub const DEFAULT_SERVER_URL: &str = "https://api.solian.app";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme_mode: ThemeMode,
    pub data_saving_mode: bool,
    pub sound_effects: bool,
    pub festival_features: bool,
    pub enter_to_send: bool,
    pub app_bar_transparent: bool,
    pub show_background_image: bool,
    pub notify_with_haptic: bool,
    pub enable_tts: bool,
    pub custom_colors: Option<ThemeColors>,
    pub window_size: Option<(u32, u32)>,
    pub window_opacity: f64,
    pub card_transparency: f64,
    pub message_display_style: MessageDisplayStyle,
    pub attachments_list_style: AttachmentsListStyle,
    pub link_collapse_mode: LinkCollapseMode,
    pub disable_animation: bool,
    pub grouped_chat_list: bool,
    pub default_screen: Option<String>,
    pub realm_display_mode: RealmDisplayMode,
    pub show_chat_system_messages: bool,
    pub media_proxy_enabled: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::System,
            data_saving_mode: false,
            sound_effects: true,
            festival_features: true,
            enter_to_send: true,
            app_bar_transparent: false,
            show_background_image: true,
            notify_with_haptic: true,
            enable_tts: false,
            custom_colors: None,
            window_size: None,
            window_opacity: 1.0,
            card_transparency: 1.0,
            message_display_style: MessageDisplayStyle::Bubble,
            attachments_list_style: AttachmentsListStyle::Row,
            link_collapse_mode: LinkCollapseMode::Expand,
            disable_animation: false,
            grouped_chat_list: false,
            default_screen: None,
            realm_display_mode: RealmDisplayMode::Card,
            show_chat_system_messages: false,
            media_proxy_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub primary: Option<u32>,
    pub secondary: Option<u32>,
    pub tertiary: Option<u32>,
    pub surface: Option<u32>,
    pub background: Option<u32>,
    pub error: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageDisplayStyle {
    Bubble,
    Compact,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AttachmentsListStyle {
    Row,
    Grid,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LinkCollapseMode {
    Expand,
    Collapse,
    AlwaysExpand,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RealmDisplayMode {
    List,
    Card,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NetworkStatus {
    Online,
    NotReady,
    Maintenance,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WebSocketState {
    Connected,
    Connecting,
    Disconnected,
    ServerDown,
    DuplicateDevice,
    Error(String),
}
