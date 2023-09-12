use quote::{__private::TokenStream, quote};

fn serde_cfg() -> TokenStream {
    quote!(feature = "serde")
}

fn serde_cfg_attr(cfg_attr: TokenStream) -> TokenStream {
    let cfg = serde_cfg();
    quote!(#[cfg_attr(#cfg, #cfg_attr)])
}

pub fn serde_derive() -> TokenStream {
    serde_cfg_attr(quote!(derive(serde::Serialize, serde::Deserialize)))
}

pub fn serde_untagged() -> TokenStream {
    serde_cfg_attr(quote!(serde(untagged)))
}

pub fn serde_rename(name: &str) -> TokenStream {
    serde_cfg_attr(quote!(serde(rename = #name)))
}

pub fn serde_default() -> TokenStream {
    serde_cfg_attr(quote!(serde(default)))
}

pub fn serde_skip_serializing_if(path: &str) -> TokenStream {
    serde_cfg_attr(quote!(serde(skip_serializing_if = #path)))
}

pub fn serde_with(with: &str) -> TokenStream {
    serde_cfg_attr(quote!(serde(with = #with)))
}
