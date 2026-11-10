//! Heroes.
use crate::lang::{Names, localized};
use crate::Lang;
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
    /// The name as written in workshop code of the given language.
    pub fn name(self, lang: Lang) -> &'static str {
        static NAMES: [&str; 54usize] = [
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
        ];
        localized(NAMES[self as usize], lang)
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
    pub fn abilities(self) -> &'static [(Ability, Names)] {
        match self {
            Self::Ana => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Betäubungspfeil\u{1f}Sleep Dart\u{1f}Dardo sedante\u{1f}Dardo tranquilizante\u{1f}Fléchette hypodermique\u{1f}Dardo Soporifero\u{1f}スリープ・ダーツ\u{1f}수면총\u{1f}Strzałka Usypiająca\u{1f}Dardo Sonífero\u{1f}Транквилизатор\u{1f}สล\u{e35}ปดาร\u{e4c}ท\u{1f}Uyku İğnesi\u{1f}麻醉镖\u{1f}睡眠鏢",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Biotische Granate\u{1f}Biotic Grenade\u{1f}Granada biótica\u{1f}Granada biótica\u{1f}Grenade biotique\u{1f}Granata Biotica\u{1f}バイオティック・グレネード\u{1f}생체 수류탄\u{1f}Granat Biotyczny\u{1f}Granada Biótica\u{1f}Биотическая граната\u{1f}ไบโอต\u{e34}กเกรเนด\u{1f}Biyotik Bomba\u{1f}生物手雷\u{1f}生化手榴彈",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Nanoboost\u{1f}Nano Boost\u{1f}Nanoestimulantes\u{1f}Nanopotenciamiento\u{1f}Nanoboost\u{1f}Biostimolatore\u{1f}ナノ・ブースト\u{1f}나노 강화제\u{1f}Nanowzmocnienie\u{1f}Estimulante\u{1f}Стимулятор\u{1f}นาโนบ\u{e39}สต\u{e4c}\u{1f}Nano Takviye\u{1f}纳米激素\u{1f}奈米強化",
                        ),
                    ),
                ]
            }
            Self::Anran => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "\u{1f}Inferno Rush\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}怒炎冲",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "\u{1f}Dancing Blaze\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}熠闪舞",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}Vermillion Ascent\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}朱羽焚",
                        ),
                    ),
                ]
            }
            Self::Ashe => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Doppelflinte\u{1f}Coach Gun\u{1f}Recortada\u{1f}Escopeta de dos cañones\u{1f}Canon scié\u{1f}Doppietta\u{1f}コーチ・ガン\u{1f}충격 샷건\u{1f}Obrzyn\u{1f}Escopeta\u{1f}Обрез\u{1f}โค\u{e49}ชก\u{e31}น\u{1f}Çifte Kırma\u{1f}短筒猎枪\u{1f}雙管散彈槍",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Dynamit\u{1f}Dynamite\u{1f}Dinamita\u{1f}Dinamita\u{1f}\u{1f}Dinamite\u{1f}ダイナマイト\u{1f}다이너마이트\u{1f}Dynamit\u{1f}Dinamite\u{1f}Динамит\u{1f}ไดนาไมต\u{e4c}\u{1f}Dinamit\u{1f}延时雷管\u{1f}炸藥",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}B.O.B.\u{1f}BOB\u{1f}\u{1f}\u{1f}\u{1f}B.O.B\u{1f}\u{1f}\u{1f}\u{1f}БОБ\u{1f}\u{1f}\u{1f}召唤鲍勃\u{1f}鮑伯",
                        ),
                    ),
                ]
            }
            Self::Baptiste => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Regenerativer Schub\u{1f}Regenerative Burst\u{1f}Energía regeneradora\u{1f}Energía regeneradora\u{1f}Salve régénérante\u{1f}Impulso Rigenerativo\u{1f}リジェネ・バースト\u{1f}치유 파동\u{1f}Impuls Regeneracyjny\u{1f}Pulso Regenerador\u{1f}Стимулятор регенерации\u{1f}ร\u{e35}เจนเนอเรท\u{e35}ฟเบ\u{e34}ร\u{e4c}สต\u{e4c}\u{1f}Canlandırıcı Parlama\u{1f}愈合冲击\u{1f}癒傷波動",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Lebenserhaltungsfeld\u{1f}Immortality Field\u{1f}Campo de inmortalidad\u{1f}Campo de inmortalidad\u{1f}Champ d’immortalité\u{1f}Campo d'Immortalità\u{1f}イモータリティ・フィールド\u{1f}불사 장치\u{1f}Pole Nieśmiertelności\u{1f}Campo de Imortalidade\u{1f}Поле бессмертия\u{1f}อ\u{e34}มมอร\u{e4c}แทล\u{e34}ท\u{e35}ฟ\u{e34}ลด\u{e4c}\u{1f}Ölümsüzlük Sahası\u{1f}维生力场\u{1f}維生力場",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Verstärkungsmatrix\u{1f}Amplification Matrix\u{1f}Matriz amplificadora\u{1f}Matriz de amplificación\u{1f}Matrice amplificatrice\u{1f}Matrice di Amplificazione\u{1f}アンプリフィケーション・マトリックス\u{1f}증폭 매트릭스\u{1f}Matryca Wzmacniająca\u{1f}Matriz Amplificadora\u{1f}Усиливающая матрица\u{1f}แอมพล\u{e34}ฟ\u{e34}เคช\u{e31}นเมทร\u{e34}กซ\u{e4c}\u{1f}Kuvvetlendirme Matrisi\u{1f}增幅矩阵\u{1f}增幅矩陣",
                        ),
                    ),
                ]
            }
            Self::Bastion => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Rekonfigurieren\u{1f}Reconfigure\u{1f}Reconfiguración\u{1f}Reconfiguración\u{1f}Reconfiguration\u{1f}Riconfigurazione\u{1f}モード変更\u{1f}재설정\u{1f}Rekonfiguracja\u{1f}Reconfigurar\u{1f}Смена режима\u{1f}ร\u{e35}คอนฟ\u{e34}เกอร\u{e4c}\u{1f}Yapılandırma\u{1f}切换模式\u{1f}切換模式",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Taktische Granate A-36\u{1f}A-36 Tactical Grenade\u{1f}Granada táctica A-36\u{1f}Granada táctica A-36\u{1f}Grenade tactique A-36\u{1f}Granata Tattica A-36\u{1f}A-36タクティカル・グレネード\u{1f}A-36 전술 수류탄\u{1f}Granat Taktyczny A-36\u{1f}Granada Tática A-36\u{1f}Тактическая граната A-36\u{1f}A-36 แทคต\u{e34}คอลเกรเนด\u{1f}A-36 Taktiksel Bomba\u{1f}A-36战术榴弹\u{1f}A-36戰術榴彈",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Konfiguration: Panzer\u{1f}Configuration: Artillery\u{1f}Modo artillería\u{1f}Configuración: tanque\u{1f}Configuration : artillerie\u{1f}Assetto: Artiglieria\u{1f}砲台モード\u{1f}설정: 포격\u{1f}Konfiguracja: Czołg\u{1f}Configuração: Artilharia\u{1f}Режим артиллерии\u{1f}คอนฟ\u{e34}เกอเรช\u{e31}น: อาร\u{e4c}ท\u{e34}ลเลอร\u{e35}\u{1f}Biçim: Top Atışı\u{1f}火炮模式\u{1f}火砲模式",
                        ),
                    ),
                ]
            }
            Self::Brigitte => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Flegelstoß\u{1f}Whip Shot\u{1f}Lanzamiento de mangual\u{1f}Azote de mangual\u{1f}Fléau cinglant\u{1f}Frustata\u{1f}ウィップ・ショット\u{1f}도리깨 투척\u{1f}Trzaśnięcie\u{1f}Disparo Açoitador\u{1f}Хлесткий удар\u{1f}ว\u{e34}ปช\u{e47}อต\u{1f}Zincir Kamçı\u{1f}流星飞锤\u{1f}懲擊",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Reparaturset\u{1f}Repair Pack\u{1f}Kit de reparación\u{1f}Kit de reparación\u{1f}Module de réparation\u{1f}Modulo Riparatore\u{1f}リペア・パック\u{1f}수리 팩\u{1f}Pakiet Regeneracyjny\u{1f}Kit de Reparos\u{1f}Ремкомплект\u{1f}ร\u{e35}แพร\u{e4c}แพ\u{e47}ค\u{1f}Tamir Çantası\u{1f}恢复包\u{1f}修復包",
                        ),
                    ),
                    (
                        Ability::Ability3,
                        Names(
                            "Schildstoß\u{1f}Shield Bash\u{1f}Carga con escudo\u{1f}Azote de escudo\u{1f}Charge de bouclier\u{1f}Colpo di Scudo\u{1f}シールド・バッシュ\u{1f}방패 밀쳐내기\u{1f}Rąbnięcie Tarczą\u{1f}Golpe de Escudo\u{1f}Удар щитом\u{1f}ช\u{e34}ลด\u{e4c}แบช\u{1f}Kalkan Darbı\u{1f}能量盾击\u{1f}持盾衝鋒",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Barrierenschild\u{1f}Barrier Shield\u{1f}Escudo barrera\u{1f}Escudo barrera\u{1f}Bouclier-écran\u{1f}Scudo Protettivo\u{1f}バリア・シールド\u{1f}방벽 방패\u{1f}Tarcza Osłonowa\u{1f}Barreira de Escudo\u{1f}Персональный барьер\u{1f}บาเร\u{e35}ยช\u{e34}ลด\u{e4c}\u{1f}Kalkan Bariyeri\u{1f}屏障护盾\u{1f}屏障防盾",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Inbrunst\u{1f}Rally\u{1f}Formación\u{1f}Motivación\u{1f}Ralliement\u{1f}Adunata\u{1f}ラリー\u{1f}집결\u{1f}Natchnienie\u{1f}Reagrupar\u{1f}Общий сбор\u{1f}แรลล\u{e35}\u{e48}\u{1f}Sancaktar\u{1f}集结号令\u{1f}全軍集結",
                        ),
                    ),
                ]
            }
            Self::Cassidy => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Steppenrolle\u{1f}Combat Roll\u{1f}Evasión\u{1f}Voltereta de combate\u{1f}Roulade\u{1f}Capriola Tattica\u{1f}コンバット・ロール\u{1f}구르기\u{1f}Przewrót Bojowy\u{1f}Rolamento\u{1f}Кувырок\u{1f}คอมแบทโรล\u{1f}Düello Manevrası\u{1f}战术翻滚\u{1f}戰術翻滾",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Blendgranate\u{1f}Flashbang\u{1f}Granada cegadora\u{1f}Granada aturdidora\u{1f}Grenade flash\u{1f}Granata Stordente\u{1f}フラッシュバン\u{1f}섬광탄\u{1f}Granat Błyskowy\u{1f}Clarão\u{1f}Светошумовая граната\u{1f}แฟลชแบง\u{1f}Flaş Bombası\u{1f}闪光弹\u{1f}閃光彈",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Revolverheld\u{1f}Deadeye\u{1f}Sin perdón\u{1f}Tiro certero\u{1f}Implacable\u{1f}Scarica di Piombo\u{1f}デッド・アイ\u{1f}황야의 무법자\u{1f}Przycelowanie\u{1f}Tiro Certeiro\u{1f}Меткий стрелок\u{1f}เดดอาย\u{1f}Şahin Göz\u{1f}神射手\u{1f}彈無虛發",
                        ),
                    ),
                ]
            }
            Self::Dmon => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Schubantrieb\u{1f}Propulsors\u{1f}Propulsores\u{1f}Propulsores\u{1f}Propulsion\u{1f}Propulsori\u{1f}ジェット・ドライブ\u{1f}추진기\u{1f}Dopalacze\u{1f}Propulsores\u{1f}Двигатели\u{1f}\u{1f}\u{1f}\u{1f}推進器",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Fusionsrepetierer\u{1f}Fusion Repeater\u{1f}Cañón automático de fusión\u{1f}Cañón automático de fusión\u{1f}Répétiteur à fusion\u{1f}Ripetitore a Fusione\u{1f}フュージョン・リピーター\u{1f}융합 연발총\u{1f}Miotacz Fuzyjny\u{1f}Repetidor de Fusão\u{1f}Атомный пулемет\u{1f}\u{1f}\u{1f}\u{1f}熱核連射砲",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Energiebarriere\u{1f}Power Barrier\u{1f}Barrera de energía\u{1f}Barrera de energía\u{1f}Écran énergétique\u{1f}Barriera Energetica\u{1f}パワー・バリア\u{1f}파워 배리어\u{1f}Bariera Mocy\u{1f}Barreira de Poder\u{1f}Энергетический щит\u{1f}\u{1f}\u{1f}\u{1f}能量屏障",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}Limit Break\u{1f}Ruptura de límites\u{1f}Rompelímites\u{1f}Point de rupture\u{1f}Oltre il Limite\u{1f}リミット・ブレイク\u{1f}한계 돌파\u{1f}Łamanie Granic\u{1f}Quebra de Limites\u{1f}Запредельный удар\u{1f}\u{1f}\u{1f}\u{1f}突破極限",
                        ),
                    ),
                ]
            }
            Self::Doctrine => {
                &[
                    (Ability::Ability1, Names("\u{1f}Shrouded Momentum")),
                    (Ability::Ability2, Names("\u{1f}Invigorating Drones")),
                    (Ability::Ultimate, Names("\u{1f}Deliverance")),
                ]
            }
            Self::Domina => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "\u{1f}Sonic Repulsors\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}音速斥力场\u{1f}音波反斥器",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "\u{1f}Crystal Charge\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}爆能水晶\u{1f}晶體炸彈",
                        ),
                    ),
                    (
                        Ability::Passive,
                        Names(
                            "\u{1f}Reconstruction\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}护盾重构\u{1f}重組",
                        ),
                    ),
                    (
                        Ability::PrimaryFire,
                        Names(
                            "\u{1f}Photon Magnum\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}光子马格南\u{1f}光子麥格農",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "\u{1f}Barrier Array\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}屏障阵列\u{1f}屏障陣列",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}Panopticon\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}全景牢笼\u{1f}全景光牢",
                        ),
                    ),
                ]
            }
            Self::Doomfist => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Faustschild\u{1f}Power Block\u{1f}Bloqueo de poder\u{1f}Bloqueo de poder\u{1f}Parade puissante\u{1f}Baluardo\u{1f}パワー・ブロック\u{1f}파워 블락\u{1f}Mocarny Blok\u{1f}Bloqueio Energético\u{1f}Мощный блок\u{1f}พาวเวอร\u{e4c}บล\u{e47}อค\u{1f}Ağır Ol\u{1f}悍猛格挡\u{1f}強力格擋",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Seismischer Schlag\u{1f}Seismic Slam\u{1f}Golpe sísmico\u{1f}Embate sísmico\u{1f}Onde de choc\u{1f}Slancio Sismico\u{1f}サイズミック・スラム\u{1f}지진 강타\u{1f}Fala Sejsmiczna\u{1f}Abalo Sísmico\u{1f}Дрожь земли\u{1f}ไซสม\u{e34}คสแลม\u{1f}Yer Titreten\u{1f}裂地重拳\u{1f}震地襲",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Dampfhammer\u{1f}Rocket Punch\u{1f}Puño cohete\u{1f}Puño cohete\u{1f}Direct d’enfer\u{1f}Pugno a Reazione\u{1f}ロケット・パンチ\u{1f}로켓 펀치\u{1f}Rakietowy Cios\u{1f}Soco Foguete\u{1f}Реактивный удар\u{1f}ร\u{e47}อกเกตพ\u{e31}นช\u{e4c}\u{1f}Roket Yumruk\u{1f}火箭重拳\u{1f}火箭拳",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Meteorschlag\u{1f}Meteor Strike\u{1f}Meteoro\u{1f}Golpe de meteoro\u{1f}Frappe météore\u{1f}Impatto Devastante\u{1f}メテオ・ストライク\u{1f}파멸의 일격\u{1f}Meteor\u{1f}Impacto Meteoro\u{1f}Удар метеора\u{1f}ม\u{e35}ท\u{e35}ออร\u{e4c}สไตรก\u{e4c}\u{1f}Meteor Vurgunu\u{1f}毁天灭地\u{1f}流星墜",
                        ),
                    ),
                ]
            }
            Self::Dva => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Booster\u{1f}Boosters\u{1f}Impulsores\u{1f}Propulsores\u{1f}Turboréacteurs\u{1f}Retrorazzi\u{1f}ブースター\u{1f}부스터\u{1f}Dopalacze\u{1f}Propulsores\u{1f}Ускорители\u{1f}บ\u{e39}สเตอร\u{e4c}\u{1f}İticiler\u{1f}推进器\u{1f}噴射推進",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Mikroraketen\u{1f}Micro Missiles\u{1f}Micromisiles\u{1f}Micromisiles\u{1f}Micro-missiles\u{1f}Micromissili\u{1f}マイクロ・ミサイル\u{1f}마이크로 미사일\u{1f}Mikro-pociski\u{1f}Micromísseis\u{1f}Микроракеты\u{1f}ไมโครม\u{e34}สไซล\u{e4c}\u{1f}Mikrofüze\u{1f}微型飞弹\u{1f}火箭匣艙",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Defensivmatrix\u{1f}Defense Matrix\u{1f}Matriz de defensa\u{1f}Matriz de defensa\u{1f}Matrice défensive\u{1f}Matrice Difensiva\u{1f}ディフェンス・マトリックス\u{1f}방어 매트릭스\u{1f}Matryca Obronna\u{1f}Matriz de Defesa\u{1f}Защитная матрица\u{1f}ด\u{e35}เฟนซ\u{e4c}เมทร\u{e34}กซ\u{e4c}\u{1f}Savunma Matrisi\u{1f}防御矩阵\u{1f}防護力場",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Selbstzerstörung\u{1f}Self-Destruct\u{1f}Autodestrucción\u{1f}Autodestrucción\u{1f}Autodestruction\u{1f}Autodistruzione\u{1f}自爆\u{1f}자폭\u{1f}Samozniszczenie\u{1f}Autodestruir\u{1f}Самоуничтожение\u{1f}เซลฟ\u{e4c}เดสตร\u{e31}คต\u{e4c}\u{1f}Kendini İmha\u{1f}自毁\u{1f}自爆",
                        ),
                    ),
                ]
            }
            Self::Echo => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Flug\u{1f}Flight\u{1f}Vuelo\u{1f}Vuelo\u{1f}Vol\u{1f}Volo\u{1f}フライト\u{1f}비행\u{1f}Lot\u{1f}Voo\u{1f}Полет\u{1f}ไฟลท\u{e4c}\u{1f}Uçuş\u{1f}飞行\u{1f}飛空",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Gebündelter Strahl\u{1f}Focusing Beam\u{1f}Haz enfocado\u{1f}Rayo de concentración\u{1f}Rayon focalisé\u{1f}Raggio Focalizzante\u{1f}フォーカス・ビーム\u{1f}광선 집중\u{1f}Skupiona Wiązka\u{1f}Feixe Concentrado\u{1f}Направленный луч\u{1f}โฟก\u{e31}สซ\u{e34}งบ\u{e35}ม\u{1f}Yoğun Işın\u{1f}聚焦光线\u{1f}聚能光束",
                        ),
                    ),
                    (
                        Ability::Passive,
                        Names(
                            "Gleitflug\u{1f}Glide\u{1f}Planeo\u{1f}Planeo\u{1f}Planeur\u{1f}Planata\u{1f}グライド\u{1f}활공\u{1f}Szybowanie\u{1f}Planar\u{1f}Планирование\u{1f}ไกลด\u{e4c}\u{1f}Süzül\u{1f}滑翔\u{1f}滑翔",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Haftbomben\u{1f}Sticky Bombs\u{1f}Bombas lapa\u{1f}Bombas pegajosas\u{1f}Bombes collantes\u{1f}Bombe Adesive\u{1f}スティッキー・ボム\u{1f}점착 폭탄\u{1f}Bomby Przyczepne\u{1f}Bombas Aderentes\u{1f}Бомбы-липучки\u{1f}สต\u{e34}กก\u{e35}\u{e49}บอมบ\u{e4c}\u{1f}Yapışkan Bomba\u{1f}黏性炸弹\u{1f}黏性炸彈",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Duplikat\u{1f}Duplicate\u{1f}Duplicado\u{1f}Copia\u{1f}Duplication\u{1f}Duplicazione\u{1f}コピー\u{1f}복제\u{1f}Duplikacja\u{1f}Duplicar\u{1f}Дубликация\u{1f}ด\u{e39}พล\u{e34}เคต\u{1f}Kopyala\u{1f}人格复制\u{1f}複製",
                        ),
                    ),
                ]
            }
            Self::Emre => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "\u{1f}Siphon Blaster\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}虹吸冲击枪\u{1f}虹吸爆能槍",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "\u{1f}Cyber Frag\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}赛博手雷\u{1f}生化手榴彈",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}Override Protocol\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}覆盖协议\u{1f}覆寫協定",
                        ),
                    ),
                ]
            }
            Self::Freja => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "\u{1f}Quick Dash\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}疾冲\u{1f}快速衝刺",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "\u{1f}Updraft\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}上升气流\u{1f}上升氣流",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "\u{1f}Take Aim\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}瞄准射击\u{1f}精確瞄準",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}Bola Shot\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}流星索\u{1f}縛錘射擊",
                        ),
                    ),
                ]
            }
            Self::Genji => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Sturmschlag\u{1f}Swift Strike\u{1f}Corte veloz\u{1f}Golpe ágil\u{1f}Frappe du vent\u{1f}Assalto Rapido\u{1f}風斬り\u{1f}질풍참\u{1f}Szybkie Cięcie\u{1f}Golpe Veloz\u{1f}Молниеносный удар\u{1f}สว\u{e34}ฟต\u{e4c}สไตรก\u{e4c}\u{1f}Çevik Kesik\u{1f}影\u{1f}迅影襲",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Reflektieren\u{1f}Deflect\u{1f}Desvío\u{1f}Desviar\u{1f}Riposte\u{1f}Deviazione\u{1f}木の葉返し\u{1f}튕겨내기\u{1f}Odbicie\u{1f}Desviar\u{1f}Отражение атак\u{1f}ด\u{e35}เฟลคต\u{e4c}\u{1f}Yansıtma\u{1f}闪\u{1f}招架",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Drachenklinge\u{1f}Dragonblade\u{1f}Hoja del dragón\u{1f}Hoja dragón\u{1f}Lame du dragon\u{1f}Spada del Drago\u{1f}龍撃剣\u{1f}용검\u{1f}Smoczy Miecz\u{1f}Lâmina do Dragão\u{1f}Клинок дракона\u{1f}ดราก\u{e49}อนเบลด\u{1f}Ejder Kılıcı\u{1f}斩\u{1f}龍一文字",
                        ),
                    ),
                ]
            }
            Self::Hanzo => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Sonarpfeil\u{1f}Sonic Arrow\u{1f}Flecha sónica\u{1f}Flecha sónica\u{1f}Flèche sonique\u{1f}Freccia Sonica\u{1f}鳴響矢水\u{1f}음파 화살\u{1f}Strzała Soniczna\u{1f}Flecha Sônica\u{1f}Звуковая стрела\u{1f}โซน\u{e34}กแอร\u{e4c}โรว\u{e4c}\u{1f}Sonik Ok\u{1f}音\u{1f}音波箭",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Sturmpfeile\u{1f}Storm Arrows\u{1f}Tormenta de flechas\u{1f}Tormenta de flechas\u{1f}Rafale\u{1f}Tempesta di Frecce\u{1f}電光石火\u{1f}폭풍 화살\u{1f}Strzały Burzowe\u{1f}Flechas da Tempestade\u{1f}Шквал\u{1f}สตอร\u{e4c}มแอร\u{e4c}โรว\u{e4c}\u{1f}Fırtına Okları\u{1f}岚\u{1f}暴風箭",
                        ),
                    ),
                    (
                        Ability::Ability3,
                        Names(
                            "Ninjasprung\u{1f}Lunge\u{1f}Agilidad de dragón\u{1f}Zancada\u{1f}Impulsion\u{1f}Balzo\u{1f}体術\u{1f}이단뛰기\u{1f}Wypad\u{1f}Salto\u{1f}Выпад\u{1f}ล\u{e31}นจ\u{e4c}\u{1f}Fırla\u{1f}跃\u{1f}突進跳躍",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Drachenschlag\u{1f}Dragonstrike\u{1f}Ataque del dragón\u{1f}Golpe dragón\u{1f}Frappe du dragon\u{1f}Assalto del Drago\u{1f}龍撃波\u{1f}용의 일격\u{1f}Smoczy Strzał\u{1f}Golpe do Dragão\u{1f}Удар дракона\u{1f}ดราก\u{e49}อนสไตรก\u{e4c}\u{1f}İkiz Ejder\u{1f}竜\u{1f}龍魂擊",
                        ),
                    ),
                ]
            }
            Self::Hazard => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Sturzsprung\u{1f}Violent Leap\u{1f}Salto violento\u{1f}Salto violento\u{1f}Saut violent\u{1f}Balzo Furioso\u{1f}バイオレント・リープ\u{1f}덤벼들기\u{1f}Gwałtowny Skok\u{1f}Salto Brutal\u{1f}Жестокий наскок\u{1f}ไวโอเลนท\u{e4c}ล\u{e35}พ\u{1f}Haşin Atlayış\u{1f}狂跃\u{1f}猛躍",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Zackenwall\u{1f}Jagged Wall\u{1f}Muro afilado\u{1f}Muro de púas\u{1f}Mur dentelé\u{1f}Muro Spinato\u{1f}ジャギー・ウォール\u{1f}가시벽\u{1f}Kolczasta Ściana\u{1f}Barreira Espinhosa\u{1f}Зубчатая стена\u{1f}แจ\u{e47}กด\u{e4c}วอลล\u{e4c}\u{1f}Dişli Duvar\u{1f}尖刺墙\u{1f}鋸齒牆",
                        ),
                    ),
                    (
                        Ability::PrimaryFire,
                        Names(
                            "Knochensporn\u{1f}Bonespur\u{1f}Lanzaespinas\u{1f}Ráfaga de púas\u{1f}Éperon osseux\u{1f}Raffica Acuminata\u{1f}ボーン・スティング\u{1f}본스퍼\u{1f}Drzazga\u{1f}Ossada\u{1f}Шипострел\u{1f}โบนสเปอร\u{e4c}\u{1f}Dikenleyici\u{1f}骨刺\u{1f}骨刺",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Stachelschutz\u{1f}Spike Guard\u{1f}Protección de púas\u{1f}Protección de púas\u{1f}Garde hérissée\u{1f}Guardia Affilata\u{1f}スパイク・ガード\u{1f}날카로운 저항\u{1f}Kolcoobrona\u{1f}Defesa Ouriçada\u{1f}Шипастая оборона\u{1f}สไปค\u{e4c}การ\u{e4c}ด\u{1f}Emniyet Dikeni\u{1f}尖刺护体\u{1f}尖刺防禦",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Vanadiumhagel\u{1f}Downpour\u{1f}Aguacero\u{1f}Diluvio\u{1f}Déluge\u{1f}Diluvio\u{1f}ヘヴィ・レイン\u{1f}가시 소나기\u{1f}Ostre Siepnięcie\u{1f}Dilúvio\u{1f}Ливень\u{1f}ดาวน\u{e4c}พ\u{e31}วร\u{e4c}\u{1f}Sağanak\u{1f}千针雨\u{1f}狂暴針雨",
                        ),
                    ),
                ]
            }
            Self::Illari => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Ausbruch\u{1f}Outburst\u{1f}Erupción\u{1f}Estallido\u{1f}Éruption\u{1f}Scoppio\u{1f}アウトバースト\u{1f}분출\u{1f}Rozbłysk\u{1f}Explosão\u{1f}Вспышка\u{1f}เอาต\u{e4c}เบ\u{e34}ร\u{e4c}สต\u{e4c}\u{1f}Gün Yüzü\u{1f}烈日冲击\u{1f}爆發",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Heilpylon\u{1f}Healing Pylon\u{1f}Helioterapia\u{1f}Torre de sanación\u{1f}Pylône de soin\u{1f}Torretta Curativa\u{1f}ヒーリング・パイロン\u{1f}치유의 태양석\u{1f}Pylon Leczący\u{1f}Obelisco de Cura\u{1f}Станция исцеления\u{1f}ฮ\u{e35}ลล\u{e34}\u{e48}งไพลอน\u{1f}Dispanser\u{1f}治疗光塔\u{1f}治療塔",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Gefangene Sonne\u{1f}Captive Sun\u{1f}Sol cautivo\u{1f}Sol cautivo\u{1f}Soleil accablant\u{1f}Sole Imprigionato\u{1f}キャプティブ・サン\u{1f}태양 작렬\u{1f}Potęga Słońca\u{1f}Sol Aprisionado\u{1f}Пленное солнце\u{1f}แคปท\u{e35}ฟซ\u{e31}น\u{1f}Tutsak Güneş\u{1f}桎梏灼日\u{1f}萬千烈陽",
                        ),
                    ),
                ]
            }
            Self::JetpackCat => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "\u{1f}Lifeline\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}救生索\u{1f}救生索",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "\u{1f}Purr\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}呼噜噜\u{1f}呼嚕",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "\u{1f}Frenetic Flight\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}咻咻飞\u{1f}狂飆飛貓",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}Catnapper\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}猫猫劫\u{1f}貓的綁架",
                        ),
                    ),
                ]
            }
            Self::JunkerQueen => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Befehlsruf\u{1f}Commanding Shout\u{1f}Grito de mando\u{1f}Alarido dominante\u{1f}Cri de commandement\u{1f}Urlo di Comando\u{1f}コマンディング・シャウト\u{1f}지휘의 외침\u{1f}Władczy Okrzyk\u{1f}Grito de Guerra\u{1f}Ободряющий клич\u{1f}คอมมานด\u{e34}งเชาท\u{e4c}\u{1f}Savaş Narası\u{1f}命令怒吼\u{1f}統率怒號",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Gemetzel\u{1f}Carnage\u{1f}Matanza\u{1f}Masacre\u{1f}\u{1f}Mattanza\u{1f}カーネイジ\u{1f}도륙\u{1f}Rzeźnia\u{1f}Carnificina\u{1f}Карнаж\u{1f}คาร\u{e4c}เนจ\u{1f}Kıyım\u{1f}血斩\u{1f}屠殺",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Gezackte Klinge\u{1f}Jagged Blade Gracie\u{1f}Cuchilla dentada\u{1f}Hoja dentada\u{1f}Lame crantée\u{1f}Lama Dentata\u{1f}ギザギザブレード\u{1f}톱니칼\u{1f}Postrzępione Ostrze\u{1f}Lâmina Serrilhada\u{1f}Зазубренный клинок\u{1f}แจ\u{e47}กด\u{e4c}เบลด\u{1f}Dişli Bıçak\u{1f}锯齿利刃\u{1f}鋸刃",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Randale\u{1f}Rampage\u{1f}Masacre\u{1f}Desenfreno\u{1f}Massacre\u{1f}Furia\u{1f}ランペイジ\u{1f}살육\u{1f}Rozwałka\u{1f}Chacina\u{1f}Буйство\u{1f}แรมเพจ\u{1f}Gözükara\u{1f}轰翻天\u{1f}瘋狂殺戮",
                        ),
                    ),
                ]
            }
            Self::Junkrat => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Ferngezündete Mine\u{1f}Concussion Mine\u{1f}Mina de conmoción\u{1f}Mina de conmoción\u{1f}Mine incapacitante\u{1f}Mina Dirompente\u{1f}コンカッション・マイン\u{1f}충격 지뢰\u{1f}Mina Ogłuszająca\u{1f}Mina de Concussão\u{1f}Фугасная мина\u{1f}คอนค\u{e31}สช\u{e31}นไมน\u{e4c}\u{1f}Sarsıntı Mayını\u{1f}震荡地雷\u{1f}震盪地雷",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Stahlfalle\u{1f}Steel Trap\u{1f}Cepo\u{1f}Trampa de acero\u{1f}Piège d’acier\u{1f}Tagliola\u{1f}スティール・トラップ\u{1f}강철 덫\u{1f}Potrzask\u{1f}Armadilha de Aço\u{1f}Капкан\u{1f}สต\u{e35}ลแทร\u{e47}ป\u{1f}Çelik Kapan\u{1f}捕兽夹\u{1f}鋼鐵陷阱",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Kamikazereifen\u{1f}RIP-Tire\u{1f}Rueda explosiva\u{1f}Neumático <br>desgarrador\u{1f}Pneumastic\u{1f}Rotobomba\u{1f}RIPタイヤ\u{1f}죽이는 타이어\u{1f}Mord-Opona\u{1f}Pneu da Morte\u{1f}Адская шина\u{1f}ร\u{e34}ปไทร\u{e4c}\u{1f}Bas Gaza\u{1f}炸弹轮胎\u{1f}地獄飛輪",
                        ),
                    ),
                ]
            }
            Self::Juno => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Gleitboost\u{1f}Glide Boost\u{1f}Impulso de planeo\u{1f}Potenciamiento de planeo\u{1f}Vol plané\u{1f}Planata Potenziata\u{1f}グライド・ブースト\u{1f}글라이드 부스터\u{1f}Szybkie Szybowanie\u{1f}Impulso de Planar\u{1f}Усиление планирования\u{1f}ไกลด\u{e4c}บ\u{e39}สต\u{e4c}\u{1f}Süzülme Desteği\u{1f}滑翔推进\u{1f}滑翔加速",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Hyperring\u{1f}Hyper Ring\u{1f}Hiperanillo\u{1f}Hiperanillo\u{1f}Hyperanneau\u{1f}Anello Sonico\u{1f}ハイパー・リング\u{1f}하이퍼 링\u{1f}Hiperpierścień\u{1f}Aro Turbo\u{1f}Гиперкольцо\u{1f}ไฮเปอร\u{e4c}ร\u{e34}ง\u{1f}Hiper Halka\u{1f}超能环域\u{1f}超能圓環",
                        ),
                    ),
                    (
                        Ability::Passive,
                        Names(
                            "Marsstiefel\u{1f}Martian Overboots\u{1f}Cubrebotas marciano\u{1f}Cubrebotas marcianas\u{1f}Bottes martiennes\u{1f}Stivali Marziani\u{1f}マーシャン・オーバーブーツ\u{1f}화성식 오버부츠\u{1f}Obuwie Marsjańskie\u{1f}Botas Marcianas\u{1f}Марсианские боты\u{1f}มาร\u{e4c}เช\u{e35}ยนโอเวอร\u{e4c}บ\u{e39}ต\u{1f}Marslı Botu\u{1f}火星套靴\u{1f}火星噴射靴",
                        ),
                    ),
                    (
                        Ability::PrimaryFire,
                        Names(
                            "\u{1f}Mediblaster\u{1f}Mediarma\u{1f}Medibláster\u{1f}Médiblaster\u{1f}\u{1f}メディブラスター\u{1f}메디블라스터\u{1f}Blaster Medyczny\u{1f}Medibláster\u{1f}Медибластер\u{1f}เมด\u{e34}บลาสเตอร\u{e4c}\u{1f}Tıbbi Ateş\u{1f}医疗冲击枪\u{1f}醫療光槍",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Neutronentorpedos\u{1f}Pulsar Torpedoes\u{1f}Torpedos de púlsar\u{1f}Torpedos de púlsar\u{1f}Torpilles à neutrons\u{1f}Siluri Pulsar\u{1f}パルサー・トーピード\u{1f}펄사 어뢰\u{1f}Torpedy Pulsarowe\u{1f}Torpedos de Pulsar\u{1f}Пульсарные торпеды\u{1f}พ\u{e31}ลซาร\u{e4c}ตอร\u{e4c}ป\u{e34}โด\u{1f}Pulsar Torpidoları\u{1f}脉冲星飞雷\u{1f}脈衝星魚雷",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Orbitalstrahl\u{1f}Orbital Ray\u{1f}Rayo orbital\u{1f}Rayo orbital\u{1f}Rayon orbital\u{1f}Flusso Cosmico\u{1f}オービタル・レイ\u{1f}궤도 광선\u{1f}Promień Orbitalny\u{1f}Raio Orbital\u{1f}Орбитальный луч\u{1f}ออร\u{e4c}บ\u{e34}ท\u{e31}ลเรย\u{e4c}\u{1f}Yörünge Işını\u{1f}轨道射线\u{1f}軌道射線",
                        ),
                    ),
                ]
            }
            Self::Kiriko => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Schneller Schritt\u{1f}Swift Step\u{1f}Paso ligero\u{1f}Paso ágil\u{1f}Pas véloce\u{1f}Passo Rapido\u{1f}神出鬼没\u{1f}순보\u{1f}Raźny Ruch\u{1f}Passo Rápido\u{1f}Быстрый шаг\u{1f}สว\u{e34}ฟต\u{e4c}สเต\u{e47}ป\u{1f}Tek Adım\u{1f}瞬\u{1f}瞬影步",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Suzu des Schutzes\u{1f}Protection Suzu\u{1f}Suzu de protección\u{1f}Suzu de protección\u{1f}Suzu de protection\u{1f}Suzu di Protezione\u{1f}鈴のご加護\u{1f}정화의 방울\u{1f}Ochronne Suzu\u{1f}Suzu de Proteção\u{1f}Талисман защиты\u{1f}โพรเทคช\u{e31}นซ\u{e39}ซ\u{e38}\u{1f}Suzu Zili\u{1f}铃\u{1f}祓除鈴",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Kitsunami\u{1f}Kitsune Rush\u{1f}Marcha del kitsune\u{1f}Arremetida de kitsune\u{1f}Ruée de kitsune\u{1f}Scatto Kitsune\u{1f}狐走り\u{1f}여우길\u{1f}Pęd Kitsune\u{1f}Corrida da Kitsune\u{1f}Рывок Кицунэ\u{1f}ค\u{e34}ตส\u{e36}เนะร\u{e31}ช\u{1f}Atıl Kitsune\u{1f}狐\u{1f}御狐衝刺",
                        ),
                    ),
                ]
            }
            Self::Lifeweaver => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Regenerativer Spurt\u{1f}Rejuvenating Dash\u{1f}Paso regenerador\u{1f}Carrera revitalizante\u{1f}Élan régénérant\u{1f}Scatto Rinvigorente\u{1f}若返りの風\u{1f}산들 걸음\u{1f}Ożywczy Pęd\u{1f}Investida Rejuvenescedora\u{1f}Рывок восстановления\u{1f}ร\u{e35}จ\u{e39}ว\u{e35}เนต\u{e34}\u{e49}งแดช\u{1f}Gençleştiren Koşu\u{1f}回春疾行\u{1f}恢復衝刺",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Rettungsanker\u{1f}Life Grip\u{1f}Agarre vital\u{1f}Agarre salvavidas\u{1f}Emprise salvatrice\u{1f}Presa Vitale\u{1f}ライフグリップ\u{1f}구원의 손길\u{1f}Pomocna Dłoń\u{1f}Abraço Vital\u{1f}Хватка жизни\u{1f}ไลฟ\u{e4c}กร\u{e34}ป\u{1f}Can Halatı\u{1f}生命之握\u{1f}賦生之握",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Blütenplattform\u{1f}Petal Platform\u{1f}Plataforma petaloide\u{1f}Plataforma de pétalos\u{1f}Plateforme de pétales\u{1f}Piattaforma di Petali\u{1f}ペタル・プラットフォーム\u{1f}연꽃 단상\u{1f}Platforma Płatkowa\u{1f}Plataforma de Pétalas\u{1f}Платформа-лепесток\u{1f}เพตเท\u{e34}ลแพลตฟอร\u{e4c}ม\u{1f}Baş Tacı\u{1f}花瓣平台\u{1f}花瓣平台",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Baum des Lebens\u{1f}Tree of Life\u{1f}Árbol de vida\u{1f}Árbol de la vida\u{1f}Arbre de vie\u{1f}Albero della Vita\u{1f}命の樹\u{1f}생명의 나무\u{1f}Drzewo Życia\u{1f}Árvore da Vida\u{1f}Древо Жизни\u{1f}ทร\u{e35}ออฟไลฟ\u{e4c}\u{1f}Şifa Ağacı\u{1f}生命之树\u{1f}生命之樹",
                        ),
                    ),
                ]
            }
            Self::Lucio => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "\u{1f}Crossfade\u{1f}Cambio de pista\u{1f}Mezclador\u{1f}\u{1f}Vibrazioni\u{1f}クロスフェード\u{1f}분위기 전환!\u{1f}Przejście\u{1f}Transição\u{1f}Кроссфейд\u{1f}ครอสเฟด\u{1f}Sonraki Parça\u{1f}切歌\u{1f}切換曲目",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Aufdrehen\u{1f}Amp It Up\u{1f}Subidón\u{1f}Sube el volumen\u{1f}Volume max.\u{1f}Volume al Massimo\u{1f}アンプ・イット・アップ\u{1f}볼륨을 높여라!\u{1f}Potencjometr\u{1f}Solta o Som\u{1f}Громкость на полную!\u{1f}แอมป\u{e4c}อ\u{e34}ทอ\u{e31}พ\u{1f}Son Ses\u{1f}强音\u{1f}音量全開",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Schallwelle\u{1f}Soundwave\u{1f}Onda sonora\u{1f}Onda sonora\u{1f}Onde sonore\u{1f}Onda d'Urto\u{1f}サウンドウェーブ\u{1f}소리 파동\u{1f}Fala Dźwiękowa\u{1f}Onda Sonora\u{1f}Звуковая волна\u{1f}ซาวนด\u{e4c}เวฟ\u{1f}Ses Dalgası\u{1f}音波\u{1f}音浪衝擊",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Soundbarriere\u{1f}Sound Barrier\u{1f}Barrera de sonido\u{1f}Barrera de sonido\u{1f}Mur de son\u{1f}Barriera Sonora\u{1f}サウンド・バリア\u{1f}소리 방벽\u{1f}Bariera Dźwiękowa\u{1f}Barreira de Som\u{1f}Звуковой барьер\u{1f}ซาวนด\u{e4c}บาเร\u{e35}ย\u{1f}Ses Bariyeri\u{1f}音障\u{1f}音波屏障",
                        ),
                    ),
                ]
            }
            Self::Mauga => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Tackle\u{1f}Overrun\u{1f}Arrollar\u{1f}Embestida\u{1f}Déferlement\u{1f}Sovrastazione\u{1f}オーバーラン\u{1f}돌파\u{1f}Taran\u{1f}Encurralar\u{1f}Напор\u{1f}โอเวอร\u{e4c}ร\u{e31}น\u{1f}Aşım\u{1f}蛮力冲撞\u{1f}暴走",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Overdrive\u{1f}Cardiac Overdrive\u{1f}Sobreestimulación cardíaca\u{1f}Sobrecarga cardíaca\u{1f}Cent à l’heure\u{1f}Sovraccarico cardiaco\u{1f}カーディアック・オーバードライブ\u{1f}터질 듯한 심장\u{1f}Przeciążenie Sercowe\u{1f}Sobrecarga Cardíaca\u{1f}Разгон сердцебиения\u{1f}คาร\u{e4c}ด\u{e34}แอคโอเวอร\u{e4c}ไดรฟ\u{e4c}\u{1f}Taşikardi\u{1f}心脏过载\u{1f}心率超載",
                        ),
                    ),
                    (
                        Ability::PrimaryFire,
                        Names(
                            "Brand-Chain-Gun\u{1f}Incendiary Chaingun\u{1f}Cañón de cadena incendiario\u{1f}Cañón de cadena incendiario\u{1f}Canon mitrailleur incendiaire\u{1f}Pistola a Catena Incendiaria\u{1f}焼夷チェーンガン\u{1f}화염 기관포\u{1f}Zapalające Działko Wielolufowe\u{1f}Metralhadora Incendiária\u{1f}Поджигающий пулемет\u{1f}อ\u{e34}นเซนเด\u{e35}ยร\u{e35}\u{e48}เชนก\u{e31}น\u{1f}Tutuşturan Makineli\u{1f}燃火链式机枪\u{1f}燃焰機關槍",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Explosive Chain Gun\u{1f}Volatile Chaingun\u{1f}Cañón de cadena volátil\u{1f}Cañón de cadena volátil\u{1f}Canon mitrailleur explosif\u{1f}Pistola a Catena Instabile\u{1f}ヴォラタイル・チェーンガン\u{1f}촉발 기관포\u{1f}Wybuchowe Działko Wielolufowe\u{1f}Metralhadora Volátil\u{1f}Взрывной пулемет\u{1f}โวลาไทล\u{e4c}เชนก\u{e31}น\u{1f}Amansız Makineli\u{1f}爆烈链式机枪\u{1f}火爆機關槍",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Käfigkampf\u{1f}Cage Fight\u{1f}Mano a mano\u{1f}Lucha en la jaula\u{1f}Combat en cage\u{1f}Lotta in Gabbia\u{1f}ケージ・ファイト\u{1f}케이지 혈투\u{1f}Walka w Klatce\u{1f}Mano a Mano\u{1f}Бойня в клетке\u{1f}เคจไฟท\u{e4c}\u{1f}Kafes Dövüşü\u{1f}笼中斗\u{1f}鐵籠戰",
                        ),
                    ),
                ]
            }
            Self::Mei => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Kryostase\u{1f}Cryo-Freeze\u{1f}Criónica\u{1f}Criocongelación\u{1f}Cryostase\u{1f}Ibernazione\u{1f}クリオフリーズ\u{1f}급속 빙결\u{1f}Kriostaza\u{1f}Congelamento Criogênico\u{1f}Криостазис\u{1f}ไครโอฟร\u{e35}ซ\u{1f}Kriyojenik\u{1f}急冻\u{1f}冰霜屏障",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Eiswall\u{1f}Ice Wall\u{1f}Muro de hielo\u{1f}Muro de hielo\u{1f}Mur de glace\u{1f}Muro di Ghiaccio\u{1f}アイス・ウォール\u{1f}빙벽\u{1f}Ściana Lodu\u{1f}Muralha de Gelo\u{1f}Ледяная стена\u{1f}ไอซ\u{e4c}วอลล\u{e4c}\u{1f}Buzdan Duvar\u{1f}冰墙\u{1f}冰牆",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}Blizzard\u{1f}Ventisca\u{1f}Ventisca\u{1f}\u{1f}Tormenta\u{1f}ブリザード\u{1f}눈보라\u{1f}Zamieć\u{1f}Nevasca\u{1f}Вьюга\u{1f}บล\u{e34}ซซาร\u{e4c}ด\u{1f}Kar Fırtınası\u{1f}暴雪\u{1f}暴風雪",
                        ),
                    ),
                ]
            }
            Self::Mercy => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Schutzengel\u{1f}Guardian Angel\u{1f}Ángel de la guarda\u{1f}Ángel guardián\u{1f}Ange gardien\u{1f}Angelo Custode\u{1f}ガーディアン・エンジェル\u{1f}수호천사\u{1f}Anioł Stróż\u{1f}Anjo da Guarda\u{1f}Ангел-хранитель\u{1f}การ\u{e4c}เด\u{e35}ยนแองเจ\u{e34}ล\u{1f}Koruyucu Melek\u{1f}守护天使\u{1f}守護天使",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Wiederauferstehung\u{1f}Resurrect\u{1f}Resurrección\u{1f}Resurrección\u{1f}Résurrection\u{1f}Resurrezione\u{1f}リザレクト\u{1f}부활\u{1f}Wskrzeszenie\u{1f}Ressurreição\u{1f}Воскрешение\u{1f}ร\u{e35}เซอร\u{e4c}เร\u{e47}ก\u{1f}Diriltme\u{1f}重生\u{1f}復活",
                        ),
                    ),
                    (
                        Ability::Passive,
                        Names(
                            "Sympathische Erholung\u{1f}Sympathetic Recovery\u{1f}Recuperación compasiva\u{1f}Recuperación empática\u{1f}Guérison favorable\u{1f}Recupero Solidale\u{1f}シンパセティック・リカバリー\u{1f}연민의 치유\u{1f}Leczenie Empatyczne\u{1f}Recuperação Solidária\u{1f}Симпатическое восстановление\u{1f}ซ\u{e34}มพาแทต\u{e34}คร\u{e35}ค\u{e31}ฟเวอร\u{e35}\u{e48}\u{1f}Özgecilik Şifası\u{1f}同情愈疗\u{1f}共鳴恢復",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}Valkyrie\u{1f}Valkiria\u{1f}Valquiria\u{1f}\u{1f}Valchiria\u{1f}ヴァルキリー\u{1f}발키리\u{1f}Walkiria\u{1f}Valquíria\u{1f}Валькирия\u{1f}วาลค\u{e34}ร\u{e35}\u{e48}\u{1f}Valkür\u{1f}女武神\u{1f}女武神",
                        ),
                    ),
                ]
            }
            Self::Mizuki => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "\u{1f}Katashiro Return\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}替魂纸人\u{1f}形代歸返",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "\u{1f}Binding Chain\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}缚魂锁链\u{1f}束縛鎖鏈",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "\u{1f}Healing Kasa\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}疗魂斗笠\u{1f}治癒之笠",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}Kekkai Sanctuary\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}护魂结界\u{1f}結界護佑",
                        ),
                    ),
                ]
            }
            Self::Moira => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Phasensprung\u{1f}Fade\u{1f}Evanescencia\u{1f}Evanescencia\u{1f}Volatilité\u{1f}Dissolvenza\u{1f}フェード\u{1f}소멸\u{1f}Zniknięcie\u{1f}Desvanecer\u{1f}Уход в тень\u{1f}เฟด\u{1f}Kaybol\u{1f}消散\u{1f}消失",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Biotische Sphäre\u{1f}Biotic Orb\u{1f}Orbe biótico\u{1f}Orbe biótico\u{1f}Orbe biotique\u{1f}Globo Biotico\u{1f}バイオティック・オーブ\u{1f}생체 구슬\u{1f}Kula Biotyczna\u{1f}Orbe Biótico\u{1f}Биотическая сфера\u{1f}ไบโอต\u{e34}กออร\u{e4c}บ\u{1f}Biyotik Küre\u{1f}生化之球\u{1f}生化球",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Koaleszenz\u{1f}Coalescence\u{1f}Coalescencia\u{1f}Coalescencia\u{1f}\u{1f}Coalescenza\u{1f}コアレッセンス\u{1f}융화\u{1f}Koalescencja\u{1f}Coalescência\u{1f}Коалесценция\u{1f}โคเอเลสเซนซ\u{e4c}\u{1f}Kaynaşım\u{1f}聚合射线\u{1f}聚合光束",
                        ),
                    ),
                ]
            }
            Self::Orisa => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Defensivprotokoll\u{1f}Fortify\u{1f}Fortificación\u{1f}Fortificar\u{1f}Blindage\u{1f}Fortificazione\u{1f}フォーティファイ\u{1f}방어 강화\u{1f}Umocnienie\u{1f}Fortificar\u{1f}Защитное поле\u{1f}ฟอร\u{e4c}ท\u{e34}ฟาย\u{1f}Tahkim\u{1f}强固防御\u{1f}強化防禦",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Speerwirbel\u{1f}Javelin Spin\u{1f}Girojabalina\u{1f}Giro de lanza\u{1f}Rotation de javelot\u{1f}Giavellotto Rotante\u{1f}ジャベリン・スピン\u{1f}수호의 창\u{1f}Obrót Włócznią\u{1f}Giro do Dardo\u{1f}Вращение копьем\u{1f}จาเวล\u{e34}นสป\u{e34}น\u{1f}Pervane Cirit\u{1f}标枪旋击\u{1f}迴旋標槍",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Energiespeer\u{1f}Energy Javelin\u{1f}Jabalina de energía\u{1f}Jabalina de energía\u{1f}Javelot d’énergie\u{1f}Giavellotto Energetico\u{1f}エネルギー・ジャベリン\u{1f}투창\u{1f}Włócznia Energetyczna\u{1f}Dardo Energético\u{1f}Энергетическое копье\u{1f}เอเนอจ\u{e35}\u{e49}จาเวล\u{e34}น\u{1f}Yüklü Cirit\u{1f}能量标枪\u{1f}能量標槍",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Terrawoge\u{1f}Terra Surge\u{1f}Terraluvión\u{1f}Impacto terrestre\u{1f}Vague Terra\u{1f}Impeto Terrestre\u{1f}テラ・サージ\u{1f}대지의 창\u{1f}Impuls Bojowy\u{1f}Rompante Terrano\u{1f}Встряска земли\u{1f}เทอร\u{e4c}ราเซ\u{e34}ร\u{e4c}จ\u{1f}Yerküre Rüzgârı\u{1f}撼地猛刺\u{1f}大地湧能",
                        ),
                    ),
                ]
            }
            Self::Pharah => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Senkrechtstarter\u{1f}Jump Jet\u{1f}Propulsores\u{1f}Propulsor aéreo\u{1f}Réacteurs\u{1f}Propulsori\u{1f}ジャンプ・ジェット\u{1f}점프 추진기\u{1f}Skok Odrzutowy\u{1f}Salto Foguete\u{1f}Реактивный ранец\u{1f}จ\u{e31}มป\u{e4c}เจ\u{e47}ท\u{1f}Havalanma\u{1f}推进背包\u{1f}跳躍噴射",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Erschütterungs<hy>impuls\u{1f}Concussive Blast\u{1f}Disparo de conmoción\u{1f}Explosión conmocionante\u{1f}Conflagration\u{1f}Scarica Esplosiva\u{1f}コンカッシブ・ブラスト\u{1f}충격탄\u{1f}Przytłaczający Strzał\u{1f}Explosão Concussiva\u{1f}Взрывная волна\u{1f}คอนค\u{e31}สซ\u{e34}ฟบลาสต\u{e4c}\u{1f}Sarsıcı Patlama\u{1f}震荡冲击\u{1f}震盪爆破",
                        ),
                    ),
                    (
                        Ability::Passive,
                        Names(
                            "Schwebedüsen\u{1f}Hover Jets\u{1f}Planeador\u{1f}Propulsores levitantes\u{1f}Stabilisateurs\u{1f}Propulsori\u{1f}ホバー・ジェット\u{1f}호버 추진기\u{1f}Zawiśnięcie\u{1f}Suspensão a Jato\u{1f}Парение\u{1f}โฮเวอร\u{e4c}เจ\u{e47}ท\u{1f}Uçuş Jeti\u{1f}悬浮背包\u{1f}滯空",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Düsenstrahl\u{1f}Jet Dash\u{1f}Impulso\u{1f}Propulsores de impulso\u{1f}Élan propulseur\u{1f}Scatto Jet\u{1f}ジェット・ダッシュ\u{1f}급속 추진\u{1f}Pęd\u{1f}Disparada a Jato\u{1f}Реактивный рывок\u{1f}เจ\u{e47}ทแดช\u{1f}Jet Hamle\u{1f}疾冲背包\u{1f}噴射衝刺",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Trommelfeuer\u{1f}Barrage\u{1f}Bombardeo\u{1f}Bombardeo\u{1f}\u{1f}Pioggia di Fuoco\u{1f}バレッジ\u{1f}포화\u{1f}Salwa\u{1f}Bombardeio\u{1f}Ракетный залп\u{1f}บาร\u{e4c}ราจ\u{1f}Yaylım\u{1f}火箭弹幕\u{1f}火箭彈幕",
                        ),
                    ),
                ]
            }
            Self::Ramattra => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Nemesisform\u{1f}Nemesis Form\u{1f}Forma de némesis\u{1f}Forma de némesis\u{1f}Forme de némésis\u{1f}Forma Nemesi\u{1f}ネメシス・フォーム\u{1f}네메시스 형태\u{1f}Postać Nemezis\u{1f}Forma Nêmesis\u{1f}Форма Немезиды\u{1f}เนเมซ\u{e34}สฟอร\u{e4c}ม\u{1f}Nemesis Formu\u{1f}天罚形态\u{1f}復仇形態",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Gefräßiger Wirbel\u{1f}Ravenous Vortex\u{1f}Vórtice voraz\u{1f}Vórtice voraz\u{1f}Vortex vorace\u{1f}Vortice Insaziabile\u{1f}貪欲な渦\u{1f}탐식의 소용돌이\u{1f}Wściekły Wir\u{1f}Vórtice Devorador\u{1f}Ненасытный вихрь\u{1f}แรฟเวอร\u{e4c}น\u{e31}สวอร\u{e4c}เทกซ\u{e4c}\u{1f}Tükeniş Girdabı\u{1f}吞噬漩涡\u{1f}貪婪漩渦",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Leerenbarriere Omnicform\u{1f}Void Barrier Omnic Form\u{1f}Barrera de vacío Forma de ómnico\u{1f}Barrera del vacío Forma de ómnico\u{1f}Barrière du vide forme d’omniaque\u{1f}Barriera del Vuoto Forma Omnic\u{1f}ヴォイド・バリア（オムニック・フォーム）\u{1f}공허 방벽 옴닉 형태\u{1f}Bariera Otchłani Postać Omnika\u{1f}Barreira do Vazio Forma Ômnica\u{1f}Барьер пустоты форма омника\u{1f}วอยด\u{e4c}บาเร\u{e35}ย ออมน\u{e34}คฟอร\u{e4c}ม\u{1f}Hiçlik Duvarı Omnik Form\u{1f}虚空屏障（智械形态）\u{1f}虛空屏障（智械形態）",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Vernichtung\u{1f}Annihilation\u{1f}Aniquilación\u{1f}Aniquilación\u{1f}\u{1f}Annientamento\u{1f}アナイアレーション\u{1f}절멸\u{1f}Anihilacja\u{1f}Aniquilação\u{1f}Аннигиляция\u{1f}อ\u{e31}นไนฮ\u{e34}เลช\u{e31}น\u{1f}Cankıyım\u{1f}诛\u{1f}殲滅",
                        ),
                    ),
                ]
            }
            Self::Reaper => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Phantom\u{1f}Wraith Form\u{1f}Forma espectral\u{1f}Forma espectral\u{1f}Forme spectrale\u{1f}Forma Spettrale\u{1f}レイス・フォーム\u{1f}망령화\u{1f}Postać Upiora\u{1f}Forma Fantasma\u{1f}Бестелесность\u{1f}เรธฟอร\u{e4c}ม\u{1f}Tayf Formu\u{1f}幽灵形态\u{1f}亡靈形態",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Schattenschritt\u{1f}Shadow Step\u{1f}Paso de las tinieblas\u{1f}Paso sombrío\u{1f}Voie des ombres\u{1f}Passo d'Ombra\u{1f}シャドウ・ステップ\u{1f}그림자 밟기\u{1f}Krok w Cień\u{1f}Passos das Sombras\u{1f}Шаг сквозь тень\u{1f}ชาโดว\u{e4c}สเต\u{e47}ป\u{1f}Gölge Geçişi\u{1f}暗影步\u{1f}幽影傳送",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Totentanz\u{1f}Death Blossom\u{1f}Espiral de muerte\u{1f}Liberación de almas\u{1f}Éclosion mortelle\u{1f}Spirale della Morte\u{1f}デス・ブロッサム\u{1f}죽음의 꽃\u{1f}Wirująca Śmierć\u{1f}Desabrochar da Morte\u{1f}Цветок смерти\u{1f}เดธบลอสซ\u{e31}ม\u{1f}Ölümsaçan\u{1f}死亡绽放\u{1f}死亡綻放",
                        ),
                    ),
                ]
            }
            Self::Reinhardt => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Ansturm\u{1f}Charge\u{1f}Embestida\u{1f}Carga\u{1f}\u{1f}Carica\u{1f}チャージ\u{1f}돌진\u{1f}Szarża\u{1f}Investida\u{1f}Рывок\u{1f}ชาร\u{e4c}จ\u{1f}Hücum\u{1f}冲锋\u{1f}衝鋒",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Feuerschlag\u{1f}Fire Strike\u{1f}Onda de fuego\u{1f}Ataque ígneo\u{1f}Frappe de feu\u{1f}Dardo di Fuoco\u{1f}ファイア・ストライク\u{1f}화염 강타\u{1f}Ogniste Uderzenie\u{1f}Tiro Flamejante\u{1f}Огненный удар\u{1f}ไฟร\u{e4c}สไตรก\u{e4c}\u{1f}Ateş Dalgası\u{1f}烈焰打击\u{1f}烈焰擊",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Barrierenfeld\u{1f}Barrier Field\u{1f}Campo protector\u{1f}Campo de escudo\u{1f}Écran\u{1f}Barriera\u{1f}バリア・フィールド\u{1f}방벽 방패\u{1f}Pole Osłonowe\u{1f}Campo de Barreira\u{1f}Энергетический барьер\u{1f}บาเร\u{e35}ยฟ\u{e34}ลด\u{e4c}\u{1f}Bariyer Alanı\u{1f}屏障力场\u{1f}屏障力場",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Erdstoß\u{1f}Earthshatter\u{1f}Seísmo\u{1f}Quebrantasuelos\u{1f}Choc sismique\u{1f}Schianto Sismico\u{1f}アース・シャター\u{1f}대지분쇄\u{1f}Zdruzgotanie\u{1f}Abalo Terrestre\u{1f}Землетрясение\u{1f}เอ\u{e34}ร\u{e4c}ทแชตเตอร\u{e4c}\u{1f}Dağdeviren\u{1f}裂地猛击\u{1f}地裂擊",
                        ),
                    ),
                ]
            }
            Self::Roadhog => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Abschlepphaken\u{1f}Chain Hook\u{1f}Garfio\u{1f}Gancho con cadena\u{1f}Traquelard\u{1f}Gancio\u{1f}チェイン・フック\u{1f}사슬 갈고리\u{1f}Hak na Łańcuchu\u{1f}Corrente de Gancho\u{1f}Цепной крюк\u{1f}เชนฮ\u{e38}ค\u{1f}Zincir Kanca\u{1f}链钩\u{1f}鎖鍊鉤",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Verschnaufpause\u{1f}Take a Breather\u{1f}Inhalador\u{1f}Toma un respiro\u{1f}Inhalateur\u{1f}Boccata d'Aria\u{1f}テイク・ア・ブリーザー\u{1f}숨 돌리기\u{1f}Sztachnięcie\u{1f}Pegando Fôlego\u{1f}Передышка\u{1f}เทคอะบร\u{e35}เธอร\u{e4c}\u{1f}Bir Soluklan\u{1f}呼吸器\u{1f}吸了再上",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Fleischwolf\u{1f}Whole Hog\u{1f}Juego sucio\u{1f}A todo jabalí\u{1f}Équarrisseur\u{1f}Porcata\u{1f}ホール・ホッグ\u{1f}돼재앙\u{1f}Wieprznięcie\u{1f}Cair Matando\u{1f}Турбосвинство\u{1f}โฮลฮ\u{e47}อก\u{1f}Domuz Çevirme\u{1f}鸡飞狗跳\u{1f}火力全開",
                        ),
                    ),
                ]
            }
            Self::Shion => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "\u{1f}Evade\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}机动闪避",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "\u{1f}Joyride\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}纵情狂飙",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "\u{1f}Execution\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}交叉枪决",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}Satsuriku Spree\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}杀戮狂宴",
                        ),
                    ),
                ]
            }
            Self::Sierra => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "\u{1f}Anchor Drone\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}锚点无人机",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "\u{1f}Tremor Charge\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}震地手雷",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "\u{1f}Tracking Shot\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}追踪弹",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}Trailblazer\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}开路先锋",
                        ),
                    ),
                ]
            }
            Self::Sigma => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Kinetische Konversion\u{1f}Kinetic Grasp\u{1f}Agarre cinético\u{1f}Agarre cinético\u{1f}Captation cinétique\u{1f}Presa Cinetica\u{1f}キネティック・グラスプ\u{1f}키네틱 손아귀\u{1f}Chwyt Kinetyczny\u{1f}Punho Cinético\u{1f}Кинетический захват\u{1f}ค\u{e34}เนต\u{e34}กแกรสป\u{e4c}\u{1f}Kinetik Yakalama\u{1f}动能俘获\u{1f}動能轉化",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Akkretion\u{1f}Accretion\u{1f}Acreción\u{1f}Acreción\u{1f}Concrétion\u{1f}Accrescimento\u{1f}アクリーション\u{1f}강착\u{1f}Akrecja\u{1f}Acreção\u{1f}Аккреция\u{1f}อะคร\u{e35}ช\u{e31}น\u{1f}Yığılım\u{1f}质量吸附\u{1f}重力崩引",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Experimentelle Barriere\u{1f}Experimental Barrier\u{1f}Barrera experimental\u{1f}Barrera experimental\u{1f}Écran expérimental\u{1f}Barriera Sperimentale\u{1f}エクスペリメンタル・バリア\u{1f}실험용 방벽\u{1f}Eksperymentalna Bariera\u{1f}Barreira Experimental\u{1f}Экспериментальный барьер\u{1f}เอ\u{e47}กซ\u{e4c}เพอร\u{e34}เมนท\u{e31}ลบาเร\u{e35}ย\u{1f}Deneysel Bariyer\u{1f}实验屏障\u{1f}實驗屏障",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Gravitationsfluss\u{1f}Gravitic Flux\u{1f}Flujo gravitacional\u{1f}Flujo gravitatorio\u{1f}Flux gravitationnel\u{1f}Flusso Gravitazionale\u{1f}グラビティ・フラックス\u{1f}중력 붕괴\u{1f}Strumień Grawitacyjny\u{1f}Fluxo Gravitacional\u{1f}Гравитационный колодец\u{1f}กราว\u{e34}ท\u{e34}คฟล\u{e31}กซ\u{e4c}\u{1f}Çekimsel Sapma\u{1f}引力乱流\u{1f}重力流",
                        ),
                    ),
                ]
            }
            Self::Sojourn => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Powerslide\u{1f}Power Slide\u{1f}Maniobra evasiva\u{1f}Deslizamiento impulsor\u{1f}Tacle\u{1f}Scivolata Energica\u{1f}パワー・スライド\u{1f}파워 슬라이드\u{1f}Superwślizg\u{1f}Deslizada Enérgica\u{1f}Суперподкат\u{1f}พาวเวอร\u{e4c}สไลด\u{e4c}\u{1f}Roketli Manevra\u{1f}机动滑铲\u{1f}強力滑行",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Disruptionsschuss\u{1f}Disruptor Shot\u{1f}Disparo inmovilizador\u{1f}Disparo disruptivo\u{1f}Tir disruptif\u{1f}Colpo Distruttivo\u{1f}ディスラプター・ショット\u{1f}분열 사격\u{1f}Strzał Zakłócający\u{1f}Disparo Disruptivo\u{1f}Дезинтегрирующий выстрел\u{1f}ด\u{e34}สร\u{e31}ปเตอร\u{e4c}ช\u{e47}อต\u{1f}Ayakbağı Alanı\u{1f}干扰弹\u{1f}擾亂射擊",
                        ),
                    ),
                    (Ability::SecondaryFire, Names("\u{1f}Charged Shot")),
                    (
                        Ability::Ultimate,
                        Names(
                            "Übertaktung\u{1f}Overclock\u{1f}Supercarga\u{1f}\u{1f}Surcadençage\u{1f}\u{1f}オーバークロック\u{1f}오버클럭\u{1f}Przeciążenie\u{1f}Sobrecarga\u{1f}Разгон\u{1f}โอเวอร\u{e4c}คล\u{e47}อค\u{1f}Güç Aşırtma\u{1f}机体超频\u{1f}磁能超頻",
                        ),
                    ),
                ]
            }
            Self::Soldier => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "\u{1f}Sprint\u{1f}\u{1f}\u{1f}\u{1f}Scatto\u{1f}スプリント\u{1f}질주\u{1f}\u{1f}Disparada\u{1f}Спринт\u{1f}สปร\u{e34}นท\u{e4c}\u{1f}Depar\u{1f}疾跑\u{1f}衝刺",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Biotisches Feld\u{1f}Biotic Field\u{1f}Campo biótico\u{1f}Campo biótico\u{1f}Champ biotique\u{1f}Rigeneratore Biotico\u{1f}バイオティック・フィールド\u{1f}생체장\u{1f}Pole Biotyczne\u{1f}Campo Biótico\u{1f}Биотическое поле\u{1f}ไบโอต\u{e34}กฟ\u{e34}ลด\u{e4c}\u{1f}Biyotik Alan\u{1f}生物力场\u{1f}生化力場",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Helix-Raketen\u{1f}Helix Rockets\u{1f}Cohetes hélice\u{1f}Cohetes Helix\u{1f}Roquettes LX\u{1f}Razzi Helix\u{1f}ヘリックス・ロケット\u{1f}나선 로켓\u{1f}Rakiety Helix\u{1f}Foguetes Hélix\u{1f}Ракетный удар\u{1f}ฮ\u{e35}ล\u{e34}กซ\u{e4c}ร\u{e47}อกเกต\u{1f}Helezon Roketleri\u{1f}螺旋飞弹\u{1f}旋風火箭",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Taktisches Visier\u{1f}Tactical Visor\u{1f}Visor táctico\u{1f}Visor táctico\u{1f}Visière tactique\u{1f}Visore Tattico\u{1f}タクティカル・バイザー\u{1f}전술 조준경\u{1f}Wizjer Taktyczny\u{1f}Visor Tático\u{1f}Тактический визор\u{1f}แทคต\u{e34}คอลไวเซอร\u{e4c}\u{1f}Taktiksel Takip\u{1f}战术目镜\u{1f}戰術鎖定",
                        ),
                    ),
                ]
            }
            Self::Sombra => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "\u{1f}Virus\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}ウイルス\u{1f}바이러스\u{1f}Wirus\u{1f}Vírus\u{1f}Урания\u{1f}ไวร\u{e31}ส\u{1f}Virüs\u{1f}病毒\u{1f}電腦病毒",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Translokator\u{1f}Translocator\u{1f}Baliza de translocación\u{1f}Translocalizador\u{1f}Transducteur\u{1f}Rilocatore\u{1f}トランズロケーター\u{1f}위치변환기\u{1f}Translokator\u{1f}Translocador\u{1f}Транслокатор\u{1f}ทรานสโลเคเตอร\u{e4c}\u{1f}Yer Değiştirici\u{1f}位移传动\u{1f}相位轉換器",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Hacken\u{1f}Hack\u{1f}Hackeo\u{1f}Hackeo\u{1f}Piratage\u{1f}Hacking\u{1f}ハック\u{1f}해킹\u{1f}Hakowanie\u{1f}Hackear\u{1f}Взлом\u{1f}แฮ\u{e47}ค\u{1f}\u{1f}黑客入侵\u{1f}駭入",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}EMP\u{1f}PEM\u{1f}PEM\u{1f}IEM\u{1f}\u{1f}\u{1f}\u{1f}Impuls Elektromagnetyczny\u{1f}PEM\u{1f}Импульс\u{1f}\u{1f}\u{1f}电磁脉冲\u{1f}電磁脈衝波",
                        ),
                    ),
                ]
            }
            Self::Symmetra => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Selbstschussanlage\u{1f}Sentry Turret\u{1f}Torreta centinela\u{1f}Torreta centinela\u{1f}Tourelle sentinelle\u{1f}Drone Sentinella\u{1f}セントリー・タレット\u{1f}감시 포탑\u{1f}Wieżyczka Strażnicza\u{1f}Torre de Sentinela \u{1f}Защитная турель\u{1f}เซนทร\u{e35}เทอร\u{e4c}เร\u{e47}ต\u{1f}Nöbetçi Taret\u{1f}哨戒炮\u{1f}哨戒塔",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "\u{1f}Teleporter\u{1f}Teletransportador\n\u{1f}Transportador\u{1f}Téléporteur\u{1f}Televarco\u{1f}テレポーター\u{1f}순간이동기\u{1f}\u{1f}Teletransportador\u{1f}Телепортер\u{1f}เทเลพอร\u{e4c}ต\u{1f}Işınlayıcı\u{1f}传送面板\u{1f}傳送器",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Photonenbarriere\u{1f}Photon Barrier\u{1f}Barrera de fotones\u{1f}Barrera de fotones\u{1f}Écran photonique\u{1f}Barriera Fotonica\u{1f}フォトン・バリア\u{1f}광자 방벽\u{1f}Bariera Fotonowa\u{1f}Barreira de Fótons\u{1f}Фотонный барьер\u{1f}โฟตอนบาเร\u{e35}ย\u{1f}Foton Bariyeri\u{1f}光子屏障\u{1f}光子屏障",
                        ),
                    ),
                ]
            }
            Self::Torbjorn => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Geschütz aufstellen\u{1f}Deploy Turret\u{1f}Desplegar torreta\u{1f}Desplegar torreta\u{1f}Déploiement de tourelle\u{1f}Posiziona Torretta\u{1f}タレット設置\u{1f}포탑 설치\u{1f}Rozstawienie Wieżyczki\u{1f}Posicionar Torre\u{1f}Установка турели\u{1f}ด\u{e35}พลอยเทอร\u{e4c}เร\u{e47}ต\u{1f}Taret Kur\u{1f}部署炮台\u{1f}部署砲塔",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Überladung\u{1f}Overload\u{1f}Sobrecarga\u{1f}Sobrecarga\u{1f}Surcharge\u{1f}Sovraccarico\u{1f}オーバーロード\u{1f}과부하\u{1f}Dopancerzenie\u{1f}Sobrecarga\u{1f}Перегрузка\u{1f}โอเวอร\u{e4c}โหลด\u{1f}Yük Aşırtma\u{1f}热力过载\u{1f}超負荷",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Geschmolzener Kern\u{1f}Molten Core\u{1f}Fusión nuclear\u{1f}Núcleo de magma\u{1f}Cœur de magma\u{1f}Nucleo Ardente\u{1f}モルテン・コア\u{1f}초고열 용광로\u{1f}Przegrzanie\u{1f}Núcleo Fundido\u{1f}Перегрев\u{1f}โมลเท\u{e47}นคอร\u{e4c}\u{1f}Eriyik Çekirdek\u{1f}熔火核心\u{1f}爐心超載",
                        ),
                    ),
                ]
            }
            Self::Tracer => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Warp\u{1f}Blink\u{1f}Traslación\n\u{1f}Traslación\u{1f}Transfert\u{1f}Traslazione\u{1f}ブリンク\u{1f}점멸\u{1f}Mignięcie\u{1f}Teleportar\u{1f}Скачок\u{1f}บล\u{e34}งค\u{e4c}\u{1f}Vın\u{1f}闪现\u{1f}閃現",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Zeitschleife\u{1f}Recall\u{1f}Regresión\n\u{1f}Regresión\u{1f}Rappel\u{1f}Flashback\u{1f}リコール\u{1f}시간 역행\u{1f}Powrót\u{1f}Recordação\u{1f}Возврат\u{1f}ร\u{e35}คอล\u{1f}Geri Sar\u{1f}闪回\u{1f}回溯",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Impulsbombe\u{1f}Pulse Bomb\u{1f}Bomba de pulsos\u{1f}Bomba de pulso\u{1f}Bombe à impulsion\u{1f}Bomba a Impulsi\u{1f}パルス・ボム\u{1f}펄스 폭탄\u{1f}Bomba Pulsacyjna\u{1f}Bomba Eletromagnética\u{1f}Импульсная бомба\u{1f}พ\u{e31}ลส\u{e4c}บอมบ\u{e4c}\u{1f}Pals Bombası\u{1f}脉冲炸弹\u{1f}脈衝炸彈",
                        ),
                    ),
                ]
            }
            Self::Vendetta => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "\u{1f}Whirlwind Dash\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}旋风疾步\u{1f}旋風衝刺",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "\u{1f}Soaring Slice\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}飞空斩击\u{1f}飛躍斬",
                        ),
                    ),
                    (
                        Ability::PrimaryFire,
                        Names(
                            "\u{1f}Palatine Fang\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}帕拉蒂尼之牙\u{1f}帕拉丁之牙",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "\u{1f}Warding Stance\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}招架姿态\u{1f}守備架式",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}Sundering Blade\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}斩地巨剑\u{1f}碎裂刀刃",
                        ),
                    ),
                ]
            }
            Self::Venture => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Graben\u{1f}Burrow\u{1f}Enterrarse\u{1f}Enterrarse\u{1f}Enfouissement\u{1f}Immersione\u{1f}バロー\u{1f}잠복\u{1f}Zagrzebanie\u{1f}Entocar\u{1f}Закапывание\u{1f}เบอร\u{e4c}โรว\u{e4c}\u{1f}Battıçıktı\u{1f}钻地\u{1f}鑽地",
                        ),
                    ),
                    (
                        Ability::PrimaryFire,
                        Names(
                            "SMART-Ausgräber\u{1f}SMART Excavator\u{1f}Excavación inteligente\u{1f}Excavadora INTELIGENTE\u{1f}Excavateur FUTÉ\u{1f}Scavatrice SMART\u{1f}スマート・エクスカベーター\u{1f}스마트 굴착기\u{1f}Inteligentny Świder\u{1f}Escavadeira Inteligente\u{1f}Археолог-землекоп\u{1f}เคร\u{e37}\u{e48}องข\u{e38}ดเจาะอ\u{e31}จฉร\u{e34}ยะ\u{1f}AKILLI Kazıcı\u{1f}智能挖掘钻\u{1f}智慧挖掘器",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Bohrspurt\u{1f}Drill Dash\u{1f}Perforación\u{1f}Ataque perforante\u{1f}Forage rapide\u{1f}Scatto Trivella\u{1f}ドリル・ダッシュ\u{1f}드릴 돌진\u{1f}Świdrowanie\u{1f}Disparada Perfurante\u{1f}Буровой рывок\u{1f}ดร\u{e34}ลล\u{e4c}แดช\u{1f}Matkap Hamlesi\u{1f}钻头突刺\u{1f}鑽頭衝刺",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Tektonisches Beben\u{1f}Tectonic Shock\u{1f}Impacto tectónico\u{1f}Choque tectónico\u{1f}Secousse sismique\u{1f}Scossa Tellurica\u{1f}テクトニック・ショック\u{1f}지각 충격\u{1f}Wstrząs Tektoniczny\u{1f}Choque Tectônico\u{1f}Тектонический толчок\u{1f}เทคโทน\u{e34}คช\u{e47}อค\u{1f}Tektonik Şok\u{1f}地壳震击\u{1f}地層震擊",
                        ),
                    ),
                ]
            }
            Self::Widowmaker => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Spinnenfaden\u{1f}Grappling Hook\u{1f}Gancho\n\u{1f}Arpeo\u{1f}Grappin\u{1f}Rampino\u{1f}グラップリング・フック\u{1f}갈고리 발사\u{1f}Kotwiczka\u{1f}Arpéu\u{1f}Крюк\u{1f}แกรปปล\u{e34}งฮ\u{e38}ค\u{1f}Borda Kancası\u{1f}抓钩\u{1f}爪鉤",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Giftmine\u{1f}Venom Mine\u{1f}Mina venenosa\n\u{1f}Mina venenosa\u{1f}Mine venimeuse\u{1f}Mina Venefica\u{1f}ヴェノム・マイン\u{1f}맹독 지뢰\u{1f}Jadowita Mina\u{1f}Mina Venenosa\u{1f}Ядовитая мина\u{1f}เวนอมไมน\u{e4c}\u{1f}Zehirli Mayın\u{1f}剧毒诡雷\u{1f}劇毒地雷",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Infrasicht\u{1f}Infra-Sight\u{1f}Infravisión\u{1f}Infravisión\u{1f}Infravision\u{1f}Visore Termico\u{1f}インフラサイト\u{1f}적외선 투시\u{1f}Infrawizja\u{1f}Visão Infravermelha\u{1f}Инфразрение\u{1f}อ\u{e34}นฟราไซต\u{e4c}\u{1f}Öte Görüş\u{1f}红外侦测\u{1f}紅外線視野",
                        ),
                    ),
                ]
            }
            Self::Winston => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Sprungantrieb\u{1f}Jump Pack\u{1f}Salto potenciado\n\u{1f}Propulsores\u{1f}Propulseurs\u{1f}Ipersalto\u{1f}ジャンプ・パック\u{1f}점프 팩\u{1f}Plecak Odrzutowy\u{1f}Salto a Jato\u{1f}Прыжковый ранец\u{1f}จ\u{e31}มป\u{e4c}แพ\u{e47}ค\u{1f}Uzun Sıçrama\u{1f}喷射背包\u{1f}噴射跳躍",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Barrierenprojektor\u{1f}Barrier Projector\u{1f}Campo de fuerza\n\u{1f}Proyector de barrera\u{1f}Générateur d’écran\u{1f}Proiettore Difensivo\u{1f}バリア・プロジェクター\u{1f}방벽 생성기\u{1f}Emiter Osłony\u{1f}Projetor de Barreira\u{1f}Защитный купол\u{1f}บาเร\u{e35}ยโพรเจกเตอร\u{e4c}\u{1f}Bariyer Projektörü\u{1f}屏障发射器\u{1f}屏障產生器",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Dschungelwut\u{1f}Primal Rage\u{1f}Rabia primigenia\n\u{1f}Ira primigenia\u{1f}Rage primordiale\u{1f}Rabbia Primordiale\u{1f}プライマル・レイジ\u{1f}원시의 분노\u{1f}Pierwotny Szał\u{1f}Fúria Primata\u{1f}Ярость зверя\u{1f}ไพรมอลเรจ\u{1f}İlkel Öfke\u{1f}原始暴怒\u{1f}野性之怒",
                        ),
                    ),
                ]
            }
            Self::WreckingBall => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Einkugeln\u{1f}Roll\u{1f}Bola arrolladora\u{1f}Rodar\u{1f}Roulé-boulé\u{1f}Rotolamento\u{1f}ロール\u{1f}구르기\u{1f}Kulowrotek\u{1f}Rolagem\u{1f}Каток\u{1f}โรล\u{1f}Yuvarlan\u{1f}动力铁球\u{1f}滾球形態",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Adaptiver Schild\u{1f}Adaptive Shield\u{1f}Escudo adaptable\u{1f}Escudo adaptable\u{1f}Bouclier dynamique\u{1f}Scudo Adattivo\u{1f}アダプティブ・シールド\u{1f}적응형 보호막\u{1f}Adaptabilne Osłony\u{1f}Escudo Adaptativo\u{1f}Адаптивный щит\u{1f}อะแดปท\u{e35}ฟช\u{e34}ลด\u{e4c}\u{1f}Uyarlanır Kalkan\u{1f}感应护盾\u{1f}自動調整型護盾",
                        ),
                    ),
                    (
                        Ability::Ability3,
                        Names(
                            "Kollisionskurs\u{1f}Piledriver\u{1f}Impacto demoledor\u{1f}Martinete\u{1f}Pilonnage\u{1f}Stella Cadente\u{1f}パイルドライバー\u{1f}파일드라이버\u{1f}Kafar\u{1f}Bate-estaca\u{1f}Копер\u{1f}ไพล\u{e4c}ไดรเวอร\u{e4c}\u{1f}Smaç\u{1f}重力坠击\u{1f}垂直打樁機",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "Abrissbirne\u{1f}Grappling Claw\u{1f}Gancho garra\u{1f}Garra mecánica\u{1f}Griffe-crampon\u{1f}Rampone\u{1f}グラップリング・クロー\u{1f}갈고리 고정\u{1f}Pazur\u{1f}Garra\u{1f}«Кошка»\u{1f}แกรปปล\u{e34}งคลอว\u{e4c}\u{1f}Asılma Kıskacı\u{1f}工程抓钩\u{1f}穩固爪鉤",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Minenfeld\u{1f}Minefield\u{1f}Campo de minas\u{1f}Campo minado\u{1f}Champ de mines\u{1f}Campo Minato\u{1f}マインフィールド\u{1f}지뢰밭\u{1f}Pole Minowe\u{1f}Campo Minado\u{1f}Минное поле\u{1f}ไมน\u{e4c}ฟ\u{e34}ลด\u{e4c}\u{1f}Mayın Tarlası\u{1f}地雷禁区\u{1f}地雷陣",
                        ),
                    ),
                ]
            }
            Self::Wuyang => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "\u{1f}Rushing Torrent\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}飞流步\u{1f}奔騰洪流",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "\u{1f}Guardian Wave\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}翻江浪\u{1f}守護之浪",
                        ),
                    ),
                    (
                        Ability::SecondaryFire,
                        Names(
                            "\u{1f}Restorative Stream\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}养神泉\u{1f}恢復水流",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "\u{1f}Tidal Blast\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}惊涛破\u{1f}濤浪爆發",
                        ),
                    ),
                ]
            }
            Self::Zarya => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Partikelbarriere\u{1f}Particle Barrier\u{1f}Barrera de partículas\u{1f}Barrera de partículas\u{1f}Écran de particules\u{1f}Barriera Particellare\u{1f}パーティクル・バリア\u{1f}입자 방벽\u{1f}Bariera Cząsteczkowa\u{1f}Barreira de Partículas\u{1f}Защитный барьер\u{1f}พาร\u{e4c}ต\u{e34}เค\u{e34}ลบาเร\u{e35}ย\u{1f}Parçacık Bariyeri\u{1f}粒子屏障\u{1f}粒子屏障",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Projizierte Barriere\u{1f}Projected Barrier\u{1f}Barrera proyectada\u{1f}Barrera proyectada\u{1f}Écran généré\u{1f}Proiezione Difensiva\u{1f}バリア・ショット\u{1f}방벽 씌우기\u{1f}Bariera Projekcyjna\u{1f}Barreira Projetada\u{1f}Дистанционный барьер\u{1f}โปรเจคบาเร\u{e35}ย\u{1f}Atımlı Bariyer\u{1f}投射屏障\u{1f}投射屏障",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Gravitonbombe\u{1f}Graviton Surge\u{1f}Bomba de gravedad\u{1f}Oleada de gravitones\u{1f}Orbe à gravitons\u{1f}Bomba Gravitonica\u{1f}グラビトン・サージ\u{1f}중력자탄\u{1f}Uderzenie Grawitonowe\u{1f}Surto de Grávitons\u{1f}Гравитонный импульс\u{1f}กราว\u{e34}ตอนเซ\u{e34}ร\u{e4c}จ\u{1f}Kütleçekim\u{1f}重力喷涌\u{1f}引力彈",
                        ),
                    ),
                ]
            }
            Self::Zenyatta => {
                &[
                    (
                        Ability::Ability1,
                        Names(
                            "Sphäre der Harmonie\u{1f}Orb of Harmony\u{1f}Orbe de armonía\n\u{1f}Orbe de armonía\u{1f}Orbe d’harmonie\u{1f}Globo dell'Armonia\u{1f}調和のオーブ\u{1f}조화의 구슬\u{1f}Kula Harmonii\u{1f}Orbe da Harmonia\u{1f}Сфера гармонии\u{1f}ออร\u{e4c}บออฟฮาร\u{e4c}โมน\u{e35}\u{e48}\u{1f}Ahenk Küresi\u{1f}谐\u{1f}和諧之珠",
                        ),
                    ),
                    (
                        Ability::Ability2,
                        Names(
                            "Sphäre der Zwietracht\u{1f}Orb of Discord\u{1f}Orbe de discordia\n\u{1f}Orbe de discordancia\u{1f}Orbe de discorde\u{1f}Globo della Discordia\u{1f}不和のオーブ\u{1f}부조화의 구슬\u{1f}Kula Rozdarcia\u{1f}Orbe da Discórdia\u{1f}Сфера диссонанса\u{1f}ออร\u{e4c}บออฟด\u{e34}สคอร\u{e4c}ด\u{1f}Kaos Küresi\u{1f}乱\u{1f}紛亂之珠",
                        ),
                    ),
                    (
                        Ability::Ultimate,
                        Names(
                            "Transzendenz\u{1f}Transcendence\u{1f}Trascendencia\n\u{1f}Trascendencia\u{1f}Transcendance\u{1f}Trascendenza\u{1f}心頭滅却\u{1f}초월\u{1f}Transcendencja\u{1f}Transcendência\u{1f}Трансцендентность\u{1f}ทรานส\u{e4c}เซนเดนซ\u{e4c}\u{1f}Ermişlik\u{1f}圣\u{1f}超凡入聖",
                        ),
                    ),
                ]
            }
        }
    }
}
