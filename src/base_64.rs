use crate::{B64CharSet, Config};

const MASK_6BITS: u8 = 0b111111;

pub(crate) const fn encoded_len(input: &[u8], _config: Config) -> usize {
    let mult = input.len() as u64 * 8;

    crate::__priv_utils::div_ceil_u64(mult, 6) as usize
}

pub(crate) const fn encode<const OUT: usize>(
    mut input: &[u8],
    config: Config,
    char_set: B64CharSet,
) -> [u8; OUT] {
    let mut buffer = 0u16;
    let mut written_bits = 0u8;
    let mut out = [0u8; OUT];
    let mut out_i = 0usize;

    let lookup = char_set.lookup();

    let output_len = encoded_len(input, config);

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

    while let [b, ref rem @ ..] = *input {
        buffer = (buffer << 8) | b as u16;
        written_bits += 8;

        while written_bits >= 6 {
            written_bits = written_bits - 6;
            write_out!(((buffer >> written_bits) as u8) & MASK_6BITS);
        }

        input = rem;
    }

    write_out! {(buffer >> 6) as u8 & MASK_6BITS}

    out
}
