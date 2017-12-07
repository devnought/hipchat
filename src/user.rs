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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum UserResponse<'a> {
    Created {
        xmpp_jid: Cow<'a, str>,
        is_deleted: bool,
        name: Cow<'a, str>,
        last_active: Option<Cow<'a, str>>,
        title: Cow<'a, str>,
        presence: Option<UserPresence<'a>>,
        created: Cow<'a, str>,
        id: isize,
        mention_name: Cow<'a, str>,
        version: Cow<'a, str>,
        roles: Vec<Cow<'a, str>>,
        is_group_admin: bool,
        timezone: Cow<'a, str>,
        is_guest: bool,
        email: Option<Cow<'a, str>>,
        photo_url: Option<Cow<'a, str>>,
    },
    Error { error: UserError<'a> },
}

#[derive(Debug, Deserialize)]
pub struct UserPresence<'a> {
    status: Option<Cow<'a, str>>,
    idle: Option<isize>,
    show: Option<Cow<'a, str>>,
    client: Option<UserClient<'a>>,
    is_online: bool,
}

#[derive(Debug, Deserialize)]
pub struct UserClient<'a> {
    version: Option<Cow<'a, str>>,
    #[serde(rename = "type")]
    client_type: Option<Cow<'a, str>>,
}

#[derive(Debug, Deserialize)]
pub struct UserError<'a> {
    code: isize,
    message: Cow<'a, str>,
    #[serde(rename = "type")]
    error_type: Cow<'a, str>,
}
