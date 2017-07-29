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
    pub fn new(api_consumer: Option<HipchatApiConsumer>, webhooks: Vec<WebHook>) -> Self {
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
    pub fn new<T, U>(link_self: T, homepage: Option<U>) -> Self
        where T: AsRef<str>,
              U: AsRef<str>
    {
        Self {
            homepage: match homepage {
                Some(h) => Some(h.as_ref().into()),
                None => None,
            },
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
    pub fn new<T>(avatar: Option<Avatar>, from_name: T, scopes: Vec<Scope>) -> Self
        where T: AsRef<str>
    {
        Self {
            avatar: avatar,
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
    pub fn new<T, U>(url: T, url2x: Option<U>) -> Self
        where T: AsRef<str>,
              U: AsRef<str>
    {
        Self {
            url: url.as_ref().into(),
            url2x: url2x.map(|x| x.as_ref().into()),
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
    event: InternalEvent,
}

impl WebHook {
    pub fn new<T, U, V>(name: Option<T>, url: U, event: Event<V>) -> Self
        where T: AsRef<str>,
              U: AsRef<str>,
              V: AsRef<str>
    {
        let (event, pattern) = match event {
            Event::RoomMessage(p) => (InternalEvent::RoomMessage, Some(p.as_ref().into())),
            e @ _ => (InternalEvent::from(&e), None),
        };

        Self {
            name: name.map(|x| x.as_ref().into()),
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

pub enum Event<T>
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

impl<'a, T> From<&'a Event<T>> for InternalEvent
    where T: AsRef<str>
{
    fn from(event: &'a Event<T>) -> InternalEvent {
        match *event {
            Event::RoomArchived => InternalEvent::RoomArchived,
            Event::RoomCreated => InternalEvent::RoomCreated,
            Event::RoomDeleted => InternalEvent::RoomDeleted,
            Event::RoomEnter => InternalEvent::RoomEnter,
            Event::RoomExit => InternalEvent::RoomExit,
            Event::RoomFileUpload => InternalEvent::RoomFileUpload,
            Event::RoomMessage(_) => InternalEvent::RoomMessage,
            Event::RoomNotification => InternalEvent::RoomNotification,
            Event::RoomTopicChange => InternalEvent::RoomTopicChange,
            Event::RoomUnarchived => InternalEvent::RoomUnarchived,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
enum InternalEvent {
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