//! Inline textures.
/// An inline texture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Texture {
    /// `ASSAULT`
    Assault,
    /// `ATTACKING`
    Attacking,
    /// `A_BUTTON`
    AButton,
    /// `BLIZZARD_SPECIAL`
    BlizzardSpecial,
    /// `BRIGITTE_PASSIVE`
    BrigittePassive,
    /// `BRIGITTE_SHIELD_BASH`
    BrigitteShieldBash,
    /// `BRONZE_BORDER`
    BronzeBorder,
    /// `B_BUTTON`
    BButton,
    /// `CAPTURE_THE_FLAG`
    CaptureTheFlag,
    /// `COMPETITIVE_ICON`
    CompetitiveIcon,
    /// `COMPETITIVE_POINTS`
    CompetitivePoints,
    /// `CONNECTIVITY_ISSUE`
    ConnectivityIssue,
    /// `CONTROL`
    Control,
    /// `DEATHMATCH`
    Deathmatch,
    /// `DEFENDING`
    Defending,
    /// `DOOMFIST_PASSIVE`
    DoomfistPassive,
    /// `DPAD_DOWN_ARROW`
    DpadDownArrow,
    /// `DPAD_LEFT_ARROW`
    DpadLeftArrow,
    /// `DPAD_RIGHT_ARROW`
    DpadRightArrow,
    /// `DPAD_UP_ARROW`
    DpadUpArrow,
    /// `DVA_ALTERNATE_ULT`
    DvaAlternateUlt,
    /// `DVA_LIGHT_GUN`
    DvaLightGun,
    /// `DVA_PASSIVE`
    DvaPassive,
    /// `ELIMINATION`
    Elimination,
    /// `EMOTE`
    Emote,
    /// `ESCORT`
    Escort,
    /// `EXPERIMENTAL_MODE_DARK`
    ExperimentalModeDark,
    /// `EXPLOSION_DARK`
    ExplosionDark,
    /// `FALL_BACK`
    FallBack,
    /// `FIRE_DARK`
    FireDark,
    /// `FLAG_ARGENTINA`
    FlagArgentina,
    /// `FLAG_BELGIUM`
    FlagBelgium,
    /// `FLAG_BULGARIA`
    FlagBulgaria,
    /// `FLAG_CHILE`
    FlagChile,
    /// `FLAG_COLOMBIA`
    FlagColombia,
    /// `FLAG_GREECE`
    FlagGreece,
    /// `FLAG_ICELAND`
    FlagIceland,
    /// `FLAG_INDIA`
    FlagIndia,
    /// `FLAG_IRELAND`
    FlagIreland,
    /// `FLASHPOINT`
    Flashpoint,
    /// `GAME_BROWSER_ICON_DARK`
    GameBrowserIconDark,
    /// `GO`
    Go,
    /// `GOING_IN`
    GoingIn,
    /// `GROUP_UP`
    GroupUp,
    /// `HALLOWEEN_TERROR`
    HalloweenTerror,
    /// `HANZO_LUNGE`
    HanzoLunge,
    /// `HEALING_PLUS_DARK`
    HealingPlusDark,
    /// `HELLO`
    Hello,
    /// `HERO_BORDER`
    HeroBorder,
    /// `HYBRID`
    Hybrid,
    /// `INCOMING`
    Incoming,
    /// `JUNKER_QUEEN_PASSIVE`
    JunkerQueenPassive,
    /// `JUNKRAT_PASSIVE`
    JunkratPassive,
    /// `LOCK`
    Lock,
    /// `LUCIO_HEALING_CROSSFADE`
    LucioHealingCrossfade,
    /// `MERCY_ANGELIC_DESCENT`
    MercyAngelicDescent,
    /// `MICROPHONE`
    Microphone,
    /// `MOUSE_CURSOR`
    MouseCursor,
    /// `MOUSE_SIDE_BUTTON`
    MouseSideButton,
    /// `NEED_HEALING`
    NeedHealing,
    /// `NEED_HELP`
    NeedHelp,
    /// `NO`
    No,
    /// `ON_MY_WAY`
    OnMyWay,
    /// `OVERWATCH_COINS`
    OverwatchCoins,
    /// `OVERWATCH_LEAGUE_TOKENS`
    OverwatchLeagueTokens,
    /// `PACKET_LOSS`
    PacketLoss,
    /// `PINK_MERCY_HEART`
    PinkMercyHeart,
    /// `PRACTICE_RANGE`
    PracticeRange,
    /// `PRESS_THE_ATTACK`
    PressTheAttack,
    /// `PUSH`
    Push,
    /// `PUSH_FORWARD`
    PushForward,
    /// `REAPER_PASSIVE`
    ReaperPassive,
    /// `RIGHT_CLICK`
    RightClick,
    /// `SCROLL`
    Scroll,
    /// `SCROLL_DOWN`
    ScrollDown,
    /// `SCROLL_UP`
    ScrollUp,
    /// `SKIRMISH`
    Skirmish,
    /// `SOMBRA_PASSIVE`
    SombraPassive,
    /// `SOMBRA_STEALTH`
    SombraStealth,
    /// `SORRY`
    Sorry,
    /// `SQUARE_GRADIENT_TRANSPARENT_BLACK`
    SquareGradientTransparentBlack,
    /// `SUMMER_GAMES`
    SummerGames,
    /// `TARGET_WITH_CHECKMARK_DARK`
    TargetWithCheckmarkDark,
    /// `TARGET_WITH_PERSON_DARK`
    TargetWithPersonDark,
    /// `TEAM_DEATHMATCH`
    TeamDeathmatch,
    /// `THANKS`
    Thanks,
    /// `TORBJORN_SCRAP_COLLECTOR`
    TorbjornScrapCollector,
    /// `TRACER_ICON_DARK`
    TracerIconDark,
    /// `ULTIMATE`
    Ultimate,
    /// `UNRANKED_ICON_DARK`
    UnrankedIconDark,
    /// `WITH_YOU`
    WithYou,
    /// `WORKSHOP`
    Workshop,
    /// `Y_BUTTON`
    YButton,
}
impl Texture {
    /// Every variant, in declaration order.
    pub const ALL: &'static [Self] = &[
        Self::Assault,
        Self::Attacking,
        Self::AButton,
        Self::BlizzardSpecial,
        Self::BrigittePassive,
        Self::BrigitteShieldBash,
        Self::BronzeBorder,
        Self::BButton,
        Self::CaptureTheFlag,
        Self::CompetitiveIcon,
        Self::CompetitivePoints,
        Self::ConnectivityIssue,
        Self::Control,
        Self::Deathmatch,
        Self::Defending,
        Self::DoomfistPassive,
        Self::DpadDownArrow,
        Self::DpadLeftArrow,
        Self::DpadRightArrow,
        Self::DpadUpArrow,
        Self::DvaAlternateUlt,
        Self::DvaLightGun,
        Self::DvaPassive,
        Self::Elimination,
        Self::Emote,
        Self::Escort,
        Self::ExperimentalModeDark,
        Self::ExplosionDark,
        Self::FallBack,
        Self::FireDark,
        Self::FlagArgentina,
        Self::FlagBelgium,
        Self::FlagBulgaria,
        Self::FlagChile,
        Self::FlagColombia,
        Self::FlagGreece,
        Self::FlagIceland,
        Self::FlagIndia,
        Self::FlagIreland,
        Self::Flashpoint,
        Self::GameBrowserIconDark,
        Self::Go,
        Self::GoingIn,
        Self::GroupUp,
        Self::HalloweenTerror,
        Self::HanzoLunge,
        Self::HealingPlusDark,
        Self::Hello,
        Self::HeroBorder,
        Self::Hybrid,
        Self::Incoming,
        Self::JunkerQueenPassive,
        Self::JunkratPassive,
        Self::Lock,
        Self::LucioHealingCrossfade,
        Self::MercyAngelicDescent,
        Self::Microphone,
        Self::MouseCursor,
        Self::MouseSideButton,
        Self::NeedHealing,
        Self::NeedHelp,
        Self::No,
        Self::OnMyWay,
        Self::OverwatchCoins,
        Self::OverwatchLeagueTokens,
        Self::PacketLoss,
        Self::PinkMercyHeart,
        Self::PracticeRange,
        Self::PressTheAttack,
        Self::Push,
        Self::PushForward,
        Self::ReaperPassive,
        Self::RightClick,
        Self::Scroll,
        Self::ScrollDown,
        Self::ScrollUp,
        Self::Skirmish,
        Self::SombraPassive,
        Self::SombraStealth,
        Self::Sorry,
        Self::SquareGradientTransparentBlack,
        Self::SummerGames,
        Self::TargetWithCheckmarkDark,
        Self::TargetWithPersonDark,
        Self::TeamDeathmatch,
        Self::Thanks,
        Self::TorbjornScrapCollector,
        Self::TracerIconDark,
        Self::Ultimate,
        Self::UnrankedIconDark,
        Self::WithYou,
        Self::Workshop,
        Self::YButton,
    ];
    /// The data key, for debugging.
    pub fn key(self) -> &'static str {
        match self {
            Self::Assault => "ASSAULT",
            Self::Attacking => "ATTACKING",
            Self::AButton => "A_BUTTON",
            Self::BlizzardSpecial => "BLIZZARD_SPECIAL",
            Self::BrigittePassive => "BRIGITTE_PASSIVE",
            Self::BrigitteShieldBash => "BRIGITTE_SHIELD_BASH",
            Self::BronzeBorder => "BRONZE_BORDER",
            Self::BButton => "B_BUTTON",
            Self::CaptureTheFlag => "CAPTURE_THE_FLAG",
            Self::CompetitiveIcon => "COMPETITIVE_ICON",
            Self::CompetitivePoints => "COMPETITIVE_POINTS",
            Self::ConnectivityIssue => "CONNECTIVITY_ISSUE",
            Self::Control => "CONTROL",
            Self::Deathmatch => "DEATHMATCH",
            Self::Defending => "DEFENDING",
            Self::DoomfistPassive => "DOOMFIST_PASSIVE",
            Self::DpadDownArrow => "DPAD_DOWN_ARROW",
            Self::DpadLeftArrow => "DPAD_LEFT_ARROW",
            Self::DpadRightArrow => "DPAD_RIGHT_ARROW",
            Self::DpadUpArrow => "DPAD_UP_ARROW",
            Self::DvaAlternateUlt => "DVA_ALTERNATE_ULT",
            Self::DvaLightGun => "DVA_LIGHT_GUN",
            Self::DvaPassive => "DVA_PASSIVE",
            Self::Elimination => "ELIMINATION",
            Self::Emote => "EMOTE",
            Self::Escort => "ESCORT",
            Self::ExperimentalModeDark => "EXPERIMENTAL_MODE_DARK",
            Self::ExplosionDark => "EXPLOSION_DARK",
            Self::FallBack => "FALL_BACK",
            Self::FireDark => "FIRE_DARK",
            Self::FlagArgentina => "FLAG_ARGENTINA",
            Self::FlagBelgium => "FLAG_BELGIUM",
            Self::FlagBulgaria => "FLAG_BULGARIA",
            Self::FlagChile => "FLAG_CHILE",
            Self::FlagColombia => "FLAG_COLOMBIA",
            Self::FlagGreece => "FLAG_GREECE",
            Self::FlagIceland => "FLAG_ICELAND",
            Self::FlagIndia => "FLAG_INDIA",
            Self::FlagIreland => "FLAG_IRELAND",
            Self::Flashpoint => "FLASHPOINT",
            Self::GameBrowserIconDark => "GAME_BROWSER_ICON_DARK",
            Self::Go => "GO",
            Self::GoingIn => "GOING_IN",
            Self::GroupUp => "GROUP_UP",
            Self::HalloweenTerror => "HALLOWEEN_TERROR",
            Self::HanzoLunge => "HANZO_LUNGE",
            Self::HealingPlusDark => "HEALING_PLUS_DARK",
            Self::Hello => "HELLO",
            Self::HeroBorder => "HERO_BORDER",
            Self::Hybrid => "HYBRID",
            Self::Incoming => "INCOMING",
            Self::JunkerQueenPassive => "JUNKER_QUEEN_PASSIVE",
            Self::JunkratPassive => "JUNKRAT_PASSIVE",
            Self::Lock => "LOCK",
            Self::LucioHealingCrossfade => "LUCIO_HEALING_CROSSFADE",
            Self::MercyAngelicDescent => "MERCY_ANGELIC_DESCENT",
            Self::Microphone => "MICROPHONE",
            Self::MouseCursor => "MOUSE_CURSOR",
            Self::MouseSideButton => "MOUSE_SIDE_BUTTON",
            Self::NeedHealing => "NEED_HEALING",
            Self::NeedHelp => "NEED_HELP",
            Self::No => "NO",
            Self::OnMyWay => "ON_MY_WAY",
            Self::OverwatchCoins => "OVERWATCH_COINS",
            Self::OverwatchLeagueTokens => "OVERWATCH_LEAGUE_TOKENS",
            Self::PacketLoss => "PACKET_LOSS",
            Self::PinkMercyHeart => "PINK_MERCY_HEART",
            Self::PracticeRange => "PRACTICE_RANGE",
            Self::PressTheAttack => "PRESS_THE_ATTACK",
            Self::Push => "PUSH",
            Self::PushForward => "PUSH_FORWARD",
            Self::ReaperPassive => "REAPER_PASSIVE",
            Self::RightClick => "RIGHT_CLICK",
            Self::Scroll => "SCROLL",
            Self::ScrollDown => "SCROLL_DOWN",
            Self::ScrollUp => "SCROLL_UP",
            Self::Skirmish => "SKIRMISH",
            Self::SombraPassive => "SOMBRA_PASSIVE",
            Self::SombraStealth => "SOMBRA_STEALTH",
            Self::Sorry => "SORRY",
            Self::SquareGradientTransparentBlack => "SQUARE_GRADIENT_TRANSPARENT_BLACK",
            Self::SummerGames => "SUMMER_GAMES",
            Self::TargetWithCheckmarkDark => "TARGET_WITH_CHECKMARK_DARK",
            Self::TargetWithPersonDark => "TARGET_WITH_PERSON_DARK",
            Self::TeamDeathmatch => "TEAM_DEATHMATCH",
            Self::Thanks => "THANKS",
            Self::TorbjornScrapCollector => "TORBJORN_SCRAP_COLLECTOR",
            Self::TracerIconDark => "TRACER_ICON_DARK",
            Self::Ultimate => "ULTIMATE",
            Self::UnrankedIconDark => "UNRANKED_ICON_DARK",
            Self::WithYou => "WITH_YOU",
            Self::Workshop => "WORKSHOP",
            Self::YButton => "Y_BUTTON",
        }
    }
}
impl Texture {
    /// The tag text that renders the texture.
    pub fn tag(self) -> &'static str {
        match self {
            Self::Lock => "<txc000000000012f3>",
            Self::MercyAngelicDescent => "<txc000000000012f6>",
            Self::BrigitteShieldBash => "<txc00000000003339>",
            Self::Attacking => "<txc000000000039d7>",
            Self::GroupUp => "<txc000000000039da>",
            Self::Ultimate => "<txc000000000039db>",
            Self::NeedHealing => "<txc000000000039dd>",
            Self::LucioHealingCrossfade => "<txc00000000003c5d>",
            Self::ReaperPassive => "<txc00000000003fcf>",
            Self::TorbjornScrapCollector => "<txc00000000003fd0>",
            Self::RightClick => "<txc00000000004080>",
            Self::DpadRightArrow => "<txc000000000044a5>",
            Self::DpadUpArrow => "<txc000000000044a6>",
            Self::AButton => "<txc000000000044af>",
            Self::BButton => "<txc000000000044b1>",
            Self::DpadLeftArrow | Self::YButton => "<txc000000000044b2>",
            Self::DpadDownArrow => "<txc000000000044b3>",
            Self::JunkratPassive => "<txc00000000004e54>",
            Self::DvaLightGun => "<txc00000000004ea9>",
            Self::DvaAlternateUlt => "<txc00000000004eab>",
            Self::DvaPassive => "<txc00000000005405>",
            Self::Emote => "<txc00000000005a0a>",
            Self::Hello => "<txc00000000005a0b>",
            Self::NeedHelp => "<txc00000000005a0c>",
            Self::Thanks => "<txc00000000005a0d>",
            Self::ScrollUp => "<txc00000000005da0>",
            Self::ScrollDown => "<txc00000000005da1>",
            Self::MouseSideButton => "<txc00000000005dab>",
            Self::Scroll => "<txc00000000005dad>",
            Self::Microphone => "<txc00000000007114>",
            Self::BronzeBorder => "<txc00000000007ea3>",
            Self::PacketLoss => "<txc0000000000894e>",
            Self::ConnectivityIssue => "<txc0000000000894f>",
            Self::OverwatchCoins => "<txc00000000008e02>",
            Self::CompetitivePoints => "<txc0000000000906e>",
            Self::SombraStealth => "<txc0000000000a6b6>",
            Self::ExplosionDark => "<txc0000000000a824>",
            Self::TargetWithCheckmarkDark => "<txc0000000000a825>",
            Self::FireDark => "<txc0000000000a826>",
            Self::HealingPlusDark => "<txc0000000000a827>",
            Self::TargetWithPersonDark => "<txc0000000000a82a>",
            Self::HalloweenTerror => "<txc0000000000abab>",
            Self::BlizzardSpecial => "<txc0000000000abac>",
            Self::SummerGames => "<txc0000000000abad>",
            Self::SombraPassive => "<txc0000000000afbf>",
            Self::TracerIconDark => "<txc0000000000b273>",
            Self::CompetitiveIcon => "<txc0000000000b274>",
            Self::GameBrowserIconDark => "<txc0000000000b275>",
            Self::UnrankedIconDark => "<txc0000000000b277>",
            Self::DoomfistPassive => "<txc0000000000f4c2>",
            Self::OverwatchLeagueTokens => "<txc00000000013dcc>",
            Self::BrigittePassive => "<txc000000000144a8>",
            Self::HanzoLunge => "<txc00000000014f0c>",
            Self::PinkMercyHeart => "<txc00000000017112>",
            Self::Workshop => "<txc0000000001f70d>",
            Self::Escort => "<txc00000000020af3>",
            Self::Elimination => "<txc00000000020af4>",
            Self::Deathmatch => "<txc00000000020af5>",
            Self::CaptureTheFlag => "<txc00000000020af6>",
            Self::Control => "<txc00000000020af7>",
            Self::Assault => "<txc00000000020af8>",
            Self::TeamDeathmatch => "<txc00000000020af9>",
            Self::Skirmish => "<txc00000000020afa>",
            Self::Hybrid => "<txc00000000020afb>",
            Self::Push => "<txc000000000215b4>",
            Self::SquareGradientTransparentBlack => "<txc00000000021cc9>",
            Self::FlagIreland => "<txc0000000002201a>",
            Self::FlagArgentina => "<txc0000000002205f>",
            Self::FlagBelgium => "<txc00000000022061>",
            Self::FlagBulgaria => "<txc00000000022062>",
            Self::FlagChile => "<txc00000000022063>",
            Self::FlagColombia => "<txc00000000022064>",
            Self::FlagGreece => "<txc00000000022065>",
            Self::FlagIceland => "<txc00000000022066>",
            Self::FlagIndia => "<txc00000000022067>",
            Self::PracticeRange => "<txc000000000280c0>",
            Self::ExperimentalModeDark => "<txc0000000002a057>",
            Self::Defending => "<txc0000000002a9ec>",
            Self::FallBack => "<txc0000000002a9ef>",
            Self::Go => "<txc0000000002a9f1>",
            Self::GoingIn => "<txc0000000002a9f2>",
            Self::Incoming => "<txc0000000002a9f5>",
            Self::No => "<txc0000000002a9f9>",
            Self::OnMyWay => "<txc0000000002a9fa>",
            Self::PressTheAttack => "<txc0000000002a9fb>",
            Self::PushForward => "<txc0000000002a9fc>",
            Self::Sorry => "<txc0000000002a9fe>",
            Self::WithYou => "<txc0000000002aa00>",
            Self::JunkerQueenPassive => "<txc0000000002be58>",
            Self::MouseCursor => "<txc0000000002dd21>",
            Self::Flashpoint => "<txc000000000320d2>",
            Self::HeroBorder => "<txc0000000003ec4b>",
        }
    }
}
