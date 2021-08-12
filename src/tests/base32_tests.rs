use crate::{
    __priv_utils::round_up_to_multiple_usize, decode, decoded_len, encode, encoded_len, Config,
    DecodeError,
};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use data_encoding as base32;

#[test]
fn test_encode_base32() {
    let mut rng = SmallRng::seed_from_u64(6249204433781597762);

    macro_rules! test_encode {
        ($in_length:literal) => {{
            let cfgs = [
                (base32::BASE32, Config::B32, true),
                (base32::BASE32_NOPAD, Config::B32.end_padding(false), false),
            ];

            const OUT_LEN_PAD: usize = encoded_len($in_length, Config::B32.end_padding(true));
            const OUT_LEN_NO_PAD: usize = encoded_len($in_length, Config::B32.end_padding(false));

            for &(ref b32_cfg, cfg, pad) in cfgs.iter() {
                for _ in 0..100 {
                    let input = rng.gen::<[u8; $in_length]>();

                    let written = if pad { OUT_LEN_PAD } else { OUT_LEN_NO_PAD };

                    let mut out_no_pad = [0u8; OUT_LEN_PAD];
                    let out_no_pad = &mut out_no_pad[..written];

                    assert_eq!(b32_cfg.encode_len(input.len()), written);
                    b32_cfg.encode_mut(&input, out_no_pad);

                    let left_no_pad;
                    let left_pad;
                    let left: &[_] = if pad {
                        left_pad = encode::<OUT_LEN_PAD>(&input, cfg).unwrap();
                        &left_pad
                    } else {
                        left_no_pad = encode::<OUT_LEN_NO_PAD>(&input, cfg).unwrap();
                        &left_no_pad
                    };
                    let right = &out_no_pad[..written];

                    assert_eq!(
                        core::str::from_utf8(left).unwrap(),
                        core::str::from_utf8(right).unwrap(),
                        "\ninput:{:x?}\n{:x?}\n{:x?}",
                        input,
                        left,
                        right,
                    );
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
    test_encode! {6}
    test_encode! {7}
    test_encode! {8}
    test_encode! {9}
    test_encode! {10}
    test_encode! {11}
    test_encode! {12}
    test_encode! {13}
}

#[test]
fn test_encode_b32_errors() {
    // No end padding
    {
        let unpad_cfg = Config::B32.end_padding(false);

        {
            let err = unpad_cfg.encode::<3>(&[0xAB, 0xCD]).unwrap_err();
            assert!(err.expected() == 3 && err.found() == 4, "{:?}", err);
        }
        assert_eq!(unpad_cfg.encode::<4>(&[0xAB, 0xCD]).unwrap(), *b"VPGQ");
        {
            let err = unpad_cfg.encode::<5>(&[0xAB, 0xCD]).unwrap_err();
            assert!(err.expected() == 5 && err.found() == 4, "{:?}", err);
        }
    }

    // With end padding
    {
        let err = Config::B32.encode::<7>(&[0xAB, 0xCD]).unwrap_err();
        assert!(err.expected() == 7 && err.found() == 8, "{:?}", err);
    }
    assert_eq!(
        Config::B32.encode::<8>(&[0xAB, 0xCD]).unwrap(),
        *b"VPGQ===="
    );
    {
        let err = Config::B32.encode::<9>(&[0xAB, 0xCD]).unwrap_err();
        assert!(err.expected() == 9 && err.found() == 8, "{:?}", err);
    }
}

#[test]
fn test_decode_base32() {
    let mut rng = SmallRng::seed_from_u64(6249204433781597762);

    macro_rules! test_decode {
        ($unencoded_len:expr, $encoded_length:expr, $encoded_length_rup:expr) => {{
            let cfgs = [
                (base32::BASE32, Config::B32, true),
                (base32::BASE32_NOPAD, Config::B32.end_padding(false), false),
            ];

            const DECODED_LEN: usize =
                decoded_len(&[0; $encoded_length], Config::B32.end_padding(false));

            assert_eq!($unencoded_len, DECODED_LEN, "QUX");

            for &(ref b32_cfg, cfg, pad) in cfgs.iter() {
                for _ in 0..100 {
                    let input = rng.gen::<[u8; DECODED_LEN]>();

                    let written_enc = if pad {
                        $encoded_length_rup
                    } else {
                        $encoded_length
                    };

                    let mut encoded = [0u8; $encoded_length + 8];
                    let encoded = &mut encoded[..written_enc];

                    assert_eq!(b32_cfg.encode_len($unencoded_len), written_enc);
                    b32_cfg.encode_mut(&input, encoded);

                    let mut decoded = [0u8; round_up_to_multiple_usize(DECODED_LEN, 5)];
                    let decoded = &mut decoded[..b32_cfg.decode_len(written_enc).unwrap()];

                    let written_dec = b32_cfg.decode_mut(encoded, decoded).unwrap();
                    let decoded = &decoded[..written_dec];

                    let left = decode::<DECODED_LEN>(encoded, cfg).expect("qux");

                    assert_eq!(
                        left, decoded,
                        "\ninput:{:x?}\nleft:{:x?}\ndecoded:{:x?}",
                        input, left, decoded,
                    );
                }
            }
        }};
    }

    test_decode! {0, 0, 0}
    test_decode! {1, 2, 8}
    test_decode! {2, 4, 8}
    test_decode! {3, 5, 8}
    test_decode! {4, 7, 8}
    test_decode! {5, 8, 8}
    test_decode! {6, 10, 16}
    test_decode! {7, 12, 16}
    test_decode! {8, 13, 16}
    test_decode! {9, 15, 16}
    test_decode! {10, 16, 16}
    test_decode! {11, 18, 24}
}

#[test]
fn test_decode_base32_errors() {
    {
        // intentionally padded to this length
        let ok = decode::<4>(b"BAACAAA=", Config::B32.end_padding(true)).unwrap();
        assert_eq!(ok, [8, 0, 32, 0]);
    }
    {
        let ok = decode::<3>(b"BAACA===", Config::B32.end_padding(true)).unwrap();
        assert_eq!(ok, [8, 0, 32]);
    }
    {
        let ok = decode::<1>(b"BA======", Config::B32.end_padding(true)).unwrap();
        assert_eq!(ok, [8]);
    }
    {
        let ok = decode::<0>(b"", Config::B32.end_padding(true)).unwrap();
        assert_eq!(ok, []);
    }

    let mut invalid_bytes = crate::test_utils::ByteSet([true; 256]);
    invalid_bytes.remove_range(b'A'..=b'Z');

    let mut invalid_std_bytes = invalid_bytes.clone();
    invalid_std_bytes.remove_range(b'2'..=b'7');

    let invalid_bytes_iters = invalid_std_bytes.iter().map(|b| (Config::B32, b));

    // InvalidByte
    for (cfg, (b, is_invalid)) in invalid_bytes_iters {
        let mut bytes = *b"AA\x00A====";
        bytes[2] = b;
        let res = decode::<2>(&bytes, cfg);

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
    {
        let err = decode::<5>(b"AAA0AAAA", Config::B32).unwrap_err();
        assert!(matches!(err, DecodeError::InvalidByte { .. }), "{:?}", err);
    }
    {
        let err = decode::<5>(b"AAAAAAA\x00", Config::B32).unwrap_err();
        assert!(matches!(err, DecodeError::InvalidByte { .. }), "{:?}", err);
    }

    // MismatchedOutputLength
    {
        let err = decode::<4>(b"AA\x00A", Config::B32).unwrap_err();
        assert!(
            matches!(err, DecodeError::MismatchedOutputLength { .. }),
            "{:?}",
            err
        );
    }
    {
        let err = decode::<3>(b"AAA\x00AAA", Config::B32).unwrap_err();
        assert!(
            matches!(err, DecodeError::MismatchedOutputLength { .. }),
            "{:?}",
            err
        );
    }
    {
        let err = decode::<6>(b"AAAAA\x00A", Config::B32).unwrap_err();
        assert!(
            matches!(err, DecodeError::MismatchedOutputLength { .. }),
            "{:?}",
            err
        );
    }
    {
        let err = decode::<5>(b"AAAAAAA\x00BB", Config::B32).unwrap_err();
        assert!(
            matches!(
                &err,
                DecodeError::MismatchedOutputLength(x)
                if x.expected() == 5 && x.found() == 6
            ),
            "{:?}",
            err
        );
    }

    // InvalidInputLength
    for invalid_len in [1, 3, 6, 9, 11].iter().copied() {
        let mut array = [0u8; 16];

        array[0..invalid_len].fill(b'A');
        array[invalid_len..].fill(b'=');

        let slice = &array[..round_up_to_multiple_usize(invalid_len, 8)];

        let err = decode::<100>(slice, Config::B32.end_padding(true)).unwrap_err();
        assert!(
            matches!(&err, DecodeError::InvalidInputLength(x) if x.length() == invalid_len),
            "{:?}",
            err
        );
    }
}
