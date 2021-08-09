#[derive(Copy, Clone)]
pub struct AsBytes<T>(pub T);

impl<'a, const N: usize> AsBytes<&'a [u8; N]> {
    pub const fn as_bytes(self) -> &'a [u8] {
        self.0
    }
}

impl<'a> AsBytes<&'a str> {
    pub const fn as_bytes(self) -> &'a [u8] {
        self.0.as_bytes()
    }
}
