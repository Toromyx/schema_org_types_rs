use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};

#[derive(Debug, Clone)]
pub enum Feature {
	Name(String),
	Any(Vec<Feature>),
	All(Vec<Feature>),
	Not(Box<Feature>),
}

impl Feature {
	pub fn negate(&self) -> Self {
		match self {
			Feature::Name(name) => Feature::Not(Box::new(Feature::Name(name.clone()))),
			Feature::Any(features) => {
				Feature::All(features.iter().map(|feature| feature.negate()).collect())
			}
			Feature::All(features) => {
				Feature::Any(features.iter().map(|feature| feature.negate()).collect())
			}
			Feature::Not(feature) => *feature.clone(),
		}
	}
}

impl ToTokens for Feature {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append_all(match self {
			Feature::Name(name) => quote!(feature = #name),
			Feature::Any(features) => quote!(any(#(#features),*)),
			Feature::All(features) => quote!(all(#(#features),*)),
			Feature::Not(feature) => quote!(not(#feature)),
		});
	}
}

impl Feature {
	pub fn as_cfg_attribute(&self) -> TokenStream {
		let features_cfg = self.to_token_stream();
		quote!(
			#[cfg(any(#features_cfg, doc))]
		)
	}

	pub fn as_cfg_attribute_no_doc(&self) -> TokenStream {
		let features_cfg = self.to_token_stream();
		quote!(
			#[cfg(#features_cfg)]
		)
	}
}
