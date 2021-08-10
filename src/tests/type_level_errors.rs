use crate::{
    msg::{
        self, IsOk, __decode_res_to_tuple, __encode_res_to_tuple, byte, byte_as_char, expected,
        found, index, length,
    },
    B64CharSet, DecodeError as DE, Encoding, InvalidByte, InvalidInputLength,
    MismatchedOutputLength,
};

macro_rules! from_decode_res {
    ($res:expr) => {
        __result_tuple_to_singleton!(__decode_res_to_tuple::<()>(&$res))
    };
}

macro_rules! from_encode_res {
    ($res:expr) => {
        __result_tuple_to_singleton!(__encode_res_to_tuple::<()>(&$res))
    };
}

//////////////////////
// Decoding results

const _: IsOk = from_decode_res!(Ok(()));

const _: msg::InvalidByte<(index<5>, byte<4>, byte_as_char<'\x04'>)> = {
    const X: DE = DE::InvalidByte(InvalidByte {
        index: 5,
        byte: 4,
        as_char: '\x04',
        encoding: Encoding::Base64(B64CharSet::Standard),
    });
    from_decode_res!(Err(X))
};

const _: msg::MismatchedOutputLength<(expected<5>, found<7>)> = {
    const X: DE = DE::MismatchedOutputLength(MismatchedOutputLength {
        expected: 5,
        found: 7,
    });
    from_decode_res!(Err(X))
};

const _: msg::InvalidInputLength<length<13>> = {
    const X: DE = DE::InvalidInputLength(InvalidInputLength { length: 13 });
    from_decode_res!(Err(X))
};

//////////////////////
// Encoding results

const _: IsOk = from_encode_res!(Ok(()));

const _: msg::MismatchedOutputLength<(expected<5>, found<7>)> =
    from_encode_res!(Err(MismatchedOutputLength {
        expected: 5,
        found: 7
    }));

#[test]
fn test_errors() {}
