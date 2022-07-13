use core::str::FromStr;

use crate::{Error, Result};
use strum::IntoEnumIterator;

use super::Format;

impl FromStr for Format
{
	type Err = Error;

	fn from_str(s: &str) -> Result<Self>
	{
		Format::iter()
			.find(|f| s.eq_ignore_ascii_case(f.into()))
			.ok_or_else(|| Error::UnsupportedFormat(s.to_string()))
	}
}
