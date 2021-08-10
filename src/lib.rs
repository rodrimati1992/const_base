//! For decoding/encoding base 64 strings at compile-time.
//!
//! # Examples
//!
//! ### Encoding
//!
//! ```rust
//! use const_base::{encode_as_str, Config};
//!
//! {
//!     // this macro can encode both `&str` and `&[u8]` constants.
//!     const OUT: &str = encode_as_str!("foo", Config::B64);
//!     
//!     assert_eq!(OUT, "Zm9v");
//! }
//! {
//!     const BYTES: &[u8] = b"hello";
//!
//!     // this macro can encode non-literal constants
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
//!     const OUT: &[u8] = decode!("Zm9v", Config::B64);
//!     
//!     assert_eq!(OUT, b"foo");
//! }
//! {
//!     const BYTES: &str = "aGVsbG8";
//!
//!     // this macro can decode non-literal constants
//!     const OUT: &[u8] = decode!(BYTES, Config::B64_URL_SAFE.end_padding(false));
//!     
//!     assert_eq!(OUT, b"hello");
//! }
//! ```
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!

#![no_std]

#[macro_use]
mod codec_macros;

#[macro_use]
mod internal_macros;

#[macro_use]
mod msg_macros;

mod encoding;

mod config;

mod base_64;

mod encode_decode_shared;

#[doc(hidden)]
pub mod __macro_args;

pub mod errors;

#[doc(hidden)]
pub mod msg;

#[doc(hidden)]
pub mod __priv_utils;

pub use crate::{
    config::Config,
    encode_decode_shared::*,
    encoding::{B64CharSet, Encoding},
    errors::DecodeError,
};

#[cfg(test)]
mod tests;

#[doc(hidden)]
pub mod __ {
    pub use core::{
        ops::Range,
        primitive::{str, u8, usize},
        result::Result::{self, Err, Ok},
    };

    pub use crate::__macro_args::*;
}
