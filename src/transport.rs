use futures::future::Either;
use libp2p::{
    PeerId,
    core::{self, Transport, muxing::StreamMuxerBox, transport::Boxed, upgrade},
    noise, quic, tcp, yamux,
};

use crate::identity::IdentityKP;

pub type FinalTransport = Boxed<(PeerId, core::muxing::StreamMuxerBox)>;

pub struct TransportBuilder {
    pub keypair: IdentityKP,
    pub enable_tcp: bool,
    pub enable_ws: bool,
    pub enable_quic: bool,
}

impl TransportBuilder {
    pub fn new(keypair: IdentityKP) -> Self {
        Self {
            keypair,
            enable_tcp: true,
            enable_ws: false,
            enable_quic: false,
        }
    }

    pub fn with_tcp(mut self, enable: bool) -> Self {
        self.enable_tcp = enable;
        self
    }

    pub fn with_ws(mut self, enable: bool) -> Self {
        self.enable_ws = enable;
        self
    }

    pub fn with_quic(mut self, enable: bool) -> Self {
        self.enable_quic = enable;
        self
    }

    pub fn build(self) -> FinalTransport {
        let mut final_transport: Option<FinalTransport> = None;
        let local_kp = self.keypair.get_kp();

        let box_transport = |transport: Boxed<(PeerId, core::muxing::StreamMuxerBox)>| {
            transport
                .map(|(peer_id, muxer), _| (peer_id, core::muxing::StreamMuxerBox::new(muxer)))
                .boxed()
        };
        let combine_transports = |t1: FinalTransport, t2: FinalTransport| {
            t1.or_transport(t2)
                .map(|either, _| match either {
                    Either::Left(v) => v,
                    Either::Right(v) => v,
                })
                .boxed()
        };

        if self.enable_tcp {
            let tcp_config = tcp::tokio::Transport::new(tcp::Config::default());
            let mux_config = yamux::Config::default();
            let noise_config =
                noise::Config::new(&local_kp).expect("Failed to create noise config");
            let transport = tcp_config
                .upgrade(upgrade::Version::V1)
                .authenticate(noise_config)
                .multiplex(mux_config)
                .boxed();
            final_transport = Some(box_transport(transport));
        }

        if self.enable_quic {
            let transport = quic::tokio::Transport::new(quic::Config::new(&local_kp)).boxed();
            let boxed_transport = box_transport(
                transport
                    .map(|(peer_id, conn), _| (peer_id, core::muxing::StreamMuxerBox::new(conn)))
                    .boxed(),
            );
            final_transport = match final_transport {
                Some(t) => Some(combine_transports(t, boxed_transport)),
                None => Some(boxed_transport),
            }
        }

        if self.enable_ws {
            let tcp_config = tcp::tokio::Transport::new(tcp::Config::default());
            let ws_config = libp2p::websocket::WsConfig::new(tcp_config);
            let mux_config = yamux::Config::default();
            let noise_config =
                noise::Config::new(&local_kp).expect("Failed to create noise config");
            let transport = ws_config
                .upgrade(upgrade::Version::V1)
                .authenticate(noise_config)
                .multiplex(mux_config)
                .boxed();
            let boxed_transport = box_transport(transport);
            final_transport = match final_transport {
                Some(t) => Some(combine_transports(t, boxed_transport)),
                None => Some(boxed_transport),
            }
        }

        final_transport.expect("At least one transport must be enabled")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> TransportBuilder {
        // Assuming IdentityKP is a wrapper around the Keypair
        TransportBuilder::new(IdentityKP::new("ed25519"))
    }

    #[test]
    fn test_builder_configurability() {
        let builder = new_builder().with_tcp(false).with_ws(true).with_quic(true);

        assert!(!builder.enable_tcp);
        assert!(builder.enable_ws);
        assert!(builder.enable_quic);
    }

    #[test]
    #[should_panic(expected = "At least one transport must be enabled")]
    fn test_build_panics_if_empty() {
        new_builder()
            .with_tcp(false)
            .with_ws(false)
            .with_quic(false)
            .build();
    }

    #[tokio::test]
    async fn test_transport_builder() {
        new_builder().with_tcp(true).with_ws(true).build();
    }
}
