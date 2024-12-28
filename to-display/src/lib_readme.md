A zero-allocation library for formatting non-displayable types like `Option<T>`.

This crate provides the [`ToDisplay`] trait which enables customizable string formatting
for types that may not implement `std::fmt::Display` directly.

# Quick Example

```rust
use to_display::ToDisplay;
use to_display::DisplayConfig;

// Option
assert_eq!(Some(1u64).display().to_string(), "1");
assert_eq!(Some(1u64).display().verbose().to_string(), "Some(1)");
assert_eq!(None::<u64>.display().verbose().to_string(), "None");

// Result
assert_eq!(Ok::<u64,u32>(1).display().to_string(), "Ok(1)");
assert_eq!(Err::<u64,u32>(2).display().to_string(), "Err(2)");

// Collection Limits
let vec = vec![1, 2, 3, 4, 5];
assert_eq!(vec.display().limit_items(3).to_string(), "[1, 2, 3, ...]");

// Time Formatting
# use std::time::Instant;
# #[cfg(feature = "std-time")]
# {
let time = Instant::now();
println!("{}", time.display()); // "10:10:10.000000"
# }
```

# Features

- Zero-allocation string formatting
- Configurable display options (verbose mode, time formats, etc.)
- Extensive type support including generics and collections
- Optional time formatting support via feature flags

# Supported Types

## Primitive Types
The following types implement [`ToDisplay`] out of the box:
- Integers: `i8`-`i128`, `u8`-`u128`, `isize`, `usize`
- Floating point: `f32`, `f64`
- Other primitives: `bool`, `char`, `String`, `&str`
- Network types: `IpAddr`, `Ipv4Addr`, `Ipv6Addr`, `SocketAddr`
- Non-zero integers: `NonZeroI8`-`NonZeroU128`

## Time Types
- `std::time::Instant` (requires `std-time` feature)
```rust
# use to_display::ToDisplay;
# use std::time::Instant;
# #[cfg(feature = "std-time")]
# {
let now = Instant::now();
println!("{}", now.display()); // -> "10:10:10.000000"
# }
```

- `tokio::time::Instant` (requires `tokio-time` feature)
```rust
# use to_display::ToDisplay;
# use to_display::DisplayConfig;
# use std::time::Instant;
# #[cfg(feature = "std-time")]
# {
let now = Instant::now();
println!("{}", now.display().use_full_time()); // -> "2024-12-28T23:37:31.646201Z+0800"
# }
```

## Generic Types
- `Option<T>` where `T: ToDisplay`
- `Result<T, E>` where `T: ToDisplay, E: Display`
- `Vec<T>` and slices `[T]` where `T: ToDisplay`
- `BTreeMap<K, V>` where `K: ToDisplay, V: ToDisplay`

# Implementation Methods

## Using Derive Macro
For types that already implement `Display`:
```rust
#[derive(to_display::ToDisplay)]
struct Foo(u64);

impl std::fmt::Display for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Foo({})", self.0)
    }
}
```

## Manual Implementation

For types requiring custom display logic or types that don't implement [`Display`],
manual implementation of [`ToDisplay`] is needed. This involves two steps:

1. Implement [`ToDisplay`] to create a display wrapper type (e.g., `DisplayFoo`)
2. Implement [`Display`] for the wrapper type to define the formatting logic

Here's a complete example:

```rust
use std::fmt;
use to_display::{Context, ToDisplay, DisplayConfig};

struct Foo(u64);

impl ToDisplay for Foo {
    type Displayer<'a> = DisplayFoo<'a>;

    fn display_with_context(&self, context: Context) -> Self::Displayer<'_> {
        DisplayFoo { foo: self, context }
    }
}

struct DisplayFoo<'a> {
    foo: &'a Foo,
    context: Context,
}

impl fmt::Display for DisplayFoo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Foo({})", self.foo.0)
    }
}

// Optional: Enable display configuration methods
impl<'a> DisplayConfig for DisplayFoo<'a> {
    fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}

assert_eq!(Foo(42).display().to_string(), "Foo(42)");
```

# Feature Flags

- `std-time`: Enables support for `std::time::Instant`
- `tokio-time`: Enables support for `tokio::time::Instant`

[`Display`]: std::fmt::Display
[`ToDisplay::display()`]: crate::ToDisplay::display