use core::str::FromStr;

use super::Format;

impl TryFrom<&str> for Format
{
	type Error = <Self as FromStr>::Err;

	fn try_from(s: &str) -> Result<Self, Self::Error>
	{
		s.parse()
	}
}
