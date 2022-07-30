#![allow(clippy::std_instead_of_core)]

use core::result::Result as StdResult;

use thiserror::Error;

/// An [`Error`](std::error::Error) for the crate.
#[derive(Debug, Error)]
pub enum Error
{
	#[allow(missing_docs)]
	#[error("{0} is not a supported export format")]
	UnsupportedFormat(String),
}

/// A [`Result`](StdResult) for the library.
pub type Result<T> = StdResult<T, Error>;
