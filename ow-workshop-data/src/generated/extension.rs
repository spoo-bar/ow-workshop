//! Workshop extensions.
/// A workshop extension.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Extension {
    /// `Beam Effects`
    BeamEffects,
    /// `Beam Sounds`
    BeamSounds,
    /// `Buff and Debuff Sounds`
    BuffAndDebuffSounds,
    /// `Buff Status Effects`
    BuffStatusEffects,
    /// `Debuff Status Effects`
    DebuffStatusEffects,
    /// `Energy Explosion Effects`
    EnergyExplosionEffects,
    /// `Explosion Sounds`
    ExplosionSounds,
    /// `Kinetic Explosion Effects`
    KineticExplosionEffects,
    /// `Play More Effects`
    PlayMoreEffects,
    /// `Projectiles`
    Projectiles,
    /// `Spawn More Dummy Bots`
    SpawnMoreDummyBots,
}
impl Extension {
    /// Every variant, in declaration order.
    pub const ALL: &'static [Self] = &[
        Self::BeamEffects,
        Self::BeamSounds,
        Self::BuffAndDebuffSounds,
        Self::BuffStatusEffects,
        Self::DebuffStatusEffects,
        Self::EnergyExplosionEffects,
        Self::ExplosionSounds,
        Self::KineticExplosionEffects,
        Self::PlayMoreEffects,
        Self::Projectiles,
        Self::SpawnMoreDummyBots,
    ];
    /// The data key, for debugging.
    pub fn key(self) -> &'static str {
        match self {
            Self::BeamEffects => "beamEffects",
            Self::BeamSounds => "beamSounds",
            Self::BuffAndDebuffSounds => "buffAndDebuffSounds",
            Self::BuffStatusEffects => "buffStatusEffects",
            Self::DebuffStatusEffects => "debuffStatusEffects",
            Self::EnergyExplosionEffects => "energyExplosionEffects",
            Self::ExplosionSounds => "explosionSounds",
            Self::KineticExplosionEffects => "kineticExplosionEffects",
            Self::PlayMoreEffects => "playMoreEffects",
            Self::Projectiles => "projectiles",
            Self::SpawnMoreDummyBots => "spawnMoreDummyBots",
        }
    }
    /// The name as written in workshop code.
    pub fn name(self) -> &'static str {
        static NAMES: [&str; 11usize] = [
            "Beam Effects",
            "Beam Sounds",
            "Buff and Debuff Sounds",
            "Buff Status Effects",
            "Debuff Status Effects",
            "Energy Explosion Effects",
            "Explosion Sounds",
            "Kinetic Explosion Effects",
            "Play More Effects",
            "Projectiles",
            "Spawn More Dummy Bots",
        ];
        NAMES[self as usize]
    }
}
impl Extension {
    /// The extension's cost in workshop points.
    pub fn points(self) -> u32 {
        match self {
            Self::BeamSounds | Self::PlayMoreEffects => 1,
            Self::BeamEffects
            | Self::BuffAndDebuffSounds
            | Self::BuffStatusEffects
            | Self::DebuffStatusEffects
            | Self::ExplosionSounds
            | Self::SpawnMoreDummyBots => 2,
            Self::EnergyExplosionEffects
            | Self::KineticExplosionEffects
            | Self::Projectiles => 4,
        }
    }
}
