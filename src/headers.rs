use core::{call_id::CallId, cseq::CSeq, max_fowards::MaxForwards, to::To};
use std::str;

pub mod auth;
pub mod capability;
pub mod control;
pub mod core;
pub mod info;
pub mod routing;
pub mod session;

use auth::{
    authentication_info::AuthenticationInfo,
    authorization::{Authorization, Credential, DigestCredential},
    proxy_authenticate::{Challenge, DigestChallenge, ProxyAuthenticate},
    proxy_authorization::ProxyAuthorization,
    www_authenticate::WWWAuthenticate,
};
use capability::{
    accept_encoding::AcceptEncoding, accept_language::AcceptLanguage, proxy_require::ProxyRequire,
    require::Require, supported::Supported, unsupported::Unsupported,
};
use control::{
    allow::Allow, expires::Expires, min_expires::MinExpires, reply_to::ReplyTo,
    retry_after::RetryAfter, timestamp::Timestamp,
};
use info::{
    alert_info::AlertInfo, call_info::CallInfo, date::Date, error_info::ErrorInfo,
    in_reply_to::InReplyTo, organization::Organization, priority::Priority, server::Server,
    subject::Subject, user_agent::UserAgent, warning::Warning,
};
use routing::{contact::Contact, record_route::RecordRoute, route::Route, via::Via};
use session::{
    accept::Accept, content_disposition::ContentDisposition, content_encoding::ContentEncoding,
    content_language::ContentLanguage, content_length::ContentLength, content_type::ContentType,
    mime_version::MimeVersion,
};

use core::from::From;

use crate::{
    macros::{parse_auth_param, read_until_byte, read_while, sip_parse_error, space},
    parser::{is_token, Result},
    scanner::Scanner,
    uri::Params,
};

