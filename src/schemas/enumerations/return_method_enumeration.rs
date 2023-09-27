/// <https://schema.org/ReturnMethodEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum ReturnMethodEnumeration {
    /// <https://schema.org/ReturnAtKiosk>
    ReturnAtKiosk,
    /// <https://schema.org/ReturnByMail>
    ReturnByMail,
    /// <https://schema.org/ReturnInStore>
    ReturnInStore,
}
