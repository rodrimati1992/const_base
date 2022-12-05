[![Rust](https://github.com/rodrimati1992/const_base/workflows/Rust/badge.svg)](https://github.com/rodrimati1992/const_base/actions)
[![crates-io](https://img.shields.io/crates/v/const_base.svg)](https://crates.io/crates/const_base)
[![api-docs](https://docs.rs/const_base/badge.svg)](https://docs.rs/const_base/*)


For decoding/encoding base 64/32/16 strings at compile-time.

# Examples

### Encoding

```rust
use const_base::{encode_as_str, Config};

{
    // the encoding macros can take both `&str` and `&[u8]` constants.
    const OUTA: &str = encode_as_str!("foo", Config::B64);
    const OUTB: &str = encode_as_str!(b"foo", Config::B64);
    
    assert_eq!(OUTA, "Zm9v");
    assert_eq!(OUTB, "Zm9v");
}
{
    const BYTES: &[u8] = b"hello";

    // the encoding macros can encode_as_str non-literal constants
    const OUT: &str = encode_as_str!(BYTES, Config::B64_URL_SAFE);
    
    assert_eq!(OUT, "aGVsbG8=");
}
```

### Decoding

```rust
use const_base::{decode, Config};

{
    const OUT: &[u8] = decode!("MZXW6===", Config::B32);
    
    assert_eq!(OUT, b"foo");
}
{
    const BYTES: &[u8] = b"f000";

    // this macro can decode non-literal constants
    const OUT: &[u8] = decode!(BYTES, Config::HEX);
    
    assert_eq!(OUT, &[0xF0, 0x00]);
}
```

# No-std support

`const_base` is `#![no_std]`, it can be used anywhere Rust can be used.

# Minimum Supported Rust Version

`const_base` requires Rust 1.64.0.
