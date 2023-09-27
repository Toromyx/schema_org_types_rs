/// <https://schema.org/WearableSizeGroupEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum WearableSizeGroupEnumeration {
    /// <https://schema.org/WearableSizeGroupBig>
    WearableSizeGroupBig,
    /// <https://schema.org/WearableSizeGroupBoys>
    WearableSizeGroupBoys,
    /// <https://schema.org/WearableSizeGroupExtraShort>
    WearableSizeGroupExtraShort,
    /// <https://schema.org/WearableSizeGroupExtraTall>
    WearableSizeGroupExtraTall,
    /// <https://schema.org/WearableSizeGroupGirls>
    WearableSizeGroupGirls,
    /// <https://schema.org/WearableSizeGroupHusky>
    WearableSizeGroupHusky,
    /// <https://schema.org/WearableSizeGroupInfants>
    WearableSizeGroupInfants,
    /// <https://schema.org/WearableSizeGroupJuniors>
    WearableSizeGroupJuniors,
    /// <https://schema.org/WearableSizeGroupMaternity>
    WearableSizeGroupMaternity,
    /// <https://schema.org/WearableSizeGroupMens>
    WearableSizeGroupMens,
    /// <https://schema.org/WearableSizeGroupMisses>
    WearableSizeGroupMisses,
    /// <https://schema.org/WearableSizeGroupPetite>
    WearableSizeGroupPetite,
    /// <https://schema.org/WearableSizeGroupPlus>
    WearableSizeGroupPlus,
    /// <https://schema.org/WearableSizeGroupRegular>
    WearableSizeGroupRegular,
    /// <https://schema.org/WearableSizeGroupShort>
    WearableSizeGroupShort,
    /// <https://schema.org/WearableSizeGroupTall>
    WearableSizeGroupTall,
    /// <https://schema.org/WearableSizeGroupWomens>
    WearableSizeGroupWomens,
}
