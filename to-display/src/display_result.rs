use std::fmt;

use crate::Context;
use crate::DisplayConfig;
use crate::ToDisplay;

/// Displays a `Result<T,E>` if T and E are `Display`.
///
/// It outputs `"Ok(...)"` or `"Err(...)"`.
pub struct DisplayResult<'a, T, E> {
    result: &'a Result<T, E>,
    context: Context,
}

impl<T, E> DisplayConfig for DisplayResult<'_, T, E> {
    fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}

impl<T, E> fmt::Display for DisplayResult<'_, T, E>
where
    T: ToDisplay,
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.result {
            Ok(ok) => {
                write!(f, "Ok({})", ok.display_with_context(self.context))
            }
            Err(err) => {
                write!(f, "Err({})", err)
            }
        }
    }
}

impl<T, E> ToDisplay for Result<T, E>
where
    T: ToDisplay,
    E: fmt::Display,
{
    type Displayer<'a>
        = DisplayResult<'a, T, E>
    where
        T: 'a,
        E: 'a;

    fn display_with_context(&self, context: Context) -> Self::Displayer<'_> {
        DisplayResult {
            result: self,
            context,
        }
    }
}
