pub mod to;
pub mod via;

use to::To;

pub struct SipHeaders<'a> {
    pub(crate) hdrs: Vec<Header<'a>>,
}


pub enum Header<'a> {
    Accept,
    AcceptEncoding,
    AcceptLanguage,
    AlertInfo,
    Allow,
    AuthenticationInfo,
    Authorization,
    CallID,
    CallInfo,
    Contact,
    ContentDisposition,
    ContentEncoding,
    ContentLanguage,
    ContentLength,
    ContentType,
    CSeq,
    Date,
    ErrorInfo,
    Expires,
    From,
    InReplyTo,
    MaxForwards,
    MimeVersion,
    MinExpires,
    Organization,
    Priority,
    ProxyAuthenticate,
    ProxyAuthorization,
    ProxyRequire,
    RecordRoute,
    ReplyTo,
    Require,
    RetryAfter,
    Route,
    Server,
    Subject,
    Supported,
    Timestamp,
    To(To<'a>),
    Unsupported,
    UserAgent,
    Via,
    Warning,
    WWWAuthenticate,
    Other
}
