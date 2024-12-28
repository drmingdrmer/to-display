use std::net;
use std::num;

use crate::Context;
use crate::ToDisplay;

macro_rules! impl_to_display_primitive {
    ($($t:ty),*) => {
        $(
            impl ToDisplay for $t {
                type Displayer<'a> = &'a Self where Self: 'a;

                fn display_with_context(&self, _context: Context) -> Self::Displayer<'_> {
                    self
                }
            }
        )*
    }
}

impl_to_display_primitive!(
    // Signed integers
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    // Unsigned integers
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    // Floating point
    f32,
    f64,
    // Other primitives
    bool,
    char,
    // String types
    String,
    &str,
    // Network types
    net::IpAddr,
    net::Ipv4Addr,
    net::Ipv6Addr,
    net::SocketAddr,
    net::SocketAddrV4,
    net::SocketAddrV6,
    // Numeric types
    num::NonZeroI8,
    num::NonZeroI16,
    num::NonZeroI32,
    num::NonZeroI64,
    num::NonZeroI128,
    num::NonZeroIsize,
    num::NonZeroU8,
    num::NonZeroU16,
    num::NonZeroU32,
    num::NonZeroU64,
    num::NonZeroU128,
    num::NonZeroUsize
);
