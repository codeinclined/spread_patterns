# spread_patterns

Rust 🦀 macros to help with pattern matching strings, etc. Not much here yet.

## Strings

### has_prefix! / has_suffix!

Proc macros which allow you to do pattern matching against the beginning or end of a string a little more easily with functions like `as_bytes()`. Any ASCII characters will be included in the pattern as byte literals and UTF-8 will be rendered as a series of hexadecimal bytes.

#### Example

```rust
use spread_patterns::{has_prefix, has_suffix};

pub fn main() {
    let my_str = "Hello 🌎! This supports unicode and ASCII 🎉";

    let msg = match my_str.as_bytes() {
        has_prefix!("Goodbye!") => "This won't match",
        has_prefix!("Hello 🌎") => "Found prefix!",
        has_suffix!("🎉") => "Found suffix!",
        _ => "Found nothing 🫤",
    };

    println!("Result: {}", msg);
}
```

> Running the code above will produce "Result: Found prefix!". The suffix would match if you changed the order of the prongs.

The code above expands to the following (using cargo-expand):

```rust
#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use spread_patterns::{has_prefix, has_suffix};
pub fn main() {
    let my_str = "Hello 🌎! This supports unicode and ASCII 🎉";
    let msg = match my_str.as_bytes() {
        [b'G', b'o', b'o', b'd', b'b', b'y', b'e', b'!', ..] => "This won't match",
        [b'H', b'e', b'l', b'l', b'o', b' ', 0xf0, 0x9f, 0x8c, 0x8e, ..] => {
            "Found prefix!"
        }
        [.., 0xf0, 0x9f, 0x8e, 0x89] => "Found suffix!",
        _ => "Found nothing 🫤",
    };
    {
        ::std::io::_print(format_args!("Result: {0}\n", msg));
    };
}
```
