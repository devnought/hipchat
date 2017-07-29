#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MessageFormat {
    Html,
    Text,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Yellow,
    Green,
    Red,
    Purple,
    Gray,
    Random,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Style {
    File,
    Image,
    Application,
    Link,
    Media,
}

#[derive(Serialize, Debug)]
struct DescriptionObject {
    value: String,
    format: String,
}

#[derive(Serialize, Debug)]
enum Description {
    Plain(String),
    Complex(DescriptionObject),
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Format {
    Compact,
    Medium,
}

#[derive(Serialize, Debug)]
struct Thumbnail {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url2x: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u32>,
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
enum Icon {
    Plain(String),
    Complex(IconObj),
}

#[derive(Serialize, Debug)]
struct IconObj {
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
        Notification {
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
        where T: AsRef<str>
    {
        Notification {
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