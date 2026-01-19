//! Background Network worker

use network_common::NetworkBackend;

pub struct NetworkWorker<B: NetworkBackend> {
    backend: B,
}
