This changelog is a summary of the changes made in each release.

# 0.2.0

### 0.2.0

Added `ArrayStr` struct.

Changed `encode` function and macro to return `ArrayStr` instead of byte array.

Renamed `InvalidInputLength` variant and struct to `WrongInputLength`

Renamed `MismatchedOutputLength` variant and struct to `WrongOutputLength`

Added `unwrap` associated function to all error types.

Swapped `expected` and `found` values for `WrongOutputLength`

Added `ExcessBits` error struct and as a variant of `DecodeError`.

Fixed excess bits detection to base-64 and base-32, it previously just ignored those bits,

Removed `unwrap_or` macro.

Bumped Minimum Supported Rust Version to 1.64.0.

Bumped edition to 2021.

Added `const_panic = "0.2"` dependency, to improve compile-time errors



# 0.1

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



