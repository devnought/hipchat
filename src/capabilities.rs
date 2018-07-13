use std::borrow::Cow;
use Event;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CapabilitiesDescriptor<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_version: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<Capabilities<'a>>,
    description: Cow<'a, str>,
    key: Cow<'a, str>,
    links: Links<'a>,
    name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vendor: Option<Vendor<'a>>,
}

impl<'a> CapabilitiesDescriptor<'a> {
    pub fn new<T>(
        capabilities: Capabilities<'a>,
        description: T,
        key: T,
        links: Links<'a>,
        name: T,
    ) -> Self
    where
        T: Into<Cow<'a, str>>,
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
pub struct Capabilities<'a> {
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
    hipchat_api_consumer: Option<HipchatApiConsumer<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    installable: Option<Installable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_consumer: Option<OAuth2Consumer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth2_provider: Option<OAuth2Provider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_panel: Option<Vec<WebPanel>>,
    webhook: Vec<WebHook<'a>>,
}

impl<'a> Capabilities<'a> {
    pub fn new(api_consumer: HipchatApiConsumer<'a>, webhooks: Vec<WebHook<'a>>) -> Self {
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
    homepage: Option<Cow<'a, str>>,

    #[serde(rename = "self")]
    link_self: Cow<'a, str>,
}

impl<'a> Links<'a> {
    pub fn new<T>(link_self: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            homepage: None,
            link_self: link_self.into(),
        }
    }

    pub fn with_homepage<T>(link_self: T, homepage: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            homepage: Some(homepage.into()),
            link_self: link_self.into(),
        }
    }
}

#[derive(Serialize, Debug)]
struct Vendor<'a> {
    name: Cow<'a, str>,
    url: Cow<'a, str>,
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
    from_name: Cow<'a, str>,
    scopes: Vec<Scope>,
}

impl<'a> HipchatApiConsumer<'a> {
    pub fn new<T>(from_name: T, scopes: Vec<Scope>) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            avatar: None,
            from_name: from_name.into(),
            scopes,
        }
    }

    pub fn with_avatar<T>(avatar: Avatar<'a>, from_name: T, scopes: Vec<Scope>) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            avatar: Some(avatar),
            from_name: from_name.into(),
            scopes,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Avatar<'a> {
    url: Cow<'a, str>,

    #[serde(rename = "url@2x", skip_serializing_if = "Option::is_none")]
    url2x: Option<Cow<'a, str>>,
}

impl<'a> Avatar<'a> {
    pub fn new<T>(url: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            url: url.into(),
            url2x: None,
        }
    }

    pub fn with_2x<T>(url: T, url2x: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            url: url.into(),
            url2x: Some(url2x.into()),
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
    name: Cow<'a, str>,
    url: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<Cow<'a, str>>,
    event: Event,
}

impl<'a> WebHook<'a> {
    pub fn new<T, U, V>(name: T, url: U, event: CapabilitiesEvent<V>) -> Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        let (event, pattern) = match event {
            CapabilitiesEvent::RoomArchived => (Event::RoomArchived, None),
            CapabilitiesEvent::RoomCreated => (Event::RoomCreated, None),
            CapabilitiesEvent::RoomDeleted => (Event::RoomDeleted, None),
            CapabilitiesEvent::RoomEnter => (Event::RoomEnter, None),
            CapabilitiesEvent::RoomExit => (Event::RoomExit, None),
            CapabilitiesEvent::RoomFileUpload => (Event::RoomFileUpload, None),
            CapabilitiesEvent::RoomMessage(p) => (Event::RoomMessage, Some(p.into())),
            CapabilitiesEvent::RoomNotification => (Event::RoomNotification, None),
            CapabilitiesEvent::RoomTopicChange => (Event::RoomTopicChange, None),
            CapabilitiesEvent::RoomUnarchived => (Event::RoomUnarchived, None),
        };

        Self {
            name: name.into(),
            url: url.into(),
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

pub enum CapabilitiesEvent<T> {
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
