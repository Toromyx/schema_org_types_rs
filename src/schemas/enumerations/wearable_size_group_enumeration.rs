/// Enumerates common size groups (also known as "size types") for wearable products.
///
/// https://schema.org/WearableSizeGroupEnumeration
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum WearableSizeGroupEnumeration {
    /// Size group "Big" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupBig
    WearableSizeGroupBig,
    /// Size group "Boys" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupBoys
    WearableSizeGroupBoys,
    /// Size group "Extra Short" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupExtraShort
    WearableSizeGroupExtraShort,
    /// Size group "Extra Tall" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupExtraTall
    WearableSizeGroupExtraTall,
    /// Size group "Girls" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupGirls
    WearableSizeGroupGirls,
    /// Size group "Husky" (or "Stocky") for wearables.
    ///
    /// https://schema.org/WearableSizeGroupHusky
    WearableSizeGroupHusky,
    /// Size group "Infants" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupInfants
    WearableSizeGroupInfants,
    /// Size group "Juniors" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupJuniors
    WearableSizeGroupJuniors,
    /// Size group "Maternity" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupMaternity
    WearableSizeGroupMaternity,
    /// Size group "Mens" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupMens
    WearableSizeGroupMens,
    /// Size group "Misses" (also known as "Missy") for wearables.
    ///
    /// https://schema.org/WearableSizeGroupMisses
    WearableSizeGroupMisses,
    /// Size group "Petite" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupPetite
    WearableSizeGroupPetite,
    /// Size group "Plus" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupPlus
    WearableSizeGroupPlus,
    /// Size group "Regular" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupRegular
    WearableSizeGroupRegular,
    /// Size group "Short" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupShort
    WearableSizeGroupShort,
    /// Size group "Tall" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupTall
    WearableSizeGroupTall,
    /// Size group "Womens" for wearables.
    ///
    /// https://schema.org/WearableSizeGroupWomens
    WearableSizeGroupWomens,
}
