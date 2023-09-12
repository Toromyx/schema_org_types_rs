use quote::{__private::TokenStream, quote};

use crate::schema::Schema;

pub trait FeatureGate {
    fn feature_gate(&self) -> TokenStream;
}

impl<T: Schema> FeatureGate for T {
    fn feature_gate(&self) -> TokenStream {
        let schema_feature_name = self.feature_name();
        let section_feature_name = self.section().feature_name();
        quote!(#[cfg(any(feature = #schema_feature_name, feature = #section_feature_name))])
    }
}
