#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "event")]
pub enum HipchatRequest<'a> {
    RoomEnter {
        item: RoomEnterItem<'a>,
        oauth_client_id: Option<&'a str>,
        webhook_id: usize,
    },
    RoomExit {
        item: RoomExitItem<'a>,
        oauth_client_id: Option<&'a str>,
        webhook_id: usize,
    },
    RoomMessage {
        item: RoomMessageItem<'a>,
        oauth_client_id: Option<&'a str>,
        webhook_id: usize,
    },
}

#[derive(Debug, Deserialize)]
pub struct RoomEnterItem<'a> {
    room: Room<'a>,
    sender: Sender<'a>,
}

#[derive(Debug, Deserialize)]
pub struct RoomExitItem<'a> {
    room: Room<'a>,
    sender: Sender<'a>,
}

#[derive(Debug, Deserialize)]
pub struct RoomMessageItem<'a> {
    message: Message<'a>,
    room: Room<'a>,
}

impl<'a> RoomMessageItem<'a> {
    pub fn message(&self) -> &str {
        &self.message.message
    }
}

#[derive(Debug, Deserialize)]
pub struct Message<'a> {
    date: &'a str,
    file: Option<MessageFile<'a>>,
    from: Sender<'a>,
    id: &'a str,
    mentions: Option<&'a [Sender<'a>]>,
    message: &'a str,
    message_links: Option<&'a [MessageLink<'a>]>,
    #[serde(rename = "type")]
    message_type: MessageType,
}

#[derive(Debug, Deserialize)]
struct MessageFile<'a> {
    name: &'a str,
    size: u32,
    thumb_url: Option<&'a str>,
    url: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct Sender<'a> {
    id: usize,
    links: Links<'a>,
    mention_name: &'a str,
    name: &'a str,
    version: &'a str,
}

#[derive(Debug, Deserialize)]
struct Links<'a> {
    #[serde(rename = "self")]
    link_self: &'a str,
}

#[derive(Debug, Deserialize)]
struct MessageLink<'a> {
    image: Option<MessageLinkImage<'a>>,
    link: Option<MessageLinkLink<'a>>,
    twitter_status: Option<MessageLinkTwitterStatus<'a>>,
    twitter_user: Option<MessageLinkTwitterUser<'a>>,
    #[serde(rename = "type")]
    message_type: MessageLinkType,
    url: &'a str,
    video: Option<MessageLinkVideo<'a>>,
}

#[derive(Debug, Deserialize)]
struct MessageLinkImage<'a> {
    image: Option<&'a str>,
    name: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageLinkLink<'a> {
    description: Option<&'a str>,
    favicon_url: Option<&'a str>,
    full_url: Option<&'a str>,
    header_text: Option<&'a str>,
    link_text: Option<&'a str>,
    title: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageLinkTwitterStatus<'a> {
    created: Option<&'a str>,
    name: Option<&'a str>,
    profile_image_url: Option<&'a str>,
    screen_name: Option<&'a str>,
    source: Option<&'a str>,
    text: &'a str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageLinkTwitterUser<'a> {
    followers: u32,
    name: &'a str,
    profile_image_url: &'a str,
    screen_name: &'a str,
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
struct MessageLinkVideo<'a> {
    author: Option<&'a str>,
    thumbnail_url: Option<&'a str>,
    title: Option<&'a str>,
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
pub struct Room<'a> {
    id: usize,
    is_archived: bool,
    links: RoomLinks<'a>,
    name: &'a str,
    privacy: &'a str,
    version: &'a str,
}

#[derive(Debug, Deserialize)]
struct RoomLinks<'a> {
    members: Option<&'a str>,
    participants: &'a str,
    #[serde(rename = "self")]
    link_self: &'a str,
    webhooks: &'a str,
}