/// Decodes the `$slice` constant into a `&[u8; N]` with the encoding determined by [`$config`].
///
/// `$slice` can be a `&'static str`, `&'static [u8; N]`, or `&'static [u8]`.
///
/// # Compile-time Errors
///
/// When this macro is passed a malformed slice, it'll produce compile-time errors in the
/// same situations where [`crate::decode`](crate::decode()) would return an error.
///
/// For an example of what those look like, look [down here](#erroring-example)
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
/// ### Base 32
///
/// ```rust
/// # fn main(){
/// use const_base::{decode, Config};
///
/// const OUT: &[u8] = decode!("MZXCA3LBNFXCQKJAPN6Q====", Config::B32);
///     
/// assert_eq!(OUT, b"fn main() {}");
/// # }
/// ```
///
/// <div id = "erroring-example"></div>
///
///
/// ### Hexadecimal
///
/// ```rust
/// use const_base::{decode, Config};
///
/// const OUT: &[u8] = decode!("F00B", Config::HEX);
///     
/// assert_eq!(OUT, &[0xF0, 0x0B]);
/// ```
///
/// <div id = "erroring-example"></div>
///
/// ### Erroring
///
/// Malformed inputs like this
///
/// ```compile_fail
/// use const_base::{decode, Config};
/// decode!("A", Config::B64);
/// ```
/// produce compile-time errors that look like this:
/// ```text
/// error[E0080]: evaluation of constant value failed
///  --> src/codec_macros.rs:67:1
///   |
/// 5 | decode!("A", Config::B64);
///   | ^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at '
///
/// invalid input length for base-64: 1
///
/// ', src/codec_macros.rs:5:1
///
/// ```
/// This macro emulates panics using type errors like those.
///
/// In this case, the error is `WrongInputLength`,
/// because the input string can't be `4 * n + 1` bytes long (`n` can be any positive integer).
///
///
/// [`$config`]: crate::Config
#[macro_export]
macro_rules! decode {
    ($slice:expr, $config:expr $(,)*) => {{
        const __P_NHPMWYD3NJA: $crate::__::CodecArgs =
            $crate::__::DecodeArgsFrom($slice, $config).conv();
        {
            const RES: &$crate::__DecodeResult<{ __P_NHPMWYD3NJA.out_len }> =
                &$crate::__priv_decode(__P_NHPMWYD3NJA.input, __P_NHPMWYD3NJA.cfg);

            const _: () = RES.assert_ok();

            &RES.array
        }
    }};
}

/// Encodes the `$slice` constant into a [`&'static ArrayStr<LEN>`](crate::ArrayStr),
/// with the encoding determined by [`$config`].
///
/// `$slice` can be a `&'static str`, `&'static [u8; N]`, or `&'static [u8]`.
///
/// There's also the [`encode_as_str`](crate::encode_as_str)
/// macro for getting a `&'static str`.
/// `encode_as_str` is just an alias for `encode!(...).as_str()`.
///
/// # Examples
///
/// ### Base 64
///
/// ```rust
/// use const_base::{encode, ArrayStr, Config};
///
/// {
///     const OUT: &ArrayStr<4> = encode!("bar", Config::B64);
///     
///     assert_eq!(OUT, "YmFy");
/// }
/// {
///     const BYTES: &[u8] = b"world";
///
///     // this macro can encode non-literal constants
///     const OUT: &[u8; 8] = encode!(BYTES, Config::B64_URL_SAFE).as_array();
///     const OUT_STR: &str = encode!(BYTES, Config::B64_URL_SAFE).as_str();
///     
///     assert_eq!(OUT, b"d29ybGQ=");
///     assert_eq!(OUT_STR, "d29ybGQ=");
/// }
/// ```
///
/// ### Base 32
///
/// ```rust
/// use const_base::{encode, Config};
///
/// const OUT: &str = encode!(&[3, 5, 8], Config::B32).as_str();
///     
/// assert_eq!(OUT, "AMCQQ===");
/// ```
///
/// ### Base 32
///
/// ```rust
/// use const_base::{encode, Config};
///
/// const OUT: &str = encode!(&[3, 5, 8], Config::B32).as_str();
///     
/// assert_eq!(OUT, "AMCQQ===");
/// ```
///
/// ### Hexadecimal
///
/// ```rust
/// use const_base::{encode, Config};
///
/// const LOWER: &str = encode!(&[0xB0, 0x01], Config::HEX_LOWER).as_str();
/// const UPPER: &str = encode!(&[0xB0, 0x01], Config::HEX).as_str();
///     
/// assert_eq!(LOWER, "b001");
/// assert_eq!(UPPER, "B001");
/// ```
///
/// [`$config`]: crate::Config
#[macro_export]
macro_rules! encode {
    ($slice:expr, $config:expr $(,)*) => {{
        const __P_NHPMWYD3NJA: $crate::__::CodecArgs =
            $crate::__::EncodeArgsFrom($slice, $config).conv();

        {
            const OUT: &$crate::ArrayStr<{ __P_NHPMWYD3NJA.out_len }> =
                &$crate::__priv_encode(__P_NHPMWYD3NJA.input, __P_NHPMWYD3NJA.cfg);

            OUT
        }
    }};
}

/// Encodes the `$slice` constant into a `&'static str`,
/// with the encoding determined by [`$config`].
///
/// `$slice` can be a `&'static str`, `&'static [u8; N]`, or `&'static [u8]`.
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
///     const OUT: &str =  encode_as_str!(BYTES, Config::B64_URL_SAFE);
///     
///     assert_eq!(OUT, "Z29vZGJ5ZQ==");
/// }
/// ```
///
/// ### Base 32
///
/// ```rust
/// use const_base::{encode_as_str, Config};
///
/// const OUT: &str = encode_as_str!(&[13, 21, 34], Config::B32);
///     
/// assert_eq!(OUT, "BUKSE===");
/// ```
///
/// ### Hexadecimal
///
/// ```rust
/// use const_base::{encode_as_str, Config};
///
/// const UPPER: &str = encode_as_str!(&[0xB1, 0x00, 0x0d], Config::HEX);
/// const LOWER: &str = encode_as_str!(&[0xB1, 0x00, 0x0d], Config::HEX_LOWER);
///     
/// assert_eq!(UPPER, "B1000D");
/// assert_eq!(LOWER, "b1000d");
/// ```
///
/// [`$config`]: crate::Config
#[macro_export]
macro_rules! encode_as_str {
    ($slice:expr, $config:expr $(,)*) => {{
        const OUT_NHPMWYD3NJA: &$crate::__::str = $crate::encode!($slice, $config).as_str();

        OUT_NHPMWYD3NJA
    }};
}
