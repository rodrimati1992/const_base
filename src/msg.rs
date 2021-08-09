#![allow(non_camel_case_types)]

use core::marker::PhantomData;

use crate::errors::{DecodeError as DE, __DecodeErrorKind as DEK};

pub trait __ConstToType<const VARIANT: isize, const B: usize, const C: usize, const D: char> {
    type Type;
    const V: Self::Type;
}

pub use crate::errors::__::*;

pub const IS_OK: isize = 0;

pub struct IsOk;

pub struct index<const X: usize>;
pub struct expected<const X: usize>;
pub struct found<const X: usize>;
pub struct byte<const X: usize>;
pub struct length<const X: usize>;
pub struct byte_as_char<const X: char>;

#[doc(hidden)]
pub type __ResultTuple = (isize, usize, usize, char);

#[doc(hidden)]
pub const fn __decode_res_to_tuple<T>(res: &Result<T, DE>) -> __ResultTuple {
    match res {
        Ok(_) => (IS_OK, 0, 0, '\0'),
        Err(err) => {
            let kind = err.kind() as isize;

            match err {
                DE::InvalidByte(x) => (kind, x.index, x.byte as _, x.as_char),
                DE::MismatchedOutputLength(x) => (kind, x.expected, x.found, '\0'),
                DE::InvalidInputLength(x) => (kind, x.length, 0, '\0'),
            }
        }
    }
}

pub const fn __encode_res_to_tuple<T>(
    res: &Result<T, crate::errors::MismatchedOutputLength>,
) -> __ResultTuple {
    match res {
        Ok(_) => (IS_OK, 0, 0, '\0'),
        Err(x) => (DEK::MismatchedOutputLength as _, x.expected, x.found, '\0'),
    }
}

impl<const B: usize, const C: usize, const D: char> __ConstToType<0, B, C, D> for () {
    type Type = IsOk;
    const V: IsOk = IsOk;
}

impl<const B: usize, const C: usize, const D: char>
    __ConstToType<{ DEK::InvalidByte as isize }, B, C, D> for ()
{
    type Type = InvalidByte<(index<B>, byte<C>, byte_as_char<D>)>;

    const V: Self::Type = InvalidByte(PhantomData);
}

impl<const B: usize, const C: usize, const D: char>
    __ConstToType<{ DEK::MismatchedOutputLength as isize }, B, C, D> for ()
{
    type Type = MismatchedOutputLength<(expected<B>, found<C>)>;

    const V: Self::Type = MismatchedOutputLength(PhantomData);
}

impl<const B: usize, const C: usize, const D: char>
    __ConstToType<{ DEK::InvalidInputLength as isize }, B, C, D> for ()
{
    type Type = InvalidInputLength<length<B>>;

    const V: Self::Type = InvalidInputLength(PhantomData);
}
