use std::str::FromStr;

use convert_case::{Case, Casing};
use derivative::Derivative;
use quote::{quote, ToTokens, TokenStreamExt, __private::TokenStream};

use crate::doc_lines::DocLines;

#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumerationVariant {
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub description: String,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub iri: String,
    pub name: String,
}

impl ToTokens for EnumerationVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let doc_lines = DocLines(vec![
            self.description.clone(),
            String::from(""),
            self.iri.clone(),
        ]);
        let name = TokenStream::from_str(&self.name.to_case(Case::UpperCamel)).unwrap();
        tokens.append_all(quote!(
            #doc_lines
            #name
        ));
    }
}
