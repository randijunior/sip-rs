use core::str;

use crate::token::is_token;
use crate::{bytes::Bytes, macros::read_while, parser::Result};

use crate::headers::SipHeader;

/// Indicates the urgency of the request as perceived by the client.
pub struct Priority<'a>(&'a str);

impl<'a> SipHeader<'a> for Priority<'a> {
    const NAME: &'static str = "Priority";

    fn parse(bytes: &mut Bytes<'a>) -> Result<Self> {
        let priority = read_while!(bytes, is_token);
        let priority = unsafe { str::from_utf8_unchecked(priority) };

        Ok(Priority(priority))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let src = b"emergency\r\n";
        let mut bytes = Bytes::new(src);
        let priority = Priority::parse(&mut bytes).unwrap();

        assert_eq!(priority.0, "emergency");
    }
}
