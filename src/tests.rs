use crate::{encode, encoded_len, Config};

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

                    println!("\ninput:{:x?}\n{:x?}\n{:x?}", input, left, right,);
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
