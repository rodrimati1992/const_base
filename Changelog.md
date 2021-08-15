This changelog is a summary of the changes made in each release.

# 0.1

### 0.1.2

Fixed representation of internally used union to `#[repr(C)]`.

### 0.1.1

Added base 32 and hexadecimal support.

Added `Config::B32`, `Config::HEX`, and `Config::HEX_LOWER` associated constants.

Added `Encoding::Base32` and `Encoding::Hex` variants.

Added `B32CharSet` enum with `Standard` variant.

Added `HexCharSet` enum with `Lowercase` and `Uppercase` variants.


### 0.1.0

Initial version with only base 64 support.

Added `DecodeError` error enum.

Added `InvalidByte`, `InvalidInputLength`, and `MismatchedOutputLength` error structs.

Added `utils` module, with `repeated` function.

Added `decode`, `encode`, `encode_as_str`, and `unwrap_or` macros.

Added `decode`, `encode`, `decoded_len`, and `encoded_len` functions.

Added `Config` struct, with `B64` and `B64_URL_SAFE` associated constants.

Added `Encoding`, with only the `Base64` variant.

Added `B64CharSet`, with the `Standard` ànd `ÙrlSafe` variants.



