//! Miscelaneous utility functions.

use const_panic::PanicVal;

/// Constructs an array by repeating `repeated`. Most useful when `LENGTH` can be inferred.
///
/// # Example
///
/// ```rust
/// use const_base::utils::repeated;
///
/// const ARR: [u8; 4] = repeated(5);
///
/// assert_eq!(ARR, [5, 5, 5, 5]);
/// ```
#[inline(always)]
pub const fn repeated<const LENGTH: usize>(repeated: u8) -> [u8; LENGTH] {
    [repeated; LENGTH]
}

#[cold]
#[track_caller]
#[inline(never)]
pub(crate) const fn cpanic(pvs: &[PanicVal<'_>]) -> ! {
    const_panic::concat_panic(&[
        &[PanicVal::write_str("\n\n")],
        pvs,
        &[PanicVal::write_str("\n\n")],
    ])
}
