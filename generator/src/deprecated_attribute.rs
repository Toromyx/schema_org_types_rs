use quote::{__private::TokenStream, quote};

use crate::schema::ReferencedSchema;

pub trait DeprecatedAttribute {
	/// Check if this schema is in the attic.
	fn in_attic(&self) -> bool;

	/// Get the schemas this schema is superseded by.
	fn superseded_by(&self) -> &[ReferencedSchema];

	fn deprecated_attribute(&self) -> Option<TokenStream> {
		let deprecated_notes: Vec<_> = [
			if self.in_attic() {
				Some(
					"This schema is archived, see <https://schema.org/docs/attic.home.html>."
						.to_string(),
				)
			} else {
				None
			},
			if !self.superseded_by().is_empty() {
				let deprecated_notice = format!(
					"This schema is superseded by {}.",
					self.superseded_by()
						.iter()
						.map(|referenced_schema| format!("<{}>", referenced_schema.iri))
						.collect::<Vec<String>>()
						.join(", ")
				);
				Some(deprecated_notice)
			} else {
				None
			},
		]
		.into_iter()
		.flatten()
		.collect();
		if !deprecated_notes.is_empty() {
			let deprecated_note = deprecated_notes.join(" ");
			Some(quote!(#[deprecated = #deprecated_note]))
		} else {
			None
		}
	}
}
