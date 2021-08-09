#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Encoding {
    Base64(B64CharSet),
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum B64CharSet {
    Standard,
    UrlSafe,
}

pub(crate) struct B64CharSetLookup {
    pub(crate) into_b64: [u8; 64],
    pub(crate) from_b64: [u8; 256],
}

macro_rules! declare_assoc_consts {
    ($(
        ($variant:ident, $assoc:ident, $value:expr)
    )*) => {

        impl B64CharSet {
            pub(crate) const fn lookup(self) -> &'static B64CharSetLookup {
                match self {
                    $(
                        Self::$variant => B64CharSetLookup::$assoc,
                    )*
                }
            }
        }

        impl B64CharSetLookup {
            $(
                pub(crate) const $assoc: &'static Self = &$value;
            )*
        }
    };
}

declare_assoc_consts! {
    (Standard, STANDARD, {
        let mut out = [0u8; 64];
        let mut out_i = 0usize;

        for_range_inc!{c in b'A', b'Z' => write_into!{out, out_i, c} }
        for_range_inc!{c in b'a', b'z' => write_into!{out, out_i, c} }
        for_range_inc!{c in b'0', b'9' => write_into!{out, out_i, c} }
        write_into!{out, out_i, b'+'}
        write_into!{out, out_i, b'/'}

        Self::new(out)
    })
    (UrlSafe, URL_SAFE, {
        let mut out = Self::STANDARD.into_b64;

        out[62] = b'-';
        out[63] = b'_';

        Self::new(out)
    })
}

pub(crate) const INVALID_B64: u8 = u8::MAX;

impl B64CharSetLookup {
    const fn new(into_b64: [u8; 64]) -> Self {
        let mut from_b64 = [INVALID_B64; 256];

        for_range! {i in 0usize..64 =>
             from_b64[into_b64[i] as usize] = i as u8;
        }

        Self { from_b64, into_b64 }
    }
}
