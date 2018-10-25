use serde_derive::Serialize;

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
    where
        T: AsRef<str>,
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
    where
        T: AsRef<str>,
    {
        Self {
            url: url.as_ref().into(),
            url2x: None,
            width: None,
            height: None,
        }
    }

    pub fn with_2x<T, U>(url: T, url2x: U) -> Self
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        Self {
            url: url.as_ref().into(),
            url2x: Some(url2x.as_ref().into()),
            width: None,
            height: None,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Activity {
    html: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
}

impl Activity {
    pub fn new<T>(html: T) -> Self
    where
        T: AsRef<str>,
    {
        Activity {
            html: html.as_ref().into(),
            icon: None,
        }
    }
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
pub struct Attribute {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<AttributeStyle>,
}

#[derive(Serialize, Debug)]
pub struct Icon {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url2x: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Card {
    pub style: Style,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    pub format: Format,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Thumbnail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<Activity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
}

impl Card {
    pub fn new<T, U>(style: Style, format: Format, title: T, id: U, activity: Activity) -> Self
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        Self {
            style: style,
            description: None,
            format: format,
            url: None,
            title: title.as_ref().into(),
            thumbnail: None,
            activity: Some(activity),
            attributes: None,
            id: id.as_ref().into(),
            icon: None,
        }
    }

    pub fn with_thumbnail<T, U>(
        style: Style,
        format: Format,
        title: T,
        id: U,
        thumbnail: Thumbnail,
    ) -> Self
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        Self {
            style: style,
            description: None,
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
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_format: Option<MessageFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
}

impl<'a> Notification {
    pub fn basic<T>(message: T, color: Color, format: MessageFormat) -> Self
    where
        T: AsRef<str>,
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

    pub fn with_card<T>(message: T, color: Color, format: MessageFormat, card: Card) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            from: None,
            message_format: Some(format),
            color: Some(color),
            attach_to: None,
            notify: None,
            message: message.as_ref().into(),
            card: Some(card),
        }
    }
}
