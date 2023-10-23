#![allow(clippy::use_self)]

mod from_str;
mod try_from;

use std::collections::BTreeMap;

use strum::{Display, EnumIter, IntoStaticStr};
use winvoice_schema::{ContactKind, Job, Timesheet};

/// A [file format](https://en.wikipedia.org/wiki/File_format) to export information to.
///
/// # Examples
///
/// ```rust
/// # use pretty_assertions::assert_str_eq as assert_eq;
/// use winvoice_export::Format;
/// assert_eq!(Format::Markdown.to_string(), "markdown");
/// ```
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Debug, Display, Eq, EnumIter, Hash, IntoStaticStr, Ord, PartialEq, PartialOrd)]
#[strum(serialize_all = "snake_case")]
pub enum Format
{
	/// The [markdown](https://en.wikipedia.org/wiki/Markdown) file format.
	#[cfg(feature = "markdown")]
	Markdown,
}

impl Format
{
	/// Export some `job` to [Markdown](crate::Format::Markdown).
	///
	/// * `contact_info` and `timesheets` are exported in the order given.
	/// * The name of the host organization is inferred from the [`Contact`](winvoice_schema::Contact) with the 'Name'
	///   label.
	///
	/// # Warnings
	///
	/// * The following fields must all contain valid syntax of the [`Format`] specified:
	///   * The `objectives` and `notes` of the `job`.
	///   * The `work_notes` of every [`Timesheet`] of the `timesheets`.
	///   * The `category` and `description` of every [`Expense`] of the `expenses` of every [`Timesheet`] of the
	///     `timesheets`.
	pub fn export_job(
		&self,
		job: &Job,
		contact_info: &BTreeMap<String, ContactKind>,
		timesheets: &[Timesheet],
	) -> String
	{
		match self
		{
			#[cfg(feature = "markdown")]
			Self::Markdown => crate::markdown::export_job(job, contact_info, timesheets),
		}
	}

	/// Returns an appropriate file extension for the given [`Format`].
	///
	/// # Examples
	///
	/// ```rust
	/// # use pretty_assertions::assert_str_eq as assert_eq;
	/// use winvoice_export::Format;
	/// assert_eq!(Format::Markdown.extension(), ".md");
	/// ```
	pub const fn extension(&self) -> &'static str
	{
		match self
		{
			#[cfg(feature = "markdown")]
			Self::Markdown => ".md",
		}
	}
}
