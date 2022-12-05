use crate::{Config, DecodeError};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use data_encoding as hex;

#[test]
fn test_encode_decode() {
    let mut rng = SmallRng::seed_from_u64(6249204433781597762);

    macro_rules! test_encode {
        ($in_length:literal) => {{
            const ENCODED_LEN: usize = $in_length * 2;

            let cfgs = [
                (hex::HEXUPPER_PERMISSIVE, Config::HEX),
                (hex::HEXLOWER_PERMISSIVE, Config::HEX_LOWER),
            ];

            for (daten_cfg, cfg) in cfgs.iter() {
                for _ in 0..100 {
                    let input = rng.gen::<[u8; $in_length]>();

                    let mut daten_encoded = [0u8; ENCODED_LEN];
                    daten_cfg.encode_mut(&input, &mut daten_encoded);
                    let encoded = cfg.encode::<ENCODED_LEN>(&input).unwrap();
                    assert_eq!(&daten_encoded, encoded.as_array());

                    let mut daten_decoded = [0u8; $in_length];
                    daten_cfg
                        .decode_mut(&daten_encoded, &mut daten_decoded)
                        .unwrap();
                    let decoded = cfg.decode::<$in_length>(encoded.as_array()).unwrap();
                    assert_eq!(daten_decoded, decoded);
                }
            }
        }};
    }

    test_encode! {0}
    test_encode! {1}
    test_encode! {2}
    test_encode! {3}
    test_encode! {4}
    test_encode! {5}
}

#[test]
fn test_encode_hex_errors() {
    {
        let err = Config::HEX.encode::<3>(&[0xAB, 0xCD]).unwrap_err();
        assert!(err.expected() == 3 && err.found() == 4, "{:?}", err);
    }
    assert_eq!(Config::HEX.encode::<4>(&[0xAB, 0xCD]).unwrap(), "ABCD");
    {
        let err = Config::HEX.encode::<5>(&[0xAB, 0xCD]).unwrap_err();
        assert!(err.expected() == 5 && err.found() == 4, "{:?}", err);
    }
}

#[test]
fn test_decode_hex_errors() {
    let mut invalid_bytes = crate::test_utils::ByteSet([true; 256]);
    invalid_bytes.remove_range(b'0'..=b'9');
    invalid_bytes.remove_range(b'A'..=b'F');
    invalid_bytes.remove_range(b'a'..=b'f');

    let invalid_bytes_iters = invalid_bytes.iter().map(|b| (Config::HEX, b));

    // InvalidByte
    for (cfg, (b, is_invalid)) in invalid_bytes_iters {
        let mut bytes = *b"00\0000";
        bytes[2] = b;
        let res = cfg.decode::<3>(&bytes);

        if is_invalid {
            let err = res.unwrap_err();
            assert!(
                matches!(
                    &err,
                    DecodeError::InvalidByte(x)
                    if x.index() == 2 &&
                        x.byte() == b &&
                        x.byte_as_char() == b as char &&
                        x.encoding() == cfg.encoding
                ),
                "{:?}",
                err
            );
        } else {
            res.unwrap();
        }
    }

    // WrongOutputLength
    {
        let err = Config::HEX.decode::<3>(b"00000000").unwrap_err();
        assert!(
            matches!(
                &err,
                DecodeError::WrongOutputLength(x)
                if x.found() == 3 && x.expected() == 4
            ),
            "{:?}",
            err
        );
    }
    {
        assert_eq!(Config::HEX.decode::<4>(b"00000000").unwrap(), [0, 0, 0, 0]);
    }
    {
        let err = Config::HEX.decode::<5>(b"00000000").unwrap_err();
        assert!(
            matches!(
                &err,
                DecodeError::WrongOutputLength(x)
                if x.found() == 5 && x.expected() == 4
            ),
            "{:?}",
            err
        );
    }

    // WrongInputLength
    for invalid_len in [1, 3, 5, 7, 9].iter().copied() {
        let mut array = [0u8; 16];

        array[0..invalid_len].fill(b'A');

        let slice = &array[..invalid_len];

        let err = Config::HEX.decode::<100>(slice).unwrap_err();
        assert!(
            matches!(&err, DecodeError::WrongInputLength(x) if x.length() == invalid_len),
            "{:?}",
            err
        );
    }
    for invalid_len in [0, 2, 4, 6, 8].iter().copied() {
        let mut array = [0u8; 16];

        array[0..invalid_len].fill(b'A');

        let slice = &array[..invalid_len];

        let err = Config::HEX.decode::<100>(slice).unwrap_err();
        assert!(
            matches!(&err, DecodeError::WrongOutputLength { .. }),
            "{:?}",
            err
        );
    }
}
