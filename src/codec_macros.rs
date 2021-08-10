/// Decodes a `&[u8; N]` from one of the supported encodings.
///
/// The input can be a `&'static str`, `&'static [u8; N]`, or `&'static [u8]`.
///
/// # Examples
///
/// ### Base 64
///
/// ```rust
/// use const_base::{decode, Config};
///
/// {
///     const OUT: &[u8] = decode!("SGVsbG8sIHdvcmxkIQ==", Config::B64);
///     
///     assert_eq!(OUT, b"Hello, world!");
/// }
/// {
///     const BYTES: &[u8] = b"TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQ";
///
///     // this macro can decode non-literal constants
///     const OUT: &[u8; 26] = decode!(BYTES, Config::B64_URL_SAFE.end_padding(false));
///     
///     assert_eq!(OUT, b"Lorem ipsum dolor sit amet");
/// }
/// ```
///
///
///
#[macro_export]
macro_rules! decode {
    ($input:expr, $config:expr $(,)*) => {{
        const __P_NHPMWYD3NJA: $crate::__::CodecArgs =
            $crate::__::DecodeArgsFrom($input, $config).conv();
        {
            const OUT: &$crate::__AdjacentResult<
                [$crate::__::u8; __P_NHPMWYD3NJA.out_len],
                $crate::DecodeError,
            > = &$crate::__priv_decode(__P_NHPMWYD3NJA.input, __P_NHPMWYD3NJA.cfg);

            const _: $crate::msg::IsOk =
                $crate::__result_tuple_to_singleton!($crate::msg::__decode_res_to_tuple(&OUT.err));

            &OUT.ok
        }
    }};
}

/// Encodes a slice into a `&[u8; N]` with one of the supported encodings.
///
/// The input slice can be a `&'static str`, `&'static [u8; N]`, or `&'static [u8]`.
///
/// # Examples
///
/// ### Base 64
///
/// ```rust
/// use const_base::{encode, Config};
///
/// {
///     const OUT: &[u8; 4] = encode!("bar", Config::B64);
///     
///     assert_eq!(OUT, b"YmFy");
/// }
/// {
///     const BYTES: &[u8] = b"world";
///
///     // this macro can encode non-literal constants
///     const OUT: &[u8] = encode!(BYTES, Config::B64_URL_SAFE);
///     
///     assert_eq!(OUT, b"d29ybGQ=");
/// }
/// ```
///
#[macro_export]
macro_rules! encode {
    ($input:expr, $config:expr $(,)*) => {{
        const __P_NHPMWYD3NJA: $crate::__::CodecArgs =
            $crate::__::EncodeArgsFrom($input, $config).conv();

        {
            const OUT: &$crate::__AdjacentResult<
                [$crate::__::u8; __P_NHPMWYD3NJA.out_len],
                $crate::errors::MismatchedOutputLength,
            > = &$crate::__priv_encode(__P_NHPMWYD3NJA.input, __P_NHPMWYD3NJA.cfg);

            const _: $crate::msg::IsOk =
                $crate::__result_tuple_to_singleton!($crate::msg::__encode_res_to_tuple(&OUT.err));

            &OUT.ok
        }
    }};
}

/// Encodes a slice into a `&str` with one of the supported encodings.
///
/// The input can be a `&'static str`, `&'static [u8; N]`, or `&'static [u8]`.
///
/// # Examples
///
/// ### Base 64
///
/// ```rust
/// use const_base::{encode_as_str, Config};
///
/// {
///     const OUT: &str = encode_as_str!("qux", Config::B64);
///     
///     assert_eq!(OUT, "cXV4");
/// }
/// {
///     const BYTES: &[u8] = b"goodbye";
///
///     // this macro can encode non-literal constants
///     const OUT: &str = encode_as_str!(BYTES, Config::B64_URL_SAFE);
///     
///     assert_eq!(OUT, "Z29vZGJ5ZQ==");
/// }
/// ```
///
#[macro_export]
macro_rules! encode_as_str {
    ($input:expr, $config:expr $(,)*) => {{
        const OUT_NHPMWYD3NJA: &$crate::__::str =
            unsafe { $crate::__priv_transmute_bytes_to_str!($crate::encode!($input, $config)) };
        OUT_NHPMWYD3NJA
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __priv_transmute_bytes_to_str {
    ($bytes:expr) => {{
        let bytes: &'static [$crate::__::u8] = $bytes;
        let string: &'static $crate::__::str = {
            $crate::__priv_utils::PtrToRef {
                ptr: bytes as *const [$crate::__::u8] as *const $crate::__::str,
            }
            .reff
        };
        string
    }};
}
