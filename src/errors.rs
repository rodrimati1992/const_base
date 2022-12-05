//! All the errors from this crate.

use crate::Encoding;

/// Error returned by [`decode`](crate::decode())
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum DecodeError {
    /// When one of the bytes in the slice passed to [`decode`]
    /// isn't in the char set of the passed encoding.
    ///
    /// Eg: a `!` in an otherwise base 64 encoded string.
    ///
    /// [`decode`]: crate::decode()
    InvalidByte(InvalidByte),

    /// When the array returned by [`decode`]
    /// isn't the length that the arguments would produce.
    ///
    /// [`decode`]: crate::decode()
    WrongOutputLength(WrongOutputLength),
    /// When the slice passed to [`decode`] is not a valid length for that encoding.
    ///
    #[doc = wrong_lengths_doc!()]
    WrongInputLength(WrongInputLength),
    /// When the last byte in the slice passed to [`decode`]
    /// has excess set bits that aren't copied to the return value.
    ExcessBits(ExcessBits),
}

macro_rules! define_unwrap_self {
    () => {
        /// Unwraps a Result with this type as the error.
        pub const fn unwrap<T: Copy>(res: Result<T, Self>) -> T {
            match res {
                Ok(x) => x,
                Err(e) => e.panic(),
            }
        }
    };
}

impl DecodeError {
    define_unwrap_self! {}

    /// Panics with this error as the message.
    #[track_caller]
    pub const fn panic(&self) -> ! {
        match self {
            DecodeError::InvalidByte(x) => x.panic(),
            DecodeError::WrongOutputLength(x) => x.panic(),
            DecodeError::WrongInputLength(x) => x.panic(),
            DecodeError::ExcessBits(x) => x.panic(),
        }
    }
}

/// When one of the bytes in the slice passed to [`decode`]
/// isn't in the char set of the passed encoding.
///
/// Eg: a `!` in an otherwise base 64 encoded string.
///
/// [`decode`]: crate::decode()
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

    define_unwrap_self! {}

    /// Panics with this error as the message.
    #[track_caller]
    pub const fn panic(&self) -> ! {
        use const_panic::{FmtArg, PanicVal};

        crate::utils::cpanic(&[
            PanicVal::write_str("invalid byte ("),
            PanicVal::from_u8(self.byte, FmtArg::DEBUG),
            PanicVal::write_str("_u8, the "),
            PanicVal::from_char(self.as_char, FmtArg::DEBUG),
            PanicVal::write_str(" character) for the "),
            PanicVal::write_str(self.encoding.full_name()),
            PanicVal::write_str(" encoding at offset "),
            PanicVal::from_usize(self.index, FmtArg::DEBUG),
        ])
    }
}

/// When the array returned by [`decode`] or [`encode`] isn't the
/// length that the arguments would produce.
///
/// [`decode`]: crate::decode()
/// [`encode`]: crate::encode()
///
/// # Example
///
/// ### Base 64
///
/// ```rust
/// use const_base::{Config, DecodeError, WrongOutputLength, decode};
///
/// const DECODED: Result<[u8; 8], DecodeError> = decode(b"AAAAAA", Config::B64);
///
/// match DECODED {
///     Err(DecodeError::WrongOutputLength(err)) => {
///         assert_eq!(err.expected(), 4);
///         assert_eq!(err.found(), 8);
///     }
///     _ => unreachable!()
/// }
///
/// ```
///
/// [`decode`]: crate::decode()
#[derive(Debug, PartialEq)]
pub struct WrongOutputLength {
    pub(crate) expected: usize,
    pub(crate) found: usize,
}

impl WrongOutputLength {
    pub const fn expected(&self) -> usize {
        self.expected
    }
    pub const fn found(&self) -> usize {
        self.found
    }

    define_unwrap_self! {}

    /// Panics with this error as the message.
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

macro_rules! wrong_lengths_doc {
    () => {
        "The input lengths that are wrong for each encoding:\n\
        - Base 64: when `input.len() % 4` equals `1`.\n\
        - Base 32: when `input.len() % 8` equals `1`, `3` , or `6`.\n\
        - Base 16: when `input.len() % 2` equals `1`.\n\
        "
    };
}
use wrong_lengths_doc;

/// When the slice passed to [`decode`] is not a valid length for the passed encoding.
///
#[doc = wrong_lengths_doc!()]
///
/// [`decode`]: crate::decode()
///
/// # Example
///
/// ### Base 64
///
/// ```rust
/// use const_base::{Config, DecodeError, Encoding, WrongInputLength, decode};
///
/// const DECODED: Result<[u8; 8], DecodeError> = decode(b"AAAAA", Config::B64);
///
/// match DECODED {
///     Err(DecodeError::WrongInputLength(err)) => {
///         assert_eq!(err.length(), 5);
///         assert!(matches!(err.encoding(), Encoding::Base64{..}));
///     }
///     _ => unreachable!()
/// }
///
/// ```
///
/// [`decode`]: crate::decode()
#[derive(Debug, PartialEq)]
pub struct WrongInputLength {
    pub(crate) length: usize,
    pub(crate) enc: Encoding,
}

impl WrongInputLength {
    /// The length of the slice argument
    pub const fn length(&self) -> usize {
        self.length
    }

    /// The encoding that was attempted to decode from.
    pub const fn encoding(&self) -> Encoding {
        self.enc
    }

    define_unwrap_self! {}

    /// Panics with this error as the message.
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

/// When the last byte in the slice passed to [`decode`]
/// has excess set bits that aren't copied to the return value.
///
/// # Example
///
/// ```rust
/// use const_base::{Config, DecodeError, decode};
///
/// assert_eq!(decode::<2>(b"ABA", Config::B64).unwrap(), [0u8, 16]);
///
/// // base 64 inputs of length 3 are 18 bits, which is 2 bytes and 2 excess bits.
/// // `ABC` is base64 for `[0b00000000, 0b00010000]` with excess `0b10` bits.
/// //
/// // Because the two unused bits at the end (which are `10`) include a set bit,
/// // it causes the `ExcessBits` error.
/// match decode::<2>(b"ABC", Config::B64) {
///     Err(DecodeError::ExcessBits(err)) => {
///         assert_eq!(err.last_byte(), b'C');
///     }
///     _ => unreachable!()
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct ExcessBits {
    pub(crate) last_byte: u8,
}

impl ExcessBits {
    pub const fn last_byte(&self) -> u8 {
        self.last_byte
    }

    define_unwrap_self! {}

    /// Panics with this error as the message.
    #[track_caller]
    pub const fn panic(&self) -> ! {
        use const_panic::{FmtArg, PanicVal};

        crate::utils::cpanic(&[
            PanicVal::write_str("excess bits in last byte: "),
            PanicVal::from_u8(self.last_byte, FmtArg::DEBUG),
            PanicVal::write_str("_u8 (the "),
            PanicVal::from_char(self.last_byte as char, FmtArg::DEBUG),
            PanicVal::write_str(" character)"),
        ])
    }
}

#[doc(hidden)]
#[track_caller]
pub const fn __unwrap_encode<const N: usize>(
    res: Result<crate::ArrayStr<N>, WrongOutputLength>,
) -> crate::ArrayStr<N> {
    match res {
        Ok(x) => x,
        Err(e) => e.panic(),
    }
}
