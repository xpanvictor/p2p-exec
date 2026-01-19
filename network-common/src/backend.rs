use std::task::{Context, Poll};

use network_types::{PeerId, ProtocolName};

pub enum NetworkBackendEvent {
    NotificationStreamOpen {
        remote: PeerId,
        protocol: ProtocolName,
    },
    NotificationStreamClosed {
        remote: PeerId,
        protocol: ProtocolName,
        reason: String,
    },
    NotificationReceived {
        remote: PeerId,
        protocol: ProtocolName,
        payload: Vec<u8>,
    },
}

pub trait NetworkBackend: Send + Unpin {
    fn poll(&mut self, cx: &mut Context) -> Poll<NetworkBackendEvent>;
    fn send_notification(&mut self, target: PeerId, protocol: ProtocolName, payload: Vec<u8>);
    fn local_peer_id(&self) -> PeerId;
}
