#![allow(clippy::use_self)]

mod from_str;
mod try_from;

use clinvoice_schema::{Contact, Job, Organization, Timesheet};
use money2::ExchangeRates;
use strum::{Display, EnumIter, IntoStaticStr};

/// A [file format](https://en.wikipedia.org/wiki/File_format) to export information to.
///
/// # Examples
///
/// ```rust
/// use clinvoice_export::Format;
/// assert_eq!(Format::Markdown.to_string(), "markdown");
/// ```
#[derive(
	Copy, Clone, Debug, Display, Eq, EnumIter, Hash, IntoStaticStr, Ord, PartialEq, PartialOrd,
)]
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
	/// `contact_info` and `timesheets` are exported in the order given.
	///
	/// # Warnings
	///
	/// * The following fields must all contain valid markdown syntax:
	///   * The `objectives` and `notes` of the `job`.
	///   * The `work_notes` of every [`Timesheet`] of the `timesheets`.
	///   * The `category` and `description` of every [`Expense`] of the `expenses` of every
	///     [`Timesheet`] of the `timesheets`.
	///
	/// # Panics
	///
	/// * When [`Timesheet::total`](clinvoice_schema::Timesheet::total) does.
	pub fn export_job(
		&self,
		job: &Job,
		contact_info: &[Contact],
		exchange_rates: Option<&ExchangeRates>,
		organization: &Organization,
		timesheets: &[Timesheet],
	) -> String
	{
		match self
		{
			#[cfg(feature = "markdown")]
			Self::Markdown => crate::markdown::export_job(
				job,
				contact_info,
				exchange_rates,
				organization,
				timesheets,
			),
		}
	}

	/// Returns an appropriate file extension for the given [`Format`].
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_export::Format;
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
