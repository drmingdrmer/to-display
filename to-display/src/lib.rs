#![doc = include_str!("lib_readme.md")]

pub(crate) mod context;
pub(crate) mod display_btreemap;
pub(crate) mod display_config;
#[cfg(feature = "std-time")]
pub(crate) mod display_instant;
pub(crate) mod display_option;
pub(crate) mod display_result;
pub(crate) mod display_slice;
pub(crate) mod to_display;
mod to_display_impls;

pub use context::Context;
pub use display_config::DisplayConfig;
pub use to_display::ToDisplay;
pub use to_display_derive::ToDisplay;

pub use self::display_btreemap::DisplayBTreeMap;
#[cfg(feature = "std-time")]
pub use self::display_instant::DisplayInstant;
pub use self::display_option::DisplayOption;
pub use self::display_result::DisplayResult;
pub use self::display_slice::DisplaySlice;
