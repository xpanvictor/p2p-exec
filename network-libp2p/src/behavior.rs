//! Behaviour managing entire structure
//! todo!()

use libp2p::{gossipsub, swarm::NetworkBehaviour};


#[derive(NetworkBehaviour)]
pub struct Libp2pExecBehavior {
    gossipsub: gossipsub::Behaviour
}
