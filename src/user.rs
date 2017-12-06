use std::borrow::Cow;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum UserRequest<'a> {
    Add {
        name: Cow<'a, str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        roles: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<Cow<'a, str>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mention_name: Option<Cow<'a, str>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_group_admin: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        timezone: Option<Cow<'a, str>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        password: Option<Cow<'a, str>>,
        email: Cow<'a, str>,
    },
    Invite {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<Cow<'a, str>>,
        email: Cow<'a, str>,
        name: Cow<'a, str>,
    },
    Delete { id_or_email: Cow<'a, str> },
}
