use crate::{Config, DecodeError, Encoding};

/// Computes the length of the encoded string from the `unencoded_length`,
/// using the encoding determined by `config`.
///
/// # Example
///
/// ### Base64
///
/// ```rust
/// use const_base::{Config, encoded_len};
///
/// const BASE64: usize = encoded_len(4, Config::B64);
/// assert_eq!(BASE64, 8);
///
/// // `.end_padding(false)` removes that trailing `=` that pads the string to
/// // a multiple of 4 bytes long.
/// const BASE64_UNPAD: usize = encoded_len(4, Config::B64.end_padding(false));
/// assert_eq!(BASE64_UNPAD, 6);
///
/// ```
///
/// ### Base32
///
/// ```rust
/// use const_base::{Config, encoded_len};
///
/// const BASE32: usize = encoded_len(3, Config::B32);
/// assert_eq!(BASE32, 8);
///
/// // `.end_padding(false)` removes that trailing `=` that pads the string to
/// // a multiple of 8 bytes long.
/// const BASE32_UNPAD: usize = encoded_len(3, Config::B32.end_padding(false));
/// assert_eq!(BASE32_UNPAD, 5);
///
/// ```
///
/// ### Hexadecimal
///
/// ```rust
/// use const_base::{Config, encoded_len};
///
/// const HEX_4: usize = encoded_len(4, Config::HEX);
/// const HEX_6: usize = encoded_len(6, Config::HEX);
/// assert_eq!(HEX_4, 8);
/// assert_eq!(HEX_6, 12);
/// ```
///
///
///
pub const fn encoded_len(unencoded_length: usize, config: Config) -> usize {
    match config.encoding {
        Encoding::Base64(_) => crate::base_64::encoded_len(unencoded_length, config),
        Encoding::Base32(_) => crate::base_32::encoded_len(unencoded_length, config),
        Encoding::Hex(_) => crate::base_16::encoded_len(unencoded_length, config),
    }
}

/// Encodes `input` into a `[u8; OUT]` with the encoding determined by `config`.
///
/// # Errors
///
/// This function returns a `WrongLength` error when
/// `OUT` doesn't equal `encoded_len(input.len(), config)`.
///
/// # Example
///
/// ### Base 64
///
/// ```rust
/// use const_base::{ArrayStr, Config, WrongLength, encode};
///
/// {
///     const ENCODED: &ArrayStr<16> = &WrongLength::unwrap(encode(b"hello worl", Config::B64));
///
///     assert_eq!(ENCODED, "aGVsbG8gd29ybA==");
/// }
/// {
///     const CFG: Config = Config::B64.end_padding(false);
///     const ENCODED: &ArrayStr<4> = &WrongLength::unwrap(encode(b"BYE", CFG));
///
///     assert_eq!(ENCODED, "QllF");
/// }
/// ```
///
/// ### Base 32
///
/// ```rust
/// use const_base::{ArrayStr, Config, WrongLength, encode};
///
/// {
///     const ENCODED: &ArrayStr<8> = &WrongLength::unwrap(encode(b"fox", Config::B32));
///
///     assert_eq!(ENCODED, "MZXXQ===");
/// }
/// {
///     const CFG: Config = Config::B32.end_padding(false);
///     const ENCODED: &ArrayStr<5> = &WrongLength::unwrap(encode(b"dog", CFG));
///
///     assert_eq!(ENCODED, "MRXWO");
/// }
/// ```
///
/// ### Hexadecimal
///
/// ```rust
/// use const_base::{ArrayStr, Config, WrongLength, encode};
///
/// {
///     const LOWER: &ArrayStr<8> = &WrongLength::unwrap(encode(b"bluh", Config::HEX_LOWER));
///
///     const UPPER: &ArrayStr<8> = &WrongLength::unwrap(encode(b"bluh", Config::HEX));
///
///     assert_eq!(LOWER, "626c7568");
///     assert_eq!(UPPER, "626C7568");
/// }
/// ```
///
pub const fn encode<const OUT: usize>(
    input: &[u8],
    config: Config,
) -> Result<crate::ArrayStr<OUT>, crate::WrongLength> {
    match config.encoding {
        Encoding::Base64(cset) => crate::base_64::encode(input, config, cset),
        Encoding::Base32(cset) => crate::base_32::encode(input, config, cset),
        Encoding::Hex(cset) => crate::base_16::encode(input, config, cset),
    }
}

#[doc(hidden)]
pub const fn __priv_encode<const OUT: usize>(input: &[u8], config: Config) -> crate::ArrayStr<OUT> {
    crate::errors::__unwrap_encode(encode(input, config))
}

