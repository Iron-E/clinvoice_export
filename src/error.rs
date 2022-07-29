#![allow(clippy::std_instead_of_core)]

use thiserror::Error;

/// An [`Error`](std::error::Error) type for the library.
#[derive(Debug, Error)]
pub enum Error
{
	#[allow(missing_docs)]
	#[error("{0} is not a supported export format")]
	UnsupportedFormat(String),
}

clinvoice_error::AliasResult!();
