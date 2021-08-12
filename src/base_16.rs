use crate::{encoding::INVALID_ENC, Config, DecodeError, HexCharSet, MismatchedOutputLength};

const UPPER_A_SUB_10: u8 = b'A' - 10;
const LOWER_A_SUB_10: u8 = b'a' - 10;

#[inline(always)]
const fn hex_to_digit(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'A'..=b'F' => hex - UPPER_A_SUB_10,
        b'a'..=b'f' => hex - LOWER_A_SUB_10,
        _ => crate::encoding::INVALID_ENC,
    }
}

pub(crate) const fn encoded_len(input_len: usize, _config: Config) -> usize {
    input_len * 2
}

pub(crate) const fn encode<const OUT: usize>(
    mut input: &[u8],
    config: Config,
    char_set: HexCharSet,
) -> Result<[u8; OUT], MismatchedOutputLength> {
    let output_len = encoded_len(input.len(), config);

    if OUT != output_len {
        return Err(crate::MismatchedOutputLength {
            expected: OUT,
            found: output_len,
        });
    }

    let mut out = [0u8; OUT];
    let mut out_i = 0usize;

    let digit_to_hex = match char_set {
        HexCharSet::Lowercase => b"0123456789abcdef",
        HexCharSet::Uppercase => b"0123456789ABCDEF",
    };

    while let [b, ref rem @ ..] = *input {
        write_into! {out, out_i, digit_to_hex[(b >> 4) as usize]}
        write_into! {out, out_i, digit_to_hex[(b & 0xF) as usize]}

        input = rem;
    }

    Ok(out)
}

pub(crate) const fn decoded_len(input: &[u8], _config: Config) -> usize {
    input.len() / 2
}

pub(crate) const fn decode<const OUT: usize>(
    mut input: &[u8],
    config: Config,
) -> Result<[u8; OUT], DecodeError> {
    let output_len = decoded_len(input, config);

    if input.len() % 2 == 1 {
        return Err(DecodeError::InvalidInputLength(crate::InvalidInputLength {
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

    let mut out = [0u8; OUT];
    let mut out_i = 0usize;
    let mut in_i = 0usize;

    while let [oa, ob, ref rem @ ..] = *input {
        let a = hex_to_digit(oa);
        let b = hex_to_digit(ob);
        if a == INVALID_ENC || b == INVALID_ENC {
            let (index, byte) = if a == INVALID_ENC {
                (in_i, oa)
            } else {
                (in_i + 1, ob)
            };

            return Err(DecodeError::InvalidByte(crate::InvalidByte {
                index,
                byte,
                as_char: byte as char,
                encoding: config.encoding,
            }));
        }

        write_into! {out, out_i, (a << 4) | b}

        input = rem;
        in_i += 2;
    }

    Ok(out)
}
