/// <https://schema.org/GamePlayMode>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GamePlayMode {
	/// <https://schema.org/CoOp>
	CoOp,
	/// <https://schema.org/MultiPlayer>
	MultiPlayer,
	/// <https://schema.org/SinglePlayer>
	SinglePlayer,
}
