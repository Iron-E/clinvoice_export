use core::fmt::{Display, Formatter, Result};

use super::Text;

impl<T> Display for Text<T>
where
	T: Display,
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		match self
		{
			Self::Bold(text) => write!(formatter, "**{text}**"),
			Self::Italic(text) => write!(formatter, "*{text}*"),
			Self::Math(text) => write!(formatter, "${text}$"),
		}
	}
}
