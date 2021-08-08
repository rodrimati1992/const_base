#[macro_export]
macro_rules! encode {
    ($input:expr, $config:expr $(,)*) => {{
        const __BYTES_NHPMWYD3NJA: &[$crate::__::u8] = $input;
        const __CFG_NHPMWYD3NJA: $crate::Config = $config;
        {
            const OUT_LEN: $crate::__::usize =
                $crate::encoded_len(__BYTES_NHPMWYD3NJA.len(), __CFG_NHPMWYD3NJA);
            const OUT: &[$crate::__::u8; OUT_LEN] =
                &$crate::encode(__BYTES_NHPMWYD3NJA, __CFG_NHPMWYD3NJA);
            OUT
        }
    }};
}

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
