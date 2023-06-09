//! This is the `rocksdb-utils-lookup` library.
#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::module_name_repetitions)]
#![warn(missing_docs)]

mod error;
mod utils;

pub use crate::error::*;
pub use crate::utils::{force_compaction, force_compaction_cf, tune_options};
