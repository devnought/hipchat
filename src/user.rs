use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub enum UserRequest<'a> {
    Add {
        name: Cow<'a, str>
    }
}