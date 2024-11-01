use core::str;

use crate::{
    bytes::Bytes,
    macros::{digits, sip_parse_error},
    parser::Result,
};

use crate::headers::SipHeader;

/// The minimum refresh interval supported for soft-state elements managed by that server.
pub struct MinExpires(u32);

impl<'a> SipHeader<'a> for MinExpires {
    const NAME: &'static str = "Min-Expires";

    fn parse(bytes: &mut Bytes<'a>) -> Result<Self> {
        let digits = digits!(bytes);
        match unsafe { str::from_utf8_unchecked(digits) }.parse() {
            Ok(expires) => Ok(MinExpires(expires)),
            Err(_) => return sip_parse_error!("invalid Min-Expires!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let src = b"60";
        let mut bytes = Bytes::new(src);
        let mime_version = MinExpires::parse(&mut bytes).unwrap();

        assert_eq!(mime_version.0, 60);
    }
}
