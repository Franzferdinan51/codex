use serde::{Deserialize, Serialize};

/// A message that has arrived from an external channel and needs to be
/// routed into the DuckHive app-server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundMessage {
    pub channel: ChannelId,
    pub sender: SenderInfo,
    pub content: MessageContent,
    pub thread_id: Option<String>,
}

/// Identifies the channel the message came from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelId {
    pub adapter: String,
    pub channel_name: String,
    pub channel_id: String,
}

/// Information about the sender of an inbound message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SenderInfo {
    pub user_id: String,
    pub display_name: Option<String>,
}

/// The body of an inbound message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageContent {
    pub text: String,
    pub attachments: Vec<Attachment>,
}

/// An attachment embedded in a message (image, file, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attachment {
    pub kind: AttachmentKind,
    pub url: Option<String>,
    pub mime_type: Option<String>,
    pub filename: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentKind {
    Image,
    File,
    Audio,
    Video,
}
