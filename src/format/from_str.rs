use core::str::FromStr;

use strum::IntoEnumIterator;

use super::Format;
use crate::{Error, Result};

impl FromStr for Format
{
	type Err = Error;

	fn from_str(s: &str) -> Result<Self>
	{
		Self::iter()
			.find(|f| s.eq_ignore_ascii_case(f.into()))
			.ok_or_else(|| Error::UnsupportedFormat(s.to_owned()))
	}
}
