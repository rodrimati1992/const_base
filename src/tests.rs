use crate::{decode, decoded_len, encode, encoded_len};
use crate::{B64CharSet, Config, DecodeError, Encoding};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[test]
fn test_encode_base64() {
    let mut rng = SmallRng::seed_from_u64(6249204433781597762);

    macro_rules! test_encode {
        ($in_length:literal) => {{
            let cfgs = [
                (base64::STANDARD, Config::B64),
                (base64::URL_SAFE, Config::B64_URL_SAFE),
            ]
            .iter()
            .copied();
            let pads = [false, true].iter().copied();

            const OUT_LEN_NO_PAD: usize = encoded_len($in_length, Config::B64.end_padding(false));
            const OUT_LEN_PAD: usize = encoded_len($in_length, Config::B64.end_padding(true));

            assert_eq!(
                encoded_len($in_length, Config::B64_URL_SAFE.end_padding(false)),
                OUT_LEN_NO_PAD
            );

            for ((mut b64_cfg, mut cfg), pad) in itertools::iproduct!(cfgs, pads) {
                b64_cfg = b64_cfg.pad(pad);
                cfg = cfg.end_padding(pad);

                for _ in 0..10 {
                    let input = rng.gen::<[u8; $in_length]>();

                    let mut out_no_pad = [0u8; OUT_LEN_PAD];
                    let written = base64::encode_config_slice(&input, b64_cfg, &mut out_no_pad);
                    if pad {
                        assert_eq!(OUT_LEN_PAD, written);
                    } else {
                        assert_eq!(OUT_LEN_NO_PAD, written);
                    }

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
}

#[test]
fn test_decode_base64() {
    let mut rng = SmallRng::seed_from_u64(6249204433781597762);

    macro_rules! test_decode {
        ($unencoded_len:expr, $encoded_length:expr, $encoded_length_rup:expr) => {{
            let cfgs = [
                (base64::STANDARD, Config::B64),
                (base64::URL_SAFE, Config::B64_URL_SAFE),
            ]
            .iter()
            .copied();
            let pads = [false, true].iter().copied();

            const DECODED_LEN: usize =
                decoded_len(&[0; $encoded_length], Config::B64.end_padding(false));

            assert_eq!(
                decoded_len(
                    &[0; $encoded_length],
                    Config::B64_URL_SAFE.end_padding(false)
                ),
                DECODED_LEN,
                "FOO",
            );
            assert_eq!($unencoded_len, DECODED_LEN, "QUX");

            for ((mut b64_cfg, mut cfg), pad) in itertools::iproduct!(cfgs, pads) {
                b64_cfg = b64_cfg.pad(pad);
                cfg = cfg.end_padding(pad);

                for _ in 0..10 {
                    let input = rng.gen::<[u8; DECODED_LEN]>();

                    let mut encoded_no_pad = [0u8; $encoded_length + 4];
                    let encoded_no_pad = {
                        let written_enc =
                            base64::encode_config_slice(&input, b64_cfg, &mut encoded_no_pad);

                        if pad {
                            assert_eq!($encoded_length_rup, written_enc, "BAR0");
                        } else {
                            assert_eq!($encoded_length, written_enc, "BAR1");
                        }

                        &encoded_no_pad[..written_enc]
                    };

                    let mut decoded_no_pad = [0u8; $encoded_length];

                    let written_dec =
                        base64::decode_config_slice(encoded_no_pad, b64_cfg, &mut decoded_no_pad)
                            .unwrap();

                    assert_eq!(DECODED_LEN, written_dec, "BAZ");

                    let left = &decode::<DECODED_LEN>(encoded_no_pad, cfg).unwrap();
                    let right = &decoded_no_pad[..written_dec];

                    assert_eq!(
                        left, right,
                        "\ninput:{:x?}\n{:x?}\n{:x?}",
                        input, left, right,
                    );
                }
            }
        }};
    }

    test_decode! {0, 0, 0}
    test_decode! {1, 2, 4}
    test_decode! {2, 3, 4}
    test_decode! {3, 4, 4}
    test_decode! {4, 6, 8}
    test_decode! {5, 7, 8}
    test_decode! {6, 8, 8}
    test_decode! {7, 10, 12}
    test_decode! {8, 11, 12}
    test_decode! {9, 12, 12}
}

#[test]
fn test_decode_base64_errors() {
    {
        // intentionally padded to this length
        let ok = decode::<5>(b"BAACAAA==", Config::B64.end_padding(true)).unwrap();
        assert_eq!(ok, [4, 0, 2, 0, 0]);
    }
    {
        let ok = decode::<5>(b"BAACAAA=", Config::B64.end_padding(true)).unwrap();
        assert_eq!(ok, [4, 0, 2, 0, 0]);
    }
    {
        let ok = decode::<4>(b"BAACAA==", Config::B64.end_padding(true)).unwrap();
        assert_eq!(ok, [4, 0, 2, 0]);
    }
    {
        let ok = decode::<1>(b"BA==", Config::B64.end_padding(true)).unwrap();
        assert_eq!(ok, [4]);
    }
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
        assert!(
            matches!(
                &err,
                DecodeError::InvalidByte(x)
                if x.index() == 2 &&
                    x.byte() == b'\x00' &&
                    x.byte_as_char() == '\x00' &&
                    x.encoding() == Encoding::Base64(B64CharSet::Standard)
            ),
            "{:?}",
            err
        );
    }
    {
        let err = decode::<4>(b"AAA\x00AA", Config::B64).unwrap_err();
        assert!(matches!(err, DecodeError::InvalidByte { .. }), "{:?}", err);
    }
    {
        let err = decode::<5>(b"AAAAA=A", Config::B64_URL_SAFE).unwrap_err();
        assert!(
            matches!(
                &err,
                DecodeError::InvalidByte(x)
                if x.index() == 5 &&
                    x.byte() == b'=' &&
                    x.byte_as_char() == '=' &&
                    x.encoding() == Encoding::Base64(B64CharSet::UrlSafe)
            ),
            "{:?}",
            err
        );
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
    {
        let err = decode::<5>(b"A===", Config::B64.end_padding(true)).unwrap_err();
        assert!(
            matches!(&err, DecodeError::InvalidInputLength(x) if x.length() == 1),
            "{:?}",
            err
        );
    }
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
