use crate::{decode, decoded_len, encode, encoded_len};
use crate::{B64CharSet, Config, DecodeError, Encoding};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

const GEN_ITERS: usize = if cfg!(miri) { 10 } else { 100 };

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

                for _ in 0..GEN_ITERS {
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
                        left_pad.as_array()
                    } else {
                        left_no_pad = encode::<OUT_LEN_NO_PAD>(&input, cfg).unwrap();
                        left_no_pad.as_array()
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

                for _ in 0..GEN_ITERS {
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
fn test_encode_b64_errors() {
    // No end padding
    {
        let unpad_cfg = Config::B64.end_padding(false);

        {
            let err = unpad_cfg.encode::<2>(&[0xAB, 0xCD]).unwrap_err();
            assert!(err.expected() == 2 && err.found() == 3, "{:?}", err);
        }
        assert_eq!(unpad_cfg.encode::<3>(&[0xAB, 0xCD]).unwrap(), "q80");
        {
            let err = unpad_cfg.encode::<4>(&[0xAB, 0xCD]).unwrap_err();
            assert!(err.expected() == 4 && err.found() == 3, "{:?}", err);
        }
    }

    // With end padding
    {
        let err = Config::B64.encode::<3>(&[0xAB, 0xCD]).unwrap_err();
        assert!(err.expected() == 3 && err.found() == 4, "{:?}", err);
    }
    assert_eq!(Config::B64.encode::<4>(&[0xAB, 0xCD]).unwrap(), "q80=");
    {
        let err = Config::B64.encode::<5>(&[0xAB, 0xCD]).unwrap_err();
        assert!(err.expected() == 5 && err.found() == 4, "{:?}", err);
    }
}

#[test]
fn test_decode_base64_trailing_bits_err() {
    {
        assert_eq!(decode::<1>(b"+A", Config::B64).unwrap(), [248u8]);
        assert_eq!(decode::<1>(b"+Q", Config::B64).unwrap(), [249u8]);
        assert_eq!(decode::<1>(b"+g", Config::B64).unwrap(), [250u8]);
        assert_eq!(decode::<1>(b"+w", Config::B64).unwrap(), [251u8]);

        match decode::<1>(b"+/", Config::B64) {
            Err(DecodeError::ExcessBits(err)) => {
                assert_eq!(err.last_byte(), b'/');
            }
            x => panic!("{x:?}"),
        }
    }
    {
        for (input, output) in [
            (b"++A", [251u8, 224]),
            (b"++E", [251u8, 225]),
            (b"++I", [251u8, 226]),
            (b"++M", [251u8, 227]),
            (b"++Q", [251u8, 228]),
            (b"++U", [251u8, 229]),
            (b"++Y", [251u8, 230]),
            (b"++c", [251u8, 231]),
            (b"++g", [251u8, 232]),
            (b"++k", [251u8, 233]),
            (b"++o", [251u8, 234]),
            (b"++s", [251u8, 235]),
            (b"++w", [251u8, 236]),
            (b"++0", [251u8, 237]),
            (b"++4", [251u8, 238]),
            (b"++8", [251u8, 239]),
        ] {
            assert_eq!(decode::<2>(input, Config::B64).unwrap(), output);
        }

        for input in [b"++/", b"++B"] {
            match decode::<2>(input, Config::B64) {
                Err(DecodeError::ExcessBits(err)) => {
                    assert_eq!(err.last_byte(), *input.last().unwrap());
                }
                x => panic!("{x:?}"),
            }
        }
    }
    {
        assert_eq!(
            decode::<4>(b"+++++A", Config::B64).unwrap(),
            [251u8, 239, 190, 248]
        );
        assert_eq!(
            decode::<4>(b"+++++Q", Config::B64).unwrap(),
            [251u8, 239, 190, 249]
        );
        assert_eq!(
            decode::<4>(b"+++++g", Config::B64).unwrap(),
            [251u8, 239, 190, 250]
        );
        assert_eq!(
            decode::<4>(b"+++++w", Config::B64).unwrap(),
            [251u8, 239, 190, 251]
        );

        match decode::<4>(b"+++++/", Config::B64) {
            Err(DecodeError::ExcessBits(err)) => {
                assert_eq!(err.last_byte(), b'/');
            }
            x => panic!("{x:?}"),
        }
    }
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

    let mut invalid_bytes = crate::test_utils::ByteSet([true; 256]);
    invalid_bytes.remove_range(b'A'..=b'Z');
    invalid_bytes.remove_range(b'a'..=b'z');
    invalid_bytes.remove_range(b'0'..=b'9');

    let mut invalid_std_bytes = invalid_bytes.clone();
    invalid_std_bytes.remove(b'+');
    invalid_std_bytes.remove(b'/');

    let mut invalid_url_bytes = invalid_bytes.clone();
    invalid_url_bytes.remove(b'-');
    invalid_url_bytes.remove(b'_');

    let invalid_bytes_iters = invalid_std_bytes
        .iter()
        .map(|b| (Config::B64, b))
        .chain(invalid_url_bytes.iter().map(|b| (Config::B64_URL_SAFE, b)));

    // InvalidByte
    for (cfg, (b, is_invalid)) in invalid_bytes_iters {
        let mut bytes = *b"AA\x00A";
        bytes[2] = b;
        let res = decode::<3>(&bytes, cfg);

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
                "\n{:?}\ncfg: {:?}",
                err,
                cfg,
            );
        } else {
            res.unwrap();
        }
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

    // WrongOutputLength
    {
        let err = decode::<4>(b"AA\x00A", Config::B64).unwrap_err();
        assert!(
            matches!(err, DecodeError::WrongOutputLength { .. }),
            "{:?}",
            err
        );
    }
    {
        let err = decode::<3>(b"AAA\x00AA", Config::B64).unwrap_err();
        assert!(
            matches!(err, DecodeError::WrongOutputLength { .. }),
            "{:?}",
            err
        );
    }
    {
        let err = decode::<6>(b"AAAAA\x00A", Config::B64).unwrap_err();
        assert!(
            matches!(err, DecodeError::WrongOutputLength { .. }),
            "{:?}",
            err
        );
    }
    {
        let err = decode::<5>(b"AAAAAAA\x00", Config::B64).unwrap_err();
        assert!(
            matches!(
                &err,
                DecodeError::WrongOutputLength(x)
                if x.found() == 5 && x.expected() == 6
            ),
            "{:?}",
            err
        );
    }

    // WrongInputLength
    {
        let err = decode::<5>(b"A===", Config::B64.end_padding(true)).unwrap_err();
        assert!(
            matches!(&err, DecodeError::WrongInputLength(x) if x.length() == 1),
            "{:?}",
            err
        );
    }
    {
        let err = decode::<5>(b"A", Config::B64).unwrap_err();
        assert!(
            matches!(err, DecodeError::WrongInputLength { .. }),
            "{:?}",
            err
        );
    }
    {
        let err = decode::<5>(b"AAAAA", Config::B64).unwrap_err();
        assert!(
            matches!(err, DecodeError::WrongInputLength { .. }),
            "{:?}",
            err
        );
    }
}
