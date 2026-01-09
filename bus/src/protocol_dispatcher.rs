use std::marker::PhantomData;
use parity_scale_codec::Decode;

use crate::bus::EventBus;

pub trait Dispatcher: Send + Sync {
    fn dispatch(&self, payload: Vec<u8>);
}

pub struct TypedDispatcher<T> {
    event_bus: EventBus,
    _marker: PhantomData<T>
}

impl<T: Decode + Clone + Send + Sync + 'static> TypedDispatcher<T> {
    pub fn new(event_bus: EventBus) -> Self {
        TypedDispatcher { event_bus, _marker: PhantomData }
    }
}

impl<T: Decode + Clone + Send + Sync + 'static> Dispatcher for TypedDispatcher<T> {
    fn dispatch(&self, payload: Vec<u8>) {
        if let Ok(data) = T::decode(&mut &payload[..]) {
            self.event_bus.publish(data);
        } else {
            // handle tracing
            eprintln!("failed to decode SCALE message");
        };
    }
}

#[cfg(test)]
mod test {
    use parity_scale_codec::{Decode, Encode};

    use crate::{ EventBus, TypedDispatcher, protocol_dispatcher::Dispatcher};

    #[test]
    fn check_scale_types() {
        #[derive(Debug, Clone, Encode, Decode)]
        struct NetworkMsg {
            name: String,
        }

        let msg = NetworkMsg {name: "hello".into()};
        let bytes = msg.encode();
        assert_eq!(msg.name, NetworkMsg::decode(&mut &bytes[..]).unwrap().name)
    }
}
