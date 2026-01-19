use std::pin::Pin;

use libp2p::{
    futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, io},
    request_response::Codec,
};
use network_types::ProtocolName;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum P2pExecRequest {}

#[derive(Clone, Serialize, Deserialize)]
pub enum P2pExecResponse {}

#[derive(Clone)]
pub struct P2pExecReqResCodec;

impl Codec for P2pExecReqResCodec {
    #[doc = " The type of protocol(s) or protocol versions being negotiated."]
    type Protocol = ProtocolName;

    #[doc = " The type of inbound and outbound requests."]
    type Request = P2pExecRequest;

    #[doc = " The type of inbound and outbound responses."]
    type Response = P2pExecResponse;

    #[doc = " Reads a request from the given I/O stream according to the"]
    #[doc = " negotiated protocol."]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    async fn read_request<'life0, 'life1, 'life2, 'async_trait, T>(
        &'life0 mut self,
        protocol: &'life1 Self::Protocol,
        io: &'life2 mut T,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = io::Result<Self::Request>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        T: AsyncRead + Unpin + Send,
        T: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;
        serde_json::from_slice(&buf)
    }

    #[doc = " Reads a response from the given I/O stream according to the"]
    #[doc = " negotiated protocol."]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    async fn read_response<'life0, 'life1, 'life2, 'async_trait, T>(
        &'life0 mut self,
        protocol: &'life1 Self::Protocol,
        io: &'life2 mut T,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = io::Result<Self::Response>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        T: AsyncRead + Unpin + Send,
        T: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf);
        serde_json::from_slice(&buf)
    }

    #[doc = " Writes a request to the given I/O stream according to the"]
    #[doc = " negotiated protocol."]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    async fn write_request<'life0, 'life1, 'life2, 'async_trait, T>(
        &'life0 mut self,
        protocol: &'life1 Self::Protocol,
        io: &'life2 mut T,
        req: Self::Request,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = io::Result<()>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        T: AsyncWrite + Unpin + Send,
        T: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        let mut buf = serde_json::to_vec(&req).expect("unable to write buffer");
        io.write(&mut buf)
    }

    #[doc = " Writes a response to the given I/O stream according to the"]
    #[doc = " negotiated protocol."]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    async fn write_response<'life0, 'life1, 'life2, 'async_trait, T>(
        &'life0 mut self,
        protocol: &'life1 Self::Protocol,
        io: &'life2 mut T,
        res: Self::Response,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = io::Result<()>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        T: AsyncWrite + Unpin + Send,
        T: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        let mut buf = serde_json::to_vec(&req).expect("unable to write buffer");
        io.write(&mut buf)
    }
}
