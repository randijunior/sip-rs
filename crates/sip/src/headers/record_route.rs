use crate::{
    bytes::Bytes,
    macros::{parse_header_param, sip_parse_error},
    parser::Result,
    uri::{NameAddr, Params, SipUri},
};

use crate::headers::SipHeader;

/// The `Record-Route` SIP header.
///
/// Keeps proxies in the signaling path for consistent routing and session control.
pub struct RecordRoute<'a> {
    addr: NameAddr<'a>,
    param: Option<Params<'a>>,
}

impl<'a> SipHeader<'a> for RecordRoute<'a> {
    const NAME: &'static str = "Record-Route";

    fn parse(bytes: &mut Bytes<'a>) -> Result<Self> {
        let addr = NameAddr::parse(bytes)?;
        let param = parse_header_param!(bytes);
        Ok(RecordRoute { addr, param })
    }
}

#[cfg(test)]
mod tests {

    use crate::uri::{HostPort, Scheme};

    use super::*;

    #[test]
    fn test_parse() {
        let src = b"<sip:server10.biloxi.com;lr>\r\n";
        let mut bytes = Bytes::new(src);
        let rr = RecordRoute::parse(&mut bytes);
        let rr = rr.unwrap();

        assert_eq!(rr.addr.display, None);
        assert_eq!(rr.addr.uri.scheme, Scheme::Sip);
        assert_eq!(
            rr.addr.uri.host,
            HostPort::DomainName {
                host: "server10.biloxi.com",
                port: None
            }
        );
        assert!(rr.addr.uri.params.is_some());

        let src = b"<sip:bigbox3.site3.atlanta.com;lr>;foo=bar\r\n";
        let mut bytes = Bytes::new(src);
        let rr = RecordRoute::parse(&mut bytes);
        let rr = rr.unwrap();

        assert_eq!(rr.addr.display, None);
        assert_eq!(rr.addr.uri.scheme, Scheme::Sip);
        assert_eq!(
            rr.addr.uri.host,
            HostPort::DomainName {
                host: "bigbox3.site3.atlanta.com",
                port: None
            }
        );
        assert_eq!(rr.param.unwrap().get("foo"), Some(&"bar"));
    }
}