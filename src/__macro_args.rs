use crate::{decoded_len, encoded_len, Config};

macro_rules! pass_types {
    ($length_fn:expr, $ty:ident) => {
        pass_types_inner! {
            $length_fn,
            for[const N: usize] $ty<&'static [u8; N]>,
            |self| self.0
        }
        pass_types_inner! {
            $length_fn,
            for[] $ty<&'static [u8]>,
            |self| self.0
        }

        pass_types_inner! {
            $length_fn,
            for[] $ty<&'static str>,
            |self| self.0.as_bytes()
        }
    };
}

macro_rules! pass_types_inner {
    (
        $length_fn:expr,
        for[$($gens:tt)*] $ty_const:ident<$ty:ty>,
        |$self:ident| $as_bytes:expr
    ) => {
        impl<$($gens)*> $ty_const<$ty> {
            pub const fn conv($self) -> CodecArgs {
                let input = $as_bytes;
                let cfg = $self.1;
                CodecArgs {
                    input,
                    cfg,
                    out_len: $length_fn(input, cfg),
                }
            }
        }
    };
}

pub struct DecodeArgsFrom<T>(pub T, pub Config);

pub struct EncodeArgsFrom<T>(pub T, pub Config);

pub struct CodecArgs {
    pub input: &'static [u8],
    pub cfg: Config,
    pub out_len: usize,
}

pass_types! {decoded_len, DecodeArgsFrom}
pass_types! {_encoded_len, EncodeArgsFrom}

#[inline(always)]
const fn _encoded_len(bytes: &[u8], cfg: Config) -> usize {
    encoded_len(bytes.len(), cfg)
}
