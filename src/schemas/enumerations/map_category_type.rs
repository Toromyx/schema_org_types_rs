/// <https://schema.org/MapCategoryType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MapCategoryType {
    /// <https://schema.org/ParkingMap>
    ParkingMap,
    /// <https://schema.org/SeatingMap>
    SeatingMap,
    /// <https://schema.org/TransitMap>
    TransitMap,
    /// <https://schema.org/VenueMap>
    VenueMap,
}
