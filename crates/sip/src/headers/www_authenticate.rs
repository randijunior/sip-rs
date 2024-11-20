use scanner::Scanner;

use crate::{
    auth::Challenge, headers::SipHeader, parser::Result
};

/// The `WWW-Authenticate` SIP header.
///
/// Consists of at least one challenge the
/// authentication scheme(s) and parameters applicable
/// to the `Request-URI`.
pub struct WWWAuthenticate<'a>(Challenge<'a>);

impl<'a> SipHeader<'a> for WWWAuthenticate<'a> {
    const NAME: &'static str = "WWW-Authenticate";

    fn parse(scanner: &mut Scanner<'a>) -> Result<Self> {
        let challenge = Challenge::parse(scanner)?;

        Ok(WWWAuthenticate(challenge))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let src = b"Digest realm=\"atlanta.com\",\
        domain=\"sip:boxesbybob.com\", qop=\"auth\",\
        nonce=\"f84f1cec41e6cbe5aea9c8e88d359\",\
        opaque=\"\", stale=FALSE, algorithm=MD5";
        let mut scanner = Scanner::new(src);
        let www_auth = WWWAuthenticate::parse(&mut scanner);
        let www_auth = www_auth.unwrap();

        assert_matches!(www_auth.0, Challenge::Digest { realm, domain, nonce, opaque, stale, algorithm, qop, .. } => {
            assert_eq!(realm, Some("atlanta.com"));
            assert_eq!(algorithm, Some("MD5"));
            assert_eq!(domain, Some("sip:boxesbybob.com"));
            assert_eq!(qop, Some("auth"));
            assert_eq!(nonce, Some("f84f1cec41e6cbe5aea9c8e88d359"));
            assert_eq!(opaque, Some(""));
            assert_eq!(stale, Some("FALSE"));
        });
    }
}
