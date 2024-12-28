use std::fmt;

use to_display::Context;
use to_display::ToDisplay;

struct Foo(u64);

impl ToDisplay for Foo {
    /// Use `DisplayFoo` to display `Foo`.
    type Displayer<'a> = DisplayFoo<'a>;

    fn display_with_context(&self, context: Context) -> Self::Displayer<'_> {
        DisplayFoo {
            foo: self,
            _context: context,
        }
    }
}

/// `DisplayFoo` contains a reference to `Foo`.
struct DisplayFoo<'a> {
    foo: &'a Foo,
    _context: Context,
}

/// Customize the display of `Foo`.
impl fmt::Display for DisplayFoo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Foo({})", self.foo.0)
    }
}

#[test]
fn test_impl_to_display() {
    assert_eq!(Foo(42).display().to_string(), "Foo(42)");
}
