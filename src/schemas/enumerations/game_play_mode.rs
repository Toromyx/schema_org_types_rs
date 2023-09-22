/// Indicates whether this game is multi-player, co-op or single-player.
///
/// https://schema.org/GamePlayMode
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GamePlayMode {
    /// Play mode: CoOp. Co-operative games, where you play on the same team with friends.
    ///
    /// https://schema.org/CoOp
    CoOp,
    /// Play mode: MultiPlayer. Requiring or allowing multiple human players to play simultaneously.
    ///
    /// https://schema.org/MultiPlayer
    MultiPlayer,
    /// Play mode: SinglePlayer. Which is played by a lone player.
    ///
    /// https://schema.org/SinglePlayer
    SinglePlayer,
}