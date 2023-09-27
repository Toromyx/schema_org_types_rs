/// <https://schema.org/InfectiousAgentClass>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum InfectiousAgentClass {
    /// <https://schema.org/Bacteria>
    Bacteria,
    /// <https://schema.org/Fungus>
    Fungus,
    /// <https://schema.org/MulticellularParasite>
    MulticellularParasite,
    /// <https://schema.org/Prion>
    Prion,
    /// <https://schema.org/Protozoa>
    Protozoa,
    /// <https://schema.org/Virus>
    Virus,
}
