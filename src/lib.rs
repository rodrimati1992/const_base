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
pub mod __convert;

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
}
