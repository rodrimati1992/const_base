#[doc(hidden)]
#[macro_export]
macro_rules! __result_tuple_to_singleton {
    ($res:expr) => {{
        const __RES_NHPMWYD3NJA: $crate::msg::__ResultTuple = $res;
        {
            <() as $crate::msg::__ConstToType<
                { __RES_NHPMWYD3NJA.0 },
                { __RES_NHPMWYD3NJA.1 },
                { __RES_NHPMWYD3NJA.2 },
                { __RES_NHPMWYD3NJA.3 },
            >>::V
        }
    }};
}
