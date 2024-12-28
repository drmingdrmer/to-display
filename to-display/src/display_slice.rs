use std::fmt;

use crate::Context;
use crate::DisplayConfig;
use crate::ToDisplay;

/// Displays a slice `[T]`.
///
/// This is the return value of calling a `[T]::display()`.
pub struct DisplaySlice<'a, T> {
    slice: &'a [T],
    context: Context,
}

impl<T> DisplayConfig for DisplaySlice<'_, T> {
    fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}

impl<T> fmt::Display for DisplaySlice<'_, T>
where
    T: ToDisplay,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_items = self.context.max_items();

        write!(f, "[")?;
        for (i, t) in self.slice.iter().take(max_items).enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", t.display_with_context(self.context))?;
        }

        if self.slice.len() > max_items {
            write!(f, ", ...")?;
        }

        write!(f, "]")
    }
}

impl<T> crate::ToDisplay for [T]
where
    T: ToDisplay,
{
    type Displayer<'a>
        = DisplaySlice<'a, T>
    where
        T: 'a;

    fn display_with_context(&self, context: Context) -> Self::Displayer<'_> {
        DisplaySlice {
            slice: self,
            context,
        }
    }
}

impl<T> crate::ToDisplay for Vec<T>
where
    T: ToDisplay,
{
    type Displayer<'a>
        = DisplaySlice<'a, T>
    where
        T: 'a;

    fn display_with_context(&self, context: Context) -> Self::Displayer<'_> {
        DisplaySlice {
            slice: self.as_slice(),
            context,
        }
    }
}
