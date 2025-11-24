use std::io::{Error, ErrorKind};

use async_std::path::PathBuf;
use libp2p::{PeerId, identity};

pub struct IdentityKP(identity::Keypair);

impl IdentityKP {
    pub fn new(encr_type: &str) -> Self {
        let keypair = match encr_type {
            "ed25519" => identity::Keypair::generate_ed25519(),
            "secp256k1" => identity::Keypair::generate_secp256k1(),
            _ => panic!("Unsupported encryption type"),
        };
        IdentityKP(keypair)
    }

    pub fn get_kp(&self) -> identity::Keypair {
        self.0.clone()
    }

    pub fn get_pk(&self) -> identity::PublicKey {
        self.0.public()
    }

    pub fn peer_id(&self) -> PeerId {
        PeerId::from(self.get_pk())
    }

    pub async fn save_to_file(&self, path: PathBuf) -> std::io::Result<()> {
        let pem = self.0.to_protobuf_encoding().map_err(|e| {
            Error::new(
                ErrorKind::InvalidInput,
                format!("Failed to encode keypair: {}", e),
            )
        })?;
        async_std::fs::write(path, pem).await?;
        Ok(())
    }

    pub async fn load_from_file(path: PathBuf) -> std::io::Result<Self> {
        let pem = async_std::fs::read(path).await?;
        let keypair = identity::Keypair::from_protobuf_encoding(&pem).map_err(|e| {
            Error::new(
                ErrorKind::InvalidInput,
                format!("Failed to decode keypair: {}", e),
            )
        })?;
        Ok(IdentityKP(keypair))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_identity_kp_save_load() {
        let initial_identity_kp = IdentityKP::new("ed25519");
        println!("Initial Peer ID: {}", initial_identity_kp.peer_id());
        let path = PathBuf::from("test_keypair.pem");
        initial_identity_kp
            .save_to_file(path.clone())
            .await
            .expect("Failed to save keypair to file");
        let loaded_identity_kp = IdentityKP::load_from_file(path.clone())
            .await
            .expect("Failed to load keypair from file");
        println!("Loaded Peer ID: {}", loaded_identity_kp.peer_id());
        assert_eq!(initial_identity_kp.peer_id(), loaded_identity_kp.peer_id());
        async_std::fs::remove_file(path)
            .await
            .expect("Failed to delete test file");
    }
}
