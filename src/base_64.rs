use crate::{
    encode_decode_shared::{decoded_len_bases, encoded_len_bases},
    B64CharSet, Config, DecodeError, Encoding, WrongLength,
};

const MASK_6BITS: u8 = 0b111111;

// Every 3 bytes from the input is converted to 4 base64 encoded bytes
const B64_CHUNK: usize = 4;

pub(crate) const fn encoded_len(input_len: usize, config: Config) -> usize {
    encoded_len_bases(input_len, config, 6, B64_CHUNK)
}

pub(crate) const fn encode<const OUT: usize>(
    mut input: &[u8],
    config: Config,
    char_set: B64CharSet,
) -> Result<crate::ArrayStr<OUT>, WrongLength> {
    crate::encode_decode_shared::encode_bases! {
        input, config, char_set,
        {
            while let [a, b, c, ref rem @ ..] = *input {
                write_out!(a >> 2);
                write_out!(((a << 4) | (b >> 4)) & MASK_6BITS);
                write_out!(((b << 2) | (c >> 6)) & MASK_6BITS);
                write_out!(c & MASK_6BITS);
                input = rem;
            }

            match *input {
                [a, b] => {
                    write_out!(a >> 2);
                    write_out!(((a << 4) | (b >> 4)) & MASK_6BITS);
                    write_out!((b << 2) & MASK_6BITS);
                }
                [a] => {
                    write_out!(a >> 2);
                    write_out!((a << 4) & MASK_6BITS);
                }
                _ => {}
            }
        }
    }
}

pub(crate) const fn decoded_len(input: &[u8], config: Config) -> usize {
    decoded_len_bases(input, config, 6)
}

pub(crate) const fn decode<const OUT: usize>(
    mut input: &[u8],
    config: Config,
    char_set: B64CharSet,
) -> Result<[u8; OUT], DecodeError> {
    crate::encode_decode_shared::decode_bases! {
        dollar = $,
        Encoding::Base64,
        input, config, char_set,
        input.len() % 4 == 1,
        |in_i| {
            while let [oa, ob, oc, od, ref rem @ ..] = *input {
                from_encoded! {a = oa, b = ob, c = oc, d = od}

                write_out!(a << 2 | (b >> 4));
                write_out!((b << 4) | (c >> 2));
                write_out!((c << 6) | d);
                input = rem;
                in_i += 4;
            }

            match *input {
                [oa, ob, oc] => {
                    from_encoded! {a = oa, b = ob, c = oc}
                    write_out!(a << 2 | (b >> 4));
                    write_out!((b << 4) | (c >> 2));
                }
                [oa, ob] => {
                    from_encoded! {a = oa, b = ob}
                    write_out!(a << 2 | (b >> 4));
                }
                [] => {}
                _ => panic!("BUG: `input` can't be an invalid length here"),
            }
        }
    }
}
