use std::str::FromStr;

use convert_case::{Case, Casing};
use derivative::Derivative;
use quote::{quote, ToTokens, TokenStreamExt, __private::TokenStream};

use crate::{
	deprecated_attribute::DeprecatedAttribute, doc_lines::DocLines, schema::ReferencedSchema,
};

#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumerationVariant {
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub iri: String,
	pub name: String,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub superseded_by: Vec<ReferencedSchema>,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub in_attic: bool,
}

impl DocLines for EnumerationVariant {
	fn doc_lines(&self) -> Vec<String> {
		vec![format!("<{}>", self.iri)]
	}
}

impl DeprecatedAttribute for EnumerationVariant {
	fn in_attic(&self) -> bool {
		self.in_attic
	}

	fn superseded_by(&self) -> &[ReferencedSchema] {
		self.superseded_by.as_slice()
	}
}

impl ToTokens for EnumerationVariant {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let doc_lines = self.doc_lines_token_stream();
		let deprecated_attribute = self.deprecated_attribute();
		let name = TokenStream::from_str(&self.name.to_case(Case::UpperCamel)).unwrap();
		tokens.append_all(quote!(
			#doc_lines
			#deprecated_attribute
			#name,
		));
	}
}
