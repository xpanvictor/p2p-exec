use std::hash::{DefaultHasher, Hash, Hasher};

use libp2p::{
    Swarm, futures::StreamExt, gossipsub, identify, identity::Keypair, swarm::SwarmEvent,
};
pub use network_common::NetworkBackend;
use network_types::{PeerId, ProtocolName};

use crate::behavior::{Libp2pExecBehavior, Libp2pExecBehaviorEvent};

pub struct Libp2pBackend {
    swarm: Swarm<Libp2pExecBehavior>,
}

impl Libp2pBackend {
    pub fn new(local_key: &Keypair) -> Self {
        // transport,
        // gossipsub
        let message_id_fn = |msg: &gossipsub::Message| {
            let mut s = DefaultHasher::new();
            msg.data.hash(&mut s);
            gossipsub::MessageId::from(s.finish().to_string())
        };
        todo!();
    }
}

impl NetworkBackend for Libp2pBackend {
    fn poll(
        &mut self,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<network_common::NetworkBackendEvent> {
        while let std::task::Poll::Ready(Some(event)) = self.swarm.poll_next_unpin(cx) {
            match event {
                SwarmEvent::Behaviour(Libp2pExecBehaviorEvent::Identify(
                    identify::Event::Received {
                        connection_id,
                        peer_id,
                        info,
                    },
                )) => {
                    tracing::debug!(name: "new peer", format!("peer id: {}", peer_id));
                    for addr in info.listen_addrs {
                        self.swarm.behaviour_mut().kademlia.add_address();
                    }
                }
                _ => {
                    tracing::warn!("not listening to other swarm events")
                }
            }
        }
        std::task::Poll::Pending
    }

    fn send_notification(&mut self, target: PeerId, protocol: ProtocolName, payload: Vec<u8>) {
        todo!()
    }

    fn local_peer_id(&self) -> PeerId {
        todo!()
    }
}
