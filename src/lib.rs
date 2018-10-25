extern crate serde;
extern crate serde_derive;

mod event;
use event::Event;

pub mod capabilities;
pub mod notification;
pub mod request;
pub mod user;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
