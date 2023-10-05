use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};

use crate::serde_attributes::serde_as;

/// Represents the underlying rust types used in Schema.org types.
#[derive(Debug, Clone)]
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
    pub fn serde_attributes(&self) -> Option<TokenStream> {
        match self {
            RustType::Boolean => None,
            RustType::Number => None,
            RustType::Integer => None,
            RustType::Date => Some(serde_as("DisplayFromStr")),
            RustType::Time => Some(serde_as("DisplayFromStr")),
            RustType::DateTime => Some(serde_as("DisplayFromStr")),
            RustType::String => None,
            RustType::Url => None,
        }
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
