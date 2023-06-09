//! Error type definition.

use thiserror::Error;

/// Error type for `tpl-rs`
#[derive(Error, Debug)]
pub enum Error {
    #[error("other error")]
    OtherError,
}
