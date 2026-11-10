//! Structural keywords: blocks, events, event filters and settings words.
use crate::Lang;
use crate::lang::localized;
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
    /// The name as written in workshop code of the given language.
    pub fn name(self, lang: Lang) -> &'static str {
        static NAMES: [&str; 102usize] = [
            "aktionen\u{1f}actions\u{1f}\u{1f}acciones\u{1f}\u{1f}azioni\u{1f}アクション\u{1f}action\u{1f}działania\u{1f}ações\u{1f}\u{1f}การดำเน\u{e34}นการ\u{1f}eylemler\u{1f}动作\u{1f}動作",
            "bedingungen\u{1f}conditions\u{1f}\u{1f}condiciones\u{1f}\u{1f}condizioni\u{1f}条件\u{1f}condition\u{1f}warunki\u{1f}condições\u{1f}\u{1f}เง\u{e37}\u{e48}อนไข\u{1f}koşullar\u{1f}条件\u{1f}條件",
            "deaktiviert\u{1f}disabled\u{1f}\u{1f}deshabilitado\u{1f}désactivé\u{1f}disattivato\u{1f}無効\u{1f}\u{1f}wył.\u{1f}desabilitado\u{1f}\u{1f}ป\u{e34}ดใช\u{e49}งาน\u{1f}devre dışı\u{1f}禁用\u{1f}停用",
            "\u{1f}event\u{1f}\u{1f}evento\u{1f}évènement\u{1f}evento\u{1f}イベント\u{1f}\u{1f}zdarzenie\u{1f}evento\u{1f}\u{1f}เหต\u{e38}การณ\u{e4c}\u{1f}etkinlik\u{1f}事件\u{1f}事件",
            "\u{1f}global\u{1f}\u{1f}\u{1f}globale\u{1f}globale\u{1f}グローバル\u{1f}\u{1f}globalne\u{1f}\u{1f}\u{1f}สากล\u{1f}küresel\u{1f}全局\u{1f}全域",
            "\u{1f}player\u{1f}jugador\u{1f}jugador\u{1f}de joueur\u{1f}giocatore\u{1f}プレイヤー\u{1f}\u{1f}gracza\u{1f}jogador\u{1f}\u{1f}ผ\u{e39}\u{e49}เล\u{e48}น\u{1f}oyuncu\u{1f}玩家\u{1f}玩家",
            "regel\u{1f}rule\u{1f}\u{1f}regla\u{1f}règle\u{1f}regola\u{1f}ルール\u{1f}\u{1f}reguła\u{1f}regra\u{1f}\u{1f}กฎ\u{1f}kural\u{1f}规则\u{1f}規則",
            "einstellungen\u{1f}settings\u{1f}ajustes\u{1f}configuración\u{1f}paramètres\u{1f}impostazioni\u{1f}設定\u{1f}\u{1f}ustawienia\u{1f}configurações\u{1f}\u{1f}การต\u{e31}\u{e49}งค\u{e48}า\u{1f}ayarlar\u{1f}设置\u{1f}設定",
            "Subroutinen\u{1f}subroutines\u{1f}subrutinas\u{1f}subrutinas\u{1f}sous-programmes\u{1f}subroutine\u{1f}サブルーチン\u{1f}\u{1f}podprogramy\u{1f}sub-rotinas\u{1f}\u{1f}ซ\u{e31}บร\u{e39}ท\u{e35}น\u{1f}alt yordam\u{1f}子程序",
            "Variablen\u{1f}variables\u{1f}\u{1f}\u{1f}\u{1f}variabili\u{1f}変数\u{1f}\u{1f}zmienne\u{1f}variáveis\u{1f}\u{1f}ต\u{e31}วแปร\u{1f}değişkenler\u{1f}变量\u{1f}變數",
            "\u{1f}Ongoing - Each Player\u{1f}\u{1f}En curso - Cada jugador\u{1f}Toute la partie - Chaque joueur\u{1f}\u{1f}進行中 - 各プレイヤー\u{1f}\u{1f}\u{1f}Em andamento - Cada Jogador\u{1f}\u{1f}ดำเน\u{e34}นอย\u{e39}\u{e48} - ผ\u{e39}\u{e49}เล\u{e48}นแต\u{e48}ละคน\u{1f}Devam Eden - Her Oyuncu\u{1f}持续 - 每名玩家",
            "\u{1f}Ongoing - Global\u{1f}\u{1f}En curso - Global\u{1f}Toute la partie - Tout le monde\u{1f}\u{1f}進行中 - グローバル\u{1f}\u{1f}\u{1f}Em andamento - Global\u{1f}\u{1f}ดำเน\u{e34}นอย\u{e39}\u{e48} - ท\u{e31}\u{e48}วโลก\u{1f}Devam Eden - Küresel\u{1f}持续 - 全局",
            "\u{1f}Player Dealt Damage\u{1f}\u{1f}El jugador infligió daño\u{1f}Un joueur inflige des dégâts\u{1f}\u{1f}プレイヤーがダメージを与えた\u{1f}\u{1f}\u{1f}Jogador Causou Dano\u{1f}Player dealt damage\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นสร\u{e49}างความเส\u{e35}ยหาย\u{1f}Oyuncu'nun Verdiği Hasar\u{1f}玩家造成伤害",
            "\u{1f}Player Dealt Final Blow\u{1f}\u{1f}El jugador asestó un golpe de gracia\u{1f}Un joueur inflige un coup de grâce\u{1f}\u{1f}ファイナル・ブロウを獲得\u{1f}\u{1f}\u{1f}Jogador Desferiu Golpe Final\u{1f}Player dealt final blow\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นท\u{e35}\u{e48}โจมต\u{e35}ส\u{e31}งหาร\u{1f}Oyuncu Son Vuruşu\u{1f}玩家造成最后一击",
            "\u{1f}Player Dealt Healing\u{1f}\u{1f}El jugador realizó una sanación\u{1f}Un joueur a prodigué des soins\u{1f}\u{1f}プレイヤーが回復を与えた\u{1f}\u{1f}\u{1f}Jogador Realizou Cura\u{1f}\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นสร\u{e49}างการฮ\u{e35}ล\u{1f}Oyuncu İyileştirme Verdi\u{1f}玩家造成治疗",
            "\u{1f}Player Dealt Knockback\u{1f}Repulsión infligida por el jugador\u{1f}El jugador infligió un derribo\u{1f}Un joueur a infligé un recul\u{1f}\u{1f}プレイヤーがノックバックを与えた\u{1f}\u{1f}\u{1f}Jogador Causou Repulsão\u{1f}\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นท\u{e35}\u{e48}สร\u{e49}างการกระเด\u{e47}นถอยหล\u{e31}ง\u{1f}Oyuncu'nun Verdiği Geri İtme\u{1f}玩家造成击退",
            "\u{1f}Player Died\u{1f}\u{1f}El jugador murió\u{1f}Un joueur meurt\u{1f}\u{1f}プレイヤーが倒れた\u{1f}\u{1f}\u{1f}Jogador morreu\u{1f}\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นเส\u{e35}ยช\u{e35}ว\u{e34}ต\u{1f}Oyuncu Öldü\u{1f}玩家阵亡",
            "\u{1f}Player Earned Elimination\u{1f}\u{1f}El jugador obtuvo una eliminación\u{1f}Un joueur obtient une élimination\u{1f}\u{1f}プレイヤーがキルを獲得\u{1f}\u{1f}\u{1f}Jogador Conseguiu Eliminação\u{1f}Player earned elimination\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นได\u{e49}ร\u{e31}บการกำจ\u{e31}ด\u{1f}Oyuncu Bertarafı\u{1f}玩家参与消灭",
            "\u{1f}Player Joined Match\u{1f}\u{1f}El jugador se unió a la partida\u{1f}Un joueur a rejoint la partie\u{1f}\u{1f}プレイヤーがマッチに参加\u{1f}\u{1f}\u{1f}Jogador Entrou na Partida\u{1f}\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นเข\u{e49}าร\u{e48}วมแมตช\u{e4c}\u{1f}Oyuncu Maça Katıldı\u{1f}玩家加入比赛",
            "\u{1f}Player Left Match\u{1f}\u{1f}El jugador abandonó la partida\u{1f}Un joueur a quitté la partie\u{1f}\u{1f}プレイヤーがマッチから離脱\u{1f}\u{1f}\u{1f}Jogador Saiu da Partida\u{1f}\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นออกจากแมตช\u{e4c}\u{1f}Oyuncu Maçtan Ayrıldı\u{1f}玩家离开比赛",
            "\u{1f}Player Received Healing\u{1f}\u{1f}El jugador recibió una sanación\u{1f}Un joueur a reçu des soins\u{1f}\u{1f}プレイヤーが回復を受けた\u{1f}\u{1f}\u{1f}Jogador Recebeu Cura\u{1f}\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นได\u{e49}ร\u{e31}บการฮ\u{e35}ล\u{1f}Oyuncu İyileştirme Aldı\u{1f}玩家受到治疗",
            "\u{1f}Player Received Knockback\u{1f}Repulsión sufrida por el jugador\u{1f}El jugador recibió un derribo\u{1f}Un joueur a reçu un recul\u{1f}\u{1f}プレイヤーがノックバックを受けた\u{1f}\u{1f}\u{1f}Jogador Recebeu Repulsão\u{1f}\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นท\u{e35}\u{e48}ได\u{e49}ร\u{e31}บการกระเด\u{e47}นถอยหล\u{e31}ง\u{1f}Oyuncu Geri İtme Aldı\u{1f}玩家受到击退",
            "\u{1f}Player Took Damage\u{1f}\u{1f}El jugador recibió daño\u{1f}Un joueur subit des dégâts\u{1f}\u{1f}プレイヤーがダメージを受けた\u{1f}\u{1f}\u{1f}Jogador Recebeu Dano\u{1f}Player took damage\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นได\u{e49}ร\u{e31}บความเส\u{e35}ยหาย\u{1f}Oyuncu Hasar Aldı\u{1f}玩家受到伤害",
            "\u{1f}Subroutine\u{1f}Subrutina\u{1f}Subrutina\u{1f}Sous-programme\u{1f}\u{1f}サブルーチン\u{1f}\u{1f}\u{1f}Sub-rotina\u{1f}\u{1f}ซ\u{e31}บร\u{e39}ท\u{e35}น\u{1f}Alt Yordam\u{1f}子程序",
            "\u{1f}All\u{1f}Todos\u{1f}Todos\u{1f}Les deux\u{1f}Tutti\u{1f}すべて\u{1f}\u{1f}Wszystkie\u{1f}Todas\u{1f}\u{1f}ท\u{e31}\u{e49}งหมด\u{1f}Tümü\u{1f}双方",
            "\u{1f}Team 1\u{1f}\u{1f}Equipo 1\u{1f}Équipe 1\u{1f}\u{1f}チーム1\u{1f}\u{1f}\u{1f}Equipe 1\u{1f}\u{1f}ท\u{e35}ม 1\u{1f}1. Takım\u{1f}队伍1",
            "\u{1f}Team 2\u{1f}\u{1f}Equipo 2\u{1f}Équipe 2\u{1f}\u{1f}チーム2\u{1f}\u{1f}\u{1f}Equipe 2\u{1f}\u{1f}ท\u{e35}ม 2\u{1f}2. Takım\u{1f}队伍2",
            "\u{1f}All\u{1f}Todos\u{1f}Todos\u{1f}Tout\u{1f}Tutti\u{1f}すべて\u{1f}\u{1f}Wszystkie\u{1f}Tudo\u{1f}Все\u{1f}ท\u{e31}\u{e49}งหมด\u{1f}Tümü\u{1f}全部",
            "\u{1f}Ana\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}アナ\u{1f}아나\u{1f}\u{1f}\u{1f}Ана\u{1f}อานา\u{1f}\u{1f}安娜\u{1f}安娜",
            "\u{1f}Anran\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}安燃",
            "\u{1f}Ashe\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}アッシュ\u{1f}애쉬\u{1f}\u{1f}\u{1f}Эш\u{1f}แอช\u{1f}\u{1f}艾什\u{1f}艾西",
            "\u{1f}Baptiste\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}バティスト\u{1f}바티스트\u{1f}\u{1f}\u{1f}Батист\u{1f}บ\u{e31}ปท\u{e34}สต\u{e4c}\u{1f}\u{1f}巴蒂斯特\u{1f}巴帝斯特",
            "\u{1f}Bastion\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}バスティオン\u{1f}바스티온\u{1f}\u{1f}\u{1f}Бастион\u{1f}แบสเช\u{e35}ยน\u{1f}\u{1f}堡垒\u{1f}壁壘機兵",
            "\u{1f}Brigitte\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ブリギッテ\u{1f}브리기테\u{1f}\u{1f}\u{1f}Бригитта\u{1f}บร\u{e34}ก\u{e34}ตตา\u{1f}\u{1f}布丽吉塔\u{1f}碧姬",
            "\u{1f}Cassidy\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}キャスディ\u{1f}캐서디\u{1f}\u{1f}\u{1f}Кэссиди\u{1f}แคสซ\u{e34}ด\u{e35}\u{e49}\u{1f}\u{1f}卡西迪\u{1f}卡西迪",
            "\u{1f}D.Mon",
            "\u{1f}Doctrine",
            "\u{1f}Domina\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}金驭\u{1f}多米娜",
            "\u{1f}Doomfist\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ドゥームフィスト\u{1f}둠피스트\u{1f}Pięść Zagłady\u{1f}\u{1f}Кулак Смерти\u{1f}ด\u{e39}มฟ\u{e34}สต\u{e4c}\u{1f}\u{1f}末日铁拳\u{1f}毀滅拳王",
            "\u{1f}D.Va\u{1f}\u{1f}D.VA\u{1f}\u{1f}D.VA\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ด\u{e35}ว\u{e48}า",
            "\u{1f}Echo\u{1f}\u{1f}\u{1f}Écho\u{1f}\u{1f}エコー\u{1f}에코\u{1f}\u{1f}\u{1f}Эхо\u{1f}เอ\u{e47}คโค\u{e48}\u{1f}\u{1f}回声\u{1f}迴音",
            "\u{1f}Emre\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}埃姆雷\u{1f}伊默",
            "\u{1f}Freja\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}弗蕾娅\u{1f}弗蕾亞",
            "\u{1f}Genji\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ゲンジ\u{1f}겐지\u{1f}\u{1f}\u{1f}Гэндзи\u{1f}เก\u{e47}นจ\u{e34}\u{1f}\u{1f}源氏\u{1f}源氏",
            "\u{1f}Hanzo\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ハンゾー\u{1f}한조\u{1f}\u{1f}\u{1f}Хандзо\u{1f}ฮ\u{e31}นโซ\u{1f}\u{1f}半藏\u{1f}半藏",
            "\u{1f}Hazard\u{1f}\u{1f}\u{1f}Danger\u{1f}\u{1f}ハザード\u{1f}해저드\u{1f}Ryzyk\u{1f}\u{1f}Азарт\u{1f}ฮาซาร\u{e4c}ด\u{1f}\u{1f}骇灾\u{1f}災害",
            "\u{1f}Illari\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}イラリー\u{1f}일리아리\u{1f}\u{1f}\u{1f}Иллари\u{1f}อ\u{e34}ลลาร\u{e35}\u{1f}\u{1f}伊拉锐\u{1f}伊拉里",
            "\u{1f}Jetpack Cat\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}飞天猫\u{1f}火箭貓",
            "\u{1f}Junker Queen\u{1f}\u{1f}\u{1f}Reine des Junkers\u{1f}Regina dei Junker\u{1f}ジャンカー・クイーン\u{1f}정커퀸\u{1f}Królowa Junkertown\u{1f}Rainha Junker\u{1f}Королева Стервятников\u{1f}จ\u{e31}งเกอร\u{e4c}คว\u{e35}น\u{1f}\u{1f}渣客女王\u{1f}垃圾鎮女王",
            "\u{1f}Junkrat\u{1f}\u{1f}\u{1f}Chacal\u{1f}\u{1f}ジャンクラット\u{1f}정크랫\u{1f}Złomiarz\u{1f}\u{1f}Крысавчик\u{1f}จ\u{e31}งค\u{e4c}แรต\u{1f}\u{1f}狂鼠\u{1f}炸彈鼠",
            "\u{1f}Juno\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ジュノ\u{1f}주노\u{1f}\u{1f}\u{1f}Юнона\u{1f}จ\u{e39}โน\u{1f}\u{1f}朱诺\u{1f}朱諾",
            "\u{1f}Kiriko\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}キリコ\u{1f}키리코\u{1f}\u{1f}\u{1f}Кирико\u{1f}ค\u{e34}ร\u{e34}โกะ\u{1f}\u{1f}雾子\u{1f}霧子",
            "\u{1f}Lifeweaver\u{1f}\u{1f}\u{1f}Vital\u{1f}\u{1f}ライフウィーバー\u{1f}라이프위버\u{1f}Życiodawca\u{1f}\u{1f}Ткач Жизни\u{1f}ไลฟ\u{e4c}ว\u{e35}เวอร\u{e4c}\u{1f}\u{1f}生命之梭\u{1f}織命",
            "\u{1f}Lúcio\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ルシオ\u{1f}루시우\u{1f}\u{1f}\u{1f}Лусио\u{1f}ล\u{e39}ซ\u{e34}โอ\u{1f}\u{1f}卢西奥\u{1f}路西歐",
            "\u{1f}Mauga\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}マウガ\u{1f}마우가\u{1f}\u{1f}\u{1f}Мауга\u{1f}มาวกา\u{1f}\u{1f}毛加\u{1f}莫加",
            "\u{1f}Mei\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}メイ\u{1f}메이\u{1f}\u{1f}\u{1f}Мэй\u{1f}เหมย\u{1f}\u{1f}美\u{1f}小美",
            "\u{1f}Mercy\u{1f}\u{1f}\u{1f}Ange\u{1f}\u{1f}マーシー\u{1f}메르시\u{1f}Łaska\u{1f}\u{1f}Ангел\u{1f}เมอร\u{e4c}ซ\u{e35}\u{e48}\u{1f}\u{1f}天使\u{1f}慈悲",
            "\u{1f}Mizuki\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}瑞稀\u{1f}瑞稀",
            "\u{1f}Moira\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}モイラ\u{1f}모이라\u{1f}\u{1f}\u{1f}Мойра\u{1f}มอยร\u{e48}า\u{1f}\u{1f}莫伊拉\u{1f}莫伊拉",
            "\u{1f}Orisa\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}オリーサ\u{1f}오리사\u{1f}\u{1f}\u{1f}Ориса\u{1f}โอร\u{e34}ซา\u{1f}\u{1f}奥丽莎\u{1f}歐瑞莎",
            "\u{1f}Pharah\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ファラ\u{1f}파라\u{1f}Fara\u{1f}\u{1f}Фарра\u{1f}ฟาราห\u{e4c}\u{1f}\u{1f}法老之鹰\u{1f}法拉",
            "\u{1f}Ramattra\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ラマットラ\u{1f}라마트라\u{1f}\u{1f}\u{1f}Раматтра\u{1f}ราม\u{e31}ตตรา\u{1f}\u{1f}拉玛刹\u{1f}拉瑪塔",
            "\u{1f}Reaper\u{1f}\u{1f}\u{1f}Faucheur\u{1f}\u{1f}リーパー\u{1f}리퍼\u{1f}Żniwiarz\u{1f}\u{1f}Жнец\u{1f}ร\u{e35}ปเปอร\u{e4c}\u{1f}\u{1f}死神\u{1f}死神",
            "\u{1f}Reinhardt\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ラインハルト\u{1f}라인하르트\u{1f}\u{1f}\u{1f}Райнхардт\u{1f}ไรน\u{e4c}ฮาร\u{e4c}ด\u{1f}\u{1f}莱因哈特\u{1f}萊因哈特",
            "\u{1f}Roadhog\u{1f}\u{1f}\u{1f}Chopper\u{1f}\u{1f}ロードホッグ\u{1f}로드호그\u{1f}Wieprzu\u{1f}\u{1f}Турбосвин\u{1f}โร\u{e49}ดฮ\u{e47}อก\u{1f}\u{1f}路霸\u{1f}攔路豬",
            "\u{1f}Shion\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}死怨",
            "\u{1f}Sierra\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}西拉",
            "\u{1f}Sigma\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}シグマ\u{1f}시그마\u{1f}\u{1f}\u{1f}Сигма\u{1f}ซ\u{e34}กม\u{e48}า\u{1f}\u{1f}西格玛\u{1f}席格馬",
            "\u{1f}Slot 0\u{1f}Ranura 0\u{1f}Puesto 0\u{1f}Emplacement 0\u{1f}\u{1f}スロット0\u{1f}\u{1f}Miejsce 0\u{1f}Espaço 0\u{1f}Ячейка 0\u{1f}ช\u{e48}อง 0\u{1f}0. Yuva\u{1f}栏位 0",
            "\u{1f}Slot 1\u{1f}Ranura 1\u{1f}Puesto 1\u{1f}Emplacement 1\u{1f}\u{1f}スロット1\u{1f}\u{1f}Miejsce 1\u{1f}Espaço 1\u{1f}Ячейка 1\u{1f}ช\u{e48}อง 1\u{1f}1. Yuva\u{1f}栏位 1",
            "\u{1f}Slot 10\u{1f}Ranura 10\u{1f}Puesto 10\u{1f}Emplacement 10\u{1f}\u{1f}スロット10\u{1f}\u{1f}Miejsce 10\u{1f}Espaço 10\u{1f}Ячейка 10\u{1f}ช\u{e48}อง 10\u{1f}10. Yuva\u{1f}栏位 10",
            "\u{1f}Slot 11\u{1f}Ranura 11\u{1f}Puesto 11\u{1f}Emplacement 11\u{1f}\u{1f}スロット11\u{1f}\u{1f}Miejsce 11\u{1f}Espaço 11\u{1f}Ячейка 11\u{1f}ช\u{e48}อง 11\u{1f}11. Yuva\u{1f}栏位 11",
            "\u{1f}Slot 2\u{1f}Ranura 2\u{1f}Puesto 2\u{1f}Emplacement 2\u{1f}\u{1f}スロット2\u{1f}\u{1f}Miejsce 2\u{1f}Espaço 2\u{1f}Ячейка 2\u{1f}ช\u{e48}อง 2\u{1f}2. Yuva\u{1f}栏位 2",
            "\u{1f}Slot 3\u{1f}Ranura 3\u{1f}Puesto 3\u{1f}Emplacement 3\u{1f}\u{1f}スロット3\u{1f}\u{1f}Miejsce 3\u{1f}Espaço 3\u{1f}Ячейка 3\u{1f}ช\u{e48}อง 3\u{1f}3. Yuva\u{1f}栏位 3",
            "\u{1f}Slot 4\u{1f}Ranura 4\u{1f}Puesto 4\u{1f}Emplacement 4\u{1f}\u{1f}スロット4\u{1f}\u{1f}Miejsce 4\u{1f}Espaço 4\u{1f}Ячейка 4\u{1f}ช\u{e48}อง 4\u{1f}4. Yuva\u{1f}栏位 4",
            "\u{1f}Slot 5\u{1f}Ranura 5\u{1f}Puesto 5\u{1f}Emplacement 5\u{1f}\u{1f}スロット5\u{1f}\u{1f}Miejsce 5\u{1f}Espaço 5\u{1f}Ячейка 5\u{1f}ช\u{e48}อง 5\u{1f}5. Yuva\u{1f}栏位 5",
            "\u{1f}Slot 6\u{1f}Ranura 6\u{1f}Puesto 6\u{1f}Emplacement 6\u{1f}\u{1f}スロット6\u{1f}\u{1f}Miejsce 6\u{1f}Espaço 6\u{1f}Ячейка 6\u{1f}ช\u{e48}อง 6\u{1f}6. Yuva\u{1f}栏位 6",
            "\u{1f}Slot 7\u{1f}Ranura 7\u{1f}Puesto 7\u{1f}Emplacement 7\u{1f}\u{1f}スロット7\u{1f}\u{1f}Miejsce 7\u{1f}Espaço 7\u{1f}Ячейка 7\u{1f}ช\u{e48}อง 7\u{1f}7. Yuva\u{1f}栏位 7",
            "\u{1f}Slot 8\u{1f}Ranura 8\u{1f}Puesto 8\u{1f}Emplacement 8\u{1f}\u{1f}スロット8\u{1f}\u{1f}Miejsce 8\u{1f}Espaço 8\u{1f}Ячейка 8\u{1f}ช\u{e48}อง 8\u{1f}8. Yuva\u{1f}栏位 8",
            "\u{1f}Slot 9\u{1f}Ranura 9\u{1f}Puesto 9\u{1f}Emplacement 9\u{1f}\u{1f}スロット9\u{1f}\u{1f}Miejsce 9\u{1f}Espaço 9\u{1f}Ячейка 9\u{1f}ช\u{e48}อง 9\u{1f}9. Yuva\u{1f}栏位 9",
            "\u{1f}Sojourn\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ソジョーン\u{1f}소전\u{1f}\u{1f}\u{1f}Соджорн\u{1f}โซเจ\u{e34}ร\u{e4c}น\u{1f}\u{1f}索杰恩\u{1f}索潔恩",
            "\u{1f}Soldier: 76\u{1f}Soldado: 76\u{1f}Soldado: 76\u{1f}Soldat : 76\u{1f}Soldato-76\u{1f}ソルジャー76\u{1f}솔저: 76\u{1f}Żołnierz-76\u{1f}Soldado: 76\u{1f}Солдат-76\u{1f}โซลเยอร\u{e4c}: 76\u{1f}\u{1f}士兵：76\u{1f}士兵76",
            "\u{1f}Sombra\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ソンブラ\u{1f}솜브라\u{1f}\u{1f}\u{1f}Сомбра\u{1f}ซอมบรา\u{1f}\u{1f}黑影\u{1f}駭影",
            "\u{1f}Symmetra\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}シンメトラ\u{1f}시메트라\u{1f}\u{1f}\u{1f}Симметра\u{1f}ซ\u{e34}มเมตรา\u{1f}\u{1f}秩序之光\u{1f}辛梅塔",
            "\u{1f}Torbjörn\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}トールビョーン\u{1f}토르비욘\u{1f}\u{1f}\u{1f}Торбьорн\u{1f}ทอร\u{e4c}บยอร\u{e4c}น\u{1f}\u{1f}托比昂\u{1f}托比昂",
            "\u{1f}Tracer\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}トレーサー\u{1f}트레이서\u{1f}Smuga\u{1f}\u{1f}Трейсер\u{1f}เทรเซอร\u{e4c}\u{1f}\u{1f}猎空\u{1f}閃光",
            "\u{1f}Vendetta\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}斩仇\u{1f}宿怨",
            "\u{1f}Venture\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ベンチャー\u{1f}벤처\u{1f}\u{1f}\u{1f}Авентюра\u{1f}เวนเจอร\u{e4c}\u{1f}\u{1f}探奇\u{1f}無畏",
            "\u{1f}Widowmaker\u{1f}\u{1f}\u{1f}Fatale\u{1f}\u{1f}ウィドウメイカー\u{1f}위도우메이커\u{1f}Trupia Wdowa\u{1f}\u{1f}Роковая Вдова\u{1f}ว\u{e34}โดว\u{e4c}เมกเกอร\u{e4c}\u{1f}\u{1f}黑百合\u{1f}奪命女",
            "\u{1f}Winston\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ウィンストン\u{1f}윈스턴\u{1f}\u{1f}\u{1f}Уинстон\u{1f}ว\u{e34}นสต\u{e31}น\u{1f}\u{1f}温斯顿\u{1f}溫斯頓",
            "\u{1f}Wrecking Ball\u{1f}\u{1f}\u{1f}Bouldozer\u{1f}\u{1f}レッキング・ボール\u{1f}레킹볼\u{1f}Burzyciel\u{1f}\u{1f}Таран\u{1f}เร\u{e47}กก\u{e34}งบอล\u{1f}\u{1f}破坏球\u{1f}火爆鋼球",
            "\u{1f}Wuyang\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}无漾\u{1f}無漾",
            "\u{1f}Zarya\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ザリア\u{1f}자리야\u{1f}Zaria\u{1f}\u{1f}Заря\u{1f}ซาเร\u{e35}ย\u{1f}\u{1f}查莉娅\u{1f}札莉雅",
            "\u{1f}Zenyatta\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ゼニヤッタ\u{1f}젠야타\u{1f}\u{1f}\u{1f}Дзенъятта\u{1f}เซ\u{e47}นย\u{e31}ตตา\u{1f}\u{1f}禅雅塔\u{1f}禪亞塔",
            "Deaktiviert\u{1f}Disabled\u{1f}No permitir\u{1f}Deshabilitado\u{1f}Désactivé\u{1f}Off\u{1f}無効\u{1f}비활성화\u{1f}Wyłączono\u{1f}Desativada\u{1f}Откл.\u{1f}ป\u{e34}ดใช\u{e49}งาน\u{1f}Devre Dışı\u{1f}禁用\u{1f}停用",
            "Aktiviert\u{1f}Enabled\u{1f}Permitir\u{1f}Habilitado\u{1f}Activé\u{1f}Attivata\u{1f}有効\u{1f}활성화\u{1f}Tryb aktywny\u{1f}Ativado\u{1f}Вкл.\u{1f}เป\u{e34}ดใช\u{e49}งานแล\u{e49}ว\u{1f}Etkin\u{1f}启用\u{1f}啟用",
            "Nein\u{1f}No\u{1f}\u{1f}\u{1f}Non\u{1f}\u{1f}いいえ\u{1f}아니요\u{1f}Nie\u{1f}Não\u{1f}Нет\u{1f}ไม\u{e48}\u{1f}Hayır\u{1f}否\u{1f}否",
            "Aus\u{1f}Off\u{1f}Desactivado\u{1f}No\u{1f}Désactivé\u{1f}\u{1f}OFF\u{1f}비활성화\u{1f}Wył.\u{1f}Desligado\u{1f}Откл.\u{1f}ป\u{e34}ด\u{1f}Kapalı\u{1f}关闭\u{1f}關閉",
            "Ein\u{1f}On\u{1f}Activado\u{1f}Activado\u{1f}Activé\u{1f}\u{1f}ON\u{1f}활성화\u{1f}Wł.\u{1f}Ligado\u{1f}Вкл.\u{1f}เป\u{e34}ด\u{1f}Açık\u{1f}开启\u{1f}開啟",
            "\u{1f}Team 1\u{1f}Equipo 1\u{1f}Equipo 1\u{1f}Équipe 1\u{1f}Squadra 1\u{1f}チーム1\u{1f}1팀\u{1f}Drużyna 1\u{1f}Equipe 1\u{1f}Команда 1\u{1f}ท\u{e35}ม 1\u{1f}1. Takım\u{1f}队伍1\u{1f}隊伍1",
            "\u{1f}Team 2\u{1f}Equipo 2\u{1f}Equipo 2\u{1f}Équipe 2\u{1f}Squadra 2\u{1f}チーム2\u{1f}2팀\u{1f}Drużyna 2\u{1f}Equipe 2\u{1f}Команда 2\u{1f}ท\u{e35}ม 2\u{1f}2. Takım\u{1f}队伍2\u{1f}隊伍2",
            "Ja\u{1f}Yes\u{1f}Sí\u{1f}Sí\u{1f}Oui\u{1f}Sì\u{1f}はい\u{1f}네\u{1f}Tak\u{1f}Sim\u{1f}Да\u{1f}ใช\u{e48}\u{1f}Evet\u{1f}是\u{1f}是",
        ];
        localized(NAMES[self as usize], lang)
    }
}
