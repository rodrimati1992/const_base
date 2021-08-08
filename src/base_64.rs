use crate::{B64CharSet, Config};

const MASK_6BITS: u8 = 0b111111;

pub(crate) const fn encoded_len(input_len: usize, _config: Config) -> usize {
    let mult = input_len as u64 * 8;

    crate::__priv_utils::div_ceil_u64(mult, 6) as usize
}

pub(crate) const fn encode<const OUT: usize>(
    mut input: &[u8],
    config: Config,
    char_set: B64CharSet,
) -> [u8; OUT] {
    let mut out = [0u8; OUT];
    let mut out_i = 0usize;

    let lookup = char_set.lookup();

    let output_len = encoded_len(input.len(), config);

    if output_len > OUT {
        [(); OUT][output_len]
    } else if output_len < OUT {
        [/* output_len is too small */][output_len]
    };

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

    out
}
