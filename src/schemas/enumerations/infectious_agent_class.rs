/// <https://schema.org/InfectiousAgentClass>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
