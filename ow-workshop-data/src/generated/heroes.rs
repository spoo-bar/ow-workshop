//! Heroes.
/// A playable hero.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Hero {
    /// `Ana`
    Ana,
    /// `Anran`
    Anran,
    /// `Ashe`
    Ashe,
    /// `Baptiste`
    Baptiste,
    /// `Bastion`
    Bastion,
    /// `Brigitte`
    Brigitte,
    /// `Cassidy`
    Cassidy,
    /// `D.Mon`
    Dmon,
    /// `Doctrine`
    Doctrine,
    /// `Domina`
    Domina,
    /// `Doomfist`
    Doomfist,
    /// `D.Va`
    Dva,
    /// `Echo`
    Echo,
    /// `Emre`
    Emre,
    /// `Freja`
    Freja,
    /// `Genji`
    Genji,
    /// `Hanzo`
    Hanzo,
    /// `Hazard`
    Hazard,
    /// `Illari`
    Illari,
    /// `Jetpack Cat`
    JetpackCat,
    /// `Junker Queen`
    JunkerQueen,
    /// `Junkrat`
    Junkrat,
    /// `Juno`
    Juno,
    /// `Kiriko`
    Kiriko,
    /// `Lifeweaver`
    Lifeweaver,
    /// `Lúcio`
    Lucio,
    /// `Mauga`
    Mauga,
    /// `Mei`
    Mei,
    /// `Mercy`
    Mercy,
    /// `Mizuki`
    Mizuki,
    /// `Moira`
    Moira,
    /// `Orisa`
    Orisa,
    /// `Pharah`
    Pharah,
    /// `Ramattra`
    Ramattra,
    /// `Reaper`
    Reaper,
    /// `Reinhardt`
    Reinhardt,
    /// `Roadhog`
    Roadhog,
    /// `Shion`
    Shion,
    /// `Sierra`
    Sierra,
    /// `Sigma`
    Sigma,
    /// `Sojourn`
    Sojourn,
    /// `Soldier: 76`
    Soldier,
    /// `Sombra`
    Sombra,
    /// `Symmetra`
    Symmetra,
    /// `Torbjörn`
    Torbjorn,
    /// `Tracer`
    Tracer,
    /// `Vendetta`
    Vendetta,
    /// `Venture`
    Venture,
    /// `Widowmaker`
    Widowmaker,
    /// `Winston`
    Winston,
    /// `Wrecking Ball`
    WreckingBall,
    /// `Wuyang`
    Wuyang,
    /// `Zarya`
    Zarya,
    /// `Zenyatta`
    Zenyatta,
}
impl Hero {
    /// Every variant, in declaration order.
    pub const ALL: &'static [Self] = &[
        Self::Ana,
        Self::Anran,
        Self::Ashe,
        Self::Baptiste,
        Self::Bastion,
        Self::Brigitte,
        Self::Cassidy,
        Self::Dmon,
        Self::Doctrine,
        Self::Domina,
        Self::Doomfist,
        Self::Dva,
        Self::Echo,
        Self::Emre,
        Self::Freja,
        Self::Genji,
        Self::Hanzo,
        Self::Hazard,
        Self::Illari,
        Self::JetpackCat,
        Self::JunkerQueen,
        Self::Junkrat,
        Self::Juno,
        Self::Kiriko,
        Self::Lifeweaver,
        Self::Lucio,
        Self::Mauga,
        Self::Mei,
        Self::Mercy,
        Self::Mizuki,
        Self::Moira,
        Self::Orisa,
        Self::Pharah,
        Self::Ramattra,
        Self::Reaper,
        Self::Reinhardt,
        Self::Roadhog,
        Self::Shion,
        Self::Sierra,
        Self::Sigma,
        Self::Sojourn,
        Self::Soldier,
        Self::Sombra,
        Self::Symmetra,
        Self::Torbjorn,
        Self::Tracer,
        Self::Vendetta,
        Self::Venture,
        Self::Widowmaker,
        Self::Winston,
        Self::WreckingBall,
        Self::Wuyang,
        Self::Zarya,
        Self::Zenyatta,
    ];
    /// The data key, for debugging.
    pub fn key(self) -> &'static str {
        match self {
            Self::Ana => "ana",
            Self::Anran => "anran",
            Self::Ashe => "ashe",
            Self::Baptiste => "baptiste",
            Self::Bastion => "bastion",
            Self::Brigitte => "brigitte",
            Self::Cassidy => "cassidy",
            Self::Dmon => "dmon",
            Self::Doctrine => "doctrine",
            Self::Domina => "domina",
            Self::Doomfist => "doomfist",
            Self::Dva => "dva",
            Self::Echo => "echo",
            Self::Emre => "emre",
            Self::Freja => "freja",
            Self::Genji => "genji",
            Self::Hanzo => "hanzo",
            Self::Hazard => "hazard",
            Self::Illari => "illari",
            Self::JetpackCat => "jetpackCat",
            Self::JunkerQueen => "junkerQueen",
            Self::Junkrat => "junkrat",
            Self::Juno => "juno",
            Self::Kiriko => "kiriko",
            Self::Lifeweaver => "lifeweaver",
            Self::Lucio => "lucio",
            Self::Mauga => "mauga",
            Self::Mei => "mei",
            Self::Mercy => "mercy",
            Self::Mizuki => "mizuki",
            Self::Moira => "moira",
            Self::Orisa => "orisa",
            Self::Pharah => "pharah",
            Self::Ramattra => "ramattra",
            Self::Reaper => "reaper",
            Self::Reinhardt => "reinhardt",
            Self::Roadhog => "roadhog",
            Self::Shion => "shion",
            Self::Sierra => "sierra",
            Self::Sigma => "sigma",
            Self::Sojourn => "sojourn",
            Self::Soldier => "soldier",
            Self::Sombra => "sombra",
            Self::Symmetra => "symmetra",
            Self::Torbjorn => "torbjorn",
            Self::Tracer => "tracer",
            Self::Vendetta => "vendetta",
            Self::Venture => "venture",
            Self::Widowmaker => "widowmaker",
            Self::Winston => "winston",
            Self::WreckingBall => "wreckingBall",
            Self::Wuyang => "wuyang",
            Self::Zarya => "zarya",
            Self::Zenyatta => "zenyatta",
        }
    }
    /// The name as written in workshop code.
    pub fn name(self) -> &'static str {
        static NAMES: [&str; 54usize] = [
            "Ana",
            "Anran",
            "Ashe",
            "Baptiste",
            "Bastion",
            "Brigitte",
            "Cassidy",
            "D.Mon",
            "Doctrine",
            "Domina",
            "Doomfist",
            "D.Va",
            "Echo",
            "Emre",
            "Freja",
            "Genji",
            "Hanzo",
            "Hazard",
            "Illari",
            "Jetpack Cat",
            "Junker Queen",
            "Junkrat",
            "Juno",
            "Kiriko",
            "Lifeweaver",
            "Lúcio",
            "Mauga",
            "Mei",
            "Mercy",
            "Mizuki",
            "Moira",
            "Orisa",
            "Pharah",
            "Ramattra",
            "Reaper",
            "Reinhardt",
            "Roadhog",
            "Shion",
            "Sierra",
            "Sigma",
            "Sojourn",
            "Soldier: 76",
            "Sombra",
            "Symmetra",
            "Torbjörn",
            "Tracer",
            "Vendetta",
            "Venture",
            "Widowmaker",
            "Winston",
            "Wrecking Ball",
            "Wuyang",
            "Zarya",
            "Zenyatta",
        ];
        NAMES[self as usize]
    }
}
/// A hero ability slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Ability {
    /// `ability1`
    Ability1,
    /// `ability2`
    Ability2,
    /// `ability3`
    Ability3,
    /// `passive`
    Passive,
    /// `primary_fire`
    PrimaryFire,
    /// `secondary_fire`
    SecondaryFire,
    /// `ultimate`
    Ultimate,
}
impl Ability {
    /// Every variant, in declaration order.
    pub const ALL: &'static [Self] = &[
        Self::Ability1,
        Self::Ability2,
        Self::Ability3,
        Self::Passive,
        Self::PrimaryFire,
        Self::SecondaryFire,
        Self::Ultimate,
    ];
    /// The data key, for debugging.
    pub fn key(self) -> &'static str {
        match self {
            Self::Ability1 => "ability1",
            Self::Ability2 => "ability2",
            Self::Ability3 => "ability3",
            Self::Passive => "passive",
            Self::PrimaryFire => "primary_fire",
            Self::SecondaryFire => "secondary_fire",
            Self::Ultimate => "ultimate",
        }
    }
}
impl Hero {
    /// The names of the hero's abilities, by slot.
    pub fn abilities(self) -> &'static [(Ability, &'static str)] {
        match self {
            Self::Ana => {
                &[
                    (
                        Ability::Ability1,
                        "Sleep Dart",
                    ),
                    (
                        Ability::Ability2,
                        "Biotic Grenade",
                    ),
                    (
                        Ability::Ultimate,
                        "Nano Boost",
                    ),
                ]
            }
            Self::Anran => {
                &[
                    (
                        Ability::Ability1,
                        "Inferno Rush",
                    ),
                    (
                        Ability::Ability2,
                        "Dancing Blaze",
                    ),
                    (
                        Ability::Ultimate,
                        "Vermillion Ascent",
                    ),
                ]
            }
            Self::Ashe => {
                &[
                    (
                        Ability::Ability1,
                        "Coach Gun",
                    ),
                    (
                        Ability::Ability2,
                        "Dynamite",
                    ),
                    (
                        Ability::Ultimate,
                        "B.O.B.",
                    ),
                ]
            }
            Self::Baptiste => {
                &[
                    (
                        Ability::Ability1,
                        "Regenerative Burst",
                    ),
                    (
                        Ability::Ability2,
                        "Immortality Field",
                    ),
                    (
                        Ability::Ultimate,
                        "Amplification Matrix",
                    ),
                ]
            }
            Self::Bastion => {
                &[
                    (
                        Ability::Ability1,
                        "Reconfigure",
                    ),
                    (
                        Ability::SecondaryFire,
                        "A-36 Tactical Grenade",
                    ),
                    (
                        Ability::Ultimate,
                        "Configuration: Artillery",
                    ),
                ]
            }
            Self::Brigitte => {
                &[
                    (
                        Ability::Ability1,
                        "Whip Shot",
                    ),
                    (
                        Ability::Ability2,
                        "Repair Pack",
                    ),
                    (
                        Ability::Ability3,
                        "Shield Bash",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Barrier Shield",
                    ),
                    (
                        Ability::Ultimate,
                        "Rally",
                    ),
                ]
            }
            Self::Cassidy => {
                &[
                    (
                        Ability::Ability1,
                        "Combat Roll",
                    ),
                    (
                        Ability::Ability2,
                        "Flashbang",
                    ),
                    (
                        Ability::Ultimate,
                        "Deadeye",
                    ),
                ]
            }
            Self::Dmon => {
                &[
                    (
                        Ability::Ability1,
                        "Propulsors",
                    ),
                    (
                        Ability::Ability2,
                        "Fusion Repeater",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Power Barrier",
                    ),
                    (
                        Ability::Ultimate,
                        "Limit Break",
                    ),
                ]
            }
            Self::Doctrine => {
                &[
                    (Ability::Ability1, "Shrouded Momentum"),
                    (Ability::Ability2, "Invigorating Drones"),
                    (Ability::Ultimate, "Deliverance"),
                ]
            }
            Self::Domina => {
                &[
                    (
                        Ability::Ability1,
                        "Sonic Repulsors",
                    ),
                    (
                        Ability::Ability2,
                        "Crystal Charge",
                    ),
                    (
                        Ability::Passive,
                        "Reconstruction",
                    ),
                    (
                        Ability::PrimaryFire,
                        "Photon Magnum",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Barrier Array",
                    ),
                    (
                        Ability::Ultimate,
                        "Panopticon",
                    ),
                ]
            }
            Self::Doomfist => {
                &[
                    (
                        Ability::Ability1,
                        "Power Block",
                    ),
                    (
                        Ability::Ability2,
                        "Seismic Slam",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Rocket Punch",
                    ),
                    (
                        Ability::Ultimate,
                        "Meteor Strike",
                    ),
                ]
            }
            Self::Dva => {
                &[
                    (
                        Ability::Ability1,
                        "Boosters",
                    ),
                    (
                        Ability::Ability2,
                        "Micro Missiles",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Defense Matrix",
                    ),
                    (
                        Ability::Ultimate,
                        "Self-Destruct",
                    ),
                ]
            }
            Self::Echo => {
                &[
                    (
                        Ability::Ability1,
                        "Flight",
                    ),
                    (
                        Ability::Ability2,
                        "Focusing Beam",
                    ),
                    (
                        Ability::Passive,
                        "Glide",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Sticky Bombs",
                    ),
                    (
                        Ability::Ultimate,
                        "Duplicate",
                    ),
                ]
            }
            Self::Emre => {
                &[
                    (
                        Ability::Ability1,
                        "Siphon Blaster",
                    ),
                    (
                        Ability::Ability2,
                        "Cyber Frag",
                    ),
                    (
                        Ability::Ultimate,
                        "Override Protocol",
                    ),
                ]
            }
            Self::Freja => {
                &[
                    (
                        Ability::Ability1,
                        "Quick Dash",
                    ),
                    (
                        Ability::Ability2,
                        "Updraft",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Take Aim",
                    ),
                    (
                        Ability::Ultimate,
                        "Bola Shot",
                    ),
                ]
            }
            Self::Genji => {
                &[
                    (
                        Ability::Ability1,
                        "Swift Strike",
                    ),
                    (
                        Ability::Ability2,
                        "Deflect",
                    ),
                    (
                        Ability::Ultimate,
                        "Dragonblade",
                    ),
                ]
            }
            Self::Hanzo => {
                &[
                    (
                        Ability::Ability1,
                        "Sonic Arrow",
                    ),
                    (
                        Ability::Ability2,
                        "Storm Arrows",
                    ),
                    (
                        Ability::Ability3,
                        "Lunge",
                    ),
                    (
                        Ability::Ultimate,
                        "Dragonstrike",
                    ),
                ]
            }
            Self::Hazard => {
                &[
                    (
                        Ability::Ability1,
                        "Violent Leap",
                    ),
                    (
                        Ability::Ability2,
                        "Jagged Wall",
                    ),
                    (
                        Ability::PrimaryFire,
                        "Bonespur",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Spike Guard",
                    ),
                    (
                        Ability::Ultimate,
                        "Downpour",
                    ),
                ]
            }
            Self::Illari => {
                &[
                    (
                        Ability::Ability1,
                        "Outburst",
                    ),
                    (
                        Ability::Ability2,
                        "Healing Pylon",
                    ),
                    (
                        Ability::Ultimate,
                        "Captive Sun",
                    ),
                ]
            }
            Self::JetpackCat => {
                &[
                    (
                        Ability::Ability1,
                        "Lifeline",
                    ),
                    (
                        Ability::Ability2,
                        "Purr",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Frenetic Flight",
                    ),
                    (
                        Ability::Ultimate,
                        "Catnapper",
                    ),
                ]
            }
            Self::JunkerQueen => {
                &[
                    (
                        Ability::Ability1,
                        "Commanding Shout",
                    ),
                    (
                        Ability::Ability2,
                        "Carnage",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Jagged Blade Gracie",
                    ),
                    (
                        Ability::Ultimate,
                        "Rampage",
                    ),
                ]
            }
            Self::Junkrat => {
                &[
                    (
                        Ability::Ability1,
                        "Concussion Mine",
                    ),
                    (
                        Ability::Ability2,
                        "Steel Trap",
                    ),
                    (
                        Ability::Ultimate,
                        "RIP-Tire",
                    ),
                ]
            }
            Self::Juno => {
                &[
                    (
                        Ability::Ability1,
                        "Glide Boost",
                    ),
                    (
                        Ability::Ability2,
                        "Hyper Ring",
                    ),
                    (
                        Ability::Passive,
                        "Martian Overboots",
                    ),
                    (
                        Ability::PrimaryFire,
                        "Mediblaster",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Pulsar Torpedoes",
                    ),
                    (
                        Ability::Ultimate,
                        "Orbital Ray",
                    ),
                ]
            }
            Self::Kiriko => {
                &[
                    (
                        Ability::Ability1,
                        "Swift Step",
                    ),
                    (
                        Ability::Ability2,
                        "Protection Suzu",
                    ),
                    (
                        Ability::Ultimate,
                        "Kitsune Rush",
                    ),
                ]
            }
            Self::Lifeweaver => {
                &[
                    (
                        Ability::Ability1,
                        "Rejuvenating Dash",
                    ),
                    (
                        Ability::Ability2,
                        "Life Grip",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Petal Platform",
                    ),
                    (
                        Ability::Ultimate,
                        "Tree of Life",
                    ),
                ]
            }
            Self::Lucio => {
                &[
                    (
                        Ability::Ability1,
                        "Crossfade",
                    ),
                    (
                        Ability::Ability2,
                        "Amp It Up",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Soundwave",
                    ),
                    (
                        Ability::Ultimate,
                        "Sound Barrier",
                    ),
                ]
            }
            Self::Mauga => {
                &[
                    (
                        Ability::Ability1,
                        "Overrun",
                    ),
                    (
                        Ability::Ability2,
                        "Cardiac Overdrive",
                    ),
                    (
                        Ability::PrimaryFire,
                        "Incendiary Chaingun",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Volatile Chaingun",
                    ),
                    (
                        Ability::Ultimate,
                        "Cage Fight",
                    ),
                ]
            }
            Self::Mei => {
                &[
                    (
                        Ability::Ability1,
                        "Cryo-Freeze",
                    ),
                    (
                        Ability::Ability2,
                        "Ice Wall",
                    ),
                    (
                        Ability::Ultimate,
                        "Blizzard",
                    ),
                ]
            }
            Self::Mercy => {
                &[
                    (
                        Ability::Ability1,
                        "Guardian Angel",
                    ),
                    (
                        Ability::Ability2,
                        "Resurrect",
                    ),
                    (
                        Ability::Passive,
                        "Sympathetic Recovery",
                    ),
                    (
                        Ability::Ultimate,
                        "Valkyrie",
                    ),
                ]
            }
            Self::Mizuki => {
                &[
                    (
                        Ability::Ability1,
                        "Katashiro Return",
                    ),
                    (
                        Ability::Ability2,
                        "Binding Chain",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Healing Kasa",
                    ),
                    (
                        Ability::Ultimate,
                        "Kekkai Sanctuary",
                    ),
                ]
            }
            Self::Moira => {
                &[
                    (
                        Ability::Ability1,
                        "Fade",
                    ),
                    (
                        Ability::Ability2,
                        "Biotic Orb",
                    ),
                    (
                        Ability::Ultimate,
                        "Coalescence",
                    ),
                ]
            }
            Self::Orisa => {
                &[
                    (
                        Ability::Ability1,
                        "Fortify",
                    ),
                    (
                        Ability::Ability2,
                        "Javelin Spin",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Energy Javelin",
                    ),
                    (
                        Ability::Ultimate,
                        "Terra Surge",
                    ),
                ]
            }
            Self::Pharah => {
                &[
                    (
                        Ability::Ability1,
                        "Jump Jet",
                    ),
                    (
                        Ability::Ability2,
                        "Concussive Blast",
                    ),
                    (
                        Ability::Passive,
                        "Hover Jets",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Jet Dash",
                    ),
                    (
                        Ability::Ultimate,
                        "Barrage",
                    ),
                ]
            }
            Self::Ramattra => {
                &[
                    (
                        Ability::Ability1,
                        "Nemesis Form",
                    ),
                    (
                        Ability::Ability2,
                        "Ravenous Vortex",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Void Barrier Omnic Form",
                    ),
                    (
                        Ability::Ultimate,
                        "Annihilation",
                    ),
                ]
            }
            Self::Reaper => {
                &[
                    (
                        Ability::Ability1,
                        "Wraith Form",
                    ),
                    (
                        Ability::Ability2,
                        "Shadow Step",
                    ),
                    (
                        Ability::Ultimate,
                        "Death Blossom",
                    ),
                ]
            }
            Self::Reinhardt => {
                &[
                    (
                        Ability::Ability1,
                        "Charge",
                    ),
                    (
                        Ability::Ability2,
                        "Fire Strike",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Barrier Field",
                    ),
                    (
                        Ability::Ultimate,
                        "Earthshatter",
                    ),
                ]
            }
            Self::Roadhog => {
                &[
                    (
                        Ability::Ability1,
                        "Chain Hook",
                    ),
                    (
                        Ability::Ability2,
                        "Take a Breather",
                    ),
                    (
                        Ability::Ultimate,
                        "Whole Hog",
                    ),
                ]
            }
            Self::Shion => {
                &[
                    (
                        Ability::Ability1,
                        "Evade",
                    ),
                    (
                        Ability::Ability2,
                        "Joyride",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Execution",
                    ),
                    (
                        Ability::Ultimate,
                        "Satsuriku Spree",
                    ),
                ]
            }
            Self::Sierra => {
                &[
                    (
                        Ability::Ability1,
                        "Anchor Drone",
                    ),
                    (
                        Ability::Ability2,
                        "Tremor Charge",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Tracking Shot",
                    ),
                    (
                        Ability::Ultimate,
                        "Trailblazer",
                    ),
                ]
            }
            Self::Sigma => {
                &[
                    (
                        Ability::Ability1,
                        "Kinetic Grasp",
                    ),
                    (
                        Ability::Ability2,
                        "Accretion",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Experimental Barrier",
                    ),
                    (
                        Ability::Ultimate,
                        "Gravitic Flux",
                    ),
                ]
            }
            Self::Sojourn => {
                &[
                    (
                        Ability::Ability1,
                        "Power Slide",
                    ),
                    (
                        Ability::Ability2,
                        "Disruptor Shot",
                    ),
                    (Ability::SecondaryFire, "Charged Shot"),
                    (
                        Ability::Ultimate,
                        "Overclock",
                    ),
                ]
            }
            Self::Soldier => {
                &[
                    (
                        Ability::Ability1,
                        "Sprint",
                    ),
                    (
                        Ability::Ability2,
                        "Biotic Field",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Helix Rockets",
                    ),
                    (
                        Ability::Ultimate,
                        "Tactical Visor",
                    ),
                ]
            }
            Self::Sombra => {
                &[
                    (
                        Ability::Ability1,
                        "Virus",
                    ),
                    (
                        Ability::Ability2,
                        "Translocator",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Hack",
                    ),
                    (
                        Ability::Ultimate,
                        "EMP",
                    ),
                ]
            }
            Self::Symmetra => {
                &[
                    (
                        Ability::Ability1,
                        "Sentry Turret",
                    ),
                    (
                        Ability::Ability2,
                        "Teleporter",
                    ),
                    (
                        Ability::Ultimate,
                        "Photon Barrier",
                    ),
                ]
            }
            Self::Torbjorn => {
                &[
                    (
                        Ability::Ability1,
                        "Deploy Turret",
                    ),
                    (
                        Ability::Ability2,
                        "Overload",
                    ),
                    (
                        Ability::Ultimate,
                        "Molten Core",
                    ),
                ]
            }
            Self::Tracer => {
                &[
                    (
                        Ability::Ability1,
                        "Blink",
                    ),
                    (
                        Ability::Ability2,
                        "Recall",
                    ),
                    (
                        Ability::Ultimate,
                        "Pulse Bomb",
                    ),
                ]
            }
            Self::Vendetta => {
                &[
                    (
                        Ability::Ability1,
                        "Whirlwind Dash",
                    ),
                    (
                        Ability::Ability2,
                        "Soaring Slice",
                    ),
                    (
                        Ability::PrimaryFire,
                        "Palatine Fang",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Warding Stance",
                    ),
                    (
                        Ability::Ultimate,
                        "Sundering Blade",
                    ),
                ]
            }
            Self::Venture => {
                &[
                    (
                        Ability::Ability1,
                        "Burrow",
                    ),
                    (
                        Ability::PrimaryFire,
                        "SMART Excavator",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Drill Dash",
                    ),
                    (
                        Ability::Ultimate,
                        "Tectonic Shock",
                    ),
                ]
            }
            Self::Widowmaker => {
                &[
                    (
                        Ability::Ability1,
                        "Grappling Hook",
                    ),
                    (
                        Ability::Ability2,
                        "Venom Mine",
                    ),
                    (
                        Ability::Ultimate,
                        "Infra-Sight",
                    ),
                ]
            }
            Self::Winston => {
                &[
                    (
                        Ability::Ability1,
                        "Jump Pack",
                    ),
                    (
                        Ability::Ability2,
                        "Barrier Projector",
                    ),
                    (
                        Ability::Ultimate,
                        "Primal Rage",
                    ),
                ]
            }
            Self::WreckingBall => {
                &[
                    (
                        Ability::Ability1,
                        "Roll",
                    ),
                    (
                        Ability::Ability2,
                        "Adaptive Shield",
                    ),
                    (
                        Ability::Ability3,
                        "Piledriver",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Grappling Claw",
                    ),
                    (
                        Ability::Ultimate,
                        "Minefield",
                    ),
                ]
            }
            Self::Wuyang => {
                &[
                    (
                        Ability::Ability1,
                        "Rushing Torrent",
                    ),
                    (
                        Ability::Ability2,
                        "Guardian Wave",
                    ),
                    (
                        Ability::SecondaryFire,
                        "Restorative Stream",
                    ),
                    (
                        Ability::Ultimate,
                        "Tidal Blast",
                    ),
                ]
            }
            Self::Zarya => {
                &[
                    (
                        Ability::Ability1,
                        "Particle Barrier",
                    ),
                    (
                        Ability::Ability2,
                        "Projected Barrier",
                    ),
                    (
                        Ability::Ultimate,
                        "Graviton Surge",
                    ),
                ]
            }
            Self::Zenyatta => {
                &[
                    (
                        Ability::Ability1,
                        "Orb of Harmony",
                    ),
                    (
                        Ability::Ability2,
                        "Orb of Discord",
                    ),
                    (
                        Ability::Ultimate,
                        "Transcendence",
                    ),
                ]
            }
        }
    }
}
