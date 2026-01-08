use libp2p::{Swarm};
pub use network_common::NetworkBackend;
use network_types::{PeerId, ProtocolName};

use crate::behavior::Libp2pExecBehavior;


pub struct Libp2pBackend {
    swarm: Swarm<Libp2pExecBehavior>
}

impl NetworkBackend for Libp2pBackend {
    fn poll(&mut self, cx: &mut std::task::Context) -> std::task::Poll<network_common::NetworkBackendEvent> {
        todo!()
    }

    fn send_notification(&mut self, target: PeerId, protocol: ProtocolName, payload: Vec<u8>) {
        todo!()
    }

    fn local_peer_id(&self) -> PeerId {
        todo!()
    }
}

