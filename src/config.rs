use crate::{B32CharSet, B64CharSet, Encoding, HexCharSet};

/// For configuring how a string is encoded/decoded.
///
/// `Config` has these values by default:
///
/// - `end_padding = false`
///
#[derive(Debug, Copy, Clone)]
pub struct Config {
    pub(crate) encoding: Encoding,
    pub(crate) end_padding: bool,
}

impl Config {
    /// Constructs a `Config` from an [Encoding]
    ///
    /// You can use [the associated constants](#associated-consts) for a more
    /// concise way to get a `Config`
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_base::{B64CharSet, Config, Encoding, encode};
    ///
    /// // The same as `Config::B64`
    /// const CFG: Config = Config::new(Encoding::Base64(B64CharSet::Standard));
    /// assert_eq!(encode!("Rust", CFG), b"UnVzdA==")
    ///
    /// ```
    pub const fn new(encoding: Encoding) -> Self {
        Self {
            encoding,
            end_padding: true,
        }
    }

    /// Determines whether the string has padding at the end.
    /// This is `true` by default.
    ///
    /// For base64 strings, the string is padded to be a multiple of 4 long, with `=`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_base::{B64CharSet, Config, Encoding, encode};
    ///
    /// assert_eq!(encode!("Rust", Config::B64), b"UnVzdA==");
    ///
    /// assert_eq!(encode!("Rust", Config::B64.end_padding(true)), b"UnVzdA==");
    ///
    /// assert_eq!(encode!("Rust", Config::B64.end_padding(false)), b"UnVzdA");
    ///
    /// ```
    pub const fn end_padding(mut self, have: bool) -> Self {
        self.end_padding = have;
        self
    }

    /// A different way to call [`encode`](crate::encode()).
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_base::{Config, encode};
    ///
    /// assert_eq!(Config::B64.encode::<8>(b"Rust"), Ok(*b"UnVzdA=="));
    /// ```
    #[inline(always)]
    pub const fn encode<const OUT: usize>(
        self,
        input: &[u8],
    ) -> Result<[u8; OUT], crate::MismatchedOutputLength> {
        crate::encode(input, self)
    }

    /// A different way to call [`decode`](crate::decode()).
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_base::{Config, decode};
    ///
    /// assert_eq!(Config::B64.decode::<4>(b"UnVzdA=="), Ok(*b"Rust"));
    /// ```
    #[inline(always)]
    pub const fn decode<const OUT: usize>(
        self,
        input: &[u8],
    ) -> Result<[u8; OUT], crate::DecodeError> {
        crate::decode(input, self)
    }
}

/// <div id = "associated-consts"></div>
impl Config {
    /// Configuration with the [`Base64`](crate::Encoding::Base64) encoding,
    /// using the [`Standard`](crate::B64CharSet::Standard) character set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_base::{Config, encode};
    ///
    /// assert_eq!(encode!(&[23, 239, 192], Config::B64), b"F+/A");
    /// ```
    ///
    pub const B64: Self = Self::new(Encoding::Base64(B64CharSet::Standard));

    /// Configuration with the [`Base64`](crate::Encoding::Base64) encoding,
    /// using the [`UrlSafe`](crate::B64CharSet::UrlSafe) character set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_base::{Config, encode};
    ///
    /// assert_eq!(encode!(&[23, 239, 192], Config::B64_URL_SAFE), b"F-_A");
    /// ```
    ///
    pub const B64_URL_SAFE: Self = Self::new(Encoding::Base64(B64CharSet::UrlSafe));

    /// Configuration with the [`Base32`](crate::Encoding::Base32) encoding,
    /// using the [`Standard`](crate::B32CharSet::Standard) character set.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_base::{Config, encode};
    ///
    /// assert_eq!(encode!(b"neat", Config::B32), b"NZSWC5A=");
    /// ```
    ///
    pub const B32: Self = Self::new(Encoding::Base32(B32CharSet::Standard));

    /// Configuration with the [`Hex`](crate::Encoding::Hex) (hexadecimal) encoding,
    /// using the [`Uppercase`](crate::HexCharSet::Uppercase) character set.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_base::{Config, decode, encode};
    ///
    /// assert_eq!(encode!(&[0xF1, 0x00, 0x0f], Config::HEX), b"F1000F");
    ///
    /// // Hexademical decoding allows mixing uppercase and lowercase
    /// assert_eq!(decode!(b"beefBEEF", Config::HEX), &[0xBE, 0xEF, 0xBE, 0xEF]);
    /// ```
    ///
    pub const HEX: Self = Self::new(Encoding::Hex(HexCharSet::Uppercase));

    /// Configuration with the [`Hex`](crate::Encoding::Hex) (hexadecimal) encoding,
    /// using the [`Lowercase`](crate::HexCharSet::Lowercase) character set.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_base::{Config, decode, encode};
    ///
    /// assert_eq!(encode!(&[0xf1, 0x00, 0x0f], Config::HEX_LOWER), b"f1000f");
    ///
    /// // Hexademical decoding allows mixing uppercase and lowercase
    /// assert_eq!(decode!(b"beefBEEF", Config::HEX_LOWER), &[0xBE, 0xEF, 0xBE, 0xEF]);
    /// ```
    ///
    pub const HEX_LOWER: Self = Self::new(Encoding::Hex(HexCharSet::Lowercase));
}
