use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "event")]
pub enum HipchatRequest {
    RoomEnter {
        item: RoomEnterItem,
        oauth_client_id: Option<String>,
        webhook_id: usize,
    },
    RoomExit {
        item: RoomExitItem,
        oauth_client_id: Option<String>,
        webhook_id: usize,
    },
    RoomMessage {
        item: RoomMessageItem,
        oauth_client_id: Option<String>,
        webhook_id: usize,
    },
}

#[derive(Debug, Deserialize)]
pub struct RoomEnterItem {
    room: Room,
    sender: Sender,
}

#[derive(Debug, Deserialize)]
pub struct RoomExitItem {
    room: Room,
    sender: Sender,
}

#[derive(Debug, Deserialize)]
pub struct RoomMessageItem {
    message: Message,
    room: Room,
}

impl RoomMessageItem {
    pub fn message(&self) -> &str {
        &self.message.message
    }
}

#[derive(Debug, Deserialize)]
pub struct Message {
    date: String,
    file: Option<MessageFile>,
    from: Sender,
    id: String,
    mentions: Option<Vec<Sender>>,
    message: String,
    message_links: Option<Vec<MessageLink>>,
    #[serde(rename = "type")]
    message_type: MessageType,
}

#[derive(Debug, Deserialize)]
struct MessageFile {
    name: String,
    size: u32,
    thumb_url: Option<String>,
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct Sender {
    id: usize,
    links: Links,
    mention_name: String,
    name: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct Links {
    #[serde(rename = "self")]
    link_self: String,
}

#[derive(Debug, Deserialize)]
struct MessageLink {
    image: Option<MessageLinkImage>,
    link: Option<MessageLinkLink>,
    twitter_status: Option<MessageLinkTwitterStatus>,
    twitter_user: Option<MessageLinkTwitterUser>,
    #[serde(rename = "type")]
    message_type: MessageLinkType,
    url: String,
    video: Option<MessageLinkVideo>,
}

#[derive(Debug, Deserialize)]
struct MessageLinkImage {
    image: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageLinkLink {
    description: Option<String>,
    favicon_url: Option<String>,
    full_url: Option<String>,
    header_text: Option<String>,
    link_text: Option<String>,
    title: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageLinkTwitterStatus {
    created: Option<String>,
    name: Option<String>,
    profile_image_url: Option<String>,
    screen_name: Option<String>,
    source: Option<String>,
    text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageLinkTwitterUser {
    followers: u32,
    name: String,
    profile_image_url: String,
    screen_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum MessageLinkType {
    Image,
    Video,
    Link,
    TwitterStatus,
    TwitterUser,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageLinkVideo {
    author: Option<String>,
    thumbnail_url: Option<String>,
    title: Option<String>,
    views: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum MessageType {
    Message,
    GuestAccess,
    Topic,
    Notification,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    id: usize,
    is_archived: bool,
    links: RoomLinks,
    name: String,
    privacy: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct RoomLinks {
    members: Option<String>,
    participants: String,
    #[serde(rename = "self")]
    link_self: String,
    webhooks: String,
}
