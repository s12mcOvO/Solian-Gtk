use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketPacket {
    pub type_field: String,
    #[serde(rename = "data")]
    pub data: Option<serde_json::Value>,
    pub endpoint: Option<String>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WsState {
    Connected,
    Connecting,
    Disconnected,
    Error,
}

pub struct WebSocketService {
    sender: broadcast::Sender<WsMessage>,
    state: Arc<RwLock<WsState>>,
}

pub enum WsMessage {
    Packet(WebSocketPacket),
    State(WsState),
    Error(String),
}

impl Clone for WsMessage {
    fn clone(&self) -> Self {
        match self {
            WsMessage::Packet(p) => WsMessage::Packet(p.clone()),
            WsMessage::State(s) => WsMessage::State(*s),
            WsMessage::Error(e) => WsMessage::Error(e.clone()),
        }
    }
}

impl WebSocketService {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self {
            sender,
            state: Arc::new(RwLock::new(WsState::Disconnected)),
        }
    }

    pub async fn connect(&self, server_url: &str, token: Option<&str>) -> Result<()> {
        let url = server_url.replace("http", "ws");
        let url = if let Some(token) = token {
            format!("{}?tk={}", url, token)
        } else {
            url
        };

        info!("WebSocket connecting to {}", url);

        let sender_connect = self.sender.clone();
        let state_connect = Arc::clone(&self.state);
        *state_connect.write().await = WsState::Connecting;
        let _ = sender_connect.send(WsMessage::State(WsState::Connecting));

        let (ws_stream, _) = connect_async(&url).await?;
        let (mut write, mut read) = ws_stream.split();

        let state_connected = Arc::clone(&self.state);
        *state_connected.write().await = WsState::Connected;
        let sender_connected = self.sender.clone();
        let _ = sender_connected.send(WsMessage::State(WsState::Connected));

        info!("WebSocket connected");

        let sender_for_task = self.sender.clone();
        let state_for_task = Arc::clone(&self.state);
        
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(std::time::Duration::from_secs(60));
            let ping_data = bytes::Bytes::from_static(b"{\"type\":\"ping\"}");
            loop {
                tokio::select! {
                    _ = ticker.tick() => {
                        let ping_msg = tokio_tungstenite::tungstenite::Message::Binary(ping_data.clone());
                        if let Err(e) = write.send(ping_msg).await {
                            warn!("WebSocket send error: {}", e);
                            break;
                        }
                    }
                    msg = read.next() => {
                        match msg {
                            Some(Ok(Message::Text(text))) => {
                                debug!("WebSocket received: {}", text);
                                if let Ok(packet) = serde_json::from_str::<WebSocketPacket>(&text) {
                                    let _ = sender_for_task.send(WsMessage::Packet(packet));
                                }
                            }
                            Some(Ok(Message::Close(_))) => {
                                info!("WebSocket closed by server");
                                break;
                            }
                            Some(Err(e)) => {
                                error!("WebSocket error: {}", e);
                                let _ = sender_for_task.send(WsMessage::Error(e.to_string()));
                                break;
                            }
                            None => {
                                warn!("WebSocket stream ended");
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
            let state_disconnected = Arc::clone(&state_for_task);
            *state_disconnected.write().await = WsState::Disconnected;
            let _ = sender_for_task.send(WsMessage::State(WsState::Disconnected));
        });

        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<WsMessage> {
        self.sender.subscribe()
    }

    pub async fn get_state(&self) -> WsState {
        *self.state.read().await
    }
}

impl Default for WebSocketService {
    fn default() -> Self {
        Self::new()
    }
}