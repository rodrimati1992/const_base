//! All the errors from this crate.

use crate::Encoding;

/// Error returned by [`decode`](crate::decode())
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum DecodeError {
    /// When one of the bytes isn't in the char set for that encoding.
    ///
    /// Eg: a `!` in an otherwise base 64 encoded string.
    InvalidByte(InvalidByte),

    /// When the array returned by [`decode`] isn't the same length as what the arguments
    /// passed to [`decode`] would produce.
    ///
    /// [`decode`]: crate::decode()
    MismatchedOutputLength(MismatchedOutputLength),
    /// When the slice passed to [`decode`] is not a valid length for that encoding.
    ///
    /// For base 64 that is when `input.len() % 4` equals `1`.
    InvalidInputLength(InvalidInputLength),
}

impl DecodeError {
    #[track_caller]
    pub const fn panic(self) -> ! {
        match self {
            DecodeError::InvalidByte(x) => x.panic(),
            DecodeError::MismatchedOutputLength(x) => x.panic(),
            DecodeError::InvalidInputLength(x) => x.panic(),
        }
    }
}

/// When one of the bytes isn't in the char set for that encoding.
///
/// Eg: a `!` in an otherwise base 64 encoded string.
///
/// # Example
///
/// ### Base 64
///
/// ```rust
/// use const_base::{Config, DecodeError, InvalidByte, decode};
///
/// const DECODED: Result<[u8; 4], DecodeError> = decode(b"bGl!ZQ", Config::B64);
///
/// assert!(matches!(DECODED, Err(DecodeError::InvalidByte(InvalidByte{..}))));
///
/// ```
#[derive(Debug, PartialEq)]
pub struct InvalidByte {
    pub(crate) index: usize,
    pub(crate) byte: u8,
    pub(crate) as_char: char,
    pub(crate) encoding: Encoding,
}

impl InvalidByte {
    pub const fn index(&self) -> usize {
        self.index
    }
    pub const fn byte(&self) -> u8 {
        self.byte
    }
    pub const fn byte_as_char(&self) -> char {
        self.as_char
    }
    pub const fn encoding(&self) -> Encoding {
        self.encoding
    }

    #[track_caller]
    pub const fn panic(&self) -> ! {
        use const_panic::{FmtArg, PanicVal};

        crate::utils::cpanic(&[
            PanicVal::write_str("invalid byte ("),
            PanicVal::from_u8(self.byte, FmtArg::DEBUG),
            PanicVal::write_str(", the "),
            PanicVal::from_char(self.as_char, FmtArg::DEBUG),
            PanicVal::write_str(" character) for the "),
            PanicVal::write_str(self.encoding.full_name()),
            PanicVal::write_str(" encoding at offset "),
            PanicVal::from_usize(self.index, FmtArg::DEBUG),
        ])
    }
}

/// When the array returned by [`decode`] isn't the same length as what the arguments
/// passed to [`decode`] would produce.
///
///
/// # Example
///
/// ### Base 64
///
/// ```rust
/// use const_base::{Config, DecodeError, MismatchedOutputLength, decode};
///
/// const DECODED: Result<[u8; 8], DecodeError> = decode(b"AAAAAA", Config::B64);
/// assert!(matches!(
///     DECODED,
///     Err(DecodeError::MismatchedOutputLength(MismatchedOutputLength{..}))
/// ));
///
/// ```
///
/// [`decode`]: crate::decode()
#[derive(Debug, PartialEq)]
pub struct MismatchedOutputLength {
    pub(crate) expected: usize,
    pub(crate) found: usize,
}

impl MismatchedOutputLength {
    pub const fn expected(&self) -> usize {
        self.expected
    }
    pub const fn found(&self) -> usize {
        self.found
    }
    #[track_caller]
    pub const fn panic(&self) -> ! {
        use const_panic::{FmtArg, PanicVal};

        crate::utils::cpanic(&[
            PanicVal::write_str("expected output length to be "),
            PanicVal::from_usize(self.expected, FmtArg::DEBUG),
            PanicVal::write_str(" but it is "),
            PanicVal::from_usize(self.found, FmtArg::DEBUG),
        ])
    }
}

/// When the slice passed to [`decode`] is not a valid length for that encoding.
///
/// For base 64 that is when `input.len() % 4` equals `1`.
///
/// # Example
///
/// ### Base 64
///
/// ```rust
/// use const_base::{Config, DecodeError, InvalidInputLength, decode};
///
/// const DECODED: Result<[u8; 8], DecodeError> = decode(b"AAAAA", Config::B64);
/// assert!(matches!(
///     DECODED,
///     Err(DecodeError::InvalidInputLength(InvalidInputLength{..}))
/// ));
///
/// ```
///
/// [`decode`]: crate::decode()
#[derive(Debug, PartialEq)]
pub struct InvalidInputLength {
    pub(crate) length: usize,
    pub(crate) enc: Encoding,
}

impl InvalidInputLength {
    pub const fn length(&self) -> usize {
        self.length
    }

    #[track_caller]
    pub const fn panic(&self) -> ! {
        use const_panic::{FmtArg, PanicVal};

        crate::utils::cpanic(&[
            PanicVal::write_str("invalid input length for "),
            PanicVal::write_str(self.enc.name()),
            PanicVal::write_str(": "),
            PanicVal::from_usize(self.length, FmtArg::DEBUG),
        ])
    }
}

#[doc(hidden)]
#[track_caller]
pub const fn __unwrap_decode<const N: usize>(res: Result<[u8; N], DecodeError>) -> [u8; N] {
    match res {
        Ok(x) => x,
        Err(e) => e.panic(),
    }
}

#[doc(hidden)]
#[track_caller]
pub const fn __unwrap_encode<const N: usize>(
    res: Result<[u8; N], MismatchedOutputLength>,
) -> [u8; N] {
    match res {
        Ok(x) => x,
        Err(e) => e.panic(),
    }
}
