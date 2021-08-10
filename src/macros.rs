/// A const equivalent of `Result::unwrap_or`, where `$else` is only evaluated on error.
///
/// # Example
///
/// ```rust
/// use const_base::{
///     unwrap_or,
///     utils::repeated,
/// };
///
/// type Res = Result<[u8; 4], ()>;
///
/// const OK: [u8; 4] = unwrap_or!(Res::Ok([3, 5, 8, 13]), repeated(0xFF));
/// const ERR: [u8; 4] = unwrap_or!(Res::Err(()), repeated(0xFF));
///
/// assert_eq!(OK, [3, 5, 8, 13]);
/// assert_eq!(ERR, [0xFF, 0xFF, 0xFF, 0xFF]);
///
/// ```
#[macro_export]
macro_rules! unwrap_or {
    ($res:expr, $else:expr) => {
        match $res {
            $crate::__::Ok(x) => x,
            $crate::__::Err(_) => $else,
        }
    };
}
