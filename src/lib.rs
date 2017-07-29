extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod capabilities;
pub use capabilities::{Avatar, CapabilitiesDescriptor, Scope, HipchatApiConsumer, WebHook, Event,
                       Capabilities, Links};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
