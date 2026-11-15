//! Maps.
use super::gamemodes::Gamemode;
/// A map.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Map {
    /// `Aatlis`
    Aatlis,
    /// `Antarctic Peninsula`
    AntarcticPeninsula,
    /// `Arena Victoriae`
    ArenaVictoriae,
    /// `Ayutthaya`
    Ayutthaya,
    /// `Black Forest`
    BlackForest,
    /// `Black Forest Winter`
    BlackForestWinter,
    /// `Blizzard World`
    BlizzWorld,
    /// `Blizzard World Winter`
    BlizzWorldWinter,
    /// `Busan`
    Busan,
    /// `Busan Downtown Lunar New Year`
    BusanDowntownLny,
    /// `Busan Sanctuary Lunar New Year`
    BusanSanctuaryLny,
    /// `Busan Stadium`
    BusanStadium,
    /// `Busan Stadium Classic`
    BusanStadiumClassic,
    /// `Castillo`
    Castillo,
    /// `Château Guillard`
    ChateauGuillard,
    /// `Château Guillard Halloween`
    ChateauGuillardHalloween,
    /// `Circuit Royal`
    CircuitRoyal,
    /// `Colosseo`
    Colosseo,
    /// `Dorado`
    Dorado,
    /// `Ecopoint: Antarctica`
    EcopointAntarctica,
    /// `Ecopoint: Antarctica Winter`
    EcopointAntarcticaWinter,
    /// `Eichenwalde`
    Eichenwalde,
    /// `Eichenwalde Halloween`
    EichenwaldeHalloween,
    /// `Esperança`
    Esperanca,
    /// `Estádio das Rãs`
    EstadioDasRas,
    /// `Gogadoro`
    Gogadoro,
    /// `Grímsvötn`
    Grimsvotn,
    /// `Hanamura`
    Hanamura,
    /// `Hanamura Winter`
    HanamuraWinter,
    /// `Hanaoka`
    Hanaoka,
    /// `Havana`
    Havana,
    /// `Hollywood`
    Hollywood,
    /// `Hollywood Halloween`
    HollywoodHalloween,
    /// `Horizon Lunar Colony`
    HorizonLunarColony,
    /// `Ilios`
    Ilios,
    /// `Ilios Lighthouse`
    IliosLighthouse,
    /// `Ilios Ruins`
    IliosRuins,
    /// `Ilios Well`
    IliosWell,
    /// `Junkenstein's Revenge`
    Junkenstein,
    /// `Junkertown`
    Junkertown,
    /// `Kanezaka`
    Kanezaka,
    /// `King's Row`
    KingsRow,
    /// `King's Row Winter`
    KingsRowWinter,
    /// `Lijiang Control Center`
    LijiangControlCenter,
    /// `Lijiang Control Center Lunar New Year`
    LijiangControlCenterLny,
    /// `Lijiang Garden`
    LijiangGarden,
    /// `Lijiang Garden Lunar New Year`
    LijiangGardenLny,
    /// `Lijiang Night Market`
    LijiangNightMarket,
    /// `Lijiang Night Market Lunar New Year`
    LijiangNightMarketLny,
    /// `Lijiang Tower`
    LijiangTower,
    /// `Lijiang Tower Lunar New Year`
    LijiangTowerLny,
    /// `Malevento`
    Malevento,
    /// `Midtown`
    Midtown,
    /// `Necropolis`
    Necropolis,
    /// `Neon Junction`
    NeonJunction,
    /// `Nepal`
    Nepal,
    /// `Nepal Sanctum`
    NepalSanctum,
    /// `Nepal Shrine`
    NepalShrine,
    /// `Nepal Village`
    NepalVillage,
    /// `Nepal Village Winter`
    NepalVillageWinter,
    /// `New Junk City`
    NewJunkCity,
    /// `New Queen Street`
    NewQueenStreet,
    /// `Numbani`
    Numbani,
    /// `Oasis`
    Oasis,
    /// `Oasis City Center`
    OasisCityCenter,
    /// `Oasis Gardens`
    OasisGardens,
    /// `Oasis University`
    OasisUniversity,
    /// `Paraíso`
    Paraiso,
    /// `Paris`
    Paris,
    /// `Petra`
    Petra,
    /// `Place Lacroix`
    PlaceLacroix,
    /// `Practice Range`
    PracticeRange,
    /// `Redwood Dam`
    RedwoodDam,
    /// `Rialto`
    Rialto,
    /// `Route 66`
    Route66,
    /// `Runasapi`
    Runasapi,
    /// `Samoa`
    Samoa,
    /// `Shambali Monastery`
    ShambaliMonastery,
    /// `Suravasa`
    Suravasa,
    /// `Sydney Harbour Arena`
    SydneyHarbourArena,
    /// `Sydney Harbour Arena Classic`
    SydneyHarbourArenaClassic,
    /// `Temple of Anubis`
    TempleOfAnubis,
    /// `Throne of Anubis`
    ThroneOfAnubis,
    /// `Volskaya Industries`
    Volskaya,
    /// `Watchpoint: Gibraltar`
    WatchpointGibraltar,
    /// `Workshop Chamber`
    WorkshopChamber,
    /// `Workshop Expanse`
    WorkshopExpanse,
    /// `Workshop Expanse Night`
    WorkshopExpanseNight,
    /// `Workshop Green Screen`
    WorkshopGreenScreen,
    /// `Workshop Island`
    WorkshopIsland,
    /// `Workshop Island Night`
    WorkshopIslandNight,
}
impl Map {
    /// Every variant, in declaration order.
    pub const ALL: &'static [Self] = &[
        Self::Aatlis,
        Self::AntarcticPeninsula,
        Self::ArenaVictoriae,
        Self::Ayutthaya,
        Self::BlackForest,
        Self::BlackForestWinter,
        Self::BlizzWorld,
        Self::BlizzWorldWinter,
        Self::Busan,
        Self::BusanDowntownLny,
        Self::BusanSanctuaryLny,
        Self::BusanStadium,
        Self::BusanStadiumClassic,
        Self::Castillo,
        Self::ChateauGuillard,
        Self::ChateauGuillardHalloween,
        Self::CircuitRoyal,
        Self::Colosseo,
        Self::Dorado,
        Self::EcopointAntarctica,
        Self::EcopointAntarcticaWinter,
        Self::Eichenwalde,
        Self::EichenwaldeHalloween,
        Self::Esperanca,
        Self::EstadioDasRas,
        Self::Gogadoro,
        Self::Grimsvotn,
        Self::Hanamura,
        Self::HanamuraWinter,
        Self::Hanaoka,
        Self::Havana,
        Self::Hollywood,
        Self::HollywoodHalloween,
        Self::HorizonLunarColony,
        Self::Ilios,
        Self::IliosLighthouse,
        Self::IliosRuins,
        Self::IliosWell,
        Self::Junkenstein,
        Self::Junkertown,
        Self::Kanezaka,
        Self::KingsRow,
        Self::KingsRowWinter,
        Self::LijiangControlCenter,
        Self::LijiangControlCenterLny,
        Self::LijiangGarden,
        Self::LijiangGardenLny,
        Self::LijiangNightMarket,
        Self::LijiangNightMarketLny,
        Self::LijiangTower,
        Self::LijiangTowerLny,
        Self::Malevento,
        Self::Midtown,
        Self::Necropolis,
        Self::NeonJunction,
        Self::Nepal,
        Self::NepalSanctum,
        Self::NepalShrine,
        Self::NepalVillage,
        Self::NepalVillageWinter,
        Self::NewJunkCity,
        Self::NewQueenStreet,
        Self::Numbani,
        Self::Oasis,
        Self::OasisCityCenter,
        Self::OasisGardens,
        Self::OasisUniversity,
        Self::Paraiso,
        Self::Paris,
        Self::Petra,
        Self::PlaceLacroix,
        Self::PracticeRange,
        Self::RedwoodDam,
        Self::Rialto,
        Self::Route66,
        Self::Runasapi,
        Self::Samoa,
        Self::ShambaliMonastery,
        Self::Suravasa,
        Self::SydneyHarbourArena,
        Self::SydneyHarbourArenaClassic,
        Self::TempleOfAnubis,
        Self::ThroneOfAnubis,
        Self::Volskaya,
        Self::WatchpointGibraltar,
        Self::WorkshopChamber,
        Self::WorkshopExpanse,
        Self::WorkshopExpanseNight,
        Self::WorkshopGreenScreen,
        Self::WorkshopIsland,
        Self::WorkshopIslandNight,
    ];
    /// The data key, for debugging.
    pub fn key(self) -> &'static str {
        match self {
            Self::Aatlis => "aatlis",
            Self::AntarcticPeninsula => "antarcticPeninsula",
            Self::ArenaVictoriae => "arenaVictoriae",
            Self::Ayutthaya => "ayutthaya",
            Self::BlackForest => "blackForest",
            Self::BlackForestWinter => "blackForestWinter",
            Self::BlizzWorld => "blizzWorld",
            Self::BlizzWorldWinter => "blizzWorldWinter",
            Self::Busan => "busan",
            Self::BusanDowntownLny => "busanDowntownLny",
            Self::BusanSanctuaryLny => "busanSanctuaryLny",
            Self::BusanStadium => "busanStadium",
            Self::BusanStadiumClassic => "busanStadiumClassic",
            Self::Castillo => "castillo",
            Self::ChateauGuillard => "chateauGuillard",
            Self::ChateauGuillardHalloween => "chateauGuillardHalloween",
            Self::CircuitRoyal => "circuitRoyal",
            Self::Colosseo => "colosseo",
            Self::Dorado => "dorado",
            Self::EcopointAntarctica => "ecopointAntarctica",
            Self::EcopointAntarcticaWinter => "ecopointAntarcticaWinter",
            Self::Eichenwalde => "eichenwalde",
            Self::EichenwaldeHalloween => "eichenwaldeHalloween",
            Self::Esperanca => "esperanca",
            Self::EstadioDasRas => "estadioDasRas",
            Self::Gogadoro => "gogadoro",
            Self::Grimsvotn => "grimsvotn",
            Self::Hanamura => "hanamura",
            Self::HanamuraWinter => "hanamuraWinter",
            Self::Hanaoka => "hanaoka",
            Self::Havana => "havana",
            Self::Hollywood => "hollywood",
            Self::HollywoodHalloween => "hollywoodHalloween",
            Self::HorizonLunarColony => "horizonLunarColony",
            Self::Ilios => "ilios",
            Self::IliosLighthouse => "iliosLighthouse",
            Self::IliosRuins => "iliosRuins",
            Self::IliosWell => "iliosWell",
            Self::Junkenstein => "junkenstein",
            Self::Junkertown => "junkertown",
            Self::Kanezaka => "kanezaka",
            Self::KingsRow => "kingsRow",
            Self::KingsRowWinter => "kingsRowWinter",
            Self::LijiangControlCenter => "lijiangControlCenter",
            Self::LijiangControlCenterLny => "lijiangControlCenterLny",
            Self::LijiangGarden => "lijiangGarden",
            Self::LijiangGardenLny => "lijiangGardenLny",
            Self::LijiangNightMarket => "lijiangNightMarket",
            Self::LijiangNightMarketLny => "lijiangNightMarketLny",
            Self::LijiangTower => "lijiangTower",
            Self::LijiangTowerLny => "lijiangTowerLny",
            Self::Malevento => "malevento",
            Self::Midtown => "midtown",
            Self::Necropolis => "necropolis",
            Self::NeonJunction => "neonJunction",
            Self::Nepal => "nepal",
            Self::NepalSanctum => "nepalSanctum",
            Self::NepalShrine => "nepalShrine",
            Self::NepalVillage => "nepalVillage",
            Self::NepalVillageWinter => "nepalVillageWinter",
            Self::NewJunkCity => "newJunkCity",
            Self::NewQueenStreet => "newQueenStreet",
            Self::Numbani => "numbani",
            Self::Oasis => "oasis",
            Self::OasisCityCenter => "oasisCityCenter",
            Self::OasisGardens => "oasisGardens",
            Self::OasisUniversity => "oasisUniversity",
            Self::Paraiso => "paraiso",
            Self::Paris => "paris",
            Self::Petra => "petra",
            Self::PlaceLacroix => "placeLacroix",
            Self::PracticeRange => "practiceRange",
            Self::RedwoodDam => "redwoodDam",
            Self::Rialto => "rialto",
            Self::Route66 => "route66",
            Self::Runasapi => "runasapi",
            Self::Samoa => "samoa",
            Self::ShambaliMonastery => "shambaliMonastery",
            Self::Suravasa => "suravasa",
            Self::SydneyHarbourArena => "sydneyHarbourArena",
            Self::SydneyHarbourArenaClassic => "sydneyHarbourArenaClassic",
            Self::TempleOfAnubis => "templeOfAnubis",
            Self::ThroneOfAnubis => "throneOfAnubis",
            Self::Volskaya => "volskaya",
            Self::WatchpointGibraltar => "watchpointGibraltar",
            Self::WorkshopChamber => "workshopChamber",
            Self::WorkshopExpanse => "workshopExpanse",
            Self::WorkshopExpanseNight => "workshopExpanseNight",
            Self::WorkshopGreenScreen => "workshopGreenScreen",
            Self::WorkshopIsland => "workshopIsland",
            Self::WorkshopIslandNight => "workshopIslandNight",
        }
    }
    /// The name as written in workshop code.
    pub fn name(self) -> &'static str {
        static NAMES: [&str; 91usize] = [
            "Aatlis",
            "Antarctic Peninsula",
            "Arena Victoriae",
            "Ayutthaya",
            "Black Forest",
            "Black Forest Winter",
            "Blizzard World",
            "Blizzard World Winter",
            "Busan",
            "Busan Downtown Lunar New Year",
            "Busan Sanctuary Lunar New Year",
            "Busan Stadium",
            "Busan Stadium Classic",
            "Castillo",
            "Château Guillard",
            "Château Guillard Halloween",
            "Circuit Royal",
            "Colosseo",
            "Dorado",
            "Ecopoint: Antarctica",
            "Ecopoint: Antarctica Winter",
            "Eichenwalde",
            "Eichenwalde Halloween",
            "Esperança",
            "Estádio das Rãs",
            "Gogadoro",
            "Grímsvötn",
            "Hanamura",
            "Hanamura Winter",
            "Hanaoka",
            "Havana",
            "Hollywood",
            "Hollywood Halloween",
            "Horizon Lunar Colony",
            "Ilios",
            "Ilios Lighthouse",
            "Ilios Ruins",
            "Ilios Well",
            "Junkenstein's Revenge",
            "Junkertown",
            "Kanezaka",
            "King's Row",
            "King's Row Winter",
            "Lijiang Control Center",
            "Lijiang Control Center Lunar New Year",
            "Lijiang Garden",
            "Lijiang Garden Lunar New Year",
            "Lijiang Night Market",
            "Lijiang Night Market Lunar New Year",
            "Lijiang Tower",
            "Lijiang Tower Lunar New Year",
            "Malevento",
            "Midtown",
            "Necropolis",
            "Neon Junction",
            "Nepal",
            "Nepal Sanctum",
            "Nepal Shrine",
            "Nepal Village",
            "Nepal Village Winter",
            "New Junk City",
            "New Queen Street",
            "Numbani",
            "Oasis",
            "Oasis City Center",
            "Oasis Gardens",
            "Oasis University",
            "Paraíso",
            "Paris",
            "Petra",
            "Place Lacroix",
            "Practice Range",
            "Redwood Dam",
            "Rialto",
            "Route 66",
            "Runasapi",
            "Samoa",
            "Shambali Monastery",
            "Suravasa",
            "Sydney Harbour Arena",
            "Sydney Harbour Arena Classic",
            "Temple of Anubis",
            "Throne of Anubis",
            "Volskaya Industries",
            "Watchpoint: Gibraltar",
            "Workshop Chamber",
            "Workshop Expanse",
            "Workshop Expanse Night",
            "Workshop Green Screen",
            "Workshop Island",
            "Workshop Island Night",
        ];
        NAMES[self as usize]
    }
}
/// A map's symmetry axis, as given in the source data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Symmetry {
    /// An axis described by the parameters `a` and `b`.
    Axis {
        /// First parameter.
        a: f64,
        /// Second parameter.
        b: f64,
    },
    /// A vertical axis at `x`.
    AxisX(f64),
}
impl Map {
    /// Whether this is a workshop-only map.
    pub fn is_workshop_map(self) -> bool {
        matches!(
            self, Self::WorkshopChamber | Self::WorkshopExpanse |
            Self::WorkshopExpanseNight | Self::WorkshopGreenScreen | Self::WorkshopIsland
            | Self::WorkshopIslandNight
        )
    }
    /// Whether the map only exists in Overwatch 1.
    pub fn only_in_ow1(self) -> bool {
        matches!(
            self, Self::BusanStadium | Self::BusanStadiumClassic | Self::EstadioDasRas |
            Self::SydneyHarbourArena | Self::SydneyHarbourArenaClassic
        )
    }
    /// The game modes this map supports.
    pub fn gamemodes(self) -> &'static [Gamemode] {
        match self {
            Self::Hanamura
            | Self::HanamuraWinter
            | Self::HorizonLunarColony
            | Self::Paris
            | Self::TempleOfAnubis
            | Self::Volskaya => {
                &[
                    Gamemode::Assault,
                    Gamemode::Skirmish,
                    Gamemode::Deathmatch,
                    Gamemode::BountyHunter,
                    Gamemode::TeamDeathmatch,
                ]
            }
            Self::BusanDowntownLny | Self::BusanSanctuaryLny => {
                &[Gamemode::CaptureTheFlag]
            }
            Self::Ayutthaya => {
                &[
                    Gamemode::CaptureTheFlag,
                    Gamemode::Elimination,
                    Gamemode::FreezethawElimination,
                ]
            }
            Self::Hanaoka | Self::ThroneOfAnubis => {
                &[Gamemode::Clash, Gamemode::Skirmish]
            }
            Self::ArenaVictoriae | Self::Gogadoro => &[Gamemode::Control],
            Self::AntarcticPeninsula
            | Self::Busan
            | Self::Ilios
            | Self::LijiangTower
            | Self::LijiangTowerLny
            | Self::Nepal
            | Self::Oasis
            | Self::Samoa => &[Gamemode::Control, Gamemode::Skirmish],
            Self::ChateauGuillard
            | Self::ChateauGuillardHalloween
            | Self::Kanezaka
            | Self::Malevento
            | Self::Petra => {
                &[Gamemode::Deathmatch, Gamemode::BountyHunter, Gamemode::TeamDeathmatch]
            }
            Self::IliosLighthouse
            | Self::IliosRuins
            | Self::IliosWell
            | Self::LijiangControlCenter
            | Self::LijiangControlCenterLny
            | Self::LijiangGarden
            | Self::LijiangGardenLny
            | Self::LijiangNightMarket
            | Self::LijiangNightMarketLny
            | Self::NepalSanctum
            | Self::NepalShrine
            | Self::NepalVillage
            | Self::OasisCityCenter
            | Self::OasisGardens
            | Self::OasisUniversity => {
                &[
                    Gamemode::Elimination,
                    Gamemode::CaptureTheFlag,
                    Gamemode::Deathmatch,
                    Gamemode::BountyHunter,
                    Gamemode::TeamDeathmatch,
                ]
            }
            Self::BlackForest
            | Self::Castillo
            | Self::EcopointAntarctica
            | Self::Necropolis => {
                &[
                    Gamemode::Elimination,
                    Gamemode::FreezethawElimination,
                    Gamemode::Deathmatch,
                    Gamemode::BountyHunter,
                    Gamemode::TeamDeathmatch,
                ]
            }
            Self::BlackForestWinter | Self::EcopointAntarcticaWinter => {
                &[
                    Gamemode::Elimination,
                    Gamemode::FreezethawElimination,
                    Gamemode::Deathmatch,
                    Gamemode::BountyHunter,
                    Gamemode::TeamDeathmatch,
                    Gamemode::MeisSnowballOffensive,
                    Gamemode::SnowballFfa,
                ]
            }
            Self::Grimsvotn
            | Self::Junkertown
            | Self::Rialto
            | Self::WatchpointGibraltar => &[Gamemode::Escort, Gamemode::Skirmish],
            Self::Dorado | Self::Havana => {
                &[
                    Gamemode::Escort,
                    Gamemode::Skirmish,
                    Gamemode::Deathmatch,
                    Gamemode::BountyHunter,
                    Gamemode::TeamDeathmatch,
                ]
            }
            Self::Route66 => {
                &[
                    Gamemode::Escort,
                    Gamemode::Skirmish,
                    Gamemode::Deathmatch,
                    Gamemode::TeamDeathmatch,
                ]
            }
            Self::Midtown => &[Gamemode::Hybrid],
            Self::Numbani => &[Gamemode::Hybrid, Gamemode::Skirmish],
            Self::BlizzWorld
            | Self::BlizzWorldWinter
            | Self::Eichenwalde
            | Self::EichenwaldeHalloween
            | Self::Hollywood
            | Self::KingsRow
            | Self::KingsRowWinter => {
                &[
                    Gamemode::Hybrid,
                    Gamemode::Skirmish,
                    Gamemode::Deathmatch,
                    Gamemode::BountyHunter,
                    Gamemode::TeamDeathmatch,
                ]
            }
            Self::HollywoodHalloween => {
                &[
                    Gamemode::Hybrid,
                    Gamemode::Skirmish,
                    Gamemode::Deathmatch,
                    Gamemode::TeamDeathmatch,
                ]
            }
            Self::Junkenstein => &[Gamemode::Junkenstein],
            Self::BusanStadium
            | Self::BusanStadiumClassic
            | Self::EstadioDasRas
            | Self::SydneyHarbourArena
            | Self::SydneyHarbourArenaClassic => &[Gamemode::Lucioball],
            Self::PracticeRange => &[Gamemode::PracticeRange],
            Self::PlaceLacroix | Self::RedwoodDam => &[Gamemode::Push],
            Self::WorkshopChamber
            | Self::WorkshopExpanse
            | Self::WorkshopExpanseNight
            | Self::WorkshopGreenScreen
            | Self::WorkshopIsland
            | Self::WorkshopIslandNight => {
                &[
                    Gamemode::Skirmish,
                    Gamemode::Elimination,
                    Gamemode::Deathmatch,
                    Gamemode::BountyHunter,
                    Gamemode::TeamDeathmatch,
                ]
            }
            Self::CircuitRoyal | Self::ShambaliMonastery => {
                &[Gamemode::Skirmish, Gamemode::Escort]
            }
            Self::Aatlis | Self::NewJunkCity | Self::Suravasa => {
                &[Gamemode::Skirmish, Gamemode::Flashpoint]
            }
            Self::NeonJunction | Self::Paraiso => &[Gamemode::Skirmish, Gamemode::Hybrid],
            Self::Colosseo | Self::Esperanca | Self::NewQueenStreet | Self::Runasapi => {
                &[Gamemode::Skirmish, Gamemode::Push]
            }
            Self::NepalVillageWinter => &[Gamemode::YetiHunter],
        }
    }
    /// The map's symmetry axis, if it has one.
    pub fn symmetry(self) -> Option<Symmetry> {
        match self {
            Self::Castillo => {
                Some(Symmetry::Axis {
                    a: -0.19009,
                    b: 38.79879,
                })
            }
            Self::IliosLighthouse => {
                Some(Symmetry::Axis {
                    a: -0.41437,
                    b: 96.86593,
                })
            }
            Self::OasisGardens => {
                Some(Symmetry::Axis {
                    a: -1.00042,
                    b: -106.1637,
                })
            }
            Self::Ayutthaya => Some(Symmetry::Axis { a: 0.0, b: -7.0 }),
            Self::EcopointAntarctica
            | Self::EcopointAntarcticaWinter
            | Self::Necropolis
            | Self::NepalSanctum
            | Self::NepalShrine
            | Self::NepalVillage
            | Self::OasisUniversity
            | Self::WorkshopChamber
            | Self::WorkshopExpanse
            | Self::WorkshopExpanseNight
            | Self::WorkshopIsland
            | Self::WorkshopIslandNight => Some(Symmetry::Axis { a: 0.0, b: 0.0 }),
            Self::BlackForest | Self::BlackForestWinter => {
                Some(Symmetry::Axis { a: 0.0, b: 5.0 })
            }
            Self::OasisCityCenter => {
                Some(Symmetry::Axis {
                    a: 1.00002,
                    b: 103.99783,
                })
            }
            Self::IliosWell => {
                Some(Symmetry::Axis {
                    a: 1.00409,
                    b: 192.85907,
                })
            }
            Self::BusanSanctuaryLny => Some(Symmetry::AxisX(-328.565)),
            Self::LijiangControlCenter
            | Self::LijiangControlCenterLny
            | Self::LijiangGarden
            | Self::LijiangGardenLny => Some(Symmetry::AxisX(0.0)),
            Self::LijiangNightMarket | Self::LijiangNightMarketLny => {
                Some(Symmetry::AxisX(1.0))
            }
            Self::IliosRuins => Some(Symmetry::AxisX(28.6)),
            Self::BusanDowntownLny => Some(Symmetry::AxisX(51.9)),
            _ => None,
        }
    }
    /// Map variants as `(key, id)` pairs.
    pub fn variants(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::NewQueenStreet => {
                &[("dawn", "972777519512068197"), ("morning", "972777519512068154")]
            }
            Self::LijiangTower => {
                &[("dawn", "972777519512068197"), ("night", "972777519512068153")]
            }
            Self::Nepal => {
                &[("dawn", "972777519512068292"), ("morning", "972777519512068154")]
            }
            Self::Ayutthaya
            | Self::BlackForest
            | Self::BlackForestWinter
            | Self::BlizzWorldWinter
            | Self::BusanDowntownLny
            | Self::BusanSanctuaryLny
            | Self::Castillo
            | Self::ChateauGuillard
            | Self::ChateauGuillardHalloween
            | Self::EcopointAntarcticaWinter
            | Self::EichenwaldeHalloween
            | Self::HanamuraWinter
            | Self::HollywoodHalloween
            | Self::IliosLighthouse
            | Self::IliosRuins
            | Self::IliosWell
            | Self::Kanezaka
            | Self::KingsRowWinter
            | Self::LijiangControlCenter
            | Self::LijiangControlCenterLny
            | Self::LijiangGarden
            | Self::LijiangGardenLny
            | Self::LijiangNightMarket
            | Self::LijiangNightMarketLny
            | Self::LijiangTowerLny
            | Self::Malevento
            | Self::NepalSanctum
            | Self::NepalShrine
            | Self::NepalVillage
            | Self::PracticeRange
            | Self::WorkshopExpanse
            | Self::WorkshopIsland => &[("default", "972777519512063901")],
            Self::WorkshopChamber | Self::WorkshopExpanseNight => {
                &[("default", "972777519512064577")]
            }
            Self::WorkshopIslandNight => &[("default", "972777519512064579")],
            Self::WorkshopGreenScreen => &[("default", "972777519512064580")],
            Self::Aatlis | Self::Necropolis | Self::ShambaliMonastery => {
                &[("default", "972777519512068153")]
            }
            Self::EcopointAntarctica
            | Self::Esperanca
            | Self::Hanamura
            | Self::Hanaoka
            | Self::HorizonLunarColony
            | Self::Junkertown
            | Self::NewJunkCity
            | Self::Numbani
            | Self::Petra
            | Self::Runasapi
            | Self::Samoa
            | Self::Suravasa
            | Self::TempleOfAnubis
            | Self::ThroneOfAnubis => &[("default", "972777519512068154")],
            Self::OasisCityCenter
            | Self::OasisGardens
            | Self::OasisUniversity
            | Self::Paris
            | Self::Volskaya => &[("default", "972777519512068292")],
            Self::WatchpointGibraltar => &[("default", "972777519512099996")],
            Self::Colosseo
            | Self::Eichenwalde
            | Self::Ilios
            | Self::Paraiso
            | Self::Rialto => {
                &[("evening", "972777519512068292"), ("morning", "972777519512068154")]
            }
            Self::Oasis => {
                &[
                    ("evening", "972777519512068292"),
                    ("morning", "972777519512068154"),
                    ("night", "972777519512068153"),
                ]
            }
            Self::Dorado | Self::KingsRow => {
                &[("evening", "972777519512068292"), ("night", "972777519512068153")]
            }
            Self::PlaceLacroix => {
                &[("morning", "972777519512055904"), ("sunset", "972777519512068292")]
            }
            Self::AntarcticPeninsula
            | Self::CircuitRoyal
            | Self::Gogadoro
            | Self::Havana
            | Self::Hollywood
            | Self::Midtown => {
                &[("morning", "972777519512068154"), ("night", "972777519512068153")]
            }
            Self::BlizzWorld | Self::Busan | Self::Route66 => {
                &[
                    ("morning", "972777519512068154"),
                    ("night", "972777519512068153"),
                    ("overcast", "972777519512068194"),
                ]
            }
            Self::RedwoodDam => {
                &[("morning", "972777519512068154"), ("sunset", "972777519512068292")]
            }
            Self::ArenaVictoriae => {
                &[("night", "972777519512068153"), ("sunset", "972777519512068292")]
            }
            _ => &[],
        }
    }
}
