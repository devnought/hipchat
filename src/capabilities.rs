#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CapabilitiesDescriptor {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<Capabilities>,
    description: String,
    key: String,
    links: Links,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    vendor: Option<Vendor>,
}

impl CapabilitiesDescriptor {
    pub fn new<T>(capabilities: Capabilities, description: T, key: T, links: Links, name: T) -> Self
        where T: Into<String>
    {
        Self {
            api_version: None,
            capabilities: Some(capabilities),
            description: description.into(),
            key: key.into(),
            links: links,
            name: name.into(),
            vendor: None,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<Action>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_page: Option<AdminPage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configurable: Option<Configurable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dialog: Option<Vec<Dialog>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_page: Option<Vec<ExternalPage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    glance: Option<Vec<Glance>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hipchat_api_consumer: Option<HipchatApiConsumer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    installable: Option<Installable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_consumer: Option<OAuth2Consumer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_provider: Option<OAuth2Provider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_panel: Option<Vec<WebPanel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook: Option<Vec<WebHook>>,
}

impl Capabilities {
    pub fn new(api_consumer: Option<HipchatApiConsumer>, webhooks: Option<Vec<WebHook>>) -> Self {
        Self {
            action: None,
            admin_page: None,
            configurable: None,
            dialog: None,
            external_page: None,
            glance: None,
            hipchat_api_consumer: api_consumer,
            installable: None,
            oauth2_consumer: None,
            oauth2_provider: None,
            web_panel: None,
            webhook: webhooks,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Links {
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage: Option<String>,

    #[serde(rename = "self")]
    link_self: String,
}

impl Links {
    pub fn new<'a, 'b>(homepage: Option<&'a str>, link_self: &'b str) -> Self {
        Self {
            homepage: match homepage {
                Some(h) => Some(h.to_string()),
                None => None,
            },
            link_self: link_self.to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
struct Vendor {
    name: String,
    url: String,
}

#[derive(Serialize, Debug)]
pub struct Action {}

#[derive(Serialize, Debug)]
struct AdminPage {}

#[derive(Serialize, Debug)]
struct Configurable {}

#[derive(Serialize, Debug)]
struct Dialog {}

#[derive(Serialize, Debug)]
struct ExternalPage {}

#[derive(Serialize, Debug)]
struct Glance {}

#[derive(Serialize, Debug)]
pub struct HipchatApiConsumer {
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<Avatar>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_name: Option<String>,
    scopes: Vec<Scope>,
}

impl HipchatApiConsumer {
    pub fn new<T>(avatar: Option<Avatar>, from_name: Option<T>, scopes: Vec<Scope>) -> Self
        where T: Into<String>
    {
        Self {
            avatar: avatar,
            from_name: from_name.map(|x| x.into()),
            scopes: scopes,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Avatar {
    url: String,

    #[serde(rename = "url@2x", skip_serializing_if = "Option::is_none")]
    url2x: Option<String>,
}

impl Avatar {
    pub fn new<T>(url: T, url2x: Option<T>) -> Self
        where T: Into<String>
    {
        Self {
            url: url.into(),
            url2x: url2x.map(|x| x.into()),
        }
    }
}

#[derive(Serialize, Debug)]
struct Installable {}

#[derive(Serialize, Debug)]
struct OAuth2Consumer {}

#[derive(Serialize, Debug)]
struct OAuth2Provider {}

#[derive(Serialize, Debug)]
struct WebPanel {}

#[derive(Serialize, Debug)]
pub struct WebHook {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    event: Event,
}

impl WebHook {
    pub fn new<T>(name: Option<T>, url: T, pattern: Option<T>, event: Event) -> Self
        where T: Into<String>
    {
        Self {
            name: name.map(|x| x.into()),
            url: url.into(),
            pattern: pattern.map(|x| x.into()),
            authentication: None,
            key: None,
            event: event,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    AdminGroup,
    AdminRoom,
    ManageRooms,
    SendMessage,
    SendNotification,
    ViewGroup,
    ViewMessages,
    ViewRoom,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    RoomArchived,
    RoomCreated,
    RoomDeleted,
    RoomEnter,
    RoomExit,
    RoomFileUpload,
    RoomMessage,
    RoomNotification,
    RoomTopicChange,
    RoomUnarchived,
}