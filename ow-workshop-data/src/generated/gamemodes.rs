//! Game modes.
use crate::Lang;
use crate::lang::localized;
/// A game mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Gamemode {
    /// `Assault`
    Assault,
    /// `Assault Balanced Overwatch`
    AssaultBalancedOverwatch,
    /// `Bounty Hunter`
    BountyHunter,
    /// `Clash`
    Clash,
    /// `Control`
    Control,
    /// `Control April Fools`
    ControlAprilFools,
    /// `Control Balanced Overwatch`
    ControlBalancedOverwatch,
    /// `Control Community`
    ControlCommunity,
    /// `Capture The Flag`
    CaptureTheFlag,
    /// `Elimination`
    Elimination,
    /// `Escort`
    Escort,
    /// `Escort April Fools`
    EscortAprilFools,
    /// `Escort Balanced Overwatch`
    EscortBalancedOverwatch,
    /// `Escort Community`
    EscortCommunity,
    /// `Deathmatch`
    Deathmatch,
    /// `Flashpoint`
    Flashpoint,
    /// `Flashpoint April Fools`
    FlashpointAprilFools,
    /// `Freezethaw Elimination`
    FreezethawElimination,
    /// `Hybrid`
    Hybrid,
    /// `Hybrid April Fools`
    HybridAprilFools,
    /// `Hybrid Balanced Overwatch`
    HybridBalancedOverwatch,
    /// `Hybrid Community`
    HybridCommunity,
    /// `Junkenstein's Revenge`
    Junkenstein,
    /// `Lúcioball`
    Lucioball,
    /// `Mei's Snowball Offensive`
    MeisSnowballOffensive,
    /// `Practice Range`
    PracticeRange,
    /// `Push`
    Push,
    /// `Push April Fools`
    PushAprilFools,
    /// `Push Balanced Overwatch`
    PushBalancedOverwatch,
    /// `Push Community`
    PushCommunity,
    /// `Skirmish`
    Skirmish,
    /// `Snowball Deathmatch`
    SnowballFfa,
    /// `Stadium Practice Range`
    StadiumPracticeRange,
    /// `Team Deathmatch`
    TeamDeathmatch,
    /// `Yeti Hunter`
    YetiHunter,
}
impl Gamemode {
    /// Every variant, in declaration order.
    pub const ALL: &'static [Self] = &[
        Self::Assault,
        Self::AssaultBalancedOverwatch,
        Self::BountyHunter,
        Self::Clash,
        Self::Control,
        Self::ControlAprilFools,
        Self::ControlBalancedOverwatch,
        Self::ControlCommunity,
        Self::CaptureTheFlag,
        Self::Elimination,
        Self::Escort,
        Self::EscortAprilFools,
        Self::EscortBalancedOverwatch,
        Self::EscortCommunity,
        Self::Deathmatch,
        Self::Flashpoint,
        Self::FlashpointAprilFools,
        Self::FreezethawElimination,
        Self::Hybrid,
        Self::HybridAprilFools,
        Self::HybridBalancedOverwatch,
        Self::HybridCommunity,
        Self::Junkenstein,
        Self::Lucioball,
        Self::MeisSnowballOffensive,
        Self::PracticeRange,
        Self::Push,
        Self::PushAprilFools,
        Self::PushBalancedOverwatch,
        Self::PushCommunity,
        Self::Skirmish,
        Self::SnowballFfa,
        Self::StadiumPracticeRange,
        Self::TeamDeathmatch,
        Self::YetiHunter,
    ];
    /// The data key, for debugging.
    pub fn key(self) -> &'static str {
        match self {
            Self::Assault => "assault",
            Self::AssaultBalancedOverwatch => "assaultBalancedOverwatch",
            Self::BountyHunter => "bountyHunter",
            Self::Clash => "clash",
            Self::Control => "control",
            Self::ControlAprilFools => "controlAprilFools",
            Self::ControlBalancedOverwatch => "controlBalancedOverwatch",
            Self::ControlCommunity => "controlCommunity",
            Self::CaptureTheFlag => "ctf",
            Self::Elimination => "elimination",
            Self::Escort => "escort",
            Self::EscortAprilFools => "escortAprilFools",
            Self::EscortBalancedOverwatch => "escortBalancedOverwatch",
            Self::EscortCommunity => "escortCommunity",
            Self::Deathmatch => "ffa",
            Self::Flashpoint => "flashpoint",
            Self::FlashpointAprilFools => "flashpointAprilFools",
            Self::FreezethawElimination => "freezethawElimination",
            Self::Hybrid => "hybrid",
            Self::HybridAprilFools => "hybridAprilFools",
            Self::HybridBalancedOverwatch => "hybridBalancedOverwatch",
            Self::HybridCommunity => "hybridCommunity",
            Self::Junkenstein => "junkenstein",
            Self::Lucioball => "lucioball",
            Self::MeisSnowballOffensive => "meisSnowballOffensive",
            Self::PracticeRange => "practiceRange",
            Self::Push => "push",
            Self::PushAprilFools => "pushAprilFools",
            Self::PushBalancedOverwatch => "pushBalancedOverwatch",
            Self::PushCommunity => "pushCommunity",
            Self::Skirmish => "skirmish",
            Self::SnowballFfa => "snowballFfa",
            Self::StadiumPracticeRange => "stadiumPracticeRange",
            Self::TeamDeathmatch => "tdm",
            Self::YetiHunter => "yetiHunter",
        }
    }
    /// The name as written in workshop code of the given language.
    pub fn name(self, lang: Lang) -> &'static str {
        static NAMES: [&str; 35usize] = [
            "Angriff\u{1f}Assault\u{1f}Asalto\u{1f}Asalto\u{1f}Attaque\u{1f}Conquista\u{1f}アサルト\u{1f}점령\u{1f}Szturm\u{1f}Ataque\u{1f}Захват точек\u{1f}จ\u{e39}\u{e48}โจม\u{1f}Taarruz\u{1f}攻防作战\u{1f}佔領",
            "\u{1f}Assault Balanced Overwatch",
            "Kopfgeldjäger\u{1f}Bounty Hunter\u{1f}Cazarrecompensas\u{1f}Cazarrecompensas\u{1f}Chasseur de primes\u{1f}Cacciatori di Taglie\u{1f}バウンティ・ハンター\u{1f}현상금 사냥꾼\u{1f}Łowca nagród\u{1f}Caçador de Recompensas\u{1f}Охота за головами\u{1f}น\u{e31}กล\u{e48}าค\u{e48}าห\u{e31}ว\u{1f}Ödül Avcısı\u{1f}赏金猎手\u{1f}賞金獵人",
            "\u{1f}Clash\u{1f}Contienda\u{1f}Batalla\u{1f}\u{1f}Scontro\u{1f}クラッシュ\u{1f}격돌\u{1f}Starcie\u{1f}Embate\u{1f}Битва\u{1f}ปะทะ\u{1f}Kapışma\u{1f}攻防阵线\u{1f}交鋒",
            "Kontrolle\u{1f}Control\u{1f}\u{1f}\u{1f}Point de contrôle\u{1f}Controllo\u{1f}コントロール\u{1f}쟁탈\u{1f}Kontrola\u{1f}Controle\u{1f}Контроль\u{1f}ควบค\u{e38}มพ\u{e37}\u{e49}นท\u{e35}\u{e48}\u{1f}Kontrol\u{1f}占领要点\u{1f}控制",
            "\u{1f}Control April Fools",
            "\u{1f}Control Balanced Overwatch",
            "\u{1f}Control Community",
            "Flaggeneroberung\u{1f}Capture The Flag\u{1f}Captura la bandera\u{1f}Captura la bandera\u{1f}Capture du drapeau\u{1f}Cattura la Bandiera\u{1f}キャプチャー・ザ・フラッグ\u{1f}깃발 뺏기\u{1f}Zdobywanie flagi\u{1f}Capture a Bandeira\u{1f}Захват флага\u{1f}ช\u{e34}งธง\u{1f}Bayrak Kapmaca\u{1f}勇夺锦旗\u{1f}搶旗",
            "Eliminierung\u{1f}Elimination\u{1f}Eliminación\u{1f}Eliminación\u{1f}Élimination\u{1f}Eliminazione\u{1f}エリミネーション\u{1f}섬멸전\u{1f}Eliminacja\u{1f}Eliminação\u{1f}Ликвидация\u{1f}อ\u{e34}ล\u{e34}ม\u{e34}เนช\u{e31}น\u{1f}Bertaraf\u{1f}决斗先锋\u{1f}鬥陣對決",
            "Eskorte\u{1f}Escort\u{1f}Escolta\u{1f}Escolta\u{1f}Escorte\u{1f}Trasporto\u{1f}エスコート\u{1f}호위\u{1f}Eskorta\u{1f}Escolta\u{1f}Сопровождение\u{1f}ค\u{e38}\u{e49}มก\u{e31}น\u{1f}Koruma\u{1f}运载目标\u{1f}護送",
            "\u{1f}Escort April Fools",
            "\u{1f}Escort Balanced Overwatch",
            "\u{1f}Escort Community",
            "\u{1f}Deathmatch\u{1f}Combate a muerte\u{1f}Combate a muerte\u{1f}Combat à mort\u{1f}\u{1f}デスマッチ\u{1f}데스매치\u{1f}\u{1f}Combate até a Morte\u{1f}Схватка\u{1f}เดธแมตช\u{e4c}\u{1f}Ölüm Maçı\u{1f}死斗\u{1f}死鬥",
            "\u{1f}Flashpoint\u{1f}Operación\u{1f}Punto crítico\u{1f}Point chaud\u{1f}Punto Critico\u{1f}フラッシュポイント\u{1f}플래시포인트\u{1f}Punkt Krytyczny\u{1f}Ponto de Tumulto\u{1f}Точка возгорания\u{1f}แย\u{e48}งช\u{e34}งพ\u{e37}\u{e49}นท\u{e35}\u{e48}\u{1f}Akın Noktası\u{1f}闪点作战\u{1f}閃擊點",
            "\u{1f}Flashpoint April Fools",
            "Frosttau-Eliminierung\u{1f}Freezethaw Elimination\u{1f}Congelación fatal\u{1f}Eliminación por congelamiento\u{1f}Congélimination\u{1f}Eliminazione Congelata\u{1f}フリーズソー・エリミネーション\u{1f}얼음땡 섬멸전\u{1f}Mroźna Eliminacja\u{1f}Eliminação Descongelante\u{1f}Ледяная ликвидация\u{1f}ฟร\u{e35}ซธอว\u{e4c}อ\u{e34}ล\u{e34}ม\u{e34}เนช\u{e31}น\u{1f}Dondur-Çözdür Bertaraf\u{1f}融冰决斗\u{1f}凍凍大作戰",
            "\u{1f}Hybrid\u{1f}Híbrido\u{1f}Híbrido\u{1f}Hybride\u{1f}Ibrida\u{1f}ハイブリッド\u{1f}혼합\u{1f}Hybryda\u{1f}Híbrido\u{1f}Гибридный режим\u{1f}ผสม\u{1f}Hibrit\u{1f}攻击护送\u{1f}混合",
            "\u{1f}Hybrid April Fools",
            "\u{1f}Hybrid Balanced Overwatch",
            "\u{1f}Hybrid Community",
            "Junkensteins Rache\u{1f}Junkenstein's Revenge\u{1f}La venganza de Junkenstein\u{1f}La venganza de Junkenstein\u{1f}La vengeance du Dr Schakalstein\u{1f}La vendetta di Junkenstein\u{1f}ジャンケン<br>シュタインの<br>復讐\u{1f}정켄슈타인의 복수\u{1f}Zemsta dr. Złomensteina\u{1f}A vingança de Junkenstein\u{1f}Месть Крысенштейна\u{1f}การแก\u{e49}แค\u{e49}นของจ\u{e31}งค\u{e4c}เค\u{e47}นสไตน\u{e4c}\u{1f}Junkenstein'ın İntikamı\u{1f}怪鼠复仇\u{1f}鼠肯斯坦復仇記",
            "\u{1f}Lúcioball\u{1f}Lúciobol\u{1f}Lúciobol\u{1f}\u{1f}\u{1f}ルシオボール\u{1f}루시우볼\u{1f}Futbolúcio\u{1f}Luciobol\u{1f}Лусиобол\u{1f}\u{1f}\u{1f}动感斗球\u{1f}路西歐競球",
            "Meis Schneeball<hy>schlacht\u{1f}Mei's Snowball Offensive\u{1f}Mei: Operación Bola de Nieve\u{1f}Mei: Operación Bola de Nieve\u{1f}Opération Boule de neige\u{1f}Mei: Operazione Palle di Neve\u{1f}メイの雪合戦\u{1f}메이의 눈싸움 대작전\u{1f}Śnieżkowa Ofensywa Mei\u{1f}Ofensiva de Bola de Neve da Mei\u{1f}Операция «Метелица»\u{1f}ศ\u{e36}กบอลห\u{e34}มะของเหมย\u{1f}Mei'nin Kar Topu Savaşı\u{1f}雪球攻势\u{1f}小美的雪球大作戰",
            "Trainingsbereich\u{1f}Practice Range\u{1f}Práctica de combate\u{1f}Campo de pruebas\u{1f}Champ de tir\u{1f}Zona d'addestramento\u{1f}練習場\u{1f}훈련장\u{1f}Obszar treningowy\u{1f}Campo de Treinamento\u{1f}УЧЕБНЫЙ ПОЛИГОН\u{1f}สนามฝ\u{e36}กซ\u{e49}อม\u{1f}Poligon\u{1f}训练靶场\u{1f}訓練中心",
            "Schub\u{1f}Push\u{1f}Avance\u{1f}Avance\u{1f}Avancée\u{1f}Scorta\u{1f}プッシュ\u{1f}밀기\u{1f}Przepychanka\u{1f}Avanço\u{1f}Натиск\u{1f}ด\u{e31}นของ\u{1f}İttir\u{1f}机动推进\u{1f}推進",
            "\u{1f}Push April Fools",
            "\u{1f}Push Balanced Overwatch",
            "\u{1f}Push Community",
            "Übungsgefecht\u{1f}Skirmish\u{1f}Escaramuza\u{1f}Escaramuza\u{1f}Échauffement\u{1f}Schermaglia\u{1f}スカーミッシュ\u{1f}연습 전투\u{1f}Potyczka\u{1f}Confronto\u{1f}Разминка\u{1f}โหมดปะทะเล\u{e47}กน\u{e49}อย\u{1f}Müsademe\u{1f}突击模式\u{1f}衝突戰",
            "Schneeball-Deathmatch\u{1f}Snowball Deathmatch\u{1f}Pelea de bolas de nieve a muerte\u{1f}Combate a muerte Bola de Nieve\u{1f}Combat à mort de boules de neige\u{1f}Deathmatch Palle di Neve\u{1f}雪合戦（デスマッチ）\u{1f}눈싸움 데스매치\u{1f}Śnieżkowy Deathmatch\u{1f}Combate até a Morte de Bola de Neve\u{1f}Снежная схватка\u{1f}สโนว\u{e4c}บอลเดธแมตช\u{e4c}\u{1f}Kar Topu Ölüm Maçı\u{1f}雪球死斗\u{1f}雪球死鬥大作戰",
            "\u{1f}Stadium Practice Range\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}决斗领域",
            "Team-Deathmatch\u{1f}Team Deathmatch\u{1f}Combate a muerte por equipos\u{1f}Combate a muerte por equipos\u{1f}Combat à mort par équipe\u{1f}Deathmatch a squadre\u{1f}チーム・デスマッチ\u{1f}팀 데스매치\u{1f}Drużynowy Deathmatch\u{1f}Combate até a Morte em Equipe\u{1f}Командная схватка\u{1f}เดธแมตช\u{e4c}ประเภทท\u{e35}ม\u{1f}Takımlı Ölüm Maçı\u{1f}团队死斗\u{1f}團隊死鬥",
            "Yetijagd\u{1f}Yeti Hunter\u{1f}Caza del yeti\u{1f}Cazadora de yetis\u{1f}Chasse au yéti\u{1f}Caccia allo Yeti\u{1f}イエティ・ハント\u{1f}예티 사냥꾼\u{1f}Polowanie na Yeti\u{1f}Caça ao Yeti\u{1f}Охота на йети\u{1f}น\u{e31}กล\u{e48}าเยต\u{e34}\u{1f}Yeti Avcısı\u{1f}雪域狩猎\u{1f}雪怪大作戰",
        ];
        localized(NAMES[self as usize], lang)
    }
}
/// Default player slots per team of a game mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Slots {
    /// Slots in team 1.
    pub team1: Option<u32>,
    /// Slots in team 2.
    pub team2: Option<u32>,
    /// Slots in free-for-all.
    pub ffa: Option<u32>,
}
impl Gamemode {
    /// Whether the mode only exists in Overwatch 1.
    pub fn only_in_ow1(self) -> bool {
        matches!(self, Self::Lucioball)
    }
    /// The mode's default slot counts.
    pub fn default_slots(self) -> Slots {
        Slots {
            team1: match self {
                Self::Elimination => Some(1),
                Self::Lucioball => Some(3),
                Self::TeamDeathmatch => Some(4),
                Self::Assault
                | Self::Clash
                | Self::Control
                | Self::ControlAprilFools
                | Self::ControlCommunity
                | Self::CaptureTheFlag
                | Self::Escort
                | Self::EscortAprilFools
                | Self::EscortCommunity
                | Self::Flashpoint
                | Self::FlashpointAprilFools
                | Self::Hybrid
                | Self::HybridAprilFools
                | Self::HybridCommunity
                | Self::MeisSnowballOffensive
                | Self::Push
                | Self::PushAprilFools
                | Self::PushCommunity
                | Self::Skirmish
                | Self::YetiHunter => Some(5),
                Self::FreezethawElimination | Self::Junkenstein | Self::PracticeRange => {
                    Some(6)
                }
                _ => None,
            },
            team2: match self {
                Self::Junkenstein | Self::PracticeRange => Some(0),
                Self::Elimination | Self::YetiHunter => Some(1),
                Self::Lucioball => Some(3),
                Self::TeamDeathmatch => Some(4),
                Self::Assault
                | Self::Clash
                | Self::Control
                | Self::ControlAprilFools
                | Self::ControlCommunity
                | Self::CaptureTheFlag
                | Self::Escort
                | Self::EscortAprilFools
                | Self::EscortCommunity
                | Self::Flashpoint
                | Self::FlashpointAprilFools
                | Self::Hybrid
                | Self::HybridAprilFools
                | Self::HybridCommunity
                | Self::MeisSnowballOffensive
                | Self::Push
                | Self::PushAprilFools
                | Self::PushCommunity
                | Self::Skirmish => Some(5),
                Self::FreezethawElimination => Some(6),
                _ => None,
            },
            ffa: match self {
                Self::BountyHunter => Some(6),
                Self::Deathmatch | Self::SnowballFfa => Some(12),
                _ => None,
            },
        }
    }
}
