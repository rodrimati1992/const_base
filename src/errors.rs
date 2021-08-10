use crate::{msg::IS_OK, Encoding};

macro_rules! declare_errors {
    ($($variant:ident $(= $value:expr)? ,)*) => (
        #[derive(Debug, PartialEq)]
        pub enum DecodeError {
            $( $variant($variant), )*
        }


        #[doc(hidden)]
        pub enum __DecodeErrorKind {
            $( $variant $(= $value)? , )*
        }

        impl DecodeError {
            pub(crate) const fn kind(&self) -> __DecodeErrorKind {
                match self {
                    $( Self::$variant{..} => __DecodeErrorKind::$variant, )*
                }
            }
        }

        #[doc(hidden)]
        pub mod __ {
            use core::marker::PhantomData;

            $( pub struct $variant<T>(pub(crate) PhantomData<T>); )*
        }
    )
}

declare_errors! {
    InvalidByte = IS_OK + 1,
    MismatchedOutputLength,
    InvalidInputLength,
}

#[derive(Debug, PartialEq)]
pub struct InvalidByte {
    pub(crate) index: usize,
    pub(crate) byte: u8,
    pub(crate) as_char: char,
    pub(crate) encoding: Encoding,
}

impl InvalidByte {
    pub const fn index(&self) -> usize {
        self.index
    }
    pub const fn byte(&self) -> u8 {
        self.byte
    }
    pub const fn byte_as_char(&self) -> char {
        self.as_char
    }
    pub const fn encoding(&self) -> Encoding {
        self.encoding
    }
}

#[derive(Debug, PartialEq)]
pub struct MismatchedOutputLength {
    pub(crate) expected: usize,
    pub(crate) found: usize,
}

impl MismatchedOutputLength {
    pub const fn expected(&self) -> usize {
        self.expected
    }
    pub const fn found(&self) -> usize {
        self.found
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidInputLength {
    pub(crate) length: usize,
}

impl InvalidInputLength {
    pub const fn length(&self) -> usize {
        self.length
    }
}
