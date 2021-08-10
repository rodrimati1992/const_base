/// Unwraps a `Result<[u8; N], E>` into the array, evaluating to `[$repeated; N]` on error.
///
/// # Example
///
/// ```rust
/// use const_base::unwrap_or_repeated;
///
/// type Res = Result<[u8; 4], ()>;
///
/// const OK: [u8; 4] = unwrap_or_repeated!(Res::Ok([3, 5, 8, 13]), 0xFF);
/// const ERR: [u8; 4] = unwrap_or_repeated!(Res::Err(()), 0xFF);
///
/// assert_eq!(OK, [3, 5, 8, 13]);
/// assert_eq!(ERR, [0xFF, 0xFF, 0xFF, 0xFF]);
///
/// ```
#[macro_export]
macro_rules! unwrap_or_repeated {
    ($res:expr, $repeated:expr) => {
        match $res {
            $crate::__::Ok(x) => x,
            $crate::__::Err(_) => $crate::utils::repeated($repeated),
        }
    };
}
