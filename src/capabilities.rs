use Event;

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
    pub fn new<T, U, V>(capabilities: Capabilities,
                        description: T,
                        key: U,
                        links: Links,
                        name: V)
                        -> Self
        where T: AsRef<str>,
              U: AsRef<str>,
              V: AsRef<str>
    {
        Self {
            api_version: None,
            capabilities: Some(capabilities),
            description: description.as_ref().into(),
            key: key.as_ref().into(),
            links: links,
            name: name.as_ref().into(),
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
    webhook: Vec<WebHook>,
}

impl Capabilities {
    pub fn new(api_consumer: HipchatApiConsumer, webhooks: Vec<WebHook>) -> Self {
        Self {
            action: None,
            admin_page: None,
            configurable: None,
            dialog: None,
            external_page: None,
            glance: None,
            hipchat_api_consumer: Some(api_consumer),
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
    pub fn new<T>(link_self: T) -> Self
        where T: AsRef<str>
    {
        Self {
            homepage: None,
            link_self: link_self.as_ref().into(),
        }
    }

    pub fn with_homepage<T, U>(link_self: T, homepage: U) -> Self
        where T: AsRef<str>,
              U: AsRef<str>
    {
        Self {
            homepage: Some(homepage.as_ref().into()),
            link_self: link_self.as_ref().into(),
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
    from_name: String,
    scopes: Vec<Scope>,
}

impl HipchatApiConsumer {
    pub fn new<T>(from_name: T, scopes: Vec<Scope>) -> Self
        where T: AsRef<str>
    {
        Self {
            avatar: None,
            from_name: from_name.as_ref().into(),
            scopes: scopes,
        }
    }

    pub fn with_avatar<T>(avatar: Avatar, from_name: T, scopes: Vec<Scope>) -> Self
        where T: AsRef<str>
    {
        Self {
            avatar: Some(avatar),
            from_name: from_name.as_ref().into(),
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
    pub fn new<T>(url: T) -> Self
        where T: AsRef<str>
    {
        Self {
            url: url.as_ref().into(),
            url2x: None,
        }
    }

    pub fn with_2x<T, U>(url: T, url2x: U) -> Self
        where T: AsRef<str>,
              U: AsRef<str>
    {
        Self {
            url: url.as_ref().into(),
            url2x: Some(url2x.as_ref().into()),
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
    name: String,
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
    pub fn new<T, U, V>(name: T, url: U, event: CapabilitiesEvent<V>) -> Self
        where T: AsRef<str>,
              U: AsRef<str>,
              V: AsRef<str>
    {
        let (event, pattern) = match event {
            CapabilitiesEvent::RoomMessage(p) => (Event::RoomMessage, Some(p.as_ref().into())),
            e @ _ => (Event::from(&e), None),
        };

        Self {
            name: name.as_ref().into(),
            url: url.as_ref().into(),
            pattern: pattern,
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

pub enum CapabilitiesEvent<T>
    where T: AsRef<str>
{
    RoomArchived,
    RoomCreated,
    RoomDeleted,
    RoomEnter,
    RoomExit,
    RoomFileUpload,
    RoomMessage(T),
    RoomNotification,
    RoomTopicChange,
    RoomUnarchived,
}

impl<'a, T> From<&'a CapabilitiesEvent<T>> for Event
    where T: AsRef<str>
{
    fn from(event: &'a CapabilitiesEvent<T>) -> Event {
        match *event {
            CapabilitiesEvent::RoomArchived => Event::RoomArchived,
            CapabilitiesEvent::RoomCreated => Event::RoomCreated,
            CapabilitiesEvent::RoomDeleted => Event::RoomDeleted,
            CapabilitiesEvent::RoomEnter => Event::RoomEnter,
            CapabilitiesEvent::RoomExit => Event::RoomExit,
            CapabilitiesEvent::RoomFileUpload => Event::RoomFileUpload,
            CapabilitiesEvent::RoomMessage(_) => Event::RoomMessage,
            CapabilitiesEvent::RoomNotification => Event::RoomNotification,
            CapabilitiesEvent::RoomTopicChange => Event::RoomTopicChange,
            CapabilitiesEvent::RoomUnarchived => Event::RoomUnarchived,
        }
    }
}
