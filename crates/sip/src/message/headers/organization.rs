use core::str;

use crate::{bytes::Bytes, macros::until_newline, parser::Result};

use crate::headers::SipHeader;

/// The name of the organization to which the SIP
/// element issuing the request or response belongs.
pub struct Organization<'a>(&'a str);

impl<'a> SipHeader<'a> for Organization<'a> {
    const NAME: &'static str = "Organization";

    fn parse(bytes: &mut Bytes<'a>) -> Result<Self> {
        let organization = until_newline!(bytes);
        let organization = str::from_utf8(organization)?;

        Ok(Organization(organization))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let src = b"Boxes by Bob\r\n";
        let mut bytes = Bytes::new(src);
        let org = Organization::parse(&mut bytes).unwrap();

        assert_eq!(org.0, "Boxes by Bob");
    }
}
