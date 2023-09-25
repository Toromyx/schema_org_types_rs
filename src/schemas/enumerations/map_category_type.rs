/// An enumeration of several kinds of Map.
///
/// https://schema.org/MapCategoryType
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MapCategoryType {
    /// A parking map.
    ///
    /// https://schema.org/ParkingMap
    ParkingMap,
    /// A seating map.
    ///
    /// https://schema.org/SeatingMap
    SeatingMap,
    /// A transit map.
    ///
    /// https://schema.org/TransitMap
    TransitMap,
    /// A venue map (e.g. for malls, auditoriums, museums, etc.).
    ///
    /// https://schema.org/VenueMap
    VenueMap,
}
