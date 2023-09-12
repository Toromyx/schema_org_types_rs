use quote::{quote, ToTokens, TokenStreamExt, __private::TokenStream};

/// This struct exists to help generate multi-lined code documentation.
///
/// Each [`String`] in the [`Vec`] is one line.
/// Line breaks are regarded.
pub struct DocLines(pub Vec<String>);

impl ToTokens for DocLines {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let doc_lines: Vec<String> = self
            .0
            .iter()
            .flat_map(|line| line.split('\n'))
            .map(|line| {
                let trimmed_line = line.trim();
                if trimmed_line.is_empty() {
                    trimmed_line.to_string()
                } else {
                    format!(" {}", trimmed_line)
                }
            })
            .collect();
        tokens.append_all(quote!(
            #( #[doc = #doc_lines] )*
        ))
    }
}
