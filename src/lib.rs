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

pub mod errors;

#[doc(hidden)]
pub mod msg;

#[doc(hidden)]
pub mod __priv_utils;

pub use crate::{
    config::Config,
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

pub const fn encoded_len(input_len: usize, config: Config) -> usize {
    match config.encoding {
        Encoding::Base64(_) => crate::base_64::encoded_len(input_len, config),
    }
}

pub const fn encode<const OUT: usize>(input: &[u8], config: Config) -> [u8; OUT] {
    match config.encoding {
        Encoding::Base64(cset) => crate::base_64::encode(input, config, cset),
    }
}

pub const fn decoded_len(input_len: usize, config: Config) -> usize {
    match config.encoding {
        Encoding::Base64(_) => crate::base_64::decoded_len(input_len, config),
    }
}

pub const fn decode<const OUT: usize>(
    input: &[u8],
    config: Config,
) -> Result<[u8; OUT], DecodeError> {
    match config.encoding {
        Encoding::Base64(cset) => crate::base_64::decode(input, config, cset),
    }
}
