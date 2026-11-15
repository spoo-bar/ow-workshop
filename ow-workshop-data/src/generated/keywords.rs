//! Structural keywords: blocks, events, event filters and settings words.
/// A structural workshop word.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Keyword {
    /// `actions`
    Actions,
    /// `conditions`
    Conditions,
    /// `disabled`
    Disabled,
    /// `event`
    Event,
    /// `global`
    Global,
    /// `player`
    Player,
    /// `rule`
    Rule,
    /// `settings`
    Settings,
    /// `subroutines`
    Subroutines,
    /// `variables`
    Variables,
    /// `Ongoing - Each Player`
    EventEachPlayer,
    /// `Ongoing - Global`
    EventGlobal,
    /// `Player Dealt Damage`
    EventPlayerDealtDamage,
    /// `Player Dealt Final Blow`
    EventPlayerDealtFinalBlow,
    /// `Player Dealt Healing`
    EventPlayerDealtHealing,
    /// `Player Dealt Knockback`
    EventPlayerDealtKnockback,
    /// `Player Died`
    EventPlayerDied,
    /// `Player Earned Elimination`
    EventPlayerEarnedElimination,
    /// `Player Joined Match`
    EventPlayerJoined,
    /// `Player Left Match`
    EventPlayerLeft,
    /// `Player Received Healing`
    EventPlayerReceivedHealing,
    /// `Player Received Knockback`
    EventPlayerReceivedKnockback,
    /// `Player Took Damage`
    EventPlayerTookDamage,
    /// `Subroutine`
    EventSubroutine,
    /// `All`
    EventTeamAll,
    /// `Team 1`
    EventTeamTeam1,
    /// `Team 2`
    EventTeamTeam2,
    /// `All`
    EventPlayerAll,
    /// `Ana`
    EventPlayerAna,
    /// `Anran`
    EventPlayerAnran,
    /// `Ashe`
    EventPlayerAshe,
    /// `Baptiste`
    EventPlayerBaptiste,
    /// `Bastion`
    EventPlayerBastion,
    /// `Brigitte`
    EventPlayerBrigitte,
    /// `Cassidy`
    EventPlayerCassidy,
    /// `D.Mon`
    EventPlayerDmon,
    /// `Doctrine`
    EventPlayerDoctrine,
    /// `Domina`
    EventPlayerDomina,
    /// `Doomfist`
    EventPlayerDoomfist,
    /// `D.Va`
    EventPlayerDva,
    /// `Echo`
    EventPlayerEcho,
    /// `Emre`
    EventPlayerEmre,
    /// `Freja`
    EventPlayerFreja,
    /// `Genji`
    EventPlayerGenji,
    /// `Hanzo`
    EventPlayerHanzo,
    /// `Hazard`
    EventPlayerHazard,
    /// `Illari`
    EventPlayerIllari,
    /// `Jetpack Cat`
    EventPlayerJetpackCat,
    /// `Junker Queen`
    EventPlayerJunkerQueen,
    /// `Junkrat`
    EventPlayerJunkrat,
    /// `Juno`
    EventPlayerJuno,
    /// `Kiriko`
    EventPlayerKiriko,
    /// `Lifeweaver`
    EventPlayerLifeweaver,
    /// `Lúcio`
    EventPlayerLucio,
    /// `Mauga`
    EventPlayerMauga,
    /// `Mei`
    EventPlayerMei,
    /// `Mercy`
    EventPlayerMercy,
    /// `Mizuki`
    EventPlayerMizuki,
    /// `Moira`
    EventPlayerMoira,
    /// `Orisa`
    EventPlayerOrisa,
    /// `Pharah`
    EventPlayerPharah,
    /// `Ramattra`
    EventPlayerRamattra,
    /// `Reaper`
    EventPlayerReaper,
    /// `Reinhardt`
    EventPlayerReinhardt,
    /// `Roadhog`
    EventPlayerRoadhog,
    /// `Shion`
    EventPlayerShion,
    /// `Sierra`
    EventPlayerSierra,
    /// `Sigma`
    EventPlayerSigma,
    /// `Slot 0`
    EventPlayerSlot0,
    /// `Slot 1`
    EventPlayerSlot1,
    /// `Slot 10`
    EventPlayerSlot10,
    /// `Slot 11`
    EventPlayerSlot11,
    /// `Slot 2`
    EventPlayerSlot2,
    /// `Slot 3`
    EventPlayerSlot3,
    /// `Slot 4`
    EventPlayerSlot4,
    /// `Slot 5`
    EventPlayerSlot5,
    /// `Slot 6`
    EventPlayerSlot6,
    /// `Slot 7`
    EventPlayerSlot7,
    /// `Slot 8`
    EventPlayerSlot8,
    /// `Slot 9`
    EventPlayerSlot9,
    /// `Sojourn`
    EventPlayerSojourn,
    /// `Soldier: 76`
    EventPlayerSoldier,
    /// `Sombra`
    EventPlayerSombra,
    /// `Symmetra`
    EventPlayerSymmetra,
    /// `Torbjörn`
    EventPlayerTorbjorn,
    /// `Tracer`
    EventPlayerTracer,
    /// `Vendetta`
    EventPlayerVendetta,
    /// `Venture`
    EventPlayerVenture,
    /// `Widowmaker`
    EventPlayerWidowmaker,
    /// `Winston`
    EventPlayerWinston,
    /// `Wrecking Ball`
    EventPlayerWreckingBall,
    /// `Wuyang`
    EventPlayerWuyang,
    /// `Zarya`
    EventPlayerZarya,
    /// `Zenyatta`
    EventPlayerZenyatta,
    /// `Disabled`
    SettingsDisabled,
    /// `Enabled`
    SettingsEnabled,
    /// `No`
    SettingsNo,
    /// `Off`
    SettingsOff,
    /// `On`
    SettingsOn,
    /// `Team 1`
    SettingsTeam1,
    /// `Team 2`
    SettingsTeam2,
    /// `Yes`
    SettingsYes,
}
impl Keyword {
    /// Every variant, in declaration order.
    pub const ALL: &'static [Self] = &[
        Self::Actions,
        Self::Conditions,
        Self::Disabled,
        Self::Event,
        Self::Global,
        Self::Player,
        Self::Rule,
        Self::Settings,
        Self::Subroutines,
        Self::Variables,
        Self::EventEachPlayer,
        Self::EventGlobal,
        Self::EventPlayerDealtDamage,
        Self::EventPlayerDealtFinalBlow,
        Self::EventPlayerDealtHealing,
        Self::EventPlayerDealtKnockback,
        Self::EventPlayerDied,
        Self::EventPlayerEarnedElimination,
        Self::EventPlayerJoined,
        Self::EventPlayerLeft,
        Self::EventPlayerReceivedHealing,
        Self::EventPlayerReceivedKnockback,
        Self::EventPlayerTookDamage,
        Self::EventSubroutine,
        Self::EventTeamAll,
        Self::EventTeamTeam1,
        Self::EventTeamTeam2,
        Self::EventPlayerAll,
        Self::EventPlayerAna,
        Self::EventPlayerAnran,
        Self::EventPlayerAshe,
        Self::EventPlayerBaptiste,
        Self::EventPlayerBastion,
        Self::EventPlayerBrigitte,
        Self::EventPlayerCassidy,
        Self::EventPlayerDmon,
        Self::EventPlayerDoctrine,
        Self::EventPlayerDomina,
        Self::EventPlayerDoomfist,
        Self::EventPlayerDva,
        Self::EventPlayerEcho,
        Self::EventPlayerEmre,
        Self::EventPlayerFreja,
        Self::EventPlayerGenji,
        Self::EventPlayerHanzo,
        Self::EventPlayerHazard,
        Self::EventPlayerIllari,
        Self::EventPlayerJetpackCat,
        Self::EventPlayerJunkerQueen,
        Self::EventPlayerJunkrat,
        Self::EventPlayerJuno,
        Self::EventPlayerKiriko,
        Self::EventPlayerLifeweaver,
        Self::EventPlayerLucio,
        Self::EventPlayerMauga,
        Self::EventPlayerMei,
        Self::EventPlayerMercy,
        Self::EventPlayerMizuki,
        Self::EventPlayerMoira,
        Self::EventPlayerOrisa,
        Self::EventPlayerPharah,
        Self::EventPlayerRamattra,
        Self::EventPlayerReaper,
        Self::EventPlayerReinhardt,
        Self::EventPlayerRoadhog,
        Self::EventPlayerShion,
        Self::EventPlayerSierra,
        Self::EventPlayerSigma,
        Self::EventPlayerSlot0,
        Self::EventPlayerSlot1,
        Self::EventPlayerSlot10,
        Self::EventPlayerSlot11,
        Self::EventPlayerSlot2,
        Self::EventPlayerSlot3,
        Self::EventPlayerSlot4,
        Self::EventPlayerSlot5,
        Self::EventPlayerSlot6,
        Self::EventPlayerSlot7,
        Self::EventPlayerSlot8,
        Self::EventPlayerSlot9,
        Self::EventPlayerSojourn,
        Self::EventPlayerSoldier,
        Self::EventPlayerSombra,
        Self::EventPlayerSymmetra,
        Self::EventPlayerTorbjorn,
        Self::EventPlayerTracer,
        Self::EventPlayerVendetta,
        Self::EventPlayerVenture,
        Self::EventPlayerWidowmaker,
        Self::EventPlayerWinston,
        Self::EventPlayerWreckingBall,
        Self::EventPlayerWuyang,
        Self::EventPlayerZarya,
        Self::EventPlayerZenyatta,
        Self::SettingsDisabled,
        Self::SettingsEnabled,
        Self::SettingsNo,
        Self::SettingsOff,
        Self::SettingsOn,
        Self::SettingsTeam1,
        Self::SettingsTeam2,
        Self::SettingsYes,
    ];
    /// The data key, for debugging.
    pub fn key(self) -> &'static str {
        match self {
            Self::Actions => "actions",
            Self::Conditions => "conditions",
            Self::Disabled => "disabled",
            Self::Event => "event",
            Self::Global => "global",
            Self::Player => "player",
            Self::Rule => "rule",
            Self::Settings => "settings",
            Self::Subroutines => "subroutines",
            Self::Variables => "variables",
            Self::EventEachPlayer => "event.eachPlayer",
            Self::EventGlobal => "event.global",
            Self::EventPlayerDealtDamage => "event.playerDealtDamage",
            Self::EventPlayerDealtFinalBlow => "event.playerDealtFinalBlow",
            Self::EventPlayerDealtHealing => "event.playerDealtHealing",
            Self::EventPlayerDealtKnockback => "event.playerDealtKnockback",
            Self::EventPlayerDied => "event.playerDied",
            Self::EventPlayerEarnedElimination => "event.playerEarnedElimination",
            Self::EventPlayerJoined => "event.playerJoined",
            Self::EventPlayerLeft => "event.playerLeft",
            Self::EventPlayerReceivedHealing => "event.playerReceivedHealing",
            Self::EventPlayerReceivedKnockback => "event.playerReceivedKnockback",
            Self::EventPlayerTookDamage => "event.playerTookDamage",
            Self::EventSubroutine => "event.subroutine",
            Self::EventTeamAll => "event_team.all",
            Self::EventTeamTeam1 => "event_team.team1",
            Self::EventTeamTeam2 => "event_team.team2",
            Self::EventPlayerAll => "event_player.all",
            Self::EventPlayerAna => "event_player.ana",
            Self::EventPlayerAnran => "event_player.anran",
            Self::EventPlayerAshe => "event_player.ashe",
            Self::EventPlayerBaptiste => "event_player.baptiste",
            Self::EventPlayerBastion => "event_player.bastion",
            Self::EventPlayerBrigitte => "event_player.brigitte",
            Self::EventPlayerCassidy => "event_player.cassidy",
            Self::EventPlayerDmon => "event_player.dmon",
            Self::EventPlayerDoctrine => "event_player.doctrine",
            Self::EventPlayerDomina => "event_player.domina",
            Self::EventPlayerDoomfist => "event_player.doomfist",
            Self::EventPlayerDva => "event_player.dva",
            Self::EventPlayerEcho => "event_player.echo",
            Self::EventPlayerEmre => "event_player.emre",
            Self::EventPlayerFreja => "event_player.freja",
            Self::EventPlayerGenji => "event_player.genji",
            Self::EventPlayerHanzo => "event_player.hanzo",
            Self::EventPlayerHazard => "event_player.hazard",
            Self::EventPlayerIllari => "event_player.illari",
            Self::EventPlayerJetpackCat => "event_player.jetpackCat",
            Self::EventPlayerJunkerQueen => "event_player.junkerQueen",
            Self::EventPlayerJunkrat => "event_player.junkrat",
            Self::EventPlayerJuno => "event_player.juno",
            Self::EventPlayerKiriko => "event_player.kiriko",
            Self::EventPlayerLifeweaver => "event_player.lifeweaver",
            Self::EventPlayerLucio => "event_player.lucio",
            Self::EventPlayerMauga => "event_player.mauga",
            Self::EventPlayerMei => "event_player.mei",
            Self::EventPlayerMercy => "event_player.mercy",
            Self::EventPlayerMizuki => "event_player.mizuki",
            Self::EventPlayerMoira => "event_player.moira",
            Self::EventPlayerOrisa => "event_player.orisa",
            Self::EventPlayerPharah => "event_player.pharah",
            Self::EventPlayerRamattra => "event_player.ramattra",
            Self::EventPlayerReaper => "event_player.reaper",
            Self::EventPlayerReinhardt => "event_player.reinhardt",
            Self::EventPlayerRoadhog => "event_player.roadhog",
            Self::EventPlayerShion => "event_player.shion",
            Self::EventPlayerSierra => "event_player.sierra",
            Self::EventPlayerSigma => "event_player.sigma",
            Self::EventPlayerSlot0 => "event_player.slot0",
            Self::EventPlayerSlot1 => "event_player.slot1",
            Self::EventPlayerSlot10 => "event_player.slot10",
            Self::EventPlayerSlot11 => "event_player.slot11",
            Self::EventPlayerSlot2 => "event_player.slot2",
            Self::EventPlayerSlot3 => "event_player.slot3",
            Self::EventPlayerSlot4 => "event_player.slot4",
            Self::EventPlayerSlot5 => "event_player.slot5",
            Self::EventPlayerSlot6 => "event_player.slot6",
            Self::EventPlayerSlot7 => "event_player.slot7",
            Self::EventPlayerSlot8 => "event_player.slot8",
            Self::EventPlayerSlot9 => "event_player.slot9",
            Self::EventPlayerSojourn => "event_player.sojourn",
            Self::EventPlayerSoldier => "event_player.soldier",
            Self::EventPlayerSombra => "event_player.sombra",
            Self::EventPlayerSymmetra => "event_player.symmetra",
            Self::EventPlayerTorbjorn => "event_player.torbjorn",
            Self::EventPlayerTracer => "event_player.tracer",
            Self::EventPlayerVendetta => "event_player.vendetta",
            Self::EventPlayerVenture => "event_player.venture",
            Self::EventPlayerWidowmaker => "event_player.widowmaker",
            Self::EventPlayerWinston => "event_player.winston",
            Self::EventPlayerWreckingBall => "event_player.wreckingBall",
            Self::EventPlayerWuyang => "event_player.wuyang",
            Self::EventPlayerZarya => "event_player.zarya",
            Self::EventPlayerZenyatta => "event_player.zenyatta",
            Self::SettingsDisabled => "settings.disabled",
            Self::SettingsEnabled => "settings.enabled",
            Self::SettingsNo => "settings.no",
            Self::SettingsOff => "settings.off",
            Self::SettingsOn => "settings.on",
            Self::SettingsTeam1 => "settings.team1",
            Self::SettingsTeam2 => "settings.team2",
            Self::SettingsYes => "settings.yes",
        }
    }
    /// The name as written in workshop code.
    pub fn name(self) -> &'static str {
        static NAMES: [&str; 102usize] = [
            "actions",
            "conditions",
            "disabled",
            "event",
            "global",
            "player",
            "rule",
            "settings",
            "subroutines",
            "variables",
            "Ongoing - Each Player",
            "Ongoing - Global",
            "Player Dealt Damage",
            "Player Dealt Final Blow",
            "Player Dealt Healing",
            "Player Dealt Knockback",
            "Player Died",
            "Player Earned Elimination",
            "Player Joined Match",
            "Player Left Match",
            "Player Received Healing",
            "Player Received Knockback",
            "Player Took Damage",
            "Subroutine",
            "All",
            "Team 1",
            "Team 2",
            "All",
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
            "Slot 0",
            "Slot 1",
            "Slot 10",
            "Slot 11",
            "Slot 2",
            "Slot 3",
            "Slot 4",
            "Slot 5",
            "Slot 6",
            "Slot 7",
            "Slot 8",
            "Slot 9",
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
            "Disabled",
            "Enabled",
            "No",
            "Off",
            "On",
            "Team 1",
            "Team 2",
            "Yes",
        ];
        NAMES[self as usize]
    }
}
