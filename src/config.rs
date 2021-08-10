use crate::{B64CharSet, Encoding};

#[derive(Debug, Copy, Clone)]
pub struct Config {
    pub(crate) encoding: Encoding,
    pub(crate) end_padding: bool,
}

impl Config {
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
    pub const fn end_padding(mut self, have: bool) -> Self {
        self.end_padding = have;
        self
    }

    #[inline(always)]
    pub const fn encode<const OUT: usize>(
        self,
        input: &[u8],
    ) -> Result<[u8; OUT], crate::errors::MismatchedOutputLength> {
        crate::encode(input, self)
    }
}

impl Config {
    /// Configuration with the [`Base64`](crate::Encoding::Base64) encoding,
    /// using the [`Standard`](crate::B64CharSet::Standard) character set.
    pub const B64: Self = Self::new(Encoding::Base64(B64CharSet::Standard));

    /// Configuration with the [`Base64`](crate::Encoding::Base64) encoding,
    /// using the [`UrlSafe`](crate::B64CharSet::UrlSafe) character set.
    pub const B64_URL_SAFE: Self = Self::new(Encoding::Base64(B64CharSet::UrlSafe));
}
