use core::{
    cmp::PartialEq,
    fmt::{self, Debug, Display},
};

pub use arr::ArrayStr;

mod arr {
    /// A utf-8 byte array
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_base::ArrayStr;
    ///
    /// static ARR_STR: ArrayStr<12> = ArrayStr::from_utf8_unwrap(*b"hello world!");
    ///
    /// assert_eq!(ARR_STR.as_str(), "hello world!");
    /// assert_eq!(ARR_STR, "hello world!");
    ///
    /// ```
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ArrayStr<const N: usize> {
        arr: [u8; N],
    }

    impl<const N: usize> ArrayStr<N> {
        /// Constructs an `ArrayStr`, returning an error if `arr` isn't valid utf-8.
        pub const fn from_utf8(arr: [u8; N]) -> Result<Self, core::str::Utf8Error> {
            match core::str::from_utf8(&arr) {
                Ok(_) => Ok(Self { arr }),
                Err(e) => Err(e),
            }
        }

        /// Constructs an `ArrayStr`
        ///
        /// # Panics
        ///
        /// Panics if `arr` isn't valid utf-8.
        pub const fn from_utf8_unwrap(arr: [u8; N]) -> Self {
            const_panic::unwrap_ok!(Self::from_utf8(arr))
        }

        /// Constructs an `ArrayStr`
        ///
        /// # Safety
        ///
        /// `arr` must be valid utf-8.
        pub const unsafe fn from_utf8_unchecked(arr: [u8; N]) -> Self {
            Self { arr }
        }

        /// Gets a reference to the wrapped byte array.
        pub const fn as_array(&self) -> &[u8; N] {
            &self.arr
        }

        /// Unwraps this into the wrapped byte array.
        pub const fn into_array(self) -> [u8; N] {
            self.arr
        }
    }
}

impl<const N: usize> ArrayStr<N> {
    /// Gets the length of this array string.
    pub const fn len(&self) -> usize {
        N
    }

    /// Gets a string slice of the entire array string.
    pub const fn as_str(&self) -> &str {
        // SAFETY: the constructor functions check that the string is valid utf-8
        unsafe { core::str::from_utf8_unchecked(self.as_array()) }
    }
}

impl<const N: usize> PartialEq<str> for ArrayStr<N> {
    fn eq(&self, rhs: &str) -> bool {
        self.as_str() == rhs
    }
}

impl<const N: usize> PartialEq<&str> for ArrayStr<N> {
    fn eq(&self, rhs: &&str) -> bool {
        self.as_str() == *rhs
    }
}

impl<const N: usize> PartialEq<ArrayStr<N>> for str {
    fn eq(&self, rhs: &ArrayStr<N>) -> bool {
        self == rhs.as_str()
    }
}

impl<const N: usize> PartialEq<ArrayStr<N>> for &str {
    fn eq(&self, rhs: &ArrayStr<N>) -> bool {
        *self == rhs.as_str()
    }
}

impl<const N: usize> Display for ArrayStr<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

impl<const N: usize> Debug for ArrayStr<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}
