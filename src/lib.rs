extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod event;
use event::Event;

pub mod capabilities;
pub mod request;
pub mod notification;
pub mod user;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
