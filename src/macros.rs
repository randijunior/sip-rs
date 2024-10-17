macro_rules! space {
    ($scanner:ident) => {{
        $scanner.read_while(crate::util::is_space);
    }};
}

macro_rules! digits {
    ($scanner:ident) => {{
        let range = $scanner.read_while(crate::util::is_digit);

        unsafe { $scanner.src.get_unchecked(range) }
    }};
}

macro_rules! read_while {
    ($scanner:expr, $func:expr) => {{
        let range = $scanner.read_while($func);

        unsafe { $scanner.src.get_unchecked(range) }
    }};
}

macro_rules! read_until_byte {
    ($scanner:expr, $byte:expr) => {{
        let range = $scanner.read_while(|b| b != $byte);

        unsafe { $scanner.src.get_unchecked(range) }
    }};
}

macro_rules! find {
    ($scanner:expr, $tag:expr) => {{
        let range = $scanner.read_tag($tag)?;

        unsafe { $scanner.src.get_unchecked(range) }
    }};
}

macro_rules! remaing {
    ($scanner:ident) => {{
        let range = $scanner.read_n($scanner.len() - $scanner.idx())?;

        unsafe { $scanner.src.get_unchecked(range) }
    }};
}

macro_rules! until_newline {
    ($scanner:ident) => {{
        let range = $scanner.read_while(|b| !crate::util::is_newline(b));

        unsafe { $scanner.src.get_unchecked(range) }
    }};
}

macro_rules! peek_while {
    ($scanner:expr, $func:expr) => {{
        let range = $scanner.peek_while($func);

        (unsafe { $scanner.src.get_unchecked(range) })
    }};
}

macro_rules! newline {
    ($scanner:ident) => {{
        $scanner.read_while(crate::util::is_newline);
    }};
}

macro_rules! alpha {
    ($scanner:ident) => {{
        let range = $scanner.read_while(crate::util::is_alphabetic);

        unsafe { $scanner.src.get_unchecked(range) }
    }};
}

macro_rules! b_map {
    ($name:ident => $( $slice:expr ),+) => {
        const $name: [bool; 256] = {
            let mut arr = [false; 256];
            $(
                let slice = $slice;
                let mut i = 0;
                while i < slice.len() {
                    arr[slice[i] as usize] = true;
                    i += 1;
                }
            )*
            arr
        };
    };
}

macro_rules! parse_param {
    ($scanner:ident, $func:expr) => {{
        crate::macros::space!($scanner);
        if let Some(&b';') = $scanner.peek() {
            let mut params = crate::uri::Params::new();
            while let Some(&b';') = $scanner.peek() {
                let param = crate::headers::parse_generic_param($scanner)?;
                if let Some(param) = $func(param) {
                    params.set(param.0, param.1);
                }
            }
            if params.is_empty() {
                None
            } else {
                Some(params)
            }
        } else {
            None
        }
    }};
}

macro_rules! parse_auth_param {
    ($scanner: expr) => {{
        if $scanner.peek() == Some(&b'=') {
            $scanner.next();
            match $scanner.peek() {
                Some(b'"') => {
                    $scanner.next();
                    let value = crate::macros::read_until_byte!($scanner, b'"');
                    $scanner.next();
                    Some((std::str::from_utf8(value)?))
                }
                Some(_) => {
                    let value = read_while!($scanner, is_token);
                    Some(unsafe { std::str::from_utf8_unchecked(value) })
                }
                None => None,
            }
        } else {
            None
        }
    }};
}

macro_rules! sip_parse_error {
    ($message:expr) => {{
        Err(crate::parser::SipParserError::from($message))
    }};
}

pub(crate) use alpha;
pub(crate) use b_map;
pub(crate) use digits;
pub(crate) use find;
pub(crate) use newline;
pub(crate) use parse_auth_param;
pub(crate) use parse_param;
pub(crate) use peek_while;
pub(crate) use read_until_byte;
pub(crate) use read_while;
pub(crate) use sip_parse_error;
pub(crate) use space;
pub(crate) use until_newline;
pub(crate) use remaing;
