use super::*;
/// Indicates the kind of product that this is a variant of. In the case of [[ProductModel]], this is a pointer (from a ProductModel) to a base product from which this product is a variant. It is safe to infer that the variant inherits all product features from the base model, unless defined locally. This is not transitive. In the case of a [[ProductGroup]], the group description also serves as a template, representing a set of Products that vary on explicitly defined, specific dimensions only (so it defines both a set of variants, as well as which values distinguish amongst those variants). When used with [[ProductGroup]], this property can apply to any [[Product]] included in the group.
///
/// <https://schema.org/isVariantOf>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IsVariantOfProperty {
    #[cfg(any(
        any(feature = "product-group-schema", feature = "pending-schema-section"),
        doc
    ))]
    ProductGroup(ProductGroup),
    #[cfg(any(
        any(feature = "product-model-schema", feature = "general-schema-section"),
        doc
    ))]
    ProductModel(ProductModel),
}
