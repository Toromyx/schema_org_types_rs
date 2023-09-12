/// Enumerates several types of return labels for product returns.
///
/// https://schema.org/ReturnLabelSourceEnumeration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReturnLabelSourceEnumeration {
    /// Indicated that creating a return label is the responsibility of the customer.
    ///
    /// https://schema.org/ReturnLabelCustomerResponsibility
    ReturnLabelCustomerResponsibility,
    /// Indicated that a return label must be downloaded and printed by the customer.
    ///
    /// https://schema.org/ReturnLabelDownloadAndPrint
    ReturnLabelDownloadAndPrint,
    /// Specifies that a return label will be provided by the seller in the shipping box.
    ///
    /// https://schema.org/ReturnLabelInBox
    ReturnLabelInBox,
}
