/// <https://schema.org/ReturnLabelSourceEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum ReturnLabelSourceEnumeration {
    /// <https://schema.org/ReturnLabelCustomerResponsibility>
    ReturnLabelCustomerResponsibility,
    /// <https://schema.org/ReturnLabelDownloadAndPrint>
    ReturnLabelDownloadAndPrint,
    /// <https://schema.org/ReturnLabelInBox>
    ReturnLabelInBox,
}
