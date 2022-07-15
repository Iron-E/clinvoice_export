use core::str::FromStr;

use super::Format;
use crate::{Error, Result};

impl FromStr for Format
{
	type Err = Error;

	fn from_str(s: &str) -> Result<Self>
	{
		s.try_into()
	}
}
