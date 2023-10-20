/// <https://schema.org/MapCategoryType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
