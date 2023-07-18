mod display;

/// [Markdown](crate::Format::Markdown) syntax for text formatting.
///
/// # Examples
///
/// ```rust
/// # use pretty_assertions::assert_str_eq;
/// use winvoice_export::markdown::Text;
///
/// assert_str_eq!(r#"*I* have a **really strong opinion** about the number $\pi$."#, format!(
///   "{} have a {} about the number {}.",
///   Text::Italic("I"),
///   Text::Bold("really strong opinion"),
///   Text::Math(r#"\pi"#),
/// ));
/// ```
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Text<T>
{
	/// Bold text.
	Bold(T),

	/// Italic text.
	Italic(T),

	/// Inline LaTeX math.
	Math(T),
}
