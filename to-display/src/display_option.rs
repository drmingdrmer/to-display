use std::fmt;
use std::fmt::Formatter;

use crate::Context;
use crate::DisplayConfig;
use crate::ToDisplay;

/// Displays a `Option<T>`.
///
/// This is the return value of calling a `Option<T>::display()`.
pub struct DisplayOption<'a, T> {
    option: &'a Option<T>,
    context: Context,
}

impl<T> DisplayConfig for DisplayOption<'_, T> {
    fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}

impl<T> fmt::Display for DisplayOption<'_, T>
where
    T: ToDisplay,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.option {
            Some(t) => {
                let d = t.display_with_context(self.context);
                if self.context.verbose() {
                    write!(f, "Some({})", d)
                } else {
                    write!(f, "{}", d)
                }
            }
            None => {
                if self.context.verbose() {
                    write!(f, "None")
                } else {
                    write!(f, "-")
                }
            }
        }
    }
}

impl<T> ToDisplay for Option<T>
where
    T: ToDisplay,
{
    type Displayer<'a>
        = DisplayOption<'a, T>
    where
        T: 'a;

    fn display_with_context(&self, context: Context) -> Self::Displayer<'_> {
        DisplayOption {
            option: self,
            context,
        }
    }
}
