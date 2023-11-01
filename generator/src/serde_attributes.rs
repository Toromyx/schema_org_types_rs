use std::str::FromStr;

use quote::__private::TokenStream;

pub fn serde_as(r#as: &str) -> TokenStream {
	let with = format!(
		"serde_with::As::<serde_with::{}>",
		r#as.replace('_', "serde_with::Same")
	);
	TokenStream::from_str(&with).unwrap()
}
