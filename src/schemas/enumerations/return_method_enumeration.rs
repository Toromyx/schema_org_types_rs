/// Enumerates several types of product return methods.
///
/// https://schema.org/ReturnMethodEnumeration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReturnMethodEnumeration {
    /// Specifies that product returns must be made at a kiosk.
    ///
    /// https://schema.org/ReturnAtKiosk
    ReturnAtKiosk,
    /// Specifies that product returns must be done by mail.
    ///
    /// https://schema.org/ReturnByMail
    ReturnByMail,
    /// Specifies that product returns must be made in a store.
    ///
    /// https://schema.org/ReturnInStore
    ReturnInStore,
}
