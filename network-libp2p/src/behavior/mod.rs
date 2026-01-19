//! Behaviour managing entire structure
//! todo!()

pub mod messaging;

use libp2p::{
    gossipsub, identify, kad, ping,
    request_response::{self},
    swarm::NetworkBehaviour,
};
use messaging::P2pExecReqResCodec;

#[derive(NetworkBehaviour)]
pub struct Libp2pExecBehavior {
    gossipsub: gossipsub::Behaviour,
    kad: kad::Behaviour<kad::store::MemoryStore>,
    identify: identify::Behaviour,
    keep_alive: ping::Behaviour,
    network: request_response::Behaviour<P2pExecReqResCodec>,
}
