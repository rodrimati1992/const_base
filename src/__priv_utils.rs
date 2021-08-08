pub union PtrToRef<'a, T: ?Sized> {
    pub ptr: *const T,
    pub reff: &'a T,
}

pub(crate) const fn div_ceil_u64(l: u64, r: u64) -> u64 {
    l / r + (l % r != 0) as u64
}
