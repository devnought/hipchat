use Event;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CapabilitiesDescriptor<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_version: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<Capabilities<'a>>,
    description: &'a str,
    key: &'a str,
    links: Links<'a>,
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    vendor: Option<Vendor<'a>>,
}

impl<'a> CapabilitiesDescriptor<'a> {
    pub fn new(capabilities: Capabilities<'a>,
               description: &'a str,
               key: &'a str,
               links: Links<'a>,
               name: &'a str)
               -> Self {
        Self {
            api_version: None,
            capabilities: Some(capabilities),
            description: description,
            key: key,
            links: links,
            name: name,
            vendor: None,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<&'a [Action]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_page: Option<AdminPage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configurable: Option<Configurable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dialog: Option<&'a [Dialog]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_page: Option<&'a [ExternalPage]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    glance: Option<&'a [Glance]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hipchat_api_consumer: Option<HipchatApiConsumer<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    installable: Option<Installable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_consumer: Option<OAuth2Consumer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_provider: Option<OAuth2Provider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_panel: Option<&'a [WebPanel]>,
    webhook: &'a [WebHook<'a>],
}

impl<'a> Capabilities<'a> {
    pub fn new(api_consumer: HipchatApiConsumer<'a>, webhooks: &'a [WebHook<'a>]) -> Self {
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
pub struct Links<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage: Option<&'a str>,

    #[serde(rename = "self")]
    link_self: &'a str,
}

impl<'a> Links<'a> {
    pub fn new(link_self: &'a str) -> Self {
        Self {
            homepage: None,
            link_self: link_self,
        }
    }

    pub fn with_homepage(link_self: &'a str, homepage: &'a str) -> Self {
        Self {
            homepage: Some(homepage),
            link_self: link_self,
        }
    }
}

#[derive(Serialize, Debug)]
struct Vendor<'a> {
    name: &'a str,
    url: &'a str,
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
pub struct HipchatApiConsumer<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<Avatar<'a>>,
    from_name: &'a str,
    scopes: &'a [Scope],
}

impl<'a> HipchatApiConsumer<'a> {
    pub fn new(from_name: &'a str, scopes: &'a [Scope]) -> Self {
        Self {
            avatar: None,
            from_name: from_name,
            scopes: scopes,
        }
    }

    pub fn with_avatar(avatar: Avatar<'a>, from_name: &'a str, scopes: &'a [Scope]) -> Self {
        Self {
            avatar: Some(avatar),
            from_name: from_name,
            scopes: scopes,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Avatar<'a> {
    url: &'a str,

    #[serde(rename = "url@2x", skip_serializing_if = "Option::is_none")]
    url2x: Option<&'a str>,
}

impl<'a> Avatar<'a> {
    pub fn new(url: &'a str) -> Self {
        Self {
            url: url,
            url2x: None,
        }
    }

    pub fn with_2x(url: &'a str, url2x: &'a str) -> Self {
        Self {
            url: url,
            url2x: Some(url2x),
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
pub struct WebHook<'a> {
    name: &'a str,
    url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<&'a str>,
    event: Event,
}

impl<'a> WebHook<'a> {
    pub fn new(name: &'a str, url: &'a str, event: CapabilitiesEvent<&'a str>) -> Self {
        let (event, pattern) = match event {
            CapabilitiesEvent::RoomMessage(p) => (Event::RoomMessage, Some(p)),
            e @ _ => (Event::from(&e), None),
        };

        Self {
            name: name,
            url: url,
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
