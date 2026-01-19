//! Substream protocol name defination

use std::borrow::Cow;

/// Protocol name NewType
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolName(Cow<'static, str>);

impl ProtocolName {
    pub const fn from_static(name: &'static str) -> ProtocolName {
        ProtocolName(Cow::Borrowed(name))
    }
}

impl AsRef<str> for ProtocolName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<&'static str> for ProtocolName {
    fn from(value: &'static str) -> Self {
        ProtocolName::from_static(value)
    }
}

#[cfg(test)]
mod test {
    use crate::protocol::ProtocolName;

    #[test]
    fn protocol_name() {
        let p = "/exec/req/1";
        let req_protocol: ProtocolName = p.into();
        assert_eq!(req_protocol.as_ref(), p);
    }
}