/// Computes the length of the string obtained from decoding `encoded`
/// with the encoding determined by `config`.
///
/// # Example
///
/// ### Base 64
///
/// ```rust
/// use const_base::{Config, decoded_len};
///
/// const BASE64: &[usize] = &[
///     // this crate allows an arbitrary amount of trailing `=` in the decoded string.
///     decoded_len(b"fooooo=======", Config::B64),
///     decoded_len(b"foo=", Config::B64),
///     decoded_len(b"fo==", Config::B64),
/// ];
/// assert_eq!(BASE64, [4, 2, 1]);
///
/// ```
///
/// ### Base 32
///
/// ```rust
/// use const_base::{Config, decoded_len};
///
/// const BASE32: &[usize] = &[
///     // this crate allows an arbitrary amount of trailing `=` in the decoded string.
///     decoded_len(b"foooooo=======", Config::B32),
///     decoded_len(b"foooo=", Config::B32),
///     decoded_len(b"fooo=", Config::B32),
///     decoded_len(b"fo==", Config::B32),
/// ];
/// assert_eq!(BASE32, [4, 3, 2, 1]);
///
/// ```
///
/// ### Hexadecimal
///
/// ```rust
/// use const_base::{Config, decoded_len};
///
/// const BASE32: &[usize] = &[
///     decoded_len(b"F000B1E5", Config::HEX),
///     decoded_len(b"F000B1", Config::HEX),
///     decoded_len(b"F00B", Config::HEX),
///     decoded_len(b"F0", Config::HEX),
/// ];
/// assert_eq!(BASE32, [4, 3, 2, 1]);
///
/// ```
pub const fn decoded_len(encoded: &[u8], config: Config) -> usize {
    match config.encoding {
        Encoding::Base64(_) => crate::base_64::decoded_len(encoded, config),
        Encoding::Base32(_) => crate::base_32::decoded_len(encoded, config),
        Encoding::Hex(_) => crate::base_16::decoded_len(encoded, config),
    }
}

/// Decodes `input` into a `[u8; OUT]` with the encoding determined by `config`.
///
/// # Errors
///
/// This function returns these errors:
///
/// - [`DecodeError::InvalidByte`]:
/// When one of the bytes isn't in the char set for that encoding.
/// Eg: a `!` in an otherwise base 64 encoded string.
///
/// - [`DecodeError::WrongLength`]:
/// When `OUT` doesn't equal `decoded_len(input, config)`.
///
/// - [`DecodeError::InvalidInputLength`]:
/// When `input.len()` is not a valid length for that encoding.
/// For base 64 that is when `input.len() % 4` equals `1`.
///
/// # Example
///
/// ### Base 64
///
/// ```rust
/// use const_base::{Config, DecodeError, decode};
///
/// {
///     const OUT: [u8; 5] = DecodeError::unwrap(decode(b"cm9ja28=", Config::B64));
///
///     assert_eq!(OUT, *b"rocko");
/// }
/// {
///     const OUT: Result<[u8; 4], DecodeError> =
///          decode(b"bGlmZQ", Config::B64.end_padding(false));
///
///     assert_eq!(OUT, Ok(*b"life"));
/// }
/// {
///     const DECODED_A: Result<[u8; 4], DecodeError> = decode(b"bGl!ZQ", Config::B64);
///     const DECODED_B: Result<[u8; 8], DecodeError> = decode(b"AAAAAA", Config::B64);
///     const DECODED_C: Result<[u8; 6], DecodeError> = decode(b"AAAAA", Config::B64);
///     
///     assert!(matches!(DECODED_A, Err(DecodeError::InvalidByte(_))));
///     assert!(matches!(DECODED_B, Err(DecodeError::WrongLength(_))));
///     assert!(matches!(DECODED_C, Err(DecodeError::InvalidInputLength(_))));
/// }
///
/// ```
///
/// ### Base 32
///
/// ```rust
/// use const_base::{ArrayStr, Config, DecodeError, decode};
///
/// {
///     const OUT: [u8; 3] = DecodeError::unwrap(decode(b"MNQXI===", Config::B32));
///
///     assert_eq!(OUT, *b"cat");
/// }
/// {
///     const OUT: [u8; 3] =
///         DecodeError::unwrap(decode(b"MNQXI", Config::B32.end_padding(false)));
///
///     assert_eq!(OUT, *b"cat");
/// }
///
/// ```
///
/// ### Hexadecimal
///
/// ```rust
/// use const_base::{Config, DecodeError, decode};
///
/// const OUT: [u8; 4] = DecodeError::unwrap(decode(b"f09f918d", Config::HEX));
/// assert_eq!(OUT, "👍".as_bytes());
///
/// ```
///
pub const fn decode<const OUT: usize>(
    input: &[u8],
    config: Config,
) -> Result<[u8; OUT], DecodeError> {
    match config.encoding {
        Encoding::Base64(cset) => crate::base_64::decode(input, config, cset),
        Encoding::Base32(cset) => crate::base_32::decode(input, config, cset),
        Encoding::Hex(_) => crate::base_16::decode(input, config),
    }
}

