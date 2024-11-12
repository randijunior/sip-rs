use crate::{
    bytes::Bytes, auth::credential::Credential, parser::Result,
};

use super::SipHeader;

/// The `Authorization` SIP header.
///
/// Contains authentication credentials of a `UA`.
pub struct Authorization<'a>(Credential<'a>);

impl<'a> Authorization<'a> {
    pub fn credential(&self) -> &Credential<'a> {
        &self.0
    }
}

impl<'a> SipHeader<'a> for Authorization<'a> {
    const NAME: &'static str = "Authorization";

    fn parse(bytes: &mut Bytes<'a>) -> Result<Authorization<'a>> {
        let credential = Credential::parse(bytes)?;

        Ok(Authorization(credential))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let src = b"Digest username=\"Alice\", realm=\"atlanta.com\", \
        nonce=\"84a4cc6f3082121f32b42a2187831a9e\",\
        response=\"7587245234b3434cc3412213e5f113a5432\"\r\n";
        let mut bytes = Bytes::new(src);
        let auth = Authorization::parse(&mut bytes).unwrap();

        assert_eq!(bytes.as_ref(), b"\r\n");
        let cred = auth.credential();

        assert_matches!(cred, Credential::Digest(digest) => {
            assert_eq!(digest.username, Some("Alice"));
            assert_eq!(digest.realm, Some("atlanta.com"));
            assert_eq!(
                digest.nonce,
                Some("84a4cc6f3082121f32b42a2187831a9e")
            );
            assert_eq!(
                digest.response,
                Some("7587245234b3434cc3412213e5f113a5432")
            );
        });
    }
}