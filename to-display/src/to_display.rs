use std::fmt;

use crate::Context;

/// Create a displayable instance for a type.
///
/// # Usage:
///
/// For example, to display an `Option<i32>`, use `.display()` to create an instance that is
/// `Display`
///
/// ```
/// use to_display::ToDisplay;
/// assert_eq!(Some(1u32).display().to_string(), "1");
/// ```
///
/// If the returned type implements [`DisplayConfig`], it can be further customized with such
/// as:
/// - `.verbose()`: to enable verbose mode.
/// - `.max_items(n)`: display at most `n` items.
/// - ...
///
/// To implement [`ToDisplay`] for a type that implements `Display`, use `#[derive(ToDisplay)]`.
///
/// It displays an Option as `Some(value)` or `None`, if `verbose` is enabled.
/// and it displays an Option as `-` or `value`, if `verbose` is disabled.
///
/// It displays a Result as `Ok(value)` or `Err(error)`, if `verbose` is enabled.
///
/// It displays a slice as `[value1, value2, ...]`, if the number of items is less than or equal to
/// `max_items`.
///
/// [`DisplayConfig`]: crate::DisplayConfig
pub trait ToDisplay {
    type Displayer<'a>: fmt::Display
    where
        Self: 'a;

    /// Return an instance that is `Display`, with a default [`Context`].
    ///
    /// User calls this method to build a displayable instance.
    fn display(&self) -> Self::Displayer<'_> {
        self.display_with_context(Context::default())
    }

    /// Return an instance that is `Display` with a provided [`Context`].
    ///
    /// User implements this method to create a customized displayable instance.
    /// This method should be called when a [`DisplayConfig`] implementation converts its children
    /// item to a [`DisplayConfig`], to inherit the context.
    ///
    /// [`DisplayConfig`]: crate::DisplayConfig
    fn display_with_context(&self, context: Context) -> Self::Displayer<'_>;
}
