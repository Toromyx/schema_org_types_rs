/// <https://schema.org/AdultOrientedEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum AdultOrientedEnumeration {
    /// <https://schema.org/AlcoholConsideration>
    AlcoholConsideration,
    /// <https://schema.org/DangerousGoodConsideration>
    DangerousGoodConsideration,
    /// <https://schema.org/HealthcareConsideration>
    HealthcareConsideration,
    /// <https://schema.org/NarcoticConsideration>
    NarcoticConsideration,
    /// <https://schema.org/ReducedRelevanceForChildrenConsideration>
    ReducedRelevanceForChildrenConsideration,
    /// <https://schema.org/SexualContentConsideration>
    SexualContentConsideration,
    /// <https://schema.org/TobaccoNicotineConsideration>
    TobaccoNicotineConsideration,
    /// <https://schema.org/UnclassifiedAdultConsideration>
    UnclassifiedAdultConsideration,
    /// <https://schema.org/ViolenceConsideration>
    ViolenceConsideration,
    /// <https://schema.org/WeaponConsideration>
    WeaponConsideration,
}
