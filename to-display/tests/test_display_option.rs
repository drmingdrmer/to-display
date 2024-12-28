use std::fmt;

use to_display::DisplayConfig;
use to_display::ToDisplay;

#[derive(ToDisplay)]
struct Foo;

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Foo")
    }
}

#[test]
fn test_display_option() {
    let foo = None::<Foo>;

    assert_eq!(foo.display().to_string(), "-");
    assert_eq!(foo.display().verbose().to_string(), "None");

    let foo = Some(Foo);

    assert_eq!(foo.display().to_string(), "Foo");
    assert_eq!(foo.display().verbose().to_string(), "Some(Foo)");

    let foo = Some(Some(Foo));

    assert_eq!(foo.display().to_string(), "Foo");
    assert_eq!(foo.display().verbose().to_string(), "Some(Some(Foo))");
}
