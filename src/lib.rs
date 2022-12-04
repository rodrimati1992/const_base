//! For decoding/encoding base 64/32/16 strings at compile-time.
//!
//! # Examples
//!
//! ### Encoding
//!
//! ```rust
//! use const_base::{encode_as_str, ArrayStr, Config};
//!
//! {
//!     // the encoding macros can take both `&str` and `&[u8]` constants.
//!     const OUTA: &str = encode_as_str!("foo", Config::B64);
//!     const OUTB: &str = encode_as_str!(b"foo", Config::B64);
//!     
//!     assert_eq!(OUTA, "Zm9v");
//!     assert_eq!(OUTB, "Zm9v");
//! }
//! {
//!     const BYTES: &[u8] = b"hello";
//!
//!     // the encoding macros can encode_as_str non-literal constants
//!     const OUT: &str = encode_as_str!(BYTES, Config::B64_URL_SAFE);
//!     
//!     assert_eq!(OUT, "aGVsbG8=");
//! }
//! ```
//!
//! ### Decoding
//!
//! ```rust
//! use const_base::{decode, Config};
//!
//! {
//!     const OUT: &[u8] = decode!("MZXW6===", Config::B32);
//!     
//!     assert_eq!(OUT, b"foo");
//! }
//! {
//!     const BYTES: &[u8] = b"f000";
//!
//!     // this macro can decode non-literal constants
//!     const OUT: &[u8] = decode!(BYTES, Config::HEX);
//!     
//!     assert_eq!(OUT, &[0xF0, 0x00]);
//! }
//! ```
//!
//! # No-std support
//!
//! `const_base` is `#![no_std]`, it can be used anywhere Rust can be used.
//!
//! # Minimum Supported Rust Version
//!
//! `const_base` requires Rust 1.51.0, because it uses const generics.
//!
//!
#![no_std]
#![deny(clippy::missing_const_for_fn)]

#[macro_use]
mod codec_macros;

#[macro_use]
mod internal_macros;

mod array_str;

mod encoding;

mod config;

mod base_16;

mod base_32;

mod base_64;

mod encode_decode_shared;

mod macros;

pub mod utils;

#[cfg(test)]
mod test_utils;

#[doc(hidden)]
pub mod __macro_args;

pub mod errors;

#[doc(hidden)]
pub mod __priv_utils;

pub use crate::{
    array_str::ArrayStr,
    config::Config,
    encode_decode_shared::*,
    encoding::{B32CharSet, B64CharSet, Encoding, HexCharSet},
    errors::{DecodeError, InvalidByte, InvalidInputLength, WrongLength},
};

#[cfg(test)]
mod tests;

#[doc(hidden)]
pub mod __ {
    pub use core::{
        ops::Range,
        primitive::{str, u8, usize},
        result::Result::{self, Err, Ok},
        str::from_utf8_unchecked,
    };

    pub use crate::__macro_args::*;
}
