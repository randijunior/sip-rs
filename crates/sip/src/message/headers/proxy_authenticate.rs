use crate::{
    bytes::Bytes, message::auth::challenge::Challenge, parser::Result,
};

use crate::headers::SipHeaderParser;

/// The authentication requirements from a proxy server to a client.
pub struct ProxyAuthenticate<'a>(Challenge<'a>);

impl<'a> SipHeaderParser<'a> for ProxyAuthenticate<'a> {
    const NAME: &'static str = "Proxy-Authenticate";

    fn parse(bytes: &mut Bytes<'a>) -> Result<Self> {
        let challenge = Challenge::parse(bytes)?;

        Ok(ProxyAuthenticate(challenge))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let src = b"Digest realm=\"atlanta.com\", \
        domain=\"sip:ss1.carrier.com\", qop=\"auth\", \
        nonce=\"f84f1cec41e6cbe5aea9c8e88d359\", \
        opaque=\"\", stale=FALSE, algorithm=MD5\r\n";
        let mut bytes = Bytes::new(src);
        let proxy_auth = ProxyAuthenticate::parse(&mut bytes).unwrap();

        match proxy_auth.0 {
            Challenge::Digest(digest) => {
                assert_eq!(digest.realm, Some("atlanta.com"));
                assert_eq!(digest.algorithm, Some("MD5"));
                assert_eq!(digest.domain, Some("sip:ss1.carrier.com"));
                assert_eq!(digest.qop, Some("auth"));
                assert_eq!(digest.nonce, Some("f84f1cec41e6cbe5aea9c8e88d359"));
                assert_eq!(digest.opaque, Some(""));
                assert_eq!(digest.stale, Some("FALSE"));
            }
            _ => unreachable!(),
        }
    }
}
