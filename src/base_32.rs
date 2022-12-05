use crate::{
    encode_decode_shared::{decoded_len_bases, encoded_len_bases},
    B32CharSet, Config, DecodeError, Encoding, WrongOutputLength,
};

const MASK_5BITS: u8 = 0b11111;
const MASK_5BITS64: u64 = 0b11111;

// Every 5 bytes from the input is converted to 8 base64 encoded bytes
const B32_CHUNK: usize = 8;

// Base32 encodes 5 bits per byte
const B32_BITS_PER_BYTE: u64 = 5;

pub(crate) const fn encoded_len(input_len: usize, config: Config) -> usize {
    encoded_len_bases(input_len, config, B32_BITS_PER_BYTE, B32_CHUNK)
}

macro_rules! cast_shl {
    (
        $($ident:ident $(<< $shift:expr)?),*
    ) => (
        $((
            ($ident as u64) $(<< $shift)?
        ))|*
    )
}
macro_rules! cast_shr {
    (
        $($ident:ident $(>> $shift:expr)?),*
    ) => (
        $((
            ($ident as u64) $(>> $shift)?
        ))|*
    )
}

pub(crate) const fn encode<const OUT: usize>(
    mut input: &[u8],
    config: Config,
    char_set: B32CharSet,
) -> Result<crate::ArrayStr<OUT>, WrongOutputLength> {
    crate::encode_decode_shared::encode_bases! {
        input, config, char_set,
        {
            while let [a, b, c, d, e, ref rem @ ..] = *input {
                let buffer = cast_shl!(a << 32, b << 24, c << 16, d << 8, e);

                write_out!{ (buffer >> 35) as u64 }
                write_out!{ (buffer >> 30) as u64 & MASK_5BITS64 }
                write_out!{ (buffer >> 25) as u64 & MASK_5BITS64 }
                write_out!{ (buffer >> 20) as u64 & MASK_5BITS64 }
                write_out!{ (buffer >> 15) as u64 & MASK_5BITS64 }
                write_out!{ (buffer >> 10) as u64 & MASK_5BITS64 }
                write_out!{ (buffer >> 5 ) as u64 & MASK_5BITS64 }
                write_out!{ buffer as u64 & MASK_5BITS64 }

                input = rem;
            }

            match *input {
                [a] => {
                    let buffer = a;

                    write_out!{ (buffer >> 3) as u8 }
                    write_out!{ (buffer << 2) as u8 & MASK_5BITS }
                }
                [a, b] => {
                    let buffer = cast_shl!(a << 8, b);

                    write_out!{ (buffer >> 11)as u8  }
                    write_out!{ (buffer >> 6) as u8 & MASK_5BITS }
                    write_out!{ (buffer >> 1) as u8 & MASK_5BITS }
                    write_out!{ (buffer << 4) as u8 & MASK_5BITS }
                }
                [a, b, c] => {
                    let buffer = cast_shl!(a << 16, b << 8, c);

                    write_out!{ (buffer >> 19) as u8 }
                    write_out!{ (buffer >> 14) as u8 & MASK_5BITS }
                    write_out!{ (buffer >> 9)  as u8 & MASK_5BITS }
                    write_out!{ (buffer >> 4)  as u8 & MASK_5BITS }
                    write_out!{ (buffer << 1)  as u8 & MASK_5BITS }
                }
                [a, b, c, d] => {
                    let buffer = cast_shl!(a << 24, b << 16, c << 8, d);

                    write_out!{ (buffer >> 27) as u8  }
                    write_out!{ (buffer >> 22) as u8 & MASK_5BITS }
                    write_out!{ (buffer >> 17) as u8 & MASK_5BITS }
                    write_out!{ (buffer >> 12) as u8 & MASK_5BITS }
                    write_out!{ (buffer >> 7)  as u8 & MASK_5BITS }
                    write_out!{ (buffer >> 2)  as u8 & MASK_5BITS }
                    write_out!{ (buffer << 3)  as u8 & MASK_5BITS }
                }
                _ => {}
            }
        }
    }
}

pub(crate) const fn decoded_len(input: &[u8], config: Config) -> usize {
    decoded_len_bases(input, config, B32_BITS_PER_BYTE)
}

pub(crate) const fn decode<const OUT: usize>(
    mut input: &[u8],
    config: Config,
    char_set: B32CharSet,
) -> Result<[u8; OUT], DecodeError> {
    crate::encode_decode_shared::decode_bases! {
        dollar = $,
        Encoding::Base32,
        input, config, char_set,
        matches!(input.len() % 8, 1 | 3 | 6),
        |in_i| {
            while let [oa, ob, oc, od, oe, of, og, oh, ref rem @ ..] = *input {
                from_encoded! {
                    a = oa, b = ob, c = oc, d = od,
                    e = oe, f = of, g = og, h = oh
                }

                let buffer =
                    cast_shl!(a << 35, b << 30, c << 25, d << 20, e << 15, f << 10, g << 5, h);

                write_out!((buffer >> 32) as u8);
                write_out!((buffer >> 24) as u8);
                write_out!((buffer >> 16) as u8);
                write_out!((buffer >> 8) as u8);
                write_out!(buffer as u8);

                input = rem;
                in_i += 4;
            }

            let res_excess_bits = match *input {
                [oa, ob, oc, od, oe, of, og] => {
                    from_encoded! {a = oa, b = ob, c = oc, d = od, e = oe, f = of, g = og}

                    let buffer =
                        cast_shl!(a << 27, b << 22, c << 17, d << 12, e << 7, f << 2) |
                        cast_shr!(g >> 3);

                    write_out!((buffer >> 24) as u8);
                    write_out!((buffer >> 16) as u8);
                    write_out!((buffer >> 8) as u8);
                    write_out!(buffer as u8);

                    crate::encoding::CheckExcessBits {
                        last_byte: og,
                        decoded_byte: g,
                        excess_bits: 3,
                    }.call()
                }
                [oa, ob, oc, od, oe] => {
                    from_encoded! {a = oa, b = ob, c = oc, d = od, e = oe}

                    let buffer = cast_shl!(a << 19, b << 14, c << 9, d << 4) | cast_shr!(e >> 1);

                    write_out!((buffer >> 16) as u8);
                    write_out!((buffer >> 8) as u8);
                    write_out!(buffer as u8);

                    crate::encoding::CheckExcessBits {
                        last_byte: oe,
                        decoded_byte: e,
                        excess_bits: 1,
                    }.call()
                }
                [oa, ob, oc, od] => {
                    from_encoded! {a = oa, b = ob, c = oc, d = od}

                    let buffer = cast_shl!(a << 11, b << 6, c << 1) | cast_shr!(d >> 4);

                    write_out!((buffer >> 8) as u8);
                    write_out!(buffer as u8);

                    crate::encoding::CheckExcessBits {
                        last_byte: od,
                        decoded_byte: d,
                        excess_bits: 4,
                    }.call()
                }
                [oa, ob] => {
                    from_encoded! {a = oa, b = ob}

                    let buffer = cast_shl!(a << 3) | cast_shr!(b >> 2);

                    write_out!(buffer as u8);

                    crate::encoding::CheckExcessBits {
                        last_byte: ob,
                        decoded_byte: b,
                        excess_bits: 2,
                    }.call()
                }
                [] => Ok(()),
                _ => panic!("BUG: `input` can't be an invalid length here."),
            };

            if let Err(e) = res_excess_bits {
                return Err(e);
            }

        }
    }
}
