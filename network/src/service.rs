use std::sync::mpsc;

use network_common::NetworkBackendEvent;

pub struct NetworkService {
    sender: mpsc::Sender<NetworkBackendEvent>,
}

impl NetworkService {
    pub fn new(sender: mpsc::Sender<NetworkBackendEvent>) -> NetworkService {
        NetworkService { sender }
    }
}
