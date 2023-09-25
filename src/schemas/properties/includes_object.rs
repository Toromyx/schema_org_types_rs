use super::*;
/// This links to a node or nodes indicating the exact quantity of the products included in  an [[Offer]] or [[ProductCollection]].
///
/// <https://schema.org/includesObject>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IncludesObjectProperty {
    #[cfg(any(
        any(
            feature = "type-and-quantity-node-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    TypeAndQuantityNode(TypeAndQuantityNode),
}
