/// <https://schema.org/GamePlayMode>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum GamePlayMode {
    /// <https://schema.org/CoOp>
    CoOp,
    /// <https://schema.org/MultiPlayer>
    MultiPlayer,
    /// <https://schema.org/SinglePlayer>
    SinglePlayer,
}
