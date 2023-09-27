use quote::{__private::TokenStream, quote};

use crate::schema::Schema;

/// This trait exists to help generate multi-lined code documentation.
pub trait DocLines {
    /// Get the documentation lines.
    ///
    /// Each [`String`] in the [`Vec`] is one line.
    /// Line breaks are regarded.
    fn doc_lines(&self) -> Vec<String>;

    fn doc_lines_token_stream(&self) -> TokenStream {
        let doc_lines: Vec<String> = self
            .doc_lines()
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
        quote!(
            #( #[doc = #doc_lines] )*
        )
    }
}

impl<T: Schema> DocLines for T {
    fn doc_lines(&self) -> Vec<String> {
        vec![format!("<{}>", self.iri())]
    }
}
