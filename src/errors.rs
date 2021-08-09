use crate::Encoding;

#[derive(Debug)]
pub enum DecodeError {
    InvalidByte(InvalidByte),
    MismatchedOutputLength(MismatchedOutputLength),
    InvalidInputLength(InvalidInputLength),
}

#[derive(Debug)]
pub struct InvalidByte {
    pub(crate) position: usize,
    pub(crate) byte: u8,
    pub(crate) as_char: char,
    pub(crate) encoding: Encoding,
}

impl InvalidByte {
    pub const fn position(&self) -> usize {
        self.position
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct InvalidInputLength {
    pub(crate) length: usize,
}

impl InvalidInputLength {
    pub const fn length(&self) -> usize {
        self.length
    }
}
