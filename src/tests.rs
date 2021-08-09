use crate::{decode, decoded_len, encode, encoded_len};
use crate::{Config, DecodeError};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[test]
fn test_encode_base64() {
    let mut rng = SmallRng::seed_from_u64(6249204433781597762);

    macro_rules! test_encode {
        ($in_length:literal) => {
            let cfgs = [
                (base64::STANDARD.pad(false), Config::B64),
                (base64::URL_SAFE.pad(false), Config::B64_URL_SAFE),
            ];

            for (b64_cfg, cfg) in cfgs.iter().copied() {
                const OUT_LEN_NO_PAD: usize = encoded_len($in_length, Config::B64);

                assert_eq!(
                    encoded_len($in_length, Config::B64_URL_SAFE),
                    OUT_LEN_NO_PAD
                );

                for _ in 0..10 {
                    let input = rng.gen::<[u8; $in_length]>();

                    let mut out_no_pad = [0u8; OUT_LEN_NO_PAD];
                    let written = base64::encode_config_slice(&input, b64_cfg, &mut out_no_pad);
                    assert_eq!(OUT_LEN_NO_PAD, written);

                    let left = &encode::<OUT_LEN_NO_PAD>(&input, cfg);
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
        };
    }

    test_encode! {0}
    test_encode! {1}
    test_encode! {2}
    test_encode! {3}
    test_encode! {4}
    test_encode! {5}
    test_encode! {6}
    test_encode! {7}
}

#[test]
fn test_decode_base64() {
    let mut rng = SmallRng::seed_from_u64(6249204433781597762);

    macro_rules! test_decode {
        ($unencoded_len:expr, $encoded_length:expr) => {
            let cfgs = [
                (base64::STANDARD.pad(false), Config::B64),
                (base64::URL_SAFE.pad(false), Config::B64_URL_SAFE),
            ];

            for (b64_cfg, cfg) in cfgs.iter().copied() {
                const DECODED_LEN: usize = decoded_len($encoded_length, Config::B64);

                assert_eq!(
                    decoded_len($encoded_length, Config::B64_URL_SAFE),
                    DECODED_LEN
                );
                assert_eq!($unencoded_len, DECODED_LEN);

                for _ in 0..10 {
                    let input = rng.gen::<[u8; DECODED_LEN]>();

                    let mut encoded_no_pad = [0u8; $encoded_length];
                    let encoded_no_pad = {
                        let written =
                            base64::encode_config_slice(&input, b64_cfg, &mut encoded_no_pad);
                        assert_eq!($encoded_length, written);
                        &encoded_no_pad[..written]
                    };

                    let mut decoded_no_pad = [0u8; $encoded_length];
                    let written =
                        base64::decode_config_slice(encoded_no_pad, b64_cfg, &mut decoded_no_pad)
                            .unwrap();

                    assert_eq!(DECODED_LEN, written);

                    let left = &decode::<DECODED_LEN>(encoded_no_pad, cfg).unwrap();
                    let right = &decoded_no_pad[..written];

                    assert_eq!(
                        left, right,
                        "\ninput:{:x?}\n{:x?}\n{:x?}",
                        input, left, right,
                    );
                }
            }
        };
    }

    test_decode! {0, 0}
    test_decode! {1, 2}
    test_decode! {2, 3}
    test_decode! {3, 4}
    test_decode! {4, 6}
    test_decode! {5, 7}
    test_decode! {6, 8}
    test_decode! {7, 10}
    test_decode! {8, 11}
    test_decode! {9, 12}
}

#[test]
fn test_decode_base64_errors() {
    {
        let ok = decode::<4>(b"BAACAA", Config::B64).unwrap();
        assert_eq!(ok, [4, 0, 2, 0]);
    }
    {
        let ok = decode::<5>(b"BAACAAA", Config::B64).unwrap();
        assert_eq!(ok, [4, 0, 2, 0, 0]);
    }

    // InvalidByte
    {
        let err = decode::<3>(b"AA\x00A", Config::B64).unwrap_err();
        assert!(matches!(err, DecodeError::InvalidByte { .. }), "{:?}", err);
    }
    {
        let err = decode::<4>(b"AAA\x00AA", Config::B64).unwrap_err();
        assert!(matches!(err, DecodeError::InvalidByte { .. }), "{:?}", err);
    }
    {
        let err = decode::<5>(b"AAAAA\x00A", Config::B64).unwrap_err();
        assert!(matches!(err, DecodeError::InvalidByte { .. }), "{:?}", err);
    }
    {
        let err = decode::<6>(b"AAAAAAA\x00", Config::B64).unwrap_err();
        assert!(matches!(err, DecodeError::InvalidByte { .. }), "{:?}", err);
    }

    // MismatchedOutputLength
    {
        let err = decode::<4>(b"AA\x00A", Config::B64).unwrap_err();
        assert!(
            matches!(err, DecodeError::MismatchedOutputLength { .. }),
            "{:?}",
            err
        );
    }
    {
        let err = decode::<3>(b"AAA\x00AA", Config::B64).unwrap_err();
        assert!(
            matches!(err, DecodeError::MismatchedOutputLength { .. }),
            "{:?}",
            err
        );
    }
    {
        let err = decode::<6>(b"AAAAA\x00A", Config::B64).unwrap_err();
        assert!(
            matches!(err, DecodeError::MismatchedOutputLength { .. }),
            "{:?}",
            err
        );
    }
    {
        let err = decode::<5>(b"AAAAAAA\x00", Config::B64).unwrap_err();
        assert!(
            matches!(err, DecodeError::MismatchedOutputLength { .. }),
            "{:?}",
            err
        );
    }

    // InvalidInputLength
    {
        let err = decode::<5>(b"A", Config::B64).unwrap_err();
        assert!(
            matches!(err, DecodeError::InvalidInputLength { .. }),
            "{:?}",
            err
        );
    }
    {
        let err = decode::<5>(b"AAAAA", Config::B64).unwrap_err();
        assert!(
            matches!(err, DecodeError::InvalidInputLength { .. }),
            "{:?}",
            err
        );
    }
}
