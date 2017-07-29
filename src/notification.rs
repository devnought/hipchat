#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MessageFormat {
    Html,
    Text,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    Yellow,
    Green,
    Red,
    Purple,
    Gray,
    Random,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Style {
    File,
    Image,
    Application,
    Link,
    Media,
}

#[derive(Serialize, Debug)]
pub struct Description {
    value: String,
    format: MessageFormat,
}

impl Description {
    pub fn new<T>(value: T, format: MessageFormat) -> Self
        where T: AsRef<str>
    {
        Self {
            value: value.as_ref().into(),
            format: format,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    Compact,
    Medium,
}

#[derive(Serialize, Debug)]
pub struct Thumbnail {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url2x: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u32>,
}

impl Thumbnail {
    pub fn new<T>(url: T) -> Self
        where T: AsRef<str>
    {
        Self {
            url: url.as_ref().into(),
            url2x: None,
            width: None,
            height: None,
        }
    }
}

#[derive(Serialize, Debug)]
struct Activity {
    html: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum AttributeStyle {
    LozengeSuccess,
    LozengeError,
    LozengeCurrent,
    LozengeComplete,
    LozengeMoved,
    Lozenge,
}

#[derive(Serialize, Debug)]
struct Attribute {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<AttributeStyle>,
}

#[derive(Serialize, Debug)]
struct Icon {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url2x: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Card {
    style: Style,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Description>,
    format: Format,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<Thumbnail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activity: Option<Activity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<Attribute>>,
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
}

impl Card {
    pub fn new<T, U>(style: Style,
                     description: Description,
                     format: Format,
                     title: T,
                     thumbnail: Thumbnail,
                     id: U)
                     -> Self
        where T: AsRef<str>,
              U: AsRef<str>
    {
        Self {
            style: style,
            description: Some(description),
            format: format,
            url: None,
            title: title.as_ref().into(),
            thumbnail: Some(thumbnail),
            activity: None,
            attributes: None,
            id: id.as_ref().into(),
            icon: None,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Notification {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_format: Option<MessageFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attach_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify: Option<bool>,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<Card>,
}

impl<'a> Notification {
    pub fn basic<T>(message: T, color: Color, format: MessageFormat) -> Self
        where T: AsRef<str>
    {
        Self {
            from: None,
            message_format: Some(format),
            color: Some(color),
            attach_to: None,
            notify: None,
            message: message.as_ref().into(),
            card: None,
        }
    }

    pub fn with_card(color: Color, card: Card, notify: bool) -> Self {
        Self {
            from: None,
            message_format: Some(MessageFormat::Text),
            color: Some(color),
            attach_to: None,
            notify: Some(notify),
            message: card.title.clone(),
            card: Some(card),
        }
    }
}