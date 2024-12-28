use std::collections::BTreeMap;
use std::fmt;

use crate::Context;
use crate::DisplayConfig;
use crate::ToDisplay;

/// Displays a `BTreeMap<K, V>`.
///
/// This is the return value of calling a `BTreeMap<K, V>::display()`.
pub struct DisplayBTreeMap<'a, K, V> {
    map: &'a BTreeMap<K, V>,
    context: Context,
}

impl<K, V> DisplayConfig for DisplayBTreeMap<'_, K, V> {
    fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}

impl<K, V> fmt::Display for DisplayBTreeMap<'_, K, V>
where
    K: ToDisplay,
    V: ToDisplay,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_items = self.context.max_items();

        write!(f, "{{")?;
        for (i, (k, v)) in self.map.iter().take(max_items).enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(
                f,
                "{}: {}",
                k.display_with_context(self.context),
                v.display_with_context(self.context)
            )?;
        }

        if self.map.len() > max_items {
            write!(f, ", ...")?;
        }

        write!(f, "}}")
    }
}

impl<K, V> crate::ToDisplay for BTreeMap<K, V>
where
    K: ToDisplay,
    V: ToDisplay,
{
    type Displayer<'a>
        = DisplayBTreeMap<'a, K, V>
    where
        K: 'a,
        V: 'a;

    fn display_with_context(&self, context: Context) -> Self::Displayer<'_> {
        DisplayBTreeMap { map: self, context }
    }
}
