use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};

use crate::serde_attributes::serde_as;

/// Represents the underlying rust types used in Schema.org types.
#[derive(Debug, Clone)]
pub enum RustType {
    /// Equivalent to [`bool`].
    Boolean,
    /// Equivalent to [`f64`].
    Double,
    /// Equivalent to [`i64`].
    Long,
    /// Equivalent to [`schema_org_types::date_types::Date`].
    Date,
    /// Equivalent to [`schema_org_types::date_types::Time`].
    Time,
    /// Equivalent to [`schema_org_types::date_types::DateTime`].
    DateTime,
    /// Equivalent to [`String`].
    String,
}

impl RustType {
    pub fn serde_attributes(&self) -> Option<TokenStream> {
        match self {
            RustType::Boolean => None,
            RustType::Double => None,
            RustType::Long => None,
            RustType::Date => Some(serde_as("DisplayFromStr")),
            RustType::Time => Some(serde_as("DisplayFromStr")),
            RustType::DateTime => Some(serde_as("DisplayFromStr")),
            RustType::String => None,
        }
    }
}

impl ToTokens for RustType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            RustType::Boolean => {
                quote!(bool)
            }
            RustType::Double => {
                quote!(f64)
            }
            RustType::Long => {
                quote!(i64)
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
        })
    }
}

impl From<&str> for RustType {
    fn from(value: &str) -> Self {
        match value {
            "Boolean" => Self::Boolean,
            "Number" => Self::Double,
            "Integer" => Self::Long,
            "Date" => Self::Date,
            "Time" => Self::Time,
            "DateTime" => Self::DateTime,
            "Text" => Self::String,
            value => {
                panic!(
                    "Tried to convert from the inconvertible schema.org data type \"{}\" to a rust type.",
                    value,
                );
            }
        }
    }
}
