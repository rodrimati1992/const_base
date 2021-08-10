//! All the errors from this crate.

use crate::{msg::IS_OK, Encoding};

macro_rules! declare_errors {
    ($($(#[$attr:meta])* $variant:ident $(= $value:expr)? ,)*) => (
        /// Error returned by [`decode`](crate::decode())
        #[derive(Debug, PartialEq)]
        pub enum DecodeError {
            $(
                $(#[$attr])*
                $variant($variant),
            )*
        }


        #[doc(hidden)]
        pub enum __DecodeErrorKind {
            $( $variant $(= $value)? , )*
        }

        impl DecodeError {
            pub(crate) const fn kind(&self) -> __DecodeErrorKind {
                match self {
                    $( Self::$variant{..} => __DecodeErrorKind::$variant, )*
                }
            }
        }

        #[doc(hidden)]
        pub mod __ {
            use core::marker::PhantomData;

            $( pub struct $variant<T>(pub(crate) PhantomData<T>); )*
        }
    )
}

declare_errors! {
    /// When one of the bytes isn't in the char set for that encoding.
    ///
    /// Eg: a `!` in an otherwise base 64 encoded string.
    InvalidByte = IS_OK + 1,

    /// When the array returned by [`decode`] isn't the same length as what the arguments
    /// passed to [`decode`] would produce.
    ///
    /// [`decode`]: crate::decode()
    MismatchedOutputLength,
    /// When the slice passed to [`decode`] is not a valid length for that encoding.
    ///
    /// For base 64 that is when `input.len() % 4` equals `1`.
    InvalidInputLength,
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
/// use const_base::{
///     Config, decode,
///     errors::{DecodeError, InvalidByte},
/// };
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
/// use const_base::{
///     Config, decode,
///     errors::{DecodeError, MismatchedOutputLength},
/// };
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
/// use const_base::{
///     Config, decode,
///     errors::{DecodeError, InvalidInputLength},
/// };
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
}

impl InvalidInputLength {
    pub const fn length(&self) -> usize {
        self.length
    }
}
