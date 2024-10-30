use core::str;

use crate::{
    bytes::Bytes,
    macros::space,
    parser::{self, Result},
};

use crate::headers::SipHeaderParser;

/// Enumerates all the extensions supported by the `UAC` or `UAS`.
pub struct Supported<'a>(Vec<&'a str>);

impl<'a> SipHeaderParser<'a> for Supported<'a> {
    const NAME: &'static str = "Supported";
    const SHORT_NAME: Option<&'static str> = Some("k");

    fn parse(bytes: &mut Bytes<'a>) -> Result<Self> {
        let tag = parser::parse_token(bytes);
        let mut tags = vec![tag];

        while let Some(b',') = bytes.peek() {
            bytes.next();
            space!(bytes);
            let tag = parser::parse_token(bytes);
            tags.push(tag);
            space!(bytes);
        }

        Ok(Supported(tags))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let src = b"100rel, other\r\n";
        let mut bytes = Bytes::new(src);
        let supported = Supported::parse(&mut bytes);
        let supported = supported.unwrap();

        assert_eq!(bytes.as_ref(), b"\r\n");
        assert_eq!(supported.0.get(0), Some(&"100rel"));
        assert_eq!(supported.0.get(1), Some(&"other"));
    }
}
