//! Miscelaneous utility functions.

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
