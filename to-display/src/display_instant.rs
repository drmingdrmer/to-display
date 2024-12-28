use std::time::Instant;
use std::time::SystemTime;

use chrono::DateTime;
use chrono::Local;
use chrono::Utc;

use crate::Context;
use crate::DisplayConfig;
use crate::ToDisplay;

/// Displays a `std::time::Instant`.
///
/// This is the return value of calling a `Instant::display()`.
#[derive()]
pub struct DisplayInstant {
    system_time: SystemTime,
    context: Context,
}

impl DisplayConfig for DisplayInstant {
    fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}

impl std::fmt::Display for DisplayInstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tfmt = self.context.time_format();

        if self.context.is_local_time() {
            let datetime: DateTime<Local> = self.system_time.into();
            write!(f, "{}", datetime.format(tfmt))
        } else {
            let datetime: DateTime<Utc> = self.system_time.into();
            write!(f, "{}", datetime.format(tfmt))
        }
    }
}

impl ToDisplay for Instant {
    type Displayer<'a> = DisplayInstant;

    fn display_with_context(&self, context: Context) -> Self::Displayer<'_> {
        DisplayInstant {
            system_time: to_system_time(*self),
            context,
        }
    }
}

#[cfg(feature = "tokio-time")]
mod impl_tokio_time {

    use super::to_system_time;
    use crate::Context;
    use crate::DisplayInstant;
    use crate::ToDisplay;

    impl ToDisplay for tokio::time::Instant {
        type Displayer<'a>
            = DisplayInstant
        where
            Self: 'a;

        fn display_with_context(&self, context: Context) -> Self::Displayer<'_> {
            DisplayInstant {
                system_time: to_system_time(self.into_std()),
                context,
            }
        }
    }
}

pub(crate) fn to_system_time<T>(t: T) -> SystemTime
where
    T: Into<Instant>,
{
    let sys_now = SystemTime::now();
    let now = Instant::now();

    let tt: Instant = t.into();

    if now >= tt {
        let d = now - tt;
        sys_now - d
    } else {
        let d = tt - now;
        sys_now + d
    }
}