#[doc(hidden)]
pub const fn __priv_decode<const OUT: usize>(input: &[u8], config: Config) -> __DecodeResult<OUT> {
    match decode(input, config) {
        Ok(array) => __DecodeResult { array, err: None },
        Err(err) => __DecodeResult {
            array: [0; OUT],
            err: Some(err),
        },
    }
}

// used to workaround error reporting for panicking constants,
// by panicking on a separate constant that's not used by anything
#[doc(hidden)]
pub struct __DecodeResult<const OUT: usize> {
    pub array: [u8; OUT],
    pub err: Option<DecodeError>,
}

impl<const OUT: usize> __DecodeResult<OUT> {
    #[track_caller]
    pub const fn assert_ok(&self) {
        if let Some(err) = &self.err {
            err.panic();
        }
    }
}

pub(crate) const fn encoded_len_bases(
    input_len: usize,
    config: Config,
    div: u64,
    chunk_size: usize,
) -> usize {
    let mult = input_len as u64 * 8;

    let div = crate::__priv_utils::div_ceil_u64(mult, div) as usize;

    if config.end_padding {
        crate::__priv_utils::round_up_to_multiple_usize(div, chunk_size)
    } else {
        div
    }
}

pub(crate) const fn decoded_len_bases(mut input: &[u8], config: Config, mult: u64) -> usize {
    if config.end_padding {
        while let [rem @ .., b'='] = input {
            input = rem;
        }
    }

    let mult = input.len() as u64 * mult;

    (mult / 8) as usize
}

macro_rules! encode_bases {
    ($input:ident, $config:ident, $char_set:ident, $encode_non_empty:expr) => {
        let mut out = [0u8; OUT];
        let mut out_i = 0usize;

        let lookup = $char_set.lookup();

        #[cfg(feature = "__test")]
        {
            let mut i = 0;
            while i < lookup.into_enc.len() {
                assert!(lookup.into_enc[i] < 128);
                i += 1;
            }
        }

        let output_len = encoded_len($input.len(), $config);

        if output_len != OUT {
            return Err(crate::WrongLength {
                expected: OUT,
                found: output_len,
            });
        }

        macro_rules! write_out {
            ($b:expr) => {
                write_into! {out, out_i, lookup.into_enc[$b as usize]}
            };
        }

        if !$input.is_empty() {
            $encode_non_empty
        }

        while out_i != OUT {
            write_into! {out, out_i, b'='}
        }

        unsafe {
            // SAFETY: all encodings from this crate produce ascii only
            // this unsafe code REQUIRES this macro not to be exported
            Ok(crate::ArrayStr::from_utf8_unchecked(out))
        }
    };
}
pub(crate) use encode_bases;

macro_rules! decode_bases {
    (
        dollar = $_:tt,
        $encoding_ctor:expr,
        $input:ident,
        $config:ident,
        $char_set:ident,
        $is_invalid_length:expr,
        |$in_i:ident| $decode_non_empty:expr
    ) => {
        use crate::encode_decode_shared::make_invalid_byte_err;
        use crate::{DecodeError, InvalidInputLength, WrongLength};

        let mut out = [0u8; OUT];
        let mut out_i = 0usize;
        let mut $in_i = 0;

        let from_enc = &$char_set.lookup().from_enc;

        let output_len = decoded_len($input, $config);

        if $config.end_padding {
            while let [rem @ .., b'='] = $input {
                $input = rem;
            }
        }

        if $is_invalid_length {
            return Err(DecodeError::InvalidInputLength(InvalidInputLength {
                length: $input.len(),
                enc: $config.encoding,
            }));
        } else if output_len != OUT {
            return Err(DecodeError::WrongLength(WrongLength {
                expected: OUT,
                found: output_len,
            }));
        }

        macro_rules! write_out {
            ($b:expr) => {
                write_into! {out, out_i, $b}
            };
        }

#[rustfmt::skip]
                macro_rules! from_encoded {
            ($_($new:ident = $old:ident),*) => (
                $_( let $new = from_enc[$old as usize]; )*
                if $_( $new == crate::encoding::INVALID_ENC )||* {
                    return Err(make_invalid_byte_err(
                        &[$_($new),*],
                        $input,
                        $in_i,
                        $encoding_ctor($char_set)
                    ));
                }
            )
        }

        if !$input.is_empty() {
            $decode_non_empty
        }

        Ok(out)
    };
}
pub(crate) use decode_bases;

pub(crate) const fn make_invalid_byte_err(
    arr: &[u8],
    input: &[u8],
    in_i: usize,
    encoding: crate::Encoding,
) -> DecodeError {
    let mut invalid_pos = !0;

    for_range! {i in 0..arr.len() =>
        if arr[i] == crate::encoding::INVALID_ENC {
            invalid_pos = i;
            break;
        }
    }

    let index = in_i + invalid_pos;
    let byte = input[invalid_pos];

    DecodeError::InvalidByte(crate::InvalidByte {
        index,
        byte,
        as_char: byte as char,
        encoding,
    })
}
