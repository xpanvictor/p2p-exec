//! Background Network worker

use std::collections::HashMap;

use network_common::NetworkBackend;

pub struct NetworkWorker<B: NetworkBackend> {
    backend: B,
}