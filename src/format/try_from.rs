use core::convert::TryFrom;

use strum::IntoEnumIterator;

use super::Format;
use crate::{Error, Result};

impl TryFrom<&str> for Format
{
	type Error = Error;

	fn try_from(s: &str) -> Result<Self>
	{
		Format::iter()
			.find(|f| s.eq_ignore_ascii_case(f.into()))
			.ok_or_else(|| Error::UnsupportedFormat(s.to_string()))
	}
}