// Headers, as defined in RFC3261.
pub enum Header<'a> {
    Accept(Accept<'a>),
    AcceptEncoding(AcceptEncoding<'a>),
    AcceptLanguage(AcceptLanguage<'a>),
    AlertInfo(AlertInfo<'a>),
    Allow(Allow<'a>),
    AuthenticationInfo(AuthenticationInfo<'a>),
    Authorization(Authorization<'a>),
    CallId(CallId<'a>),
    CallInfo(CallInfo<'a>),
    Contact(Contact<'a>),
    ContentDisposition(ContentDisposition<'a>),
    ContentEncoding(ContentEncoding<'a>),
    ContentLanguage(ContentLanguage<'a>),
    ContentLength(ContentLength),
    ContentType(ContentType<'a>),
    CSeq(CSeq<'a>),
    Date(Date<'a>),
    ErrorInfo(ErrorInfo<'a>),
    Expires(Expires),
    From(From<'a>),
    InReplyTo(InReplyTo<'a>),
    MaxForwards(MaxForwards),
    MimeVersion(MimeVersion),
    MinExpires(MinExpires),
    Organization(Organization<'a>),
    Priority(Priority<'a>),
    ProxyAuthenticate(ProxyAuthenticate<'a>),
    ProxyAuthorization(ProxyAuthorization<'a>),
    ProxyRequire(ProxyRequire<'a>),
    RecordRoute(RecordRoute<'a>),
    ReplyTo(ReplyTo<'a>),
    Require(Require<'a>),
    RetryAfter(RetryAfter<'a>),
    Route(Route<'a>),
    Server(Server<'a>),
    Subject(Subject<'a>),
    Supported(Supported<'a>),
    Timestamp(Timestamp<'a>),
    To(To<'a>),
    Unsupported(Unsupported<'a>),
    UserAgent(UserAgent<'a>),
    Via(Via<'a>),
    Warning(Warning<'a>),
    WWWAuthenticate(WWWAuthenticate<'a>),
    Other { name: &'a str, value: &'a str },
}

pub struct SipHeaders<'a> {
    pub(crate) hdrs: Vec<Header<'a>>,
}

impl<'a> SipHeaders<'a> {
    pub fn new() -> Self {
        Self { hdrs: Vec::new() }
    }
    pub fn push_header(&mut self, hdr: Header<'a>) {
        self.hdrs.push(hdr);
    }
}

pub(crate) fn parse_generic_param<'a>(
    scanner: &mut Scanner<'a>,
) -> Result<(&'a str, Option<&'a str>)> {
    // take ';' character
    scanner.next();
    space!(scanner);

    let name = read_while!(scanner, is_token);
    let name = unsafe { str::from_utf8_unchecked(name) };
    let value = if scanner.peek() == Some(&b'=') {
        scanner.next();
        let value = read_while!(scanner, is_token);
        Some(unsafe { str::from_utf8_unchecked(value) })
    } else {
        None
    };

    Ok((name, value))
}

pub(crate) trait SipHeaderParser<'a>: Sized {
    const NAME: &'static [u8];
    const SHORT_NAME: Option<&'static [u8]> = None;

    fn parse(scanner: &mut Scanner<'a>) -> Result<Self>;

    #[inline]
    fn match_name(name: &[u8]) -> bool {
        name.eq_ignore_ascii_case(Self::NAME)
            || Self::SHORT_NAME.is_some_and(|s_name| name == s_name)
    }

    fn parse_q_value(param: Option<&str>) -> Option<f32> {
        if let Some(q_param) = param {
            if let Ok(value) = q_param.parse::<f32>() {
                if (0.0..=1.0).contains(&value) {
                    return Some(value);
                }
            }
            return None;
        }
        None
    }

    fn parse_auth_credential(scanner: &mut Scanner<'a>) -> Result<Credential<'a>> {
        let scheme = match scanner.peek() {
            Some(b'"') => {
                scanner.next();
                let value = read_until_byte!(scanner, b'"');
                scanner.next();
                value
            }
            Some(_) => {
                read_while!(scanner, is_token)
            }
            None => return sip_parse_error!("eof!"),
        };

        match scheme {
            b"Digest" => Ok(Credential::Digest(DigestCredential::parse(scanner)?)),
            other => {
                space!(scanner);
                let other = std::str::from_utf8(other)?;
                let name = read_while!(scanner, is_token);
                let name = unsafe { std::str::from_utf8_unchecked(name) };
                let val = parse_auth_param!(scanner);
                let mut params = Params::new();
                params.set(name, val);

                while let Some(b',') = scanner.peek() {
                    space!(scanner);
                    let name = read_while!(scanner, is_token);
                    let name = unsafe { std::str::from_utf8_unchecked(name) };
                    let val = parse_auth_param!(scanner);
                    params.set(name, val);
                }

                Ok(Credential::Other {
                    scheme: other,
                    param: params,
                })
            }
        }
    }

    fn parse_auth_challenge(scanner: &mut Scanner<'a>) -> Result<Challenge<'a>> {
        let scheme = match scanner.peek() {
            Some(b'"') => {
                scanner.next();
                let value = read_until_byte!(scanner, b'"');
                scanner.next();
                value
            }
            Some(_) => {
                read_while!(scanner, is_token)
            }
            None => return sip_parse_error!("eof!"),
        };

        match scheme {
            b"Digest" => Ok(Challenge::Digest(DigestChallenge::parse(scanner)?)),
            other => {
                space!(scanner);
                let other = std::str::from_utf8(other)?;
                let name = read_while!(scanner, is_token);
                let name = unsafe { std::str::from_utf8_unchecked(name) };
                let val = parse_auth_param!(scanner);
                let mut params = Params::new();
                params.set(name, val);

                while let Some(b',') = scanner.peek() {
                    space!(scanner);
                    let name = read_while!(scanner, is_token);
                    let name = unsafe { std::str::from_utf8_unchecked(name) };
                    let val = parse_auth_param!(scanner);
                    params.set(name, val);
                }

                Ok(Challenge::Other {
                    scheme: other,
                    param: params,
                })
            }
        }
    }
}