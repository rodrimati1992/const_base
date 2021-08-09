use crate::{
    encoding::INVALID_B64,
    errors::{DecodeError, InvalidByte, InvalidInputLength, MismatchedOutputLength},
    B64CharSet, Config, Encoding,
};

const MASK_6BITS: u8 = 0b111111;

// Every 3 bytes from the input is converted to 4 base64 encoded bytes
const B64_CHUNK: usize = 4;

pub(crate) const fn encoded_len(input_len: usize, config: Config) -> usize {
    let mult = input_len as u64 * 8;

    let div = crate::__priv_utils::div_ceil_u64(mult, 6) as usize;

    if config.end_padding {
        crate::__priv_utils::round_up_to_multiple_usize(div, B64_CHUNK)
    } else {
        div
    }
}

pub(crate) const fn encode<const OUT: usize>(
    mut input: &[u8],
    config: Config,
    char_set: B64CharSet,
) -> Result<[u8; OUT], MismatchedOutputLength> {
    let mut out = [0u8; OUT];
    let mut out_i = 0usize;

    let lookup = char_set.lookup();

    let output_len = encoded_len(input.len(), config);

    if output_len != OUT {
        return Err(MismatchedOutputLength {
            expected: OUT,
            found: output_len,
        });
    }

    macro_rules! write_out {
        ($b:expr) => {
            write_into! {out, out_i, lookup.into_b64[$b as usize]}
        };
    }

    if !input.is_empty() {
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

    while out_i != OUT {
        write_into! {out, out_i, b'='}
    }

    Ok(out)
}

pub(crate) const fn decoded_len(mut input: &[u8], config: Config) -> usize {
    if config.end_padding {
        while let [rem @ .., b'='] = input {
            input = rem;
        }
    }

    let mult = input.len() as u64 * 6;

    let div = (mult / 8) as usize;

    div
}

pub(crate) const fn decode<const OUT: usize>(
    mut input: &[u8],
    config: Config,
    char_set: B64CharSet,
) -> Result<[u8; OUT], DecodeError> {
    let mut out = [0u8; OUT];
    let mut out_i = 0usize;
    let mut in_i = 0;

    let from_b64 = &char_set.lookup().from_b64;

    let output_len = decoded_len(input, config);

    if config.end_padding {
        while let [rem @ .., b'='] = input {
            input = rem;
        }
    }

    if input.len() % 4 == 1 {
        return Err(DecodeError::InvalidInputLength(InvalidInputLength {
            length: input.len(),
        }));
    } else if output_len != OUT {
        return Err(DecodeError::MismatchedOutputLength(
            MismatchedOutputLength {
                expected: OUT,
                found: output_len,
            },
        ));
    }

    macro_rules! write_out {
        ($b:expr) => {
            write_into! {out, out_i, $b}
        };
    }

    const fn find_first_non_b64<const N: usize>(arr: &[u8; N]) -> usize {
        for_range! {i in 0..N =>
            if arr[i] == INVALID_B64 {
                return i;
            }
        }
        !0
    }

    macro_rules! from_b64 {
        ($($new:ident = $old:ident),*) => (
            $( let $new = from_b64[$old as usize]; )*
            if $( $new == INVALID_B64 )||* {
                let news = [$($new),*];
                let invalid_pos = find_first_non_b64(&news);
                let index = in_i + invalid_pos;
                let byte = input[invalid_pos];

                return Err(DecodeError::InvalidByte(InvalidByte{
                    index,
                    byte,
                    as_char: byte as char,
                    encoding: Encoding::Base64(char_set),
                }))
            }
        )
    }

    if !input.is_empty() {
        while let [oa, ob, oc, od, ref rem @ ..] = *input {
            from_b64! {a = oa, b = ob, c = oc, d = od}

            write_out!(a << 2 | (b >> 4));
            write_out!((b << 4) | (c >> 2));
            write_out!((c << 6) | d);
            input = rem;
            in_i += 4;
        }

        match *input {
            [oa, ob, oc] => {
                from_b64! {a = oa, b = ob, c = oc}
                write_out!(a << 2 | (b >> 4));
                write_out!((b << 4) | (c >> 2));
            }
            [oa, ob] => {
                from_b64! {a = oa, b = ob}
                write_out!(a << 2 | (b >> 4));
            }
            [_] => [/* unreachable */][input.len()],
            _ => {}
        }
    }

    Ok(out)
}
