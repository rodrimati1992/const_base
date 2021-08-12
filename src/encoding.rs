/// Determines which encoding is used.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Encoding {
    /// The Base64 encoding.
    Base64(B64CharSet),
    /// The Base32 encoding.
    Base32(B32CharSet),
}

/// Determines which characters are used for the Base64 encoding
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum B64CharSet {
    /// Uses these characters:
    ///
    /// ```text
    /// ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/
    /// ```
    Standard,
    /// Uses these characters:
    ///
    /// ```text
    /// ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_
    /// ```
    UrlSafe,
}

/// Determines which characters are used for the Base32 encoding
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum B32CharSet {
    /// Uses these characters:
    ///
    /// ```text
    /// ABCDEFGHIJKLMNOPQRSTUVWXYZ234567
    /// ```
    Standard,
}

pub(crate) struct CharSetLookup<const CHARS: usize> {
    pub(crate) into_enc: [u8; CHARS],
    pub(crate) from_enc: [u8; 256],
}

macro_rules! declare_assoc_consts {
    (
        char_set = $char_set:ident,
        characters = $chars:expr,
        $(
            ($variant:ident, $assoc:ident, $value:expr)
        )*
    ) => {

        impl $char_set {
            pub(crate) const fn lookup(self) -> &'static CharSetLookup<$chars> {
                match self {
                    $(
                        Self::$variant => <CharSetLookup<$chars>>::$assoc,
                    )*
                }
            }
        }

        impl CharSetLookup<$chars> {
            $(
                pub(crate) const $assoc: &'static Self = &$value;
            )*
        }
    };
}

declare_assoc_consts! {
    char_set = B64CharSet,
    characters = 64,

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
        let mut out = Self::STANDARD.into_enc;

        out[62] = b'-';
        out[63] = b'_';

        Self::new(out)
    })
}

declare_assoc_consts! {
    char_set = B32CharSet,
    characters = 32,

    (Standard, STANDARD, {
        let mut out = [0u8; 32];
        let mut out_i = 0usize;

        for_range_inc!{c in b'A', b'Z' => write_into!{out, out_i, c} }
        for_range_inc!{c in b'2', b'7' => write_into!{out, out_i, c} }

        Self::new(out)
    })
}

pub(crate) const INVALID_ENC: u8 = u8::MAX;

impl<const N: usize> CharSetLookup<N> {
    const fn new(into_enc: [u8; N]) -> Self {
        let mut from_enc = [INVALID_ENC; 256];

        for_range! {i in 0usize..N =>
             from_enc[into_enc[i] as usize] = i as u8;
        }

        Self { from_enc, into_enc }
    }
}
