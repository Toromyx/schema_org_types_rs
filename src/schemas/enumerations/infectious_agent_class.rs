/// Classes of agents or pathogens that transmit infectious diseases. Enumerated type.
///
/// https://schema.org/InfectiousAgentClass
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InfectiousAgentClass {
    /// Pathogenic bacteria that cause bacterial infection.
    ///
    /// https://schema.org/Bacteria
    Bacteria,
    /// Pathogenic fungus.
    ///
    /// https://schema.org/Fungus
    Fungus,
    /// Multicellular parasite that causes an infection.
    ///
    /// https://schema.org/MulticellularParasite
    MulticellularParasite,
    /// A prion is an infectious agent composed of protein in a misfolded form.
    ///
    /// https://schema.org/Prion
    Prion,
    /// Single-celled organism that causes an infection.
    ///
    /// https://schema.org/Protozoa
    Protozoa,
    /// Pathogenic virus that causes viral infection.
    ///
    /// https://schema.org/Virus
    Virus,
}
