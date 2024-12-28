#[derive(Default, Clone, Debug, Copy, PartialEq, Eq)]
pub enum LocalOrUTC {
    #[default]
    Local,
    Utc,
}

/// Configuration that controls how values are formatted.
///
/// This configuration is created when [`ToDisplay::display_with_context()`] is called
/// and propagates to all nested display implementations.
///
/// This struct is used by the [`Display`] implementation to get the current formatting
/// specification.
///
/// [`Display`]: std::fmt::Display
/// [`ToDisplay::display_with_context()`]: crate::ToDisplay::display_with_context
#[derive(Default, Clone, Debug, Copy)]
pub struct Context {
    pub(crate) verbose: Option<bool>,
    pub(crate) max_items: Option<usize>,
    pub(crate) local_or_utc: Option<LocalOrUTC>,
    pub(crate) time_format: Option<&'static str>,
}

impl Context {
    /// Returns whether verbose formatting is enabled.
    ///
    /// When enabled:
    /// - `Option` values show as `Some(v)` or `None` instead of `v` or `-`
    /// - Collections may include type information
    /// - Additional details may be included depending on the type
    pub fn verbose(&self) -> bool {
        self.verbose.unwrap_or(false)
    }

    /// Returns the maximum number of items to display for collections.
    ///
    /// This affects the formatting of slices, vectors, maps, and similar collections.
    /// When a collection exceeds this limit, it will be truncated with "...".
    ///
    /// Defaults to 32 items.
    pub fn max_items(&self) -> usize {
        self.max_items.unwrap_or(32)
    }

    /// Returns whether times should be displayed in local time.
    pub fn is_local_time(&self) -> bool {
        self.local_or_utc.unwrap_or_default() == LocalOrUTC::Local
    }

    /// Returns whether times should be displayed in UTC.
    pub fn is_utc_time(&self) -> bool {
        self.local_or_utc.unwrap_or_default() == LocalOrUTC::Utc
    }

    /// Returns the time format string used for formatting timestamps.
    ///
    /// Format options:
    /// - Default/`"SIMPLE"`: `"%H:%M:%S%.6f"` (e.g., "23:59:59.123456")
    /// - `"FULL"`: `"%Y-%m-%dT%H:%M:%S%.6fZ%z"` (e.g., "2024-12-28T23:59:59.123456Z+0800")
    /// - Custom format string using [strftime format specifiers](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)
    pub fn time_format(&self) -> &'static str {
        const SIMPLE_FORMAT: &str = "%H:%M:%S%.6f";
        const FULL_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.6fZ%z";

        match self.time_format {
            Some("SIMPLE") => SIMPLE_FORMAT,
            Some("FULL") => FULL_FORMAT,
            Some(format) => format,
            None => SIMPLE_FORMAT,
        }
    }
}
