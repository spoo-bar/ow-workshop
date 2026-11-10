//! Workshop extensions.
use crate::Lang;
use crate::lang::localized;
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
    /// The name as written in workshop code of the given language.
    pub fn name(self, lang: Lang) -> &'static str {
        static NAMES: [&str; 11usize] = [
            "Strahleffekte\u{1f}Beam Effects\u{1f}Efectos de haz\u{1f}Efectos de rayo\u{1f}Effets de rayon\u{1f}\u{1f}ビーム・エフェクト\u{1f}\u{1f}Efekty wiązek\u{1f}Efeitos de Feixe\u{1f}\u{1f}เอฟเฟกต\u{e4c}ลำแสง\u{1f}Işın Efektleri\u{1f}光束效果",
            "Sounds von Strahlen\u{1f}Beam Sounds\u{1f}Sonidos de haz\u{1f}Sonidos de rayo\u{1f}Sons de rayon\u{1f}\u{1f}ビーム音\u{1f}\u{1f}Dźwięki wiązek\u{1f}Sons de Feixes\u{1f}\u{1f}เส\u{e35}ยงลำแสง\u{1f}Işın Sesleri\u{1f}光束声音",
            "Sounds von Stärkungs- und Schwächungseffekten\u{1f}Buff and Debuff Sounds\u{1f}Sonidos de beneficios y perjuicios\u{1f}Sonidos de potenciaciones y despotenciaciones\u{1f}Sons des améliorations et des affaiblissements\u{1f}\u{1f}バフおよびデバフの音\u{1f}\u{1f}Dźwięki wzmocnienia i osłabienia\u{1f}Sons de Melhorias e Penalidades\u{1f}\u{1f}เส\u{e35}ยงบ\u{e31}ฟและด\u{e35}บ\u{e31}ฟ\u{1f}Güçlendirme ve Zayıflatma Sesleri\u{1f}增益效果和减益效果声音",
            "Effekte von Stärkungseffekten\u{1f}Buff Status Effects\u{1f}Efectos de estados de mejoras\u{1f}Potenciar efectos de estados\u{1f}Effets de statut d’amélioration\u{1f}\u{1f}バフステータスのエフェクト\u{1f}\u{1f}Efekty statusu wzmocnienia\u{1f}Efeitos de Melhoria de Status\u{1f}\u{1f}เอฟเฟกต\u{e4c}บ\u{e31}ฟสถานะ\u{1f}Güçlendirme Durum Efektleri\u{1f}增益状态效果",
            "Effekte von Schwächungseffekten\u{1f}Debuff Status Effects\u{1f}Efectos de estados de perjuicios\u{1f}Despotenciar efectos de estados\u{1f}Effets de statut d’affaiblissement\u{1f}\u{1f}デバフステータスのエフェクト\u{1f}\u{1f}Efekty statusu osłabienia\u{1f}Efeitos de Status Negativos\u{1f}\u{1f}เอฟเฟกต\u{e4c}สถานะด\u{e35}บ\u{e31}ฟ\u{1f}Zayıflatma Durum Efektleri\u{1f}减益状态效果",
            "Effekte von Energieexplosionen\u{1f}Energy Explosion Effects\u{1f}Efectos de explosión de energía\u{1f}Efectos de explosión de energía\u{1f}Effets d’explosion énergétique\u{1f}\u{1f}エネルギー爆発エフェクト\u{1f}\u{1f}Efekty eksplozji energetycznych\u{1f}Efeitos de Explosão de Energia\u{1f}\u{1f}เอฟเฟกต\u{e4c}ระเบ\u{e34}ดพล\u{e31}งงาน\u{1f}Enerji Patlama Efektleri\u{1f}能量爆炸效果",
            "Sounds von Explosionen\u{1f}Explosion Sounds\u{1f}Sonidos de explosiones\u{1f}Sonidos de explosión\u{1f}Sons des explosions\u{1f}\u{1f}爆発音\u{1f}\u{1f}Dźwięki eksplozji\u{1f}Sons de Explosão\u{1f}\u{1f}เส\u{e35}ยงระเบ\u{e34}ด\u{1f}Patlama Sesleri\u{1f}爆炸声音",
            "Effekte von kinetischen Explosionen\u{1f}Kinetic Explosion Effects\u{1f}Efectos de explosión cinética\u{1f}Efectos de explosión cinética\u{1f}Effets d’explosion cinétique\u{1f}\u{1f}キネティック爆発エフェクト\u{1f}\u{1f}Efekty eksplozji kinetycznych\u{1f}Efeitos de Explosões Cinéticas\u{1f}\u{1f}เอฟเฟกต\u{e4c}ระเบ\u{e34}ดพล\u{e31}งงานจลน\u{e4c}\u{1f}Kinetik Patlama Efektleri\u{1f}运动爆炸效果",
            "Mehr Effekte abspielen\u{1f}Play More Effects\u{1f}Reproducir más efectos\u{1f}Reproducir más efectos\u{1f}Jouer plus d’effets\u{1f}\u{1f}エフェクトの再生数を増加\u{1f}\u{1f}Odtwórz więcej efektów\u{1f}Reproduzir Mais Efeitos\u{1f}\u{1f}เล\u{e48}นเอฟเฟกต\u{e4c}เพ\u{e34}\u{e48}มเต\u{e34}ม\u{1f}Daha Fazla Efekt Oynat\u{1f}播放更多效果",
            "Projektile\u{1f}Projectiles\u{1f}Proyectiles\u{1f}Proyectiles\u{1f}\u{1f}Proiettili\u{1f}投射物\u{1f}\u{1f}Pociski\u{1f}Projéteis\u{1f}Снаряды\u{1f}โพรเจกไทล\u{e4c}\u{1f}Atılır Cisimler\u{1f}弹道\u{1f}拋射物",
            "Mehr Bots erschaffen\u{1f}Spawn More Dummy Bots\u{1f}Generar más robots\u{1f}Aparecer más robots de entrenamiento\u{1f}Faire apparaître plus d’I.A.\u{1f}\u{1f}ダミーボットの生成数を増加\u{1f}\u{1f}Stwórz więcej atrap botów\u{1f}Gerar Mais Bots\u{1f}\u{1f}เก\u{e34}ดบอทฝ\u{e36}กซ\u{e49}อมเพ\u{e34}\u{e48}มเต\u{e34}ม\u{1f}Daha Fazla Eğitim Robotu Ekle\u{1f}生成更多机器人",
        ];
        localized(NAMES[self as usize], lang)
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
