mod display;

/// Syntax for different groupings of text.
///
/// # Examples
///
/// ```rust
/// use core::fmt::Write;
/// # use pretty_assertions::assert_str_eq as assert_eq;
/// use winvoice_export::markdown::Block;
///
/// let mut expected = String::new();
///
/// writeln!(expected, "{}", Block::Heading {
///   indents: 1,
///   text: "This is a test heading!",
/// })
/// .unwrap();
///
/// writeln!(expected, "{}", Block::Heading {
///   indents: 2,
///   text: "Paragraphs",
/// })
/// .unwrap();
///
/// writeln!(expected, "{}", Block::Text("I can create a paragraph.")).unwrap();
///
/// writeln!(expected, "{}", Block::Heading {
///   indents: 2,
///   text: "Lists",
/// })
/// .unwrap();
///
/// writeln!(expected, "{}", Block::OrderedList {
///   indents: 0,
///   text: "Ordered lists are not a problem.",
/// })
/// .unwrap();
///
/// writeln!(expected, "{}", Block::OrderedList {
///   indents: 0,
///   text: "Continuing is just fine.",
/// })
/// .unwrap();
///
/// writeln!(expected, "{}", Block::<String>::Break).unwrap();
///
/// writeln!(expected, "{}", Block::UnorderedList {
///   indents: 0,
///   text: "I can break at any point.",
/// })
/// .unwrap();
///
/// writeln!(expected, "{}", Block::UnorderedList {
///   indents: 1,
///   text: "Indenting? Eazy breezy.",
/// })
/// .unwrap();
///
/// write!(expected, "{}", Block::UnorderedList {
///   indents: 0,
///   text: "De-indenting? Easier!",
/// })
/// .unwrap();
///
/// assert_eq!(&expected,
/// "# This is a test heading!
///
/// ### Paragraphs
///
/// I can create a paragraph.
///
/// ### Lists
///
/// 1. Ordered lists are not a problem.
/// 1. Continuing is just fine.
///
/// - I can break at any point.
/// \t- Indenting? Eazy breezy.
/// - De-indenting? Easier!");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Block<T>
{
	/// A horizontal spacer. Typically used to terminate a list (i.e. [`OrderedList`],
	/// [UnorderedList`]).
	Break,

	/// A heading. `indents` is how many preceding `#`s there are.
	Heading
	{
		/// The number of leading "#" characters.
		indents: usize,

		/// The text of the heading.
		text: T,
	},

	/// A list which ascends in number with each successive use.
	OrderedList
	{
		/// The number of leading tab characters.
		indents: usize,

		/// The text of the list.
		text: T,
	},

	/// Plaintext.
	Text(T),

	/// A list which has no inherent order.
	UnorderedList
	{
		/// The number of leading tab characters.
		indents: usize,

		/// The text of the list.
		text: T,
	},
}
