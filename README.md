[![Rust](https://github.com/rodrimati1992/const_base/workflows/Rust/badge.svg)](https://github.com/rodrimati1992/const_base/actions)
[![crates-io](https://img.shields.io/crates/v/const_base.svg)](https://crates.io/crates/const_base)
[![api-docs](https://docs.rs/const_base/badge.svg)](https://docs.rs/const_base/*)


For decoding/encoding base 64 strings at compile-time.

# Examples

### Encoding

```rust
use const_base::{encode, encode_as_str, Config};

{
    // the encoding macros can take both `&str` and `&[u8]` constants.
    const OUTA: &[u8; 4] = encode!("foo", Config::B64);
    const OUTB: &[u8; 4] = encode!(b"foo", Config::B64);
    
    assert_eq!(OUTA, b"Zm9v");
    assert_eq!(OUTB, b"Zm9v");
}
{
    const BYTES: &[u8] = b"hello";

    // the encoding macros can encode non-literal constants
    const OUT: &str = encode_as_str!(BYTES, Config::B64_URL_SAFE);
    
    assert_eq!(OUT, "aGVsbG8=");
}
```

### Decoding

```rust
use const_base::{decode, Config};

{
    const OUT: &[u8] = decode!("Zm9v", Config::B64);
    
    assert_eq!(OUT, b"foo");
}
{
    const BYTES: &str = "aGVsbG8";

    // this macro can decode non-literal constants
    const OUT: &[u8] = decode!(BYTES, Config::B64_URL_SAFE.end_padding(false));
    
    assert_eq!(OUT, b"hello");
}
```

# No-std support

`const_base` is `#![no_std]`, it can be used anywhere Rust can be used.

# Minimum Supported Rust Version

`const_base` requires Rust 1.51.0, because it uses const generics.

