/// <https://schema.org/ReturnLabelSourceEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReturnLabelSourceEnumeration {
    /// <https://schema.org/ReturnLabelCustomerResponsibility>
    ReturnLabelCustomerResponsibility,
    /// <https://schema.org/ReturnLabelDownloadAndPrint>
    ReturnLabelDownloadAndPrint,
    /// <https://schema.org/ReturnLabelInBox>
    ReturnLabelInBox,
}
