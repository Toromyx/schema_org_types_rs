use std::cmp::Ordering;

use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};

/// Represents the underlying rust types used in Schema.org types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustType {
	Boolean,
	Number,
	Integer,
	Date,
	Time,
	DateTime,
	String,
	Url,
}

impl RustType {
	pub fn serde_as(&self) -> Option<&str> {
		match self {
			RustType::Boolean => None,
			RustType::Number => None,
			RustType::Integer => None,
			RustType::Date => Some("DisplayFromStr"),
			RustType::Time => Some("DisplayFromStr"),
			RustType::DateTime => Some("DisplayFromStr"),
			RustType::String => None,
			RustType::Url => None,
		}
	}

	fn order(&self) -> isize {
		match self {
			RustType::Boolean => 5,
			RustType::Number => 7,
			RustType::Integer => 6,
			RustType::Date => 2,
			RustType::Time => 3,
			RustType::DateTime => 4,
			RustType::String => 8,
			RustType::Url => 1,
		}
	}
}

impl PartialOrd for RustType {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for RustType {
	fn cmp(&self, other: &Self) -> Ordering {
		self.order().cmp(&other.order())
	}
}

impl ToTokens for RustType {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append_all(match self {
			RustType::Boolean => {
				quote!(bool)
			}
			RustType::Number => {
				quote!(crate::number_types::Number)
			}
			RustType::Integer => {
				quote!(crate::number_types::Integer)
			}
			RustType::Date => {
				quote!(crate::date_types::Date)
			}
			RustType::Time => {
				quote!(crate::date_types::Time)
			}
			RustType::DateTime => {
				quote!(crate::date_types::DateTime)
			}
			RustType::String => {
				quote!(String)
			}
			RustType::Url => {
				quote!(::url::Url)
			}
		})
	}
}

impl From<&str> for RustType {
	fn from(value: &str) -> Self {
		match value {
			"Boolean" => Self::Boolean,
			"Number" => Self::Number,
			"Integer" => Self::Integer,
			"Date" => Self::Date,
			"Time" => Self::Time,
			"DateTime" => Self::DateTime,
			"Text" => Self::String,
			"URL" => Self::Url,
			value => {
				panic!(
					"Tried to convert from the inconvertible schema.org data type \"{}\" to a rust type.",
					value,
				);
			}
		}
	}
}
