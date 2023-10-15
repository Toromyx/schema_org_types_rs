use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};

use crate::schema::Schema;

impl<T: Schema> From<&T> for Feature {
    fn from(value: &T) -> Self {
        Feature::Any(vec![
            Feature::Name(value.feature_name()),
            Feature::Name(value.section().feature_name().to_string()),
        ])
    }
}

#[derive(Debug, Clone)]
pub enum Feature {
    Name(String),
    Any(Vec<Feature>),
}

impl ToTokens for Feature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            Feature::Name(name) => quote!(feature = #name),
            Feature::Any(features) => quote!(any(#(#features),*)),
        });
    }
}

impl Feature {
    pub fn feature_gate(&self) -> TokenStream {
        let features_cfg = self.to_token_stream();
        quote!(
            #[cfg(any(#features_cfg, doc))]
        )
    }
}
