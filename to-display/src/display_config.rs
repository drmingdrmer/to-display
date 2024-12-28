use crate::context::LocalOrUTC;
use crate::Context;

/// A customizable display wrapper.
///
/// Provides a builder-style interface to control how values are formatted when displayed.
/// Supports configuration of:
/// - Verbosity level (e.g., `Some(1)` vs `1`)
/// - Collection size limits
/// - Time formatting options (timezone and format patterns)
///
/// # Example:
/// ```
/// use to_display::DisplayConfig;
/// use to_display::ToDisplay;
///
/// assert_eq!(Some(1u32).display().to_string(), "1");
/// assert_eq!(Some(1u32).display().verbose().to_string(), "Some(1)");
/// ```
///
/// [`ToDisplay::display()`]: crate::ToDisplay::display
pub trait DisplayConfig: Sized {
    /// Return a mutable reference to the context to let the caller modify it.
    fn context_mut(&mut self) -> &mut Context;

    /// Enable verbose mode: Display [`Option`] with `Some(v)/None` or `v/-`
    fn verbose(mut self) -> Self {
        self.context_mut().verbose = Some(true);
        self
    }

    /// Set the maximum number of items to display for collections.
    ///
    /// Applies to slices, vectors, maps, and other collection types.
    /// Elements beyond this limit will be indicated with an ellipsis.
    fn limit_items(mut self, max_items: usize) -> Self {
        self.context_mut().max_items = Some(max_items);
        self
    }

    /// Configures timestamps to display in local time.
    fn use_local_time(mut self) -> Self {
        self.context_mut().local_or_utc = Some(LocalOrUTC::Local);
        self
    }

    /// Configures timestamps to display in UTC.
    fn use_utc_time(mut self) -> Self {
        self.context_mut().local_or_utc = Some(LocalOrUTC::Utc);
        self
    }

    /// Sets a concise time format (`%H:%M:%S%.6f`).
    fn use_short_time(mut self) -> Self {
        self.context_mut().time_format = Some("SIMPLE");
        self
    }

    /// Sets a detailed time format (`%Y-%m-%dT%H:%M:%S%.6fZ%z`).
    fn use_full_time(mut self) -> Self {
        self.context_mut().time_format = Some("FULL");
        self
    }

    /// Sets a custom time format string.
    ///
    /// Uses the format syntax from the `chrono` crate.
    /// See [chrono::format::strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)
    /// for the full specification.
    fn with_time_format(mut self, time_format: &'static str) -> Self {
        self.context_mut().time_format = Some(time_format);
        self
    }
}
