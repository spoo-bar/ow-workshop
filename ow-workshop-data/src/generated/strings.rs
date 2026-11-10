//! Localized phrases.
use crate::Lang;
use crate::lang::localized;
/// A phrase accepted by the workshop `String` value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LocalizedString {
    /// The empty phrase.
    Empty,
    /// `!`
    SymBang,
    /// `!!`
    SymBangBang,
    /// `!!!`
    SymBangBangBang,
    /// `#{0}`
    SymHashA,
    /// `({0})`
    SymParensA,
    /// `*`
    SymStar,
    /// `----------`
    SymDashs,
    /// `-> {0}`
    SymRightArrowSpA,
    /// `...`
    SymDotDotDot,
    /// `<- {0}`
    SymLeftArrowSpA,
    /// `<-> {0}`
    SymBothArrowsSpA,
    /// `?`
    SymQuestion,
    /// `??`
    SymQuestionQuestion,
    /// `???`
    SymQuestionQuestionQuestion,
    /// `Abilities`
    Abilities,
    /// `Ability`
    Ability,
    /// `Ability 1`
    Ability1,
    /// `Ability 2`
    Ability2,
    /// `Alert`
    Alert,
    /// `Alive`
    Alive,
    /// `Allies`
    Allies,
    /// `Ally`
    Ally,
    /// `Ammunition`
    Ammunition,
    /// `Angle`
    Angle,
    /// `Attack`
    Attack,
    /// `Attacked`
    Attacked,
    /// `Attacking`
    Attacking,
    /// `Attempt`
    Attempt,
    /// `Attempts`
    Attempts,
    /// `Average`
    Average,
    /// `Avoid`
    Avoid,
    /// `Avoided`
    Avoided,
    /// `Avoiding`
    Avoiding,
    /// `Backward`
    Backward,
    /// `Bad`
    Bad,
    /// `Ban`
    Ban,
    /// `Banned`
    Banned,
    /// `Banning`
    Banning,
    /// `Best`
    Best,
    /// `Better`
    Better,
    /// `Bid`
    Bid,
    /// `Bids`
    Bids,
    /// `Block`
    Block,
    /// `Blocked`
    Blocked,
    /// `Blocking`
    Blocking,
    /// `Blue`
    Blue,
    /// `Bonus`
    Bonus,
    /// `Bonuses`
    Bonuses,
    /// `Boss`
    Boss,
    /// `Bosses`
    Bosses,
    /// `Bought`
    Bought,
    /// `Build`
    Build,
    /// `Building`
    Building,
    /// `Built`
    Built,
    /// `Burn`
    Burn,
    /// `Burning`
    Burning,
    /// `Burnt`
    Burnt,
    /// `Buy`
    Buy,
    /// `Buying`
    Buying,
    /// `Capture`
    Capture,
    /// `Captured`
    Captured,
    /// `Capturing`
    Capturing,
    /// `Caution`
    Caution,
    /// `Center`
    Center,
    /// `Challenge Accepted`
    ChallengeAccepted,
    /// `Charisma`
    Charisma,
    /// `Chase`
    Chase,
    /// `Chased`
    Chased,
    /// `Chasing`
    Chasing,
    /// `Checkpoint`
    Checkpoint,
    /// `Checkpoints`
    Checkpoints,
    /// `Cloud`
    Cloud,
    /// `Clouds`
    Clouds,
    /// `Club`
    Club,
    /// `Clubs`
    Clubs,
    /// `Combo`
    Combo,
    /// `Come Here`
    ComeHere,
    /// `Condition`
    Condition,
    /// `Congratulations`
    Congratulations,
    /// `Connect`
    Connect,
    /// `Connected`
    Connected,
    /// `Connecting`
    Connecting,
    /// `Constitution`
    Constitution,
    /// `Control Point`
    ControlPoint,
    /// `Control Points`
    ControlPoints,
    /// `Cooldown`
    Cooldown,
    /// `Cooldowns`
    Cooldowns,
    /// `Corrupt`
    Corrupt,
    /// `Corrupted`
    Corrupted,
    /// `Corrupting`
    Corrupting,
    /// `Credit`
    Credit,
    /// `Credits`
    Credits,
    /// `Critical`
    Critical,
    /// `Crouch`
    Crouch,
    /// `Crouched`
    Crouched,
    /// `Crouching`
    Crouching,
    /// `Current`
    Current,
    /// `Current Allies`
    CurrentAllies,
    /// `Current Ally`
    CurrentAlly,
    /// `Current Attempt`
    CurrentAttempt,
    /// `Current Checkpoint`
    CurrentCheckpoint,
    /// `Current Enemies`
    CurrentEnemies,
    /// `Current Enemy`
    CurrentEnemy,
    /// `Current Form`
    CurrentForm,
    /// `Current Game`
    CurrentGame,
    /// `Current Hero`
    CurrentHero,
    /// `Current Heroes`
    CurrentHeroes,
    /// `Current Hostage`
    CurrentHostage,
    /// `Current Hostages`
    CurrentHostages,
    /// `Current Level`
    CurrentLevel,
    /// `Current Mission`
    CurrentMission,
    /// `Current Object`
    CurrentObject,
    /// `Current Objective`
    CurrentObjective,
    /// `Current Objects`
    CurrentObjects,
    /// `Current Phase`
    CurrentPhase,
    /// `Current Player`
    CurrentPlayer,
    /// `Current Players`
    CurrentPlayers,
    /// `Current Round`
    CurrentRound,
    /// `Current Target`
    CurrentTarget,
    /// `Current Targets`
    CurrentTargets,
    /// `Current Upgrade`
    CurrentUpgrade,
    /// `Damage`
    Damage,
    /// `Damaged`
    Damaged,
    /// `Damaging`
    Damaging,
    /// `Danger`
    Danger,
    /// `Dead`
    Dead,
    /// `Deal`
    Deal,
    /// `Dealing`
    Dealing,
    /// `Dealt`
    Dealt,
    /// `Deck`
    Deck,
    /// `Decks`
    Decks,
    /// `Defeat`
    Defeat,
    /// `Defend`
    Defend,
    /// `Defended`
    Defended,
    /// `Defending`
    Defending,
    /// `Defense`
    Defense,
    /// `Deliver`
    Deliver,
    /// `Delivered`
    Delivered,
    /// `Delivering`
    Delivering,
    /// `Depth`
    Depth,
    /// `Destabilize`
    Destabilize,
    /// `Destabilized`
    Destabilized,
    /// `Destabilizing`
    Destabilizing,
    /// `Destroy`
    Destroy,
    /// `Destroyed`
    Destroyed,
    /// `Destroying`
    Destroying,
    /// `Detect`
    Detect,
    /// `Detected`
    Detected,
    /// `Detecting`
    Detecting,
    /// `Dexterity`
    Dexterity,
    /// `Diamond`
    Diamond,
    /// `Diamonds`
    Diamonds,
    /// `Die`
    Die,
    /// `Discard`
    Discard,
    /// `Discarded`
    Discarded,
    /// `Discarding`
    Discarding,
    /// `Disconnect`
    Disconnect,
    /// `Disconnected`
    Disconnected,
    /// `Disconnecting`
    Disconnecting,
    /// `Distance`
    Distance,
    /// `Distances`
    Distances,
    /// `Dodge`
    Dodge,
    /// `Dodged`
    Dodged,
    /// `Dodging`
    Dodging,
    /// `Dome`
    Dome,
    /// `Domes`
    Domes,
    /// `Down`
    Down,
    /// `Download`
    Download,
    /// `Downloaded`
    Downloaded,
    /// `Downloading`
    Downloading,
    /// `Draw`
    Draw,
    /// `Drawing`
    Drawing,
    /// `Drawn`
    Drawn,
    /// `Drop`
    Drop,
    /// `Dropped`
    Dropped,
    /// `Dropping`
    Dropping,
    /// `Dying`
    Dying,
    /// `East`
    East,
    /// `Eliminate`
    Eliminate,
    /// `Eliminated`
    Eliminated,
    /// `Eliminating`
    Eliminating,
    /// `Elimination`
    Elimination,
    /// `Eliminations`
    Eliminations,
    /// `Enemies`
    Enemies,
    /// `Enemy`
    Enemy,
    /// `Entrance`
    Entrance,
    /// `Escort`
    Escort,
    /// `Escorted`
    Escorted,
    /// `Escorting`
    Escorting,
    /// `Excellent`
    Excellent,
    /// `Exit`
    Exit,
    /// `Experience`
    Experience,
    /// `Extreme`
    Extreme,
    /// `Face`
    Face,
    /// `Faces`
    Faces,
    /// `Facing`
    Facing,
    /// `Failed`
    Failed,
    /// `Failing`
    Failing,
    /// `Failure`
    Failure,
    /// `Fall`
    Fall,
    /// `Fallen`
    Fallen,
    /// `Falling`
    Falling,
    /// `Far`
    Far,
    /// `Fast`
    Fast,
    /// `Faster`
    Faster,
    /// `Fastest`
    Fastest,
    /// `Fault`
    Fault,
    /// `Faults`
    Faults,
    /// `Final`
    Final,
    /// `Final Allies`
    FinalAllies,
    /// `Final Ally`
    FinalAlly,
    /// `Final Attempt`
    FinalAttempt,
    /// `Final Checkpoint`
    FinalCheckpoint,
    /// `Final Enemies`
    FinalEnemies,
    /// `Final Enemy`
    FinalEnemy,
    /// `Final Form`
    FinalForm,
    /// `Final Game`
    FinalGame,
    /// `Final Hero`
    FinalHero,
    /// `Final Heroes`
    FinalHeroes,
    /// `Final Hostage`
    FinalHostage,
    /// `Final Hostages`
    FinalHostages,
    /// `Final Item`
    FinalItem,
    /// `Final Level`
    FinalLevel,
    /// `Final Mission`
    FinalMission,
    /// `Final Object`
    FinalObject,
    /// `Final Objective`
    FinalObjective,
    /// `Final Objects`
    FinalObjects,
    /// `Final Phase`
    FinalPhase,
    /// `Final Player`
    FinalPlayer,
    /// `Final Players`
    FinalPlayers,
    /// `Final Round`
    FinalRound,
    /// `Final Target`
    FinalTarget,
    /// `Final Targets`
    FinalTargets,
    /// `Final Time`
    FinalTime,
    /// `Final Upgrade`
    FinalUpgrade,
    /// `Find`
    Find,
    /// `Finding`
    Finding,
    /// `Finish`
    Finish,
    /// `Finished`
    Finished,
    /// `Finishing`
    Finishing,
    /// `Flown`
    Flown,
    /// `Fly`
    Fly,
    /// `Flying`
    Flying,
    /// `Fold`
    Fold,
    /// `Folded`
    Folded,
    /// `Folding`
    Folding,
    /// `Form`
    Form,
    /// `Forms`
    Forms,
    /// `Forward`
    Forward,
    /// `Found`
    Found,
    /// `Freeze`
    Freeze,
    /// `Freezing`
    Freezing,
    /// `Frozen`
    Frozen,
    /// `GG`
    GG,
    /// `Game`
    Game,
    /// `Games`
    Games,
    /// `Games Lost`
    GamesLost,
    /// `Games Won`
    GamesWon,
    /// `Go`
    Go,
    /// `Goal`
    Goal,
    /// `Goals`
    Goals,
    /// `Going`
    Going,
    /// `Good`
    Good,
    /// `Good Luck`
    GoodLuck,
    /// `Goodbye`
    Goodbye,
    /// `Green`
    Green,
    /// `Guilty`
    Guilty,
    /// `Hack`
    Hack,
    /// `Hacked`
    Hacked,
    /// `Hacking`
    Hacking,
    /// `Hand`
    Hand,
    /// `Hands`
    Hands,
    /// `Heal`
    Heal,
    /// `Healed`
    Healed,
    /// `Healer`
    Healer,
    /// `Healers`
    Healers,
    /// `Healing`
    Healing,
    /// `Heart`
    Heart,
    /// `Hearts`
    Hearts,
    /// `Height`
    Height,
    /// `Hello`
    Hello,
    /// `Help`
    Help,
    /// `Here`
    Here,
    /// `Hero`
    Hero,
    /// `Heroes`
    Heroes,
    /// `Hidden`
    Hidden,
    /// `Hide`
    Hide,
    /// `Hiding`
    Hiding,
    /// `High Score`
    HighScore,
    /// `High Scores`
    HighScores,
    /// `Hit`
    Hit,
    /// `Hitting`
    Hitting,
    /// `Hmmm`
    Hmmm,
    /// `Hostage`
    Hostage,
    /// `Hostages`
    Hostages,
    /// `Huh`
    Huh,
    /// `Hunt`
    Hunt,
    /// `Hunted`
    Hunted,
    /// `Hunter`
    Hunter,
    /// `Hunters`
    Hunters,
    /// `Hunting`
    Hunting,
    /// `I Give Up`
    IGiveUp,
    /// `I Tried`
    ITried,
    /// `In View`
    InView,
    /// `Income`
    Income,
    /// `Incoming`
    Incoming,
    /// `Initial`
    Initial,
    /// `Initial Allies`
    InitialAllies,
    /// `Initial Ally`
    InitialAlly,
    /// `Initial Attempt`
    InitialAttempt,
    /// `Initial Checkpoint`
    InitialCheckpoint,
    /// `Initial Enemies`
    InitialEnemies,
    /// `Initial Enemy`
    InitialEnemy,
    /// `Initial Form`
    InitialForm,
    /// `Initial Game`
    InitialGame,
    /// `Initial Hero`
    InitialHero,
    /// `Initial Heroes`
    InitialHeroes,
    /// `Initial Hostage`
    InitialHostage,
    /// `Initial Level`
    InitialLevel,
    /// `Initial Mission`
    InitialMission,
    /// `Initial Object`
    InitialObject,
    /// `Initial Objective`
    InitialObjective,
    /// `Initial Objects`
    InitialObjects,
    /// `Initial Phase`
    InitialPhase,
    /// `Initial Player`
    InitialPlayer,
    /// `Initial Players`
    InitialPlayers,
    /// `Initial Round`
    InitialRound,
    /// `Initial Target`
    InitialTarget,
    /// `Initial Targets`
    InitialTargets,
    /// `Initial Upgrade`
    InitialUpgrade,
    /// `Innocent`
    Innocent,
    /// `Inside`
    Inside,
    /// `Intelligence`
    Intelligence,
    /// `Interact`
    Interact,
    /// `Invisible`
    Invisible,
    /// `Item`
    Item,
    /// `Items`
    Items,
    /// `Join`
    Join,
    /// `Joined`
    Joined,
    /// `Joining`
    Joining,
    /// `Jump`
    Jump,
    /// `Jumping`
    Jumping,
    /// `Kill`
    Kill,
    /// `Kills`
    Kills,
    /// `Killstreak`
    Killstreak,
    /// `Killstreaks`
    Killstreaks,
    /// `Leader`
    Leader,
    /// `Leaders`
    Leaders,
    /// `Least`
    Least,
    /// `Left`
    Left,
    /// `Less`
    Less,
    /// `Level`
    Level,
    /// `Level Down`
    LevelDown,
    /// `Level Up`
    LevelUp,
    /// `Levels`
    Levels,
    /// `Life`
    Life,
    /// `Limited`
    Limited,
    /// `Lives`
    Lives,
    /// `Load`
    Load,
    /// `Loaded`
    Loaded,
    /// `Loading`
    Loading,
    /// `Location`
    Location,
    /// `Lock`
    Lock,
    /// `Locked`
    Locked,
    /// `Locking`
    Locking,
    /// `Loser`
    Loser,
    /// `Losers`
    Losers,
    /// `Loss`
    Loss,
    /// `Losses`
    Losses,
    /// `Max`
    Max,
    /// `Mild`
    Mild,
    /// `Min`
    Min,
    /// `Mission`
    Mission,
    /// `Mission Aborted`
    MissionAborted,
    /// `Mission Accomplished`
    MissionAccomplished,
    /// `Mission Failed`
    MissionFailed,
    /// `Missions`
    Missions,
    /// `Moderate`
    Moderate,
    /// `Money`
    Money,
    /// `Monster`
    Monster,
    /// `Monsters`
    Monsters,
    /// `More`
    More,
    /// `Most`
    Most,
    /// `My Mistake`
    MyMistake,
    /// `Near`
    Near,
    /// `New High Score`
    NewHighScore,
    /// `New Record`
    NewRecord,
    /// `Next`
    Next,
    /// `Next Allies`
    NextAllies,
    /// `Next Ally`
    NextAlly,
    /// `Next Attempt`
    NextAttempt,
    /// `Next Checkpoint`
    NextCheckpoint,
    /// `Next Enemies`
    NextEnemies,
    /// `Next Enemy`
    NextEnemy,
    /// `Next Form`
    NextForm,
    /// `Next Game`
    NextGame,
    /// `Next Hero`
    NextHero,
    /// `Next Heroes`
    NextHeroes,
    /// `Next Hostage`
    NextHostage,
    /// `Next Hostages`
    NextHostages,
    /// `Next Level`
    NextLevel,
    /// `Next Mission`
    NextMission,
    /// `Next Object`
    NextObject,
    /// `Next Objective`
    NextObjective,
    /// `Next Objects`
    NextObjects,
    /// `Next Phase`
    NextPhase,
    /// `Next Player`
    NextPlayer,
    /// `Next Players`
    NextPlayers,
    /// `Next Round`
    NextRound,
    /// `Next Target`
    NextTarget,
    /// `Next Targets`
    NextTargets,
    /// `Next Upgrade`
    NextUpgrade,
    /// `Nice Try`
    NiceTry,
    /// `No`
    No,
    /// `No Thanks`
    NoThanks,
    /// `None`
    None,
    /// `Normal`
    Normal,
    /// `North`
    North,
    /// `Northeast`
    Northeast,
    /// `Northwest`
    Northwest,
    /// `Not Today`
    NotToday,
    /// `Object`
    Object,
    /// `Objective`
    Objective,
    /// `Objectives`
    Objectives,
    /// `Objects`
    Objects,
    /// `Obtain`
    Obtain,
    /// `Obtained`
    Obtained,
    /// `Obtaining`
    Obtaining,
    /// `Off`
    Off,
    /// `On`
    On,
    /// `Oof`
    Oof,
    /// `Oops`
    Oops,
    /// `Optimal`
    Optimal,
    /// `Optimize`
    Optimize,
    /// `Optimized`
    Optimized,
    /// `Optimizing`
    Optimizing,
    /// `Out of View`
    OutOfView,
    /// `Outgoing`
    Outgoing,
    /// `Outside`
    Outside,
    /// `Over`
    Over,
    /// `Overtime`
    Overtime,
    /// `Participant`
    Participant,
    /// `Participants`
    Participants,
    /// `Payload`
    Payload,
    /// `Payloads`
    Payloads,
    /// `Phase`
    Phase,
    /// `Phases`
    Phases,
    /// `Pick`
    Pick,
    /// `Picked`
    Picked,
    /// `Picking`
    Picking,
    /// `Pile`
    Pile,
    /// `Piles`
    Piles,
    /// `Play`
    Play,
    /// `Played`
    Played,
    /// `Player`
    Player,
    /// `Players`
    Players,
    /// `Playing`
    Playing,
    /// `Point`
    Point,
    /// `Points`
    Points,
    /// `Points Earned`
    PointsEarned,
    /// `Points Lost`
    PointsLost,
    /// `Position`
    Position,
    /// `Power`
    Power,
    /// `Power-up`
    PowerDashUp,
    /// `Power-ups`
    PowerDashUps,
    /// `Price`
    Price,
    /// `Primary Fire`
    PrimaryFire,
    /// `Projectile`
    Projectile,
    /// `Projectiles`
    Projectiles,
    /// `Protect`
    Protect,
    /// `Protected`
    Protected,
    /// `Protecting`
    Protecting,
    /// `Purified`
    Purified,
    /// `Purify`
    Purify,
    /// `Purifying`
    Purifying,
    /// `Purple`
    Purple,
    /// `Raise`
    Raise,
    /// `Raised`
    Raised,
    /// `Raising`
    Raising,
    /// `Rank`
    Rank,
    /// `Rank A`
    RankA,
    /// `Rank B`
    RankB,
    /// `Rank C`
    RankC,
    /// `Rank D`
    RankD,
    /// `Rank E`
    RankE,
    /// `Rank F`
    RankF,
    /// `Rank S`
    RankS,
    /// `Reach`
    Reach,
    /// `Reached`
    Reached,
    /// `Reaching`
    Reaching,
    /// `Ready`
    Ready,
    /// `Record`
    Record,
    /// `Records`
    Records,
    /// `Recover`
    Recover,
    /// `Recovered`
    Recovered,
    /// `Recovering`
    Recovering,
    /// `Red`
    Red,
    /// `Remain`
    Remain,
    /// `Remaining`
    Remaining,
    /// `Rescue`
    Rescue,
    /// `Rescued`
    Rescued,
    /// `Rescuing`
    Rescuing,
    /// `Resource`
    Resource,
    /// `Resources`
    Resources,
    /// `Resurrect`
    Resurrect,
    /// `Resurrected`
    Resurrected,
    /// `Resurrecting`
    Resurrecting,
    /// `Reveal`
    Reveal,
    /// `Revealed`
    Revealed,
    /// `Revealing`
    Revealing,
    /// `Reverse`
    Reverse,
    /// `Reversed`
    Reversed,
    /// `Reversing`
    Reversing,
    /// `Right`
    Right,
    /// `Round`
    Round,
    /// `Round {0}`
    RoundA,
    /// `Rounds`
    Rounds,
    /// `Rounds Lost`
    RoundsLost,
    /// `Rounds Won`
    RoundsWon,
    /// `Run`
    Run,
    /// `Running`
    Running,
    /// `Safe`
    Safe,
    /// `Save`
    Save,
    /// `Saved`
    Saved,
    /// `Saving`
    Saving,
    /// `Score`
    Score,
    /// `Scores`
    Scores,
    /// `Secondary Fire`
    SecondaryFire,
    /// `Secure`
    Secure,
    /// `Secured`
    Secured,
    /// `Securing`
    Securing,
    /// `Select`
    Select,
    /// `Selected`
    Selected,
    /// `Selecting`
    Selecting,
    /// `Sell`
    Sell,
    /// `Selling`
    Selling,
    /// `Server Load`
    ServerLoad,
    /// `Server Load Average`
    ServerLoadAverage,
    /// `Server Load Peak`
    ServerLoadPeak,
    /// `Sever`
    Sever,
    /// `Severe`
    Severe,
    /// `Severed`
    Severed,
    /// `Severing`
    Severing,
    /// `Shop`
    Shop,
    /// `Shops`
    Shops,
    /// `Shuffle`
    Shuffle,
    /// `Shuffled`
    Shuffled,
    /// `Sink`
    Sink,
    /// `Sinking`
    Sinking,
    /// `Skip`
    Skip,
    /// `Skipped`
    Skipped,
    /// `Skipping`
    Skipping,
    /// `Sleep`
    Sleep,
    /// `Sleeping`
    Sleeping,
    /// `Slept`
    Slept,
    /// `Slow`
    Slow,
    /// `Slower`
    Slower,
    /// `Slowest`
    Slowest,
    /// `Sold`
    Sold,
    /// `Sorry`
    Sorry,
    /// `South`
    South,
    /// `Southeast`
    Southeast,
    /// `Southwest`
    Southwest,
    /// `Spade`
    Spade,
    /// `Spades`
    Spades,
    /// `Sparkles`
    Sparkles,
    /// `Spawn`
    Spawn,
    /// `Spawned`
    Spawned,
    /// `Spawning`
    Spawning,
    /// `Speed`
    Speed,
    /// `Speeds`
    Speeds,
    /// `Sphere`
    Sphere,
    /// `Spheres`
    Spheres,
    /// `Stabilize`
    Stabilize,
    /// `Stabilized`
    Stabilized,
    /// `Stabilizing`
    Stabilizing,
    /// `Stable`
    Stable,
    /// `Star`
    Star,
    /// `Stars`
    Stars,
    /// `Start`
    Start,
    /// `Started`
    Started,
    /// `Starting`
    Starting,
    /// `Status`
    Status,
    /// `Stay`
    Stay,
    /// `Stay Away`
    StayAway,
    /// `Stayed`
    Stayed,
    /// `Staying`
    Staying,
    /// `Stop`
    Stop,
    /// `Stopped`
    Stopped,
    /// `Stopping`
    Stopping,
    /// `Strength`
    Strength,
    /// `Stun`
    Stun,
    /// `Stunned`
    Stunned,
    /// `Stunning`
    Stunning,
    /// `Suboptimal`
    Suboptimal,
    /// `Success`
    Success,
    /// `Sudden Death`
    SuddenDeath,
    /// `Sunk`
    Sunk,
    /// `Superb`
    Superb,
    /// `Survive`
    Survive,
    /// `Survived`
    Survived,
    /// `Surviving`
    Surviving,
    /// `Target`
    Target,
    /// `Targets`
    Targets,
    /// `Team`
    Team,
    /// `Teammate`
    Teammate,
    /// `Teammates`
    Teammates,
    /// `Teams`
    Teams,
    /// `Terrible`
    Terrible,
    /// `Thank You`
    ThankYou,
    /// `Thanks`
    Thanks,
    /// `That Was Awesome`
    ThatWasAwesome,
    /// `Threat`
    Threat,
    /// `Threat Level`
    ThreatLevel,
    /// `Threat Levels`
    ThreatLevels,
    /// `Threats`
    Threats,
    /// `Tiebreaker`
    Tiebreaker,
    /// `Time`
    Time,
    /// `Times`
    Times,
    /// `Total`
    Total,
    /// `Trade`
    Trade,
    /// `Traded`
    Traded,
    /// `Trading`
    Trading,
    /// `Traitor`
    Traitor,
    /// `Traitors`
    Traitors,
    /// `Transfer`
    Transfer,
    /// `Transferred`
    Transferred,
    /// `Transferring`
    Transferring,
    /// `Try Again`
    TryAgain,
    /// `Turret`
    Turret,
    /// `Turrets`
    Turrets,
    /// `Ugh`
    Ugh,
    /// `Ultimate Ability`
    UltimateAbility,
    /// `Under`
    Under,
    /// `Unknown`
    Unknown,
    /// `Unlimited`
    Unlimited,
    /// `Unlock`
    Unlock,
    /// `Unlocked`
    Unlocked,
    /// `Unlocking`
    Unlocking,
    /// `Unsafe`
    Unsafe,
    /// `Unstable`
    Unstable,
    /// `Up`
    Up,
    /// `Upgrade`
    Upgrade,
    /// `Upgrades`
    Upgrades,
    /// `Upload`
    Upload,
    /// `Uploaded`
    Uploaded,
    /// `Uploading`
    Uploading,
    /// `Use Ability 1`
    UseAbility1,
    /// `Use Ability 2`
    UseAbility2,
    /// `Use Ultimate Ability`
    UseUltimateAbility,
    /// `Victory`
    Victory,
    /// `Visible`
    Visible,
    /// `Vortex`
    Vortex,
    /// `Vortices`
    Vortices,
    /// `Wait`
    Wait,
    /// `Waiting`
    Waiting,
    /// `Wall`
    Wall,
    /// `Walls`
    Walls,
    /// `Warning`
    Warning,
    /// `Welcome`
    Welcome,
    /// `Well Played`
    WellPlayed,
    /// `West`
    West,
    /// `White`
    White,
    /// `Wild`
    Wild,
    /// `Win: %1$s XP`
    WinColonPercent1SXP,
    /// `Winner`
    Winner,
    /// `Winners`
    Winners,
    /// `Wins`
    Wins,
    /// `Wisdom`
    Wisdom,
    /// `Worse`
    Worse,
    /// `Worst`
    Worst,
    /// `Wow`
    Wow,
    /// `Yellow`
    Yellow,
    /// `Yes`
    Yes,
    /// `You`
    You,
    /// `You Lose`
    YouLose,
    /// `You Win`
    YouWin,
    /// `Zone`
    Zone,
    /// `Zones`
    Zones,
    /// `{0} != {1}`
    SymASpNeSpB,
    /// `{0} * {1}`
    SymASpStarSpB,
    /// `{0} + {1}`
    SymASpPlusSpB,
    /// `{0} - {1}`
    SymASpDashSpB,
    /// `{0} - {1} - {2}`
    SymASpDashSpBSpDashSpC,
    /// `{0} ->`
    SymASpRightArrow,
    /// `{0} -> {1}`
    SymASpRightArrowSpB,
    /// `{0} / {1}`
    SymASpSlashSpB,
    /// `{0} : {1} : {2}`
    SymASpColonSpBSpColonSpC,
    /// `{0} < {1}`
    SymASpLtSpB,
    /// `{0} <-`
    SymASpLeftArrow,
    /// `{0} <- {1}`
    SymASpLeftArrowSpB,
    /// `{0} <->`
    SymASpBothArrows,
    /// `{0} <-> {1}`
    SymASpBothArrowsSpB,
    /// `{0} <= {1}`
    SymASpLeSpB,
    /// `{0} = {1}`
    SymASpEqSpB,
    /// `{0} == {1}`
    SymASpEqEqSpB,
    /// `{0} > {1}`
    SymASpGtSpB,
    /// `{0} >= {1}`
    SymASpGeSpB,
    /// `{0} and {1}`
    AAndB,
    /// `{0} m`
    AM,
    /// `{0} m/s`
    AMSlashS,
    /// `{0} sec`
    ASec,
    /// `{0} vs {1}`
    AVsB,
    /// `{0} {1}`
    SymASpB,
    /// `{0} {1} {2}`
    SymASpBSpC,
    /// `{0}!`
    SymABang,
    /// `{0}!!`
    SymABangBang,
    /// `{0}!!!`
    SymABangBangBang,
    /// `{0}%`
    SymAPercent,
    /// `{0}, {1}`
    SymACommaSpB,
    /// `{0}, {1}, and {2}`
    ACommaBCommaAndC,
    /// `{0}:`
    SymAColon,
    /// `{0}: {1}`
    SymAColonSpB,
    /// `{0}: {1} and {2}`
    AColonBAndC,
    /// `{0}:{1}`
    SymAColonB,
    /// `{0}?`
    SymAQuestion,
    /// `{0}??`
    SymAQuestionQuestion,
    /// `{0}???`
    SymAQuestionQuestionQuestion,
    /// `¡{0}!`
    SymInvBangABang,
    /// `¿{0}?`
    SymInvQuestionAQuestion,
}
impl LocalizedString {
    /// Every variant, in declaration order.
    pub const ALL: &'static [Self] = &[
        Self::Empty,
        Self::SymBang,
        Self::SymBangBang,
        Self::SymBangBangBang,
        Self::SymHashA,
        Self::SymParensA,
        Self::SymStar,
        Self::SymDashs,
        Self::SymRightArrowSpA,
        Self::SymDotDotDot,
        Self::SymLeftArrowSpA,
        Self::SymBothArrowsSpA,
        Self::SymQuestion,
        Self::SymQuestionQuestion,
        Self::SymQuestionQuestionQuestion,
        Self::Abilities,
        Self::Ability,
        Self::Ability1,
        Self::Ability2,
        Self::Alert,
        Self::Alive,
        Self::Allies,
        Self::Ally,
        Self::Ammunition,
        Self::Angle,
        Self::Attack,
        Self::Attacked,
        Self::Attacking,
        Self::Attempt,
        Self::Attempts,
        Self::Average,
        Self::Avoid,
        Self::Avoided,
        Self::Avoiding,
        Self::Backward,
        Self::Bad,
        Self::Ban,
        Self::Banned,
        Self::Banning,
        Self::Best,
        Self::Better,
        Self::Bid,
        Self::Bids,
        Self::Block,
        Self::Blocked,
        Self::Blocking,
        Self::Blue,
        Self::Bonus,
        Self::Bonuses,
        Self::Boss,
        Self::Bosses,
        Self::Bought,
        Self::Build,
        Self::Building,
        Self::Built,
        Self::Burn,
        Self::Burning,
        Self::Burnt,
        Self::Buy,
        Self::Buying,
        Self::Capture,
        Self::Captured,
        Self::Capturing,
        Self::Caution,
        Self::Center,
        Self::ChallengeAccepted,
        Self::Charisma,
        Self::Chase,
        Self::Chased,
        Self::Chasing,
        Self::Checkpoint,
        Self::Checkpoints,
        Self::Cloud,
        Self::Clouds,
        Self::Club,
        Self::Clubs,
        Self::Combo,
        Self::ComeHere,
        Self::Condition,
        Self::Congratulations,
        Self::Connect,
        Self::Connected,
        Self::Connecting,
        Self::Constitution,
        Self::ControlPoint,
        Self::ControlPoints,
        Self::Cooldown,
        Self::Cooldowns,
        Self::Corrupt,
        Self::Corrupted,
        Self::Corrupting,
        Self::Credit,
        Self::Credits,
        Self::Critical,
        Self::Crouch,
        Self::Crouched,
        Self::Crouching,
        Self::Current,
        Self::CurrentAllies,
        Self::CurrentAlly,
        Self::CurrentAttempt,
        Self::CurrentCheckpoint,
        Self::CurrentEnemies,
        Self::CurrentEnemy,
        Self::CurrentForm,
        Self::CurrentGame,
        Self::CurrentHero,
        Self::CurrentHeroes,
        Self::CurrentHostage,
        Self::CurrentHostages,
        Self::CurrentLevel,
        Self::CurrentMission,
        Self::CurrentObject,
        Self::CurrentObjective,
        Self::CurrentObjects,
        Self::CurrentPhase,
        Self::CurrentPlayer,
        Self::CurrentPlayers,
        Self::CurrentRound,
        Self::CurrentTarget,
        Self::CurrentTargets,
        Self::CurrentUpgrade,
        Self::Damage,
        Self::Damaged,
        Self::Damaging,
        Self::Danger,
        Self::Dead,
        Self::Deal,
        Self::Dealing,
        Self::Dealt,
        Self::Deck,
        Self::Decks,
        Self::Defeat,
        Self::Defend,
        Self::Defended,
        Self::Defending,
        Self::Defense,
        Self::Deliver,
        Self::Delivered,
        Self::Delivering,
        Self::Depth,
        Self::Destabilize,
        Self::Destabilized,
        Self::Destabilizing,
        Self::Destroy,
        Self::Destroyed,
        Self::Destroying,
        Self::Detect,
        Self::Detected,
        Self::Detecting,
        Self::Dexterity,
        Self::Diamond,
        Self::Diamonds,
        Self::Die,
        Self::Discard,
        Self::Discarded,
        Self::Discarding,
        Self::Disconnect,
        Self::Disconnected,
        Self::Disconnecting,
        Self::Distance,
        Self::Distances,
        Self::Dodge,
        Self::Dodged,
        Self::Dodging,
        Self::Dome,
        Self::Domes,
        Self::Down,
        Self::Download,
        Self::Downloaded,
        Self::Downloading,
        Self::Draw,
        Self::Drawing,
        Self::Drawn,
        Self::Drop,
        Self::Dropped,
        Self::Dropping,
        Self::Dying,
        Self::East,
        Self::Eliminate,
        Self::Eliminated,
        Self::Eliminating,
        Self::Elimination,
        Self::Eliminations,
        Self::Enemies,
        Self::Enemy,
        Self::Entrance,
        Self::Escort,
        Self::Escorted,
        Self::Escorting,
        Self::Excellent,
        Self::Exit,
        Self::Experience,
        Self::Extreme,
        Self::Face,
        Self::Faces,
        Self::Facing,
        Self::Failed,
        Self::Failing,
        Self::Failure,
        Self::Fall,
        Self::Fallen,
        Self::Falling,
        Self::Far,
        Self::Fast,
        Self::Faster,
        Self::Fastest,
        Self::Fault,
        Self::Faults,
        Self::Final,
        Self::FinalAllies,
        Self::FinalAlly,
        Self::FinalAttempt,
        Self::FinalCheckpoint,
        Self::FinalEnemies,
        Self::FinalEnemy,
        Self::FinalForm,
        Self::FinalGame,
        Self::FinalHero,
        Self::FinalHeroes,
        Self::FinalHostage,
        Self::FinalHostages,
        Self::FinalItem,
        Self::FinalLevel,
        Self::FinalMission,
        Self::FinalObject,
        Self::FinalObjective,
        Self::FinalObjects,
        Self::FinalPhase,
        Self::FinalPlayer,
        Self::FinalPlayers,
        Self::FinalRound,
        Self::FinalTarget,
        Self::FinalTargets,
        Self::FinalTime,
        Self::FinalUpgrade,
        Self::Find,
        Self::Finding,
        Self::Finish,
        Self::Finished,
        Self::Finishing,
        Self::Flown,
        Self::Fly,
        Self::Flying,
        Self::Fold,
        Self::Folded,
        Self::Folding,
        Self::Form,
        Self::Forms,
        Self::Forward,
        Self::Found,
        Self::Freeze,
        Self::Freezing,
        Self::Frozen,
        Self::GG,
        Self::Game,
        Self::Games,
        Self::GamesLost,
        Self::GamesWon,
        Self::Go,
        Self::Goal,
        Self::Goals,
        Self::Going,
        Self::Good,
        Self::GoodLuck,
        Self::Goodbye,
        Self::Green,
        Self::Guilty,
        Self::Hack,
        Self::Hacked,
        Self::Hacking,
        Self::Hand,
        Self::Hands,
        Self::Heal,
        Self::Healed,
        Self::Healer,
        Self::Healers,
        Self::Healing,
        Self::Heart,
        Self::Hearts,
        Self::Height,
        Self::Hello,
        Self::Help,
        Self::Here,
        Self::Hero,
        Self::Heroes,
        Self::Hidden,
        Self::Hide,
        Self::Hiding,
        Self::HighScore,
        Self::HighScores,
        Self::Hit,
        Self::Hitting,
        Self::Hmmm,
        Self::Hostage,
        Self::Hostages,
        Self::Huh,
        Self::Hunt,
        Self::Hunted,
        Self::Hunter,
        Self::Hunters,
        Self::Hunting,
        Self::IGiveUp,
        Self::ITried,
        Self::InView,
        Self::Income,
        Self::Incoming,
        Self::Initial,
        Self::InitialAllies,
        Self::InitialAlly,
        Self::InitialAttempt,
        Self::InitialCheckpoint,
        Self::InitialEnemies,
        Self::InitialEnemy,
        Self::InitialForm,
        Self::InitialGame,
        Self::InitialHero,
        Self::InitialHeroes,
        Self::InitialHostage,
        Self::InitialLevel,
        Self::InitialMission,
        Self::InitialObject,
        Self::InitialObjective,
        Self::InitialObjects,
        Self::InitialPhase,
        Self::InitialPlayer,
        Self::InitialPlayers,
        Self::InitialRound,
        Self::InitialTarget,
        Self::InitialTargets,
        Self::InitialUpgrade,
        Self::Innocent,
        Self::Inside,
        Self::Intelligence,
        Self::Interact,
        Self::Invisible,
        Self::Item,
        Self::Items,
        Self::Join,
        Self::Joined,
        Self::Joining,
        Self::Jump,
        Self::Jumping,
        Self::Kill,
        Self::Kills,
        Self::Killstreak,
        Self::Killstreaks,
        Self::Leader,
        Self::Leaders,
        Self::Least,
        Self::Left,
        Self::Less,
        Self::Level,
        Self::LevelDown,
        Self::LevelUp,
        Self::Levels,
        Self::Life,
        Self::Limited,
        Self::Lives,
        Self::Load,
        Self::Loaded,
        Self::Loading,
        Self::Location,
        Self::Lock,
        Self::Locked,
        Self::Locking,
        Self::Loser,
        Self::Losers,
        Self::Loss,
        Self::Losses,
        Self::Max,
        Self::Mild,
        Self::Min,
        Self::Mission,
        Self::MissionAborted,
        Self::MissionAccomplished,
        Self::MissionFailed,
        Self::Missions,
        Self::Moderate,
        Self::Money,
        Self::Monster,
        Self::Monsters,
        Self::More,
        Self::Most,
        Self::MyMistake,
        Self::Near,
        Self::NewHighScore,
        Self::NewRecord,
        Self::Next,
        Self::NextAllies,
        Self::NextAlly,
        Self::NextAttempt,
        Self::NextCheckpoint,
        Self::NextEnemies,
        Self::NextEnemy,
        Self::NextForm,
        Self::NextGame,
        Self::NextHero,
        Self::NextHeroes,
        Self::NextHostage,
        Self::NextHostages,
        Self::NextLevel,
        Self::NextMission,
        Self::NextObject,
        Self::NextObjective,
        Self::NextObjects,
        Self::NextPhase,
        Self::NextPlayer,
        Self::NextPlayers,
        Self::NextRound,
        Self::NextTarget,
        Self::NextTargets,
        Self::NextUpgrade,
        Self::NiceTry,
        Self::No,
        Self::NoThanks,
        Self::None,
        Self::Normal,
        Self::North,
        Self::Northeast,
        Self::Northwest,
        Self::NotToday,
        Self::Object,
        Self::Objective,
        Self::Objectives,
        Self::Objects,
        Self::Obtain,
        Self::Obtained,
        Self::Obtaining,
        Self::Off,
        Self::On,
        Self::Oof,
        Self::Oops,
        Self::Optimal,
        Self::Optimize,
        Self::Optimized,
        Self::Optimizing,
        Self::OutOfView,
        Self::Outgoing,
        Self::Outside,
        Self::Over,
        Self::Overtime,
        Self::Participant,
        Self::Participants,
        Self::Payload,
        Self::Payloads,
        Self::Phase,
        Self::Phases,
        Self::Pick,
        Self::Picked,
        Self::Picking,
        Self::Pile,
        Self::Piles,
        Self::Play,
        Self::Played,
        Self::Player,
        Self::Players,
        Self::Playing,
        Self::Point,
        Self::Points,
        Self::PointsEarned,
        Self::PointsLost,
        Self::Position,
        Self::Power,
        Self::PowerDashUp,
        Self::PowerDashUps,
        Self::Price,
        Self::PrimaryFire,
        Self::Projectile,
        Self::Projectiles,
        Self::Protect,
        Self::Protected,
        Self::Protecting,
        Self::Purified,
        Self::Purify,
        Self::Purifying,
        Self::Purple,
        Self::Raise,
        Self::Raised,
        Self::Raising,
        Self::Rank,
        Self::RankA,
        Self::RankB,
        Self::RankC,
        Self::RankD,
        Self::RankE,
        Self::RankF,
        Self::RankS,
        Self::Reach,
        Self::Reached,
        Self::Reaching,
        Self::Ready,
        Self::Record,
        Self::Records,
        Self::Recover,
        Self::Recovered,
        Self::Recovering,
        Self::Red,
        Self::Remain,
        Self::Remaining,
        Self::Rescue,
        Self::Rescued,
        Self::Rescuing,
        Self::Resource,
        Self::Resources,
        Self::Resurrect,
        Self::Resurrected,
        Self::Resurrecting,
        Self::Reveal,
        Self::Revealed,
        Self::Revealing,
        Self::Reverse,
        Self::Reversed,
        Self::Reversing,
        Self::Right,
        Self::Round,
        Self::RoundA,
        Self::Rounds,
        Self::RoundsLost,
        Self::RoundsWon,
        Self::Run,
        Self::Running,
        Self::Safe,
        Self::Save,
        Self::Saved,
        Self::Saving,
        Self::Score,
        Self::Scores,
        Self::SecondaryFire,
        Self::Secure,
        Self::Secured,
        Self::Securing,
        Self::Select,
        Self::Selected,
        Self::Selecting,
        Self::Sell,
        Self::Selling,
        Self::ServerLoad,
        Self::ServerLoadAverage,
        Self::ServerLoadPeak,
        Self::Sever,
        Self::Severe,
        Self::Severed,
        Self::Severing,
        Self::Shop,
        Self::Shops,
        Self::Shuffle,
        Self::Shuffled,
        Self::Sink,
        Self::Sinking,
        Self::Skip,
        Self::Skipped,
        Self::Skipping,
        Self::Sleep,
        Self::Sleeping,
        Self::Slept,
        Self::Slow,
        Self::Slower,
        Self::Slowest,
        Self::Sold,
        Self::Sorry,
        Self::South,
        Self::Southeast,
        Self::Southwest,
        Self::Spade,
        Self::Spades,
        Self::Sparkles,
        Self::Spawn,
        Self::Spawned,
        Self::Spawning,
        Self::Speed,
        Self::Speeds,
        Self::Sphere,
        Self::Spheres,
        Self::Stabilize,
        Self::Stabilized,
        Self::Stabilizing,
        Self::Stable,
        Self::Star,
        Self::Stars,
        Self::Start,
        Self::Started,
        Self::Starting,
        Self::Status,
        Self::Stay,
        Self::StayAway,
        Self::Stayed,
        Self::Staying,
        Self::Stop,
        Self::Stopped,
        Self::Stopping,
        Self::Strength,
        Self::Stun,
        Self::Stunned,
        Self::Stunning,
        Self::Suboptimal,
        Self::Success,
        Self::SuddenDeath,
        Self::Sunk,
        Self::Superb,
        Self::Survive,
        Self::Survived,
        Self::Surviving,
        Self::Target,
        Self::Targets,
        Self::Team,
        Self::Teammate,
        Self::Teammates,
        Self::Teams,
        Self::Terrible,
        Self::ThankYou,
        Self::Thanks,
        Self::ThatWasAwesome,
        Self::Threat,
        Self::ThreatLevel,
        Self::ThreatLevels,
        Self::Threats,
        Self::Tiebreaker,
        Self::Time,
        Self::Times,
        Self::Total,
        Self::Trade,
        Self::Traded,
        Self::Trading,
        Self::Traitor,
        Self::Traitors,
        Self::Transfer,
        Self::Transferred,
        Self::Transferring,
        Self::TryAgain,
        Self::Turret,
        Self::Turrets,
        Self::Ugh,
        Self::UltimateAbility,
        Self::Under,
        Self::Unknown,
        Self::Unlimited,
        Self::Unlock,
        Self::Unlocked,
        Self::Unlocking,
        Self::Unsafe,
        Self::Unstable,
        Self::Up,
        Self::Upgrade,
        Self::Upgrades,
        Self::Upload,
        Self::Uploaded,
        Self::Uploading,
        Self::UseAbility1,
        Self::UseAbility2,
        Self::UseUltimateAbility,
        Self::Victory,
        Self::Visible,
        Self::Vortex,
        Self::Vortices,
        Self::Wait,
        Self::Waiting,
        Self::Wall,
        Self::Walls,
        Self::Warning,
        Self::Welcome,
        Self::WellPlayed,
        Self::West,
        Self::White,
        Self::Wild,
        Self::WinColonPercent1SXP,
        Self::Winner,
        Self::Winners,
        Self::Wins,
        Self::Wisdom,
        Self::Worse,
        Self::Worst,
        Self::Wow,
        Self::Yellow,
        Self::Yes,
        Self::You,
        Self::YouLose,
        Self::YouWin,
        Self::Zone,
        Self::Zones,
        Self::SymASpNeSpB,
        Self::SymASpStarSpB,
        Self::SymASpPlusSpB,
        Self::SymASpDashSpB,
        Self::SymASpDashSpBSpDashSpC,
        Self::SymASpRightArrow,
        Self::SymASpRightArrowSpB,
        Self::SymASpSlashSpB,
        Self::SymASpColonSpBSpColonSpC,
        Self::SymASpLtSpB,
        Self::SymASpLeftArrow,
        Self::SymASpLeftArrowSpB,
        Self::SymASpBothArrows,
        Self::SymASpBothArrowsSpB,
        Self::SymASpLeSpB,
        Self::SymASpEqSpB,
        Self::SymASpEqEqSpB,
        Self::SymASpGtSpB,
        Self::SymASpGeSpB,
        Self::AAndB,
        Self::AM,
        Self::AMSlashS,
        Self::ASec,
        Self::AVsB,
        Self::SymASpB,
        Self::SymASpBSpC,
        Self::SymABang,
        Self::SymABangBang,
        Self::SymABangBangBang,
        Self::SymAPercent,
        Self::SymACommaSpB,
        Self::ACommaBCommaAndC,
        Self::SymAColon,
        Self::SymAColonSpB,
        Self::AColonBAndC,
        Self::SymAColonB,
        Self::SymAQuestion,
        Self::SymAQuestionQuestion,
        Self::SymAQuestionQuestionQuestion,
        Self::SymInvBangABang,
        Self::SymInvQuestionAQuestion,
    ];
    /// The name as written in workshop code of the given language.
    pub fn text(self, lang: Lang) -> &'static str {
        static NAMES: [&str; 721usize] = [
            "",
            "\u{1f}!\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}！\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}！",
            "\u{1f}!!\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}！！\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}！！",
            "\u{1f}!!!\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}！！！\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}！！！",
            "\u{1f}#{0}",
            "\u{1f}({0})",
            "\u{1f}*",
            "\u{1f}----------",
            "\u{1f}-> {0}",
            "\u{1f}...\u{1f}\u{1f}\u{1f}…\u{1f}\u{1f}…\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}……",
            "\u{1f}<- {0}",
            "\u{1f}<-> {0}",
            "\u{1f}?\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}？\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}？",
            "\u{1f}??\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}？？\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}？？",
            "\u{1f}???\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}？？？\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}？？？",
            "Fähigkeiten\u{1f}Abilities\u{1f}Habilidades\u{1f}Habilidades\u{1f}Capacités\u{1f}Abilità\u{1f}アビリティ\u{1f}\u{1f}Zdolności\u{1f}Habilidades\u{1f}Способности\u{1f}ความสามารถ\u{1f}Kabiliyetler\u{1f}技能\u{1f}技能",
            "Fähigkeit\u{1f}Ability\u{1f}Habilidad\u{1f}Habilidad\u{1f}Capacité\u{1f}Abilità\u{1f}アビリティ\u{1f}\u{1f}Zdolność\u{1f}Habilidade\u{1f}Способность\u{1f}ความสามารถ\u{1f}Kabiliyet\u{1f}技能\u{1f}技能",
            "Fähigkeit 1\u{1f}Ability 1\u{1f}Habilidad 1\u{1f}Habilidad 1\u{1f}Capacité 1\u{1f}Abilità 1\u{1f}アビリティ1\u{1f}\u{1f}Zdolność 1\u{1f}Habilidade 1\u{1f}Способность 1\u{1f}ความสามารถ 1\u{1f}1. Kabiliyet\u{1f}技能1\u{1f}技能1",
            "Fähigkeit 2\u{1f}Ability 2\u{1f}Habilidad 2\u{1f}Habilidad 2\u{1f}Capacité 2\u{1f}Abilità 2\u{1f}アビリティ2\u{1f}\u{1f}Zdolność 2\u{1f}Habilidade 2\u{1f}Способность 2\u{1f}ความสามารถ 2\u{1f}2. Kabiliyet\u{1f}技能2\u{1f}技能2",
            "Achtung\u{1f}Alert\u{1f}Alerta\u{1f}Alerta\u{1f}Alerte\u{1f}Allerta\u{1f}注意\u{1f}\u{1f}\u{1f}Alerta\u{1f}Оповещение\u{1f}เต\u{e37}อน\u{1f}Alarm\u{1f}警报\u{1f}警報",
            "Lebendig\u{1f}Alive\u{1f}Con vida\u{1f}Vivo\u{1f}En vie\u{1f}Vivo\u{1f}生存\u{1f}\u{1f}Żyje\u{1f}Vivo\u{1f}В живых\u{1f}ม\u{e35}ช\u{e35}ว\u{e34}ต\u{1f}Canlı\u{1f}存活\u{1f}生存",
            "Verbündete\u{1f}Allies\u{1f}Aliados\u{1f}Aliados\u{1f}Alliés\u{1f}Alleati\u{1f}味方\u{1f}\u{1f}Sojusznicy\u{1f}Aliados\u{1f}Союзники\u{1f}เพ\u{e37}\u{e48}อนร\u{e48}วมท\u{e35}ม\u{1f}Müttefikler\u{1f}盟友\u{1f}盟友",
            "Verbündeter\u{1f}Ally\u{1f}Aliado\u{1f}Aliado\u{1f}Allié\u{1f}Alleato\u{1f}味方\u{1f}\u{1f}Sojusznik\u{1f}Aliado\u{1f}Союзник\u{1f}เพ\u{e37}\u{e48}อนร\u{e48}วมท\u{e35}ม\u{1f}Müttefik\u{1f}盟友\u{1f}盟友",
            "Munition\u{1f}Ammunition\u{1f}Munición\u{1f}Munición\u{1f}Munitions\u{1f}Munizioni\u{1f}矢/弾\u{1f}\u{1f}Amunicja\u{1f}Munição\u{1f}Боеприпасы\u{1f}กระส\u{e38}น\u{1f}Mühimmat\u{1f}弹药\u{1f}彈藥",
            "Winkel\u{1f}Angle\u{1f}Ángulo\u{1f}Ángulo\u{1f}\u{1f}Angolo\u{1f}角度\u{1f}\u{1f}Kąt\u{1f}Ângulo\u{1f}Угол\u{1f}ม\u{e38}ม\u{1f}Açı\u{1f}角度\u{1f}角度",
            "Angreifen\u{1f}Attack\u{1f}Atacar\u{1f}Atacar\u{1f}Attaquer\u{1f}Attacca\u{1f}攻撃\u{1f}\u{1f}Atakuj\u{1f}Atacar\u{1f}Атаковать\u{1f}โจมต\u{e35}\u{1f}Saldırı\u{1f}进攻\u{1f}攻擊",
            "Angegriffen\u{1f}Attacked\u{1f}Atacado\u{1f}Atacado\u{1f}Attaqué\u{1f}Attaccato\u{1f}攻撃した\u{1f}\u{1f}Zaatakowano\u{1f}Atacado\u{1f}Атакован\u{1f}โจมต\u{e35}แล\u{e49}ว\u{1f}Saldırılan\u{1f}已进攻\u{1f}被攻擊",
            "Angriff\u{1f}Attacking\u{1f}Atacando\u{1f}Atacando\u{1f}Attaque\u{1f}Attacco in Corso\u{1f}攻撃中\u{1f}\u{1f}Atakowanie\u{1f}Atacando\u{1f}Атакует\u{1f}กำล\u{e31}งโจมต\u{e35}\u{1f}Saldıran\u{1f}正在进攻\u{1f}正在攻擊",
            "Versuch\u{1f}Attempt\u{1f}Intento\u{1f}Intento\u{1f}Tentative\u{1f}Tentativo\u{1f}挑戦\u{1f}\u{1f}Próba\u{1f}Tentativa\u{1f}Попытка\u{1f}ความพยายาม\u{1f}Deneme\u{1f}尝试\u{1f}機會",
            "Versuche\u{1f}Attempts\u{1f}Intentos\u{1f}Intentos\u{1f}Tentatives\u{1f}Tentativi\u{1f}挑戦\u{1f}\u{1f}Próby\u{1f}Tentativas\u{1f}Попытки\u{1f}ความพยายาม\u{1f}Denemeler\u{1f}尝试\u{1f}機會",
            "Durchschnitt\u{1f}Average\u{1f}Medio\u{1f}Promedio\u{1f}Moyenne\u{1f}Medio\u{1f}平均\u{1f}\u{1f}Średnio\u{1f}Médio\u{1f}В среднем\u{1f}เฉล\u{e35}\u{e48}ย\u{1f}Ortalama\u{1f}平均\u{1f}平均",
            "Meiden\u{1f}Avoid\u{1f}Evitar\u{1f}Evitar\u{1f}Éviter\u{1f}Evita\u{1f}回避\u{1f}\u{1f}Uniknij\u{1f}Evitar\u{1f}Избегание\u{1f}หล\u{e35}กเล\u{e35}\u{e48}ยง\u{1f}Sakınma\u{1f}躲避\u{1f}避開",
            "Gemieden\u{1f}Avoided\u{1f}Evitado\u{1f}Evitado\u{1f}Évité\u{1f}Evitato\u{1f}回避した\u{1f}\u{1f}Uniknięto\u{1f}Evitado\u{1f}Избежал\u{1f}หล\u{e35}กเล\u{e35}\u{e48}ยงแล\u{e49}ว\u{1f}Sakınılan\u{1f}已躲避\u{1f}被避開",
            "Vermeidung\u{1f}Avoiding\u{1f}Evitando\u{1f}Evitando\u{1f}Évitement\u{1f}Evitamento in Corso\u{1f}回避中\u{1f}\u{1f}Unikanie\u{1f}Evitando\u{1f}Избегает\u{1f}กำล\u{e31}งหล\u{e35}กเล\u{e35}\u{e48}ยง\u{1f}Sakınan\u{1f}正在躲避\u{1f}正在避開",
            "Rückwärts\u{1f}Backward\u{1f}Hacia atrás\u{1f}Atrás\u{1f}Arrière\u{1f}Indietro\u{1f}後方\u{1f}\u{1f}Do tyłu\u{1f}Para Trás\u{1f}Назад\u{1f}ย\u{e49}อนหล\u{e31}ง\u{1f}Geri\u{1f}后\u{1f}後退",
            "Schlecht\u{1f}Bad\u{1f}Malo\u{1f}Malo\u{1f}Mauvais\u{1f}Brutto\u{1f}悪い\u{1f}\u{1f}Źle\u{1f}Ruim\u{1f}Плохо\u{1f}ไม\u{e48}ด\u{e35}\u{1f}Kötü\u{1f}差\u{1f}壞",
            "Bannen\u{1f}Ban\u{1f}Bloquear\u{1f}Prohibir\u{1f}Bannir\u{1f}Sospendi\u{1f}BAN\u{1f}\u{1f}Zawieś\u{1f}Banir\u{1f}Блокировка\u{1f}แบน\u{1f}Engelle\u{1f}屏蔽\u{1f}禁止",
            "Gebannt\u{1f}Banned\u{1f}Bloqueado\u{1f}Prohibido\u{1f}Banni\u{1f}Sospeso\u{1f}BANした\u{1f}\u{1f}Zawieszono\u{1f}Banido\u{1f}Заблокирован\u{1f}แบนแล\u{e49}ว\u{1f}Engellenen\u{1f}已屏蔽\u{1f}被禁止",
            "Bann\u{1f}Banning\u{1f}Bloqueando\u{1f}Prohibiendo\u{1f}Bannissement\u{1f}Sospensione in Corso\u{1f}BAN中\u{1f}\u{1f}Zawieszanie\u{1f}Banindo\u{1f}Блокирует\u{1f}กำล\u{e31}งแบน\u{1f}Engelleyen\u{1f}正在屏蔽\u{1f}正在禁止",
            "Am besten\u{1f}Best\u{1f}El mejor\u{1f}El mejor\u{1f}Le meilleur\u{1f}Migliore\u{1f}最高\u{1f}\u{1f}Najlepiej\u{1f}O Melhor\u{1f}Лучший\u{1f}ด\u{e35}ท\u{e35}\u{e48}ส\u{e38}ด\u{1f}En İyi\u{1f}最佳\u{1f}最佳",
            "Besser\u{1f}Better\u{1f}Mejor\u{1f}Mejor\u{1f}Meilleur\u{1f}Meglio\u{1f}より良い\u{1f}\u{1f}Lepiej\u{1f}Melhor\u{1f}Лучше\u{1f}ด\u{e35}กว\u{e48}า\u{1f}Daha İyi\u{1f}优秀\u{1f}較佳",
            "Gebot\u{1f}Bid\u{1f}Puja\u{1f}Apuesta\u{1f}Enchère\u{1f}Puntata\u{1f}ビッド\u{1f}\u{1f}Zakład\u{1f}Aposta\u{1f}Ставка\u{1f}ประม\u{e39}ล\u{1f}Teklif\u{1f}加注\u{1f}競標",
            "Gebote\u{1f}Bids\u{1f}Pujas\u{1f}Apuestas\u{1f}Enchères\u{1f}Puntate\u{1f}ビッド\u{1f}\u{1f}Zakłady\u{1f}Apostas\u{1f}Ставки\u{1f}ประม\u{e39}ล\u{1f}Teklifler\u{1f}加注\u{1f}競標",
            "Blockieren\u{1f}Block\u{1f}Bloquear\u{1f}Bloquear\u{1f}Bloquer\u{1f}Ferma\u{1f}ブロックする\u{1f}\u{1f}Zablokuj\u{1f}Bloquear\u{1f}Блокировать\u{1f}บล\u{e47}อค\u{1f}Engelle\u{1f}阻挡\u{1f}阻擋",
            "Blockiert\u{1f}Blocked\u{1f}Bloqueado\u{1f}Bloqueado\u{1f}Bloqué\u{1f}Fermato\u{1f}ブロックした\u{1f}\u{1f}Zablokowano\u{1f}Bloqueado\u{1f}Заблокировал\u{1f}บล\u{e47}อคแล\u{e49}ว\u{1f}Engellenen\u{1f}已阻挡\u{1f}被阻擋",
            "Block\u{1f}Blocking\u{1f}Bloqueando\u{1f}Bloqueando\u{1f}Bloque\u{1f}Fermata in Corso\u{1f}ブロックしている\u{1f}\u{1f}Blokowanie\u{1f}Bloqueando\u{1f}Блокирует\u{1f}กำล\u{e31}งบล\u{e47}อค\u{1f}Engelleyen\u{1f}正在阻挡\u{1f}正在阻擋",
            "Blau\u{1f}Blue\u{1f}\u{1f}Azul\u{1f}Bleu\u{1f}Blu\u{1f}青\u{1f}\u{1f}Niebieski\u{1f}Azul\u{1f}Синий\u{1f}ฟ\u{e49}า\u{1f}Mavi\u{1f}蓝色\u{1f}藍",
            "\u{1f}Bonus\u{1f}Bonificación\u{1f}Bonificación\u{1f}Prime\u{1f}\u{1f}ボーナス\u{1f}\u{1f}Premia\u{1f}Bônus\u{1f}Бонус\u{1f}โบน\u{e31}ส\u{1f}\u{1f}奖励\u{1f}獎勵",
            "Boni\u{1f}Bonuses\u{1f}Bonificaciones\u{1f}Bonificaciones\u{1f}Primes\u{1f}Bonus\u{1f}ボーナス\u{1f}\u{1f}Premie\u{1f}Bônus\u{1f}Бонусы\u{1f}โบน\u{e31}ส\u{1f}Bonuslar\u{1f}奖励\u{1f}獎勵",
            "\u{1f}Boss\u{1f}Jefe\u{1f}Jefe\u{1f}\u{1f}\u{1f}ボス\u{1f}\u{1f}\u{1f}Chefe\u{1f}Босс\u{1f}บอส\u{1f}Baş Düşman\u{1f}首领\u{1f}首領",
            "Bosse\u{1f}Bosses\u{1f}Jefes\u{1f}Jefes\u{1f}Boss\u{1f}Boss\u{1f}ボス\u{1f}\u{1f}Bossowie\u{1f}Chefes\u{1f}Боссы\u{1f}บอส\u{1f}Baş Düşmanlar\u{1f}首领\u{1f}首領",
            "Gekauft\u{1f}Bought\u{1f}Comprado\u{1f}Comprado\u{1f}Acheté\u{1f}Acquistato\u{1f}購入した\u{1f}\u{1f}Kupiono\u{1f}Comprado\u{1f}Куплен\u{1f}ซ\u{e37}\u{e49}อแล\u{e49}ว\u{1f}Satın Alındı\u{1f}已购买\u{1f}已購買",
            "Bauen\u{1f}Build\u{1f}Construir\u{1f}Construir\u{1f}Construire\u{1f}Costruisci\u{1f}設置\u{1f}\u{1f}Zbuduj\u{1f}Construir\u{1f}Строить\u{1f}สร\u{e49}าง\u{1f}Oluştur\u{1f}建造\u{1f}建造",
            "Gebäude\u{1f}Building\u{1f}Construyendo\u{1f}Construyendo\u{1f}Construction\u{1f}Costruzione in Corso\u{1f}設置中\u{1f}\u{1f}Budowanie\u{1f}Construindo\u{1f}Строение\u{1f}กำล\u{e31}งสร\u{e49}าง\u{1f}Oluşturma\u{1f}正在建造\u{1f}正在建造",
            "Gebaut\u{1f}Built\u{1f}Construido\u{1f}Construido\u{1f}Construit\u{1f}Costruito\u{1f}設置した\u{1f}\u{1f}Zbudowano\u{1f}Construído\u{1f}Построен\u{1f}สร\u{e49}างแล\u{e49}ว\u{1f}Kurulan\u{1f}已建造\u{1f}已建造",
            "Verbrennen\u{1f}Burn\u{1f}Quemar\u{1f}Quemar\u{1f}Brûler\u{1f}Brucia\u{1f}燃焼\u{1f}\u{1f}Spal\u{1f}Queimar\u{1f}Горение\u{1f}เผาไหม\u{e49}\u{1f}Yanma\u{1f}燃烧\u{1f}燃燒",
            "Brand\u{1f}Burning\u{1f}Quemando\u{1f}En llamas\u{1f}Enflammé\u{1f}Bruciatura in Corso\u{1f}燃焼中\u{1f}\u{1f}Palenie\u{1f}Queimando\u{1f}Горит\u{1f}กำล\u{e31}งเผาไหม\u{e49}\u{1f}Yakan\u{1f}正在燃烧\u{1f}正在燃燒",
            "Verbrannt\u{1f}Burnt\u{1f}Quemado\u{1f}Quemado\u{1f}Brûlé\u{1f}Bruciato\u{1f}燃焼した\u{1f}\u{1f}Spalono\u{1f}Queimado\u{1f}Сгорел\u{1f}เผาไหม\u{e49}แล\u{e49}ว\u{1f}Yanan\u{1f}已燃烧\u{1f}受到燃燒",
            "Kaufen\u{1f}Buy\u{1f}Comprar\u{1f}Comprar\u{1f}Acheter\u{1f}Acquista\u{1f}購入\u{1f}\u{1f}Kup\u{1f}Comprar\u{1f}Купить\u{1f}ซ\u{e37}\u{e49}อ\u{1f}Satın Al\u{1f}购买\u{1f}購買",
            "Kauf\u{1f}Buying\u{1f}Comprando\u{1f}Comprando\u{1f}Achat\u{1f}Acquisto in Corso\u{1f}購入中\u{1f}\u{1f}Kupowanie\u{1f}Comprando\u{1f}Покупает\u{1f}กำล\u{e31}งซ\u{e37}\u{e49}อ\u{1f}Satın Alınıyor\u{1f}正在购买\u{1f}正在購買",
            "Erobern\u{1f}Capture\u{1f}Capturar\u{1f}Capturar\u{1f}Capturer\u{1f}Cattura\u{1f}確保\u{1f}\u{1f}Przejmij\u{1f}Capturar\u{1f}Захват\u{1f}ย\u{e36}ดครอง\u{1f}Kap\u{1f}夺取\u{1f}捕捉",
            "Erobert\u{1f}Captured\u{1f}Capturado\u{1f}Capturado\u{1f}Capturé\u{1f}Catturato\u{1f}確保した\u{1f}\u{1f}Przejęto\u{1f}Capturado\u{1f}Захвачен\u{1f}ย\u{e36}ดได\u{e49}\u{1f}Ele Geçirilen\u{1f}已夺取\u{1f}遭到捕捉",
            "Eroberung\u{1f}Capturing\u{1f}Capturando\u{1f}Capturando\u{1f}Capture\u{1f}Cattura in Corso\u{1f}確保中\u{1f}\u{1f}Przejmowanie\u{1f}Capturando\u{1f}Захватывает\u{1f}กำล\u{e31}งย\u{e36}ด\u{1f}Ele Geçiren\u{1f}正在夺取\u{1f}正在捕捉",
            "Vorsicht\u{1f}Caution\u{1f}Precaución\u{1f}Peligro\u{1f}Prudence\u{1f}Attenzione\u{1f}注意\u{1f}\u{1f}Uwaga\u{1f}Perigo\u{1f}Осторожно\u{1f}คำเต\u{e37}อน\u{1f}Dikkat\u{1f}危险\u{1f}注意",
            "Mitte\u{1f}Center\u{1f}Centro\u{1f}Centro\u{1f}Centre\u{1f}Centro\u{1f}中心\u{1f}\u{1f}Środek\u{1f}Centro\u{1f}Центр\u{1f}ศ\u{e39}นย\u{e4c}กลาง\u{1f}Merkez\u{1f}中\u{1f}中央",
            "Herausforderung angenommen\u{1f}Challenge Accepted\u{1f}Desafío aceptado\u{1f}Desafío aceptado\u{1f}Défi accepté\u{1f}Sfida Accettata\u{1f}売られたケンカは買うぞ\u{1f}\u{1f}Przyjmuję wyzwanie\u{1f}Desafio Aceito\u{1f}Вызов принят\u{1f}ยอมร\u{e31}บการท\u{e49}าทายแล\u{e49}ว\u{1f}Hodri Meydan\u{1f}接受挑战\u{1f}接受挑戰",
            "\u{1f}Charisma\u{1f}Carisma\u{1f}Encanto\u{1f}Charisme\u{1f}Carisma\u{1f}カリスマ\u{1f}\u{1f}Charyzma\u{1f}Carisma\u{1f}Обаяние\u{1f}ความม\u{e35}เสน\u{e48}ห\u{e4c}\u{1f}Karizma\u{1f}魅力\u{1f}魅力",
            "Verfolgen\u{1f}Chase\u{1f}Perseguir\u{1f}Perseguir\u{1f}Poursuivre\u{1f}Segui\u{1f}追跡\u{1f}\u{1f}Dogoń\u{1f}Acompanhar\u{1f}Преследовать\u{1f}ไล\u{e48}ตาม\u{1f}Kovala\u{1f}追击\u{1f}追逐",
            "Verfolgt\u{1f}Chased\u{1f}Perseguido\u{1f}Perseguido\u{1f}Poursuivi\u{1f}Seguito\u{1f}追跡した\u{1f}\u{1f}Dogoniono\u{1f}Acompanhado\u{1f}Преследовал\u{1f}ไล\u{e48}ตามแล\u{e49}ว\u{1f}Kovalanan\u{1f}已追击\u{1f}被追逐",
            "Verfolgung\u{1f}Chasing\u{1f}Persiguiendo\u{1f}Persiguiendo\u{1f}Poursuite\u{1f}Inseguimento in Corso\u{1f}追跡中\u{1f}\u{1f}Gonienie\u{1f}Acompanhando\u{1f}Преследует\u{1f}กำล\u{e31}งไล\u{e48}ตาม\u{1f}Kovalayan\u{1f}正在追击\u{1f}正在追逐",
            "\u{1f}Checkpoint\u{1f}Punto de control\u{1f}Punto de control\u{1f}Point de contrôle\u{1f}\u{1f}チェックポイント\u{1f}\u{1f}Punkt kontrolny\u{1f}Ponto de Verificação\u{1f}Контрольная точка\u{1f}เช\u{e47}คพอยต\u{e4c}\u{1f}Kontrol Noktası\u{1f}检查点\u{1f}檢查點",
            "\u{1f}Checkpoints\u{1f}Puntos de control\u{1f}Puntos de control\u{1f}Points de contrôle\u{1f}Checkpoint\u{1f}チェックポイント\u{1f}\u{1f}Punkty kontrolne\u{1f}Pontos de Verificação\u{1f}Контрольные точки\u{1f}เช\u{e47}คพอยต\u{e4c}\u{1f}Kontrol Noktaları\u{1f}检查点\u{1f}檢查點",
            "Wolke\u{1f}Cloud\u{1f}Nube\u{1f}Nube\u{1f}Nuage\u{1f}Nuvola\u{1f}雲\u{1f}\u{1f}Chmura\u{1f}Nuvem\u{1f}Облако\u{1f}เมฆ\u{1f}Bulut\u{1f}云朵\u{1f}雲",
            "Wolken\u{1f}Clouds\u{1f}Nubes\u{1f}Nubes\u{1f}Nuages\u{1f}Nuvole\u{1f}雲\u{1f}\u{1f}Chmury\u{1f}Nuvens\u{1f}Облака\u{1f}เมฆ\u{1f}Bulut\u{1f}云朵\u{1f}雲",
            "Kreuz\u{1f}Club\u{1f}Basto\u{1f}Trébol\u{1f}Trèfle\u{1f}Fiore\u{1f}クラブ\u{1f}\u{1f}Trefl\u{1f}Paus\u{1f}Трефы\u{1f}ดอกจ\u{e34}ก\u{1f}Sinek\u{1f}梅花\u{1f}梅花",
            "Kreuz\u{1f}Clubs\u{1f}Bastos\u{1f}Tréboles\u{1f}Trèfle\u{1f}Fiori\u{1f}クラブ\u{1f}\u{1f}Trefl\u{1f}Paus\u{1f}Трефы\u{1f}ดอกจ\u{e34}ก\u{1f}Sinekler\u{1f}梅花\u{1f}梅花",
            "\u{1f}Combo\u{1f}\u{1f}\u{1f}\u{1f}Combinazione\u{1f}コンボ\u{1f}\u{1f}Kombos\u{1f}\u{1f}Комбо\u{1f}คอมโบ\u{1f}Kombo\u{1f}连击\u{1f}連擊",
            "Hierher kommen\u{1f}Come Here\u{1f}Ven aquí\u{1f}Ven aquí\u{1f}Venez ici\u{1f}Vieni Qui\u{1f}こっちに来い\u{1f}\u{1f}Dawać tutaj\u{1f}Venha Aqui\u{1f}Иди сюда\u{1f}มาน\u{e35}\u{e48}ส\u{e34}\u{1f}Buraya\u{1f}到这儿来\u{1f}過來",
            "Zustand\u{1f}Condition\u{1f}Condición\u{1f}Condición\u{1f}\u{1f}Condizione\u{1f}条件\u{1f}\u{1f}Warunek\u{1f}Condição\u{1f}Условие\u{1f}เง\u{e37}\u{e48}อนไข\u{1f}Koşul\u{1f}条件\u{1f}狀況",
            "Glückwunsch\u{1f}Congratulations\u{1f}Enhorabuena\u{1f}Felicitaciones\u{1f}Félicitations\u{1f}Congratulazioni\u{1f}おめでとう！\u{1f}\u{1f}Gratulacje\u{1f}Parabéns\u{1f}Поздравляем\u{1f}ย\u{e34}นด\u{e35}ด\u{e49}วย\u{1f}Tebrikler\u{1f}恭喜\u{1f}恭喜",
            "Verbinden\u{1f}Connect\u{1f}Conectar\u{1f}Conectar\u{1f}Connecter\u{1f}Connetti\u{1f}接続\u{1f}\u{1f}Połącz\u{1f}Conectar\u{1f}Подключиться\u{1f}เช\u{e37}\u{e48}อมต\u{e48}อ\u{1f}Bağla\u{1f}连接\u{1f}連接",
            "Verbunden\u{1f}Connected\u{1f}Conectado\u{1f}Conectado\u{1f}Connecté(e)\u{1f}Connesso\u{1f}接続した\u{1f}\u{1f}Połączono\u{1f}Conectado\u{1f}Подключен\u{1f}การเช\u{e37}\u{e48}อมต\u{e48}อแล\u{e49}ว\u{1f}Bağlı\u{1f}已连接\u{1f}獲得連接",
            "Verbindung\u{1f}Connecting\u{1f}Conectando\u{1f}Conectándose\u{1f}Connexion\u{1f}Connessione in Corso\u{1f}接続中\u{1f}\u{1f}Łączenie\u{1f}Conectando\u{1f}Подключается\u{1f}กำล\u{e31}งเช\u{e37}\u{e48}อมต\u{e48}อ\u{1f}Bağlı\u{1f}正在连接\u{1f}正在連接",
            "Konstitution\u{1f}Constitution\u{1f}Constitución\u{1f}Constitución\u{1f}\u{1f}Costituzione\u{1f}体格\u{1f}\u{1f}Kondycja\u{1f}Constituição\u{1f}Телосложение\u{1f}ความแข\u{e47}งแกร\u{e48}ง\u{1f}Dayanıklılık\u{1f}建筑\u{1f}體質",
            "Kontrollpunkt\u{1f}Control Point\u{1f}Punto de control\u{1f}Punto de control\u{1f}Point de contrôle\u{1f}Punto di Controllo\u{1f}コントロール・ポイント\u{1f}\u{1f}Punkt kontrolny\u{1f}Ponto de Controle\u{1f}Контрольная точка\u{1f}จ\u{e38}ดควบค\u{e38}ม\u{1f}Kontrol Noktası\u{1f}控制点\u{1f}控制點",
            "Kontrollpunkte\u{1f}Control Points\u{1f}Puntos de control\u{1f}Puntos de control\u{1f}Points de contrôle\u{1f}Punti di Controllo\u{1f}コントロール・ポイント\u{1f}\u{1f}Punkty kontrolne\u{1f}Pontos de Controle\u{1f}Контрольные точки\u{1f}จ\u{e38}ดควบค\u{e38}ม\u{1f}Kontrol Noktaları\u{1f}控制点\u{1f}控制點",
            "Abklingzeit\u{1f}Cooldown\u{1f}Tiempo de reutilización\u{1f}Tiempo de reutilización\u{1f}Temps de recharge\u{1f}Tempo di recupero\u{1f}クールダウン\u{1f}\u{1f}Czas odnowienia\u{1f}Tempo de Recarga\u{1f}Время восстановления\u{1f}ค\u{e39}ลดาวน\u{e4c}\u{1f}Dolum Süresi\u{1f}冷却时间\u{1f}冷卻時間",
            "Abklingzeiten\u{1f}Cooldowns\u{1f}Tiempos de reutilización\u{1f}Tiempos de reutilización\u{1f}Temps de recharge\u{1f}Tempi di recupero\u{1f}クールダウン\u{1f}\u{1f}Czasy odnowienia\u{1f}Tempos de Recarga\u{1f}Время восстановления способностей\u{1f}ค\u{e39}ลดาวน\u{e4c}\u{1f}Dolum Süresi\u{1f}冷却时间\u{1f}冷卻時間",
            "Verderben\u{1f}Corrupt\u{1f}Corromper\u{1f}Corromper\u{1f}Corrompre\u{1f}Corrompi\u{1f}破損\u{1f}\u{1f}Uszkodź\u{1f}Corromper\u{1f}Нарушить\u{1f}ทำให\u{e49}เส\u{e35}ยหาย\u{1f}Boz\u{1f}腐化\u{1f}腐化",
            "Verdorben\u{1f}Corrupted\u{1f}Corrupto\u{1f}Corrupto\u{1f}Corrompu\u{1f}Corrotto\u{1f}破損した\u{1f}\u{1f}Uszkodzono\u{1f}Corrompido\u{1f}Нарушен\u{1f}ทำให\u{e49}เส\u{e35}ยหายแล\u{e49}ว\u{1f}Bozuk\u{1f}已腐化\u{1f}受到腐化",
            "Verderbnis\u{1f}Corrupting\u{1f}Corrompiendo\u{1f}Corrompiendo\u{1f}Corruption\u{1f}Corruzione in Corso\u{1f}破損中\u{1f}\u{1f}Uszkadzanie\u{1f}Corrompendo\u{1f}Нарушает\u{1f}กำล\u{e31}งทำให\u{e49}เส\u{e35}ยหาย\u{1f}Bozan\u{1f}正在腐化\u{1f}正在腐化",
            "\u{1f}Credit\u{1f}Crédito\u{1f}Crédito\u{1f}Crédit\u{1f}Credito\u{1f}ヒーロー・ダスト\u{1f}\u{1f}Kredyt\u{1f}Crédito\u{1f}Кредит\u{1f}เครด\u{e34}ต\u{1f}Kredi\u{1f}货币\u{1f}次數",
            "\u{1f}Credits\u{1f}Créditos\u{1f}Créditos\u{1f}Crédits\u{1f}Crediti\u{1f}ヒーロー・ダスト\u{1f}\u{1f}Kredyty\u{1f}Créditos\u{1f}Кредиты\u{1f}เครด\u{e34}ต\u{1f}Kredi\u{1f}货币\u{1f}次數",
            "Kritisch\u{1f}Critical\u{1f}Crítico\u{1f}Crítico\u{1f}Critique\u{1f}Critico\u{1f}クリティカル\u{1f}\u{1f}Krytyczne\u{1f}Crítico\u{1f}Критический\u{1f}คร\u{e34}ต\u{e34}คอล\u{1f}Kritik\u{1f}严重\u{1f}致命",
            "Ducken\u{1f}Crouch\u{1f}Agacharse\u{1f}Agacharse\u{1f}S’accroupir\u{1f}Accovacciati\u{1f}しゃがみ\u{1f}\u{1f}Kucnij\u{1f}Agachar\u{1f}Пригибаться\u{1f}ย\u{e48}อต\u{e31}ว\u{1f}Çömel\u{1f}蹲下\u{1f}蹲下",
            "Geduckt\u{1f}Crouched\u{1f}Agachado\u{1f}Agachado\u{1f}Accroupi\u{1f}Accovacciato\u{1f}しゃがんだ\u{1f}\u{1f}Kucnięto\u{1f}Agachado\u{1f}Пригнулся\u{1f}ย\u{e48}อต\u{e31}วแล\u{e49}ว\u{1f}Çömelmiş\u{1f}已蹲下\u{1f}已蹲下",
            "Ducken\u{1f}Crouching\u{1f}Agachándose\u{1f}Agachándose\u{1f}Accroupissement\u{1f}Accovacciamento in Corso\u{1f}しゃがんでいる\u{1f}\u{1f}Kucanie\u{1f}Agachando\u{1f}Пригибается\u{1f}กำล\u{e31}งย\u{e48}อต\u{e31}ว\u{1f}Çömelen\u{1f}正在蹲下\u{1f}正在蹲下",
            "Aktuell\u{1f}Current\u{1f}Actual\u{1f}Actual\u{1f}Actuel\u{1f}Attuale\u{1f}現在\u{1f}\u{1f}Bieżący\u{1f}Atual\u{1f}Текущий\u{1f}ป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut\u{1f}当前\u{1f}目前",
            "Aktuelle Verbündete\u{1f}Current Allies\u{1f}Aliados actuales\u{1f}Aliados actuales\u{1f}Alliés actuels\u{1f}Alleati Attuali\u{1f}現在の味方\u{1f}\u{1f}Bieżący sojusznicy\u{1f}Aliados Atuais\u{1f}Текущие союзники\u{1f}เพ\u{e37}\u{e48}อนร\u{e48}วมท\u{e35}มป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Müttefikler\u{1f}当前盟友\u{1f}目前盟友",
            "Aktueller Verbündeter\u{1f}Current Ally\u{1f}Aliado actual\u{1f}Aliado actual\u{1f}Allié actuel\u{1f}Alleato Attuale\u{1f}現在の味方\u{1f}\u{1f}Bieżący sojusznik\u{1f}Aliado Atual\u{1f}Текущий союзник\u{1f}เพ\u{e37}\u{e48}อนร\u{e48}วมท\u{e35}มป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Müttefik\u{1f}当前盟友\u{1f}目前盟友",
            "Aktueller Versuch\u{1f}Current Attempt\u{1f}Intento actual\u{1f}Intento actual\u{1f}Tentative actuelle\u{1f}Tentativo Attuale\u{1f}現在の挑戦\u{1f}\u{1f}Bieżąca próba\u{1f}Tentativa Atual\u{1f}Текущая попытка\u{1f}ความพยายามป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Deneme\u{1f}当前尝试\u{1f}目前機會",
            "Aktueller Checkpoint\u{1f}Current Checkpoint\u{1f}Punto de control actual\u{1f}Punto de control actual\u{1f}Point de contrôle actuel\u{1f}Checkpoint Attuale\u{1f}現在のチェックポイント\u{1f}\u{1f}Bieżący punkt kontrolny\u{1f}Ponto de Verificação Atual\u{1f}Текущая контрольная точка\u{1f}เช\u{e47}คพอยต\u{e4c}ป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Kontrol Noktası\u{1f}当前检查点\u{1f}目前檢查點",
            "Aktuelle Gegner\u{1f}Current Enemies\u{1f}Enemigos actuales\u{1f}Enemigos actuales\u{1f}Ennemis actuels\u{1f}Nemici Attuali\u{1f}現在の敵\u{1f}\u{1f}Bieżący wrogowie\u{1f}Inimigos Atuais\u{1f}Текущие враги\u{1f}ศ\u{e31}ตร\u{e39}ป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Düşmanlar\u{1f}当前敌人\u{1f}目前敵人",
            "Aktueller Gegner\u{1f}Current Enemy\u{1f}Enemigo actual\u{1f}Enemigo actual\u{1f}Ennemi actuel\u{1f}Nemico Attuale\u{1f}現在の敵\u{1f}\u{1f}Bieżący wróg\u{1f}Inimigo Atual\u{1f}Текущий враг\u{1f}ศ\u{e31}ตร\u{e39}ป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Düşman\u{1f}当前敌人\u{1f}目前敵人",
            "Aktuelle Form\u{1f}Current Form\u{1f}Forma actual\u{1f}Forma actual\u{1f}Forme actuelle\u{1f}Forma Attuale\u{1f}現在のフォーム\u{1f}\u{1f}Bieżąca forma\u{1f}Forma Atual\u{1f}Текущая форма\u{1f}ร\u{e39}ปแบบป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Biçim\u{1f}当前形态\u{1f}目前型態",
            "Aktuelles Spiel\u{1f}Current Game\u{1f}Partida actual\u{1f}Partida actual\u{1f}Partie actuelle\u{1f}Partita Attuale\u{1f}現在のマッチ\u{1f}\u{1f}Bieżąca gra\u{1f}Jogo Atual\u{1f}Текущий матч\u{1f}เกมป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Oyun\u{1f}当前游戏\u{1f}目前遊戲",
            "Aktueller Held\u{1f}Current Hero\u{1f}Héroe actual\u{1f}Héroe actual\u{1f}Héros actuel\u{1f}Eroe Attuale\u{1f}現在のヒーロー\u{1f}\u{1f}Bieżący bohater\u{1f}Herói Atual\u{1f}Текущий герой\u{1f}ฮ\u{e35}โร\u{e48}ป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Kahraman\u{1f}当前英雄\u{1f}目前英雄",
            "Aktuelle Helden\u{1f}Current Heroes\u{1f}Héroes actuales\u{1f}Héroes actuales\u{1f}Héros actuels\u{1f}Eroi Attuali\u{1f}現在のヒーロー\u{1f}\u{1f}Bieżący bohaterowie\u{1f}Heróis Atuais\u{1f}Текущие герои\u{1f}ฮ\u{e35}โร\u{e48}ป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Kahramanlar\u{1f}当前英雄\u{1f}目前英雄",
            "Aktuelle Geisel\u{1f}Current Hostage\u{1f}Rehén actual\u{1f}Rehén actual\u{1f}Otage actuel\u{1f}Ostaggio Attuale\u{1f}現在の人質\u{1f}\u{1f}Bieżący zakładnik\u{1f}Refém Atual\u{1f}Текущий заложник\u{1f}ต\u{e31}วประก\u{e31}นป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Rehine\u{1f}当前人质\u{1f}目前人質",
            "Aktuelle Geiseln\u{1f}Current Hostages\u{1f}Rehenes actuales\u{1f}Rehenes actuales\u{1f}Otages actuels\u{1f}Ostaggi Attuali\u{1f}現在の人質\u{1f}\u{1f}Bieżący zakładnicy\u{1f}Reféns Atuais\u{1f}Текущие заложники\u{1f}ต\u{e31}วประก\u{e31}นป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Rehineler\u{1f}当前人质\u{1f}目前人質",
            "Aktueller Level\u{1f}Current Level\u{1f}Nivel actual\u{1f}Nivel actual\u{1f}Niveau actuel\u{1f}Livello Attuale\u{1f}現在のレベル\u{1f}\u{1f}Bieżący poziom\u{1f}Nível Atual\u{1f}Текущий уровень\u{1f}เลเวลป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Seviye\u{1f}当前等级\u{1f}目前等級",
            "Aktuelle Mission\u{1f}Current Mission\u{1f}Misión actual\u{1f}Misión actual\u{1f}Mission actuelle\u{1f}Missione Attuale\u{1f}現在の任務\u{1f}\u{1f}Bieżąca misja\u{1f}Missão Atual\u{1f}Текущее задание\u{1f}ภารก\u{e34}จป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Görev\u{1f}当前任务\u{1f}目前任務",
            "Aktuelles Objekt\u{1f}Current Object\u{1f}Objeto actual\u{1f}Objeto actual\u{1f}Object actuel\u{1f}Oggetto Attuale\u{1f}現在のオブジェクト\u{1f}\u{1f}Bieżący obiekt\u{1f}Objeto Atual\u{1f}Текущий объект\u{1f}ว\u{e31}ตถ\u{e38}ป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Nesne\u{1f}当前对象\u{1f}目前目標",
            "Aktueller Zielpunkt\u{1f}Current Objective\u{1f}Objetivo actual\u{1f}Objetivo actual\u{1f}Objectif actuel\u{1f}Obiettivo Attuale\u{1f}現在の目標\u{1f}\u{1f}Bieżące zadanie\u{1f}Objetivo Atual\u{1f}Текущая задача\u{1f}เป\u{e49}าหมายป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Hedef\u{1f}当前目标点\u{1f}目前目標",
            "Aktuelle Objekte\u{1f}Current Objects\u{1f}Objetos actuales\u{1f}Objetos actuales\u{1f}Objets actuels\u{1f}Oggetti Attuali\u{1f}現在のオブジェクト\u{1f}\u{1f}Bieżące obiekty\u{1f}Objetos Atuais\u{1f}Текущие объекты\u{1f}ว\u{e31}ตถ\u{e38}ป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Nesneler\u{1f}当前对象\u{1f}目前目標",
            "Aktuelle Phase\u{1f}Current Phase\u{1f}Fase actual\u{1f}Fase actual\u{1f}Phase actuelle\u{1f}Fase Attuale\u{1f}現在のフェーズ\u{1f}\u{1f}Bieżąca faza\u{1f}Fase Atual\u{1f}Текущая фаза\u{1f}เฟสป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Safha\u{1f}当前阶段\u{1f}目前階段",
            "Aktueller Spieler\u{1f}Current Player\u{1f}Jugador actual\u{1f}Jugador actual\u{1f}Joueur actuel\u{1f}Giocatore Attuale\u{1f}現在のプレイヤー\u{1f}\u{1f}Bieżący gracz\u{1f}Jogador Atual\u{1f}Текущий игрок\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Oyuncu\u{1f}当前玩家\u{1f}目前玩家",
            "Aktuelle Spieler\u{1f}Current Players\u{1f}Jugadores actuales\u{1f}Jugadores actuales\u{1f}Joueurs actuels\u{1f}Giocatori Attuali\u{1f}現在のプレイヤー\u{1f}\u{1f}Bieżący gracze\u{1f}Jogadores Atuais\u{1f}Текущие игроки\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Oyuncular\u{1f}当前玩家\u{1f}目前玩家",
            "Aktuelle Runde\u{1f}Current Round\u{1f}Ronda actual\u{1f}Ronda actual\u{1f}Manche actuelle\u{1f}Round Attuale\u{1f}現在のラウンド\u{1f}\u{1f}Bieżąca runda\u{1f}Rodada Atual\u{1f}Текущий раунд\u{1f}รอบป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Tur\u{1f}当前回合\u{1f}目前回合",
            "Aktuelles Ziel\u{1f}Current Target\u{1f}Objetivo actual\u{1f}Objetivo actual\u{1f}Cible actuelle\u{1f}Bersaglio Attuale\u{1f}現在のターゲット\u{1f}\u{1f}Bieżący cel\u{1f}Alvo Atual\u{1f}Текущая цель\u{1f}เป\u{e49}าป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Hedef\u{1f}当前目标\u{1f}目前目標",
            "Aktuelle Ziele\u{1f}Current Targets\u{1f}Objetivos actuales\u{1f}Objetivos actuales\u{1f}Cibles actuelles\u{1f}Bersagli Attuali\u{1f}現在のターゲット\u{1f}\u{1f}Bieżące cele\u{1f}Alvos Atuais\u{1f}Текущие цели\u{1f}เป\u{e49}าป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Hedefler\u{1f}当前目标\u{1f}目前目標",
            "Aktuelles Upgrade\u{1f}Current Upgrade\u{1f}Mejora actual\u{1f}Mejora actual\u{1f}Amélioration actuelle\u{1f}Miglioramento Attuale\u{1f}現在のアップグレード\u{1f}\u{1f}Bieżące ulepszenie\u{1f}Melhoria Atual\u{1f}Текущее улучшение\u{1f}การอ\u{e31}พเกรดป\u{e31}จจ\u{e38}บ\u{e31}น\u{1f}Mevcut Geliştirme\u{1f}当前升级\u{1f}目前升級",
            "Schaden\u{1f}Damage\u{1f}Dañar\u{1f}Infligir daño\u{1f}Infliger des dégâts\u{1f}Danneggia\u{1f}ダメージを与える\u{1f}\u{1f}Zrań\u{1f}Danificar\u{1f}Наносить урон\u{1f}สร\u{e49}างความเส\u{e35}ยหาย\u{1f}Hasar\u{1f}伤害\u{1f}攻擊",
            "Beschädigt\u{1f}Damaged\u{1f}Dañado\u{1f}Dañado\u{1f}Endommagé\u{1f}Danneggiato\u{1f}ダメージを受けた\u{1f}\u{1f}Zraniono\u{1f}Danificado\u{1f}Нанес урон\u{1f}สร\u{e49}างความเส\u{e35}ยหายแล\u{e49}ว\u{1f}Hasar Alan\u{1f}已伤害\u{1f}受到攻擊",
            "Beschädigen\u{1f}Damaging\u{1f}Dañando\u{1f}Infligiendo daño\u{1f}Inflige des dégâts\u{1f}Danneggiamento in Corso\u{1f}ダメージを与えている\u{1f}\u{1f}Ranienie\u{1f}Danificando\u{1f}Наносит урон\u{1f}กำล\u{e31}งสร\u{e49}างความเส\u{e35}ยหาย\u{1f}Hasar Veren\u{1f}正在伤害\u{1f}正在攻擊",
            "Gefahr\u{1f}Danger\u{1f}Peligro\u{1f}Peligro\u{1f}\u{1f}Pericolo\u{1f}危険\u{1f}\u{1f}Zagrożenie\u{1f}Perigo\u{1f}Опасность\u{1f}อ\u{e31}นตราย\u{1f}Tehlike\u{1f}危险\u{1f}危險",
            "Tot\u{1f}Dead\u{1f}Muerto\u{1f}Muerto\u{1f}Mort\u{1f}Morto\u{1f}死亡した\u{1f}\u{1f}Martwy\u{1f}Morto\u{1f}Мертвый\u{1f}ตายแล\u{e49}ว\u{1f}Ölen\u{1f}已死亡\u{1f}死亡",
            "Austeilen\u{1f}Deal\u{1f}Repartir\u{1f}Repartir\u{1f}Distribuer\u{1f}Distribuisci\u{1f}ディール\u{1f}\u{1f}Rozdaj\u{1f}Dar\u{1f}Нанести\u{1f}จ\u{e31}ดการ\u{1f}Ver\u{1f}发牌\u{1f}發牌",
            "Geben\u{1f}Dealing\u{1f}Repartiendo\u{1f}Repartiendo\u{1f}Distribution\u{1f}Distribuzione in Corso\u{1f}ディールする\u{1f}\u{1f}Rozdanie\u{1f}Dando\u{1f}Наносит\u{1f}กำล\u{e31}งจ\u{e31}ดการ\u{1f}Veren\u{1f}正在发牌\u{1f}發牌",
            "Ausgeteilt\u{1f}Dealt\u{1f}Repartido\u{1f}Repartido\u{1f}Distribué\u{1f}Distribuito\u{1f}与えた\u{1f}\u{1f}Rozdano\u{1f}Dada\u{1f}Нанес\u{1f}จ\u{e31}ดการแล\u{e49}ว\u{1f}Verilen\u{1f}已发牌\u{1f}發牌",
            "\u{1f}Deck\u{1f}Mazo\u{1f}Baraja\u{1f}Tas\u{1f}Mazzo\u{1f}デッキ\u{1f}\u{1f}Talia\u{1f}Baralho\u{1f}Колода\u{1f}เคร\u{e37}\u{e48}องเล\u{e48}นเทป\u{1f}Deste\u{1f}牌堆\u{1f}牌庫",
            "\u{1f}Decks\u{1f}Mazos\u{1f}Barajas\u{1f}Tas\u{1f}Mazzi\u{1f}デッキ\u{1f}\u{1f}Talie\u{1f}Baralhos\u{1f}Колоды\u{1f}เคร\u{e37}\u{e48}องเล\u{e48}นเทป\u{1f}Desteler\u{1f}牌堆\u{1f}牌庫",
            "Niederlage\u{1f}Defeat\u{1f}Derrota\u{1f}Derrota\u{1f}Défaite\u{1f}Sconfitta\u{1f}敗北\u{1f}\u{1f}Przegrana\u{1f}Derrota\u{1f}Поражение\u{1f}ชนะ\u{1f}Yenilgi\u{1f}战败\u{1f}落敗",
            "Verteidigen\u{1f}Defend\u{1f}Defender\u{1f}Defender\u{1f}Défendre\u{1f}Difendi\u{1f}防衛\u{1f}\u{1f}Obroń\u{1f}Defender\u{1f}Оборонять\u{1f}ป\u{e49}องก\u{e31}น\u{1f}Savun\u{1f}防守\u{1f}防守",
            "Verteidigt\u{1f}Defended\u{1f}Defendido\u{1f}Defendido\u{1f}Défendu\u{1f}Difeso\u{1f}防衛した\u{1f}\u{1f}Obroniono\u{1f}Defendido\u{1f}Оборонил\u{1f}ป\u{e49}องก\u{e31}นแล\u{e49}ว\u{1f}Savunulan\u{1f}已防守\u{1f}受到防守",
            "Beschützen\u{1f}Defending\u{1f}Defendiendo\u{1f}Defendiendo\u{1f}Défense\u{1f}Difesa in Corso\u{1f}防衛中\u{1f}\u{1f}Bronienie\u{1f}Defendendo\u{1f}Обороняет\u{1f}กำล\u{e31}งป\u{e49}องก\u{e31}น\u{1f}Savunan\u{1f}正在防守\u{1f}正在防守",
            "Verteidigung\u{1f}Defense\u{1f}Defensa\u{1f}Defensa\u{1f}Défense\u{1f}Difesa\u{1f}ディフェンス\u{1f}\u{1f}Obrona\u{1f}Defesa\u{1f}Защита\u{1f}การป\u{e49}องก\u{e31}น\u{1f}Savunma\u{1f}防御\u{1f}防禦",
            "Liefern\u{1f}Deliver\u{1f}Entregar\u{1f}Entregar\u{1f}Livrer\u{1f}Consegna\u{1f}移送\u{1f}\u{1f}Dostarcz\u{1f}Entregar\u{1f}Доставить\u{1f}ส\u{e48}ง\u{1f}Teslim Et\u{1f}送达\u{1f}抵達",
            "Geliefert\u{1f}Delivered\u{1f}Entregado\u{1f}Entregado\u{1f}Livré\u{1f}Consegnato\u{1f}移送した\u{1f}\u{1f}Dostarczono\u{1f}Entregue\u{1f}Доставил\u{1f}ส\u{e48}งแล\u{e49}ว\u{1f}Teslim Edilen\u{1f}已送达\u{1f}已抵達",
            "Lieferung\u{1f}Delivering\u{1f}Entregando\u{1f}Entregando\u{1f}Livraison\u{1f}Consegna in Corso\u{1f}移送中\u{1f}\u{1f}Dostarczanie\u{1f}Entregando\u{1f}Доставляет\u{1f}กำล\u{e31}งส\u{e48}ง\u{1f}Teslim Ediliyor\u{1f}正在送达\u{1f}正在抵達",
            "Tiefe\u{1f}Depth\u{1f}Profundidad\u{1f}Profundidad\u{1f}Profondeur\u{1f}Profondità\u{1f}深さ\u{1f}\u{1f}Głębokość\u{1f}Profundidade\u{1f}Глубина\u{1f}ความล\u{e36}ก\u{1f}Derinlik\u{1f}深度\u{1f}深度",
            "Destabilisieren\u{1f}Destabilize\u{1f}Desestabilizar\u{1f}Desestabilizar\u{1f}Déstabiliser\u{1f}Destabilizza\u{1f}不安定化\u{1f}\u{1f}Destabilizuj\u{1f}Desestabilizar\u{1f}Дестабилизировать\u{1f}ทำให\u{e49}ไม\u{e48}เสถ\u{e35}ยร\u{1f}Etkisiz Hâle Getirme\u{1f}干扰\u{1f}動搖",
            "Destabilisiert\u{1f}Destabilized\u{1f}Desestabilizado\u{1f}Desestabilizado\u{1f}Déstabilisé\u{1f}Destabilizzato\u{1f}不安定化した\u{1f}\u{1f}Zdestabilizowano\u{1f}Desestabilizado\u{1f}Дестабилизировал\u{1f}ทำให\u{e49}ไม\u{e48}เสถ\u{e35}ยรแล\u{e49}ว\u{1f}Etkisiz Hâle Gelen\u{1f}已干扰\u{1f}受到動搖",
            "Destabilisierung\u{1f}Destabilizing\u{1f}Desestabilizando\u{1f}Desestabilizando\u{1f}Déstabilisation\u{1f}Destabilizzazione in Corso\u{1f}不安定化している\u{1f}\u{1f}Destablizowanie\u{1f}Desestabilizando\u{1f}Дестабилизирует\u{1f}กำล\u{e31}งทำให\u{e49}ไม\u{e48}เสถ\u{e35}ยร\u{1f}Etkisiz Hâle Getiren\u{1f}正在干扰\u{1f}正在動搖",
            "Zerstören\u{1f}Destroy\u{1f}Destruir\u{1f}Destruir\u{1f}Détruire\u{1f}Distruggi\u{1f}破壊\u{1f}\u{1f}Zniszcz\u{1f}Destruir\u{1f}Уничтожить\u{1f}ทำลาย\u{1f}Yok Et\u{1f}摧毁\u{1f}摧毀",
            "Zerstört\u{1f}Destroyed\u{1f}Destruido\u{1f}Destruido\u{1f}Détruit\u{1f}Distrutto\u{1f}破壊した\u{1f}\u{1f}Zniszczono\u{1f}Destruído\u{1f}Уничтожен\u{1f}ทำลายแล\u{e49}ว\u{1f}Yok Edilen\u{1f}已摧毁\u{1f}遭到摧毀",
            "Zerstörung\u{1f}Destroying\u{1f}Destruyendo\u{1f}Destruyendo\u{1f}Destruction\u{1f}Distruzione in Corso\u{1f}破壊中\u{1f}\u{1f}Niszczenie\u{1f}Destruindo\u{1f}Уничтожает\u{1f}กำล\u{e31}งทำลาย\u{1f}Yok Eden\u{1f}正在摧毁\u{1f}正在摧毀",
            "Entdecken\u{1f}Detect\u{1f}Detectar\u{1f}Detectar\u{1f}Détecter\u{1f}Individua\u{1f}発見する\u{1f}\u{1f}Wykryj\u{1f}Detectar\u{1f}Обнаружить\u{1f}ตรวจจ\u{e31}บ\u{1f}Algılama\u{1f}探测\u{1f}偵測",
            "Entdeckt\u{1f}Detected\u{1f}Detectado\u{1f}Detectado\u{1f}Détecté\u{1f}Individuato\u{1f}発見された\u{1f}\u{1f}Wykryto\u{1f}Detectado\u{1f}Обнаружили\u{1f}ตรวจพบ\u{1f}Algılandı\u{1f}被探测\u{1f}遭到偵測",
            "Entdeckung\u{1f}Detecting\u{1f}Detectando\u{1f}Detectando\u{1f}Détection\u{1f}Individuazione in Corso\u{1f}発見している\u{1f}\u{1f}Wykrywanie\u{1f}Detectando\u{1f}Обнаружение\u{1f}กำล\u{e31}งตรวจจ\u{e31}บ\u{1f}Algılayan\u{1f}正在探测\u{1f}正在偵測",
            "Geschicklichkeit\u{1f}Dexterity\u{1f}Destreza\u{1f}Destreza\u{1f}Dextérité\u{1f}Destrezza\u{1f}敏捷性\u{1f}\u{1f}Zręczność\u{1f}Destreza\u{1f}Ловкость\u{1f}ความว\u{e48}องไว\u{1f}Beceriklilik\u{1f}灵巧\u{1f}靈敏",
            "Karo\u{1f}Diamond\u{1f}Diamante\u{1f}Diamante\u{1f}Carreau\u{1f}Quadro\u{1f}ダイヤモンド\u{1f}\u{1f}Karo\u{1f}Ouros\u{1f}Бубны\u{1f}ข\u{e49}าวหลามต\u{e31}ด\u{1f}Karo\u{1f}方块\u{1f}方塊",
            "Karo\u{1f}Diamonds\u{1f}Diamantes\u{1f}Diamantes\u{1f}Carreau\u{1f}Quadri\u{1f}ダイヤモンド\u{1f}\u{1f}Karo\u{1f}Ouros\u{1f}Бубны\u{1f}ไดมอนด\u{e4c}\u{1f}Karolar\u{1f}方块\u{1f}方塊",
            "Sterben\u{1f}Die\u{1f}Morir\u{1f}Morir\u{1f}Mourir\u{1f}Muori\u{1f}死亡\u{1f}\u{1f}Zgiń\u{1f}Morrer\u{1f}Умереть\u{1f}ตาย\u{1f}Öl\u{1f}死亡\u{1f}死亡",
            "Abwerfen\u{1f}Discard\u{1f}Descartar\u{1f}Descartar\u{1f}Défausser\u{1f}Scarta\u{1f}破棄\u{1f}\u{1f}Odrzuć\u{1f}Descartar\u{1f}Сбросить\u{1f}ท\u{e34}\u{e49}ง\u{1f}At\u{1f}弃牌\u{1f}棄牌",
            "Abgeworfen\u{1f}Discarded\u{1f}Descartado\u{1f}Descartado\u{1f}Défaussé\u{1f}Scartato\u{1f}破棄された\u{1f}\u{1f}Odrzucono\u{1f}Descartada\u{1f}Сбросил\u{1f}ท\u{e34}\u{e49}งแล\u{e49}ว\u{1f}Atılan\u{1f}已弃牌\u{1f}棄牌",
            "Verwerfen\u{1f}Discarding\u{1f}Descartando\u{1f}Descartando\u{1f}Défausse\u{1f}Scarto in Corso\u{1f}破棄している\u{1f}\u{1f}Odrzucanie\u{1f}Descartando\u{1f}Сбрасывает\u{1f}กำล\u{e31}งท\u{e34}\u{e49}ง\u{1f}Atan\u{1f}正在弃牌\u{1f}棄牌",
            "Unterbrechen\u{1f}Disconnect\u{1f}Desconectar\u{1f}Desconectar\u{1f}Déconnecter\u{1f}Disconnetti\u{1f}接続を解除\u{1f}\u{1f}Rozłącz\u{1f}Desconectar\u{1f}Отключиться\u{1f}ต\u{e31}ดการเช\u{e37}\u{e48}อมต\u{e48}อ\u{1f}Bağlantıyı Kes\u{1f}断开\u{1f}中斷連接",
            "Unterbrochen\u{1f}Disconnected\u{1f}Desconectado\u{1f}Desconectado\u{1f}Déconnecté(e)\u{1f}Disconnesso\u{1f}接続を解除した\u{1f}\u{1f}Rozłączono\u{1f}Desconectado\u{1f}Отключен\u{1f}ต\u{e31}ดการเช\u{e37}\u{e48}อมต\u{e48}อแล\u{e49}ว\u{1f}Bağlantı Kesildi\u{1f}已断开\u{1f}中斷連接",
            "Unterbrechung\u{1f}Disconnecting\u{1f}Desconectando\u{1f}Desconectándose\u{1f}Déconnexion\u{1f}Disconnessione in Corso\u{1f}接続を解除中\u{1f}\u{1f}Rozłączanie\u{1f}Desconectando\u{1f}Отключается\u{1f}กำล\u{e31}งต\u{e31}ดการเช\u{e37}\u{e48}อมต\u{e48}อ\u{1f}Bağlantı Kesiliyor\u{1f}正在断开\u{1f}正在中斷連接",
            "Distanz\u{1f}Distance\u{1f}Distancia\u{1f}Distancia\u{1f}\u{1f}Distanza\u{1f}距離\u{1f}\u{1f}Odległość\u{1f}Distância\u{1f}Расстояние\u{1f}ระยะทาง\u{1f}Mesafe\u{1f}距离\u{1f}距離",
            "Distanzen\u{1f}Distances\u{1f}Distancias\u{1f}Distancias\u{1f}\u{1f}Distanze\u{1f}距離\u{1f}\u{1f}Odległości\u{1f}Distâncias\u{1f}Расстояния\u{1f}ระยะทาง\u{1f}Mesafeler\u{1f}距离\u{1f}距離",
            "Ausweichen\u{1f}Dodge\u{1f}Esquivar\u{1f}Esquivar\u{1f}Esquiver\u{1f}Schiva\u{1f}回避\u{1f}\u{1f}Uskocz\u{1f}Esquivar\u{1f}Уклониться\u{1f}หลบหล\u{e35}ก\u{1f}Atlat\u{1f}躲闪\u{1f}閃躲",
            "Ausgewichen\u{1f}Dodged\u{1f}Esquivado\u{1f}Esquivado\u{1f}Esquivé\u{1f}Schivato\u{1f}回避した\u{1f}\u{1f}Uskoczono\u{1f}Esquivado\u{1f}Уклонился\u{1f}หลบหล\u{e35}กแล\u{e49}ว\u{1f}Atlatılan\u{1f}已躲闪\u{1f}遭到閃躲",
            "Ausweichen\u{1f}Dodging\u{1f}Esquivando\u{1f}Esquivando\u{1f}Esquive\u{1f}Schivata in Corso\u{1f}回避中\u{1f}\u{1f}Uskakiwanie\u{1f}Esquivando\u{1f}Уклоняется\u{1f}กำล\u{e31}งหลบหล\u{e35}ก\u{1f}Atlatan\u{1f}正在躲闪\u{1f}正在閃躲",
            "Kuppel\u{1f}Dome\u{1f}Cúpula\u{1f}Domo\u{1f}Dôme\u{1f}Cupola\u{1f}ドーム\u{1f}\u{1f}Kopuła\u{1f}Domo\u{1f}Купол\u{1f}โดม\u{1f}Kubbe\u{1f}罩子\u{1f}穹頂",
            "Kuppeln\u{1f}Domes\u{1f}Cúpula\u{1f}Domos\u{1f}Dômes\u{1f}Cupole\u{1f}ドーム\u{1f}\u{1f}Kopuły\u{1f}Domos\u{1f}Купола\u{1f}โดม\u{1f}Kubbe\u{1f}罩子\u{1f}穹頂",
            "Nach unten\u{1f}Down\u{1f}Abajo\u{1f}Abajo\u{1f}Bas\u{1f}Giù\u{1f}下\u{1f}\u{1f}Dół\u{1f}Baixo\u{1f}Вниз\u{1f}ด\u{e49}านล\u{e48}าง\u{1f}Ekle\u{1f}下\u{1f}下",
            "Herunterladen\u{1f}Download\u{1f}Descargar\u{1f}Descargar\u{1f}Télécharger\u{1f}\u{1f}ダウンロード\u{1f}\u{1f}Pobierz\u{1f}Baixar\u{1f}Загрузить\u{1f}ดาวน\u{e4c}โหลด\u{1f}İndir\u{1f}下载\u{1f}下載",
            "Heruntergeladen\u{1f}Downloaded\u{1f}Descargado\u{1f}Descargado\u{1f}Téléchargé\u{1f}Download Effettuato\u{1f}ダウンロードした\u{1f}\u{1f}Pobrano\u{1f}Baixado\u{1f}Загрузил\u{1f}ดาวน\u{e4c}โหลดแล\u{e49}ว\u{1f}İndirilen\u{1f}已下载\u{1f}已下載",
            "Download\u{1f}Downloading\u{1f}Descargando\u{1f}Descargando\u{1f}Téléchargement\u{1f}Download in Corso\u{1f}ダウンロード中\u{1f}\u{1f}Pobieranie\u{1f}Baixando\u{1f}Загружает\u{1f}กำล\u{e31}งดาวน\u{e4c}โหลด\u{1f}İndiren\u{1f}正在下载\u{1f}正在下載",
            "Unentschieden\u{1f}Draw\u{1f}Empate\u{1f}Empate\u{1f}Match nul\u{1f}Pareggio\u{1f}引き分け\u{1f}\u{1f}Remis\u{1f}Empate\u{1f}Ничья\u{1f}เสมอ\u{1f}Çek\u{1f}平局\u{1f}平手",
            "Ziehen\u{1f}Drawing\u{1f}Robando\u{1f}Revelando\u{1f}Pioche\u{1f}Pescare\u{1f}ドローしている\u{1f}\u{1f}Dobywanie\u{1f}Comprando\u{1f}Берет\u{1f}กำล\u{e31}งด\u{e36}ง\u{1f}Çeken\u{1f}正在抽牌\u{1f}抽牌",
            "Gezogen\u{1f}Drawn\u{1f}Robado\u{1f}Revelado\u{1f}Pioché\u{1f}Pescato\u{1f}ドローした\u{1f}\u{1f}Dobyto\u{1f}Comprada\u{1f}Взял\u{1f}ด\u{e36}งแล\u{e49}ว\u{1f}Çekilen\u{1f}已抽牌\u{1f}抽牌",
            "Fallen lassen\u{1f}Drop\u{1f}Soltar\u{1f}Soltar\u{1f}Larguer\u{1f}Lascia\u{1f}落とす\u{1f}\u{1f}Upuść\u{1f}Largar\u{1f}Бросить\u{1f}ตก\u{1f}Düş\u{1f}掉落\u{1f}掉落",
            "Fallen gelassen\u{1f}Dropped\u{1f}Soltado\u{1f}Soltado\u{1f}Largué\u{1f}Lasciato\u{1f}落とされた\u{1f}\u{1f}Upuszczono\u{1f}Largado\u{1f}Бросил\u{1f}ตกแล\u{e49}ว\u{1f}Düşen\u{1f}已掉落\u{1f}已掉落",
            "Fall\u{1f}Dropping\u{1f}Soltando\u{1f}Soltando\u{1f}Largage\u{1f}Rilascio in Corso\u{1f}落としている\u{1f}\u{1f}Upuszczanie\u{1f}Largando\u{1f}Бросает\u{1f}กำล\u{e31}งตก\u{1f}Düşüren\u{1f}正在掉落\u{1f}正在掉落",
            "Stirb\u{1f}Dying\u{1f}Muriendo\u{1f}Muriéndose\u{1f}Mort\u{1f}Morte in Corso\u{1f}死亡中\u{1f}\u{1f}Umiera\u{1f}Morrendo\u{1f}Умирает\u{1f}กำล\u{e31}งตาย\u{1f}Ölen\u{1f}正在死亡\u{1f}正在死亡",
            "Osten\u{1f}East\u{1f}Este\u{1f}Este\u{1f}Est\u{1f}Est\u{1f}東\u{1f}\u{1f}Wschód\u{1f}Leste\u{1f}Восток\u{1f}ตะว\u{e31}นออก\u{1f}Doğu\u{1f}东\u{1f}東",
            "Eliminieren\u{1f}Eliminate\u{1f}Eliminar\u{1f}Eliminar\u{1f}Éliminer\u{1f}Elimina\u{1f}排除\u{1f}\u{1f}Eliminuj\u{1f}Eliminar\u{1f}Убить\u{1f}กำจ\u{e31}ด\u{1f}Bertaraf\u{1f}消灭\u{1f}擊殺",
            "Eliminiert\u{1f}Eliminated\u{1f}Eliminado\u{1f}Eliminado\u{1f}Éliminé\u{1f}Eliminato\u{1f}排除した\u{1f}\u{1f}Wyeliminowano\u{1f}Eliminado\u{1f}Убит\u{1f}กำจ\u{e31}ดแล\u{e49}ว\u{1f}Bertaraf Edilen\u{1f}已消灭\u{1f}遭到擊殺",
            "Ausschalten\u{1f}Eliminating\u{1f}Eliminando\u{1f}Eliminando\u{1f}Élimination\u{1f}Eliminazione in Corso\u{1f}排除中\u{1f}\u{1f}Eliminowanie\u{1f}Eliminando\u{1f}Убивает\u{1f}กำล\u{e31}งกำจ\u{e31}ด\u{1f}Bertaraf Eden\u{1f}正在消灭\u{1f}正在擊殺",
            "Eliminierung\u{1f}Elimination\u{1f}Eliminación\u{1f}Eliminación\u{1f}Élimination\u{1f}Eliminazione\u{1f}排除\u{1f}\u{1f}Likwidacja\u{1f}Eliminação\u{1f}Убийство\u{1f}การกำจ\u{e31}ด\u{1f}Bertaraf\u{1f}消灭\u{1f}擊殺",
            "Eliminierungen\u{1f}Eliminations\u{1f}Eliminaciones\u{1f}Eliminaciones\u{1f}Éliminations\u{1f}Eliminazioni\u{1f}排除\u{1f}\u{1f}Likwidacje\u{1f}Eliminações\u{1f}Убийства\u{1f}การกำจ\u{e31}ด\u{1f}Bertaraf\u{1f}消灭\u{1f}擊殺",
            "Feinde\u{1f}Enemies\u{1f}Enemigos\u{1f}Enemigos\u{1f}Ennemis\u{1f}Nemici\u{1f}敵\u{1f}\u{1f}Wrogowie\u{1f}Inimigos\u{1f}Враги\u{1f}ศ\u{e31}ตร\u{e39}\u{1f}Düşmanlar\u{1f}敌人\u{1f}敵人",
            "Gegner\u{1f}Enemy\u{1f}Enemigo\u{1f}Enemigo\u{1f}Ennemi\u{1f}Nemico\u{1f}敵\u{1f}\u{1f}Wróg\u{1f}Inimigo\u{1f}Враг\u{1f}ศ\u{e31}ตร\u{e39}\u{1f}Düşman\u{1f}敌人\u{1f}敵人",
            "Eingang\u{1f}Entrance\u{1f}Entrada\u{1f}Entrada\u{1f}Entrée\u{1f}Entrata\u{1f}入口\u{1f}\u{1f}Wejście\u{1f}Entrada\u{1f}Вход\u{1f}ทางเข\u{e49}า\u{1f}Giriş\u{1f}进入\u{1f}入口",
            "Eskortieren\u{1f}Escort\u{1f}Escoltar\u{1f}Escolta\u{1f}Escorter\u{1f}Trasporta\u{1f}エスコート\u{1f}\u{1f}Eskortuj\u{1f}Escolta\u{1f}Сопровождать\u{1f}ค\u{e38}\u{e49}มก\u{e31}น\u{1f}Koruma\u{1f}护送\u{1f}護送",
            "Eskortiert\u{1f}Escorted\u{1f}Escoltado\u{1f}Escoltado\u{1f}Escorté\u{1f}Trasportato\u{1f}エスコートした\u{1f}\u{1f}Eskortowano\u{1f}Escoltado\u{1f}Сопроводил\u{1f}ค\u{e38}\u{e49}มก\u{e31}นแล\u{e49}ว\u{1f}Korunan\u{1f}已护送\u{1f}護送",
            "Eskorte\u{1f}Escorting\u{1f}Escoltando\u{1f}Escoltando\u{1f}Escorte\u{1f}Trasporto in Corso\u{1f}エスコート中\u{1f}\u{1f}Eskortowanie\u{1f}Escoltando\u{1f}Сопровождает\u{1f}กำล\u{e31}งค\u{e38}\u{e49}มก\u{e31}น\u{1f}Koruyan\u{1f}正在护送\u{1f}正在護送",
            "Ausgezeichnet\u{1f}Excellent\u{1f}Excelente\u{1f}Excelente\u{1f}\u{1f}Eccellente\u{1f}素晴らしい\u{1f}\u{1f}Doskonale\u{1f}Excelente\u{1f}Отлично\u{1f}ยอดเย\u{e35}\u{e48}ยม\u{1f}Harika\u{1f}优秀\u{1f}優異",
            "Verlassen\u{1f}Exit\u{1f}Salida\u{1f}Salida\u{1f}Sortie\u{1f}Uscita\u{1f}終了\u{1f}\u{1f}Wyjście\u{1f}Saída\u{1f}Выход\u{1f}ทางออก\u{1f}Çıkış\u{1f}离开\u{1f}出口",
            "Erfahrung\u{1f}Experience\u{1f}Experiencia\u{1f}Experiencia\u{1f}Expérience\u{1f}Esperienza\u{1f}経験値\u{1f}\u{1f}Doświadczenie\u{1f}Experiência\u{1f}Опыт\u{1f}ประสบการณ\u{e4c}\u{1f}Tecrübe\u{1f}经验值\u{1f}經驗值",
            "Extrem\u{1f}Extreme\u{1f}Extremo\u{1f}Extremo\u{1f}Extrême\u{1f}Estremo\u{1f}最高\u{1f}\u{1f}Ekstremalnie\u{1f}Extremo\u{1f}Предельный\u{1f}ส\u{e38}ดข\u{e35}ด\u{1f}Ekstrem\u{1f}极端\u{1f}極致",
            "Ansehen\u{1f}Face\u{1f}Mirar a\u{1f}Mirar\u{1f}Regarder vers\u{1f}Guarda\u{1f}向く\u{1f}\u{1f}Skieruj\u{1f}Encarar\u{1f}Смотреть на\u{1f}หน\u{e49}า\u{1f}Bak\u{1f}面向\u{1f}面向",
            "Gesicht\u{1f}Faces\u{1f}Mira a\u{1f}Mira\u{1f}Regarde vers\u{1f}Guardato\u{1f}向く\u{1f}\u{1f}Skierowano\u{1f}Encara\u{1f}Смотрит на\u{1f}หน\u{e49}า\u{1f}Bakıyor\u{1f}面向\u{1f}面向",
            "Ausrichtung\u{1f}Facing\u{1f}Mirando a\u{1f}Mirando\u{1f}Face à\u{1f}Sguardo in Corso\u{1f}向いている\u{1f}\u{1f}Skierowanie\u{1f}Encarando\u{1f}Смотрит в сторону\u{1f}การห\u{e31}นหน\u{e49}า\u{1f}Konumlandırma\u{1f}正在面向\u{1f}面向",
            "Gescheitert\u{1f}Failed\u{1f}Fallido\u{1f}Fallido\u{1f}Échoué\u{1f}Fallito\u{1f}失敗\u{1f}\u{1f}Nieudane\u{1f}Falhou\u{1f}Потерпел неудачу\u{1f}ล\u{e49}มเหลว\u{1f}Başarısız\u{1f}已失败\u{1f}失敗",
            "Scheitern\u{1f}Failing\u{1f}Fallando\u{1f}Fallando\u{1f}Échec\u{1f}Fallimento in Corso\u{1f}失敗している\u{1f}\u{1f}Zawodzi\u{1f}Falhando\u{1f}Терпит неудачу\u{1f}กำล\u{e31}งล\u{e49}มเหลว\u{1f}Başarısız\u{1f}即将失败\u{1f}失敗",
            "Fehlschlag\u{1f}Failure\u{1f}Fallo\u{1f}Fracaso\u{1f}Échec\u{1f}Fallimento\u{1f}失敗\u{1f}\u{1f}Porażka\u{1f}Fracasso\u{1f}Неудача\u{1f}ความล\u{e49}มเหลว\u{1f}Başarısızlık\u{1f}失败\u{1f}失敗",
            "Fallen\u{1f}Fall\u{1f}Caer\u{1f}Caer\u{1f}Tomber\u{1f}Cade\u{1f}落下する\u{1f}\u{1f}Upadnij\u{1f}Cair\u{1f}Упасть\u{1f}ร\u{e48}วง\u{1f}Düş\u{1f}跌落\u{1f}墜落",
            "Gefallen\u{1f}Fallen\u{1f}Caído\u{1f}Caído\u{1f}Tombé\u{1f}Caduto\u{1f}落下した\u{1f}\u{1f}Upadnięto\u{1f}Caído\u{1f}Упал\u{1f}ร\u{e48}วงแล\u{e49}ว\u{1f}Düşen\u{1f}已跌落\u{1f}已墜落",
            "Falle\u{1f}Falling\u{1f}Cayendo\u{1f}Cayendo\u{1f}Chute\u{1f}Caduta in Corso\u{1f}落下\u{1f}\u{1f}Upadanie\u{1f}Caindo\u{1f}Падает\u{1f}กำล\u{e31}งร\u{e48}วง\u{1f}Düşüren\u{1f}正在跌落\u{1f}正在墜落",
            "Entfernt\u{1f}Far\u{1f}Lejos\u{1f}Lejos\u{1f}Loin\u{1f}Lontano\u{1f}遠く\u{1f}\u{1f}Daleko\u{1f}Longe\u{1f}Далеко\u{1f}ไกล\u{1f}Uzak\u{1f}远\u{1f}遠",
            "Schnell\u{1f}Fast\u{1f}Rápido\u{1f}Rápido\u{1f}Rapide\u{1f}Veloce\u{1f}速い\u{1f}\u{1f}Szybko\u{1f}Rápido\u{1f}Быстро\u{1f}เร\u{e47}ว\u{1f}Hızlı\u{1f}快\u{1f}快",
            "Schneller\u{1f}Faster\u{1f}Más rápido\u{1f}Más rápido\u{1f}Plus rapide\u{1f}Più Veloce\u{1f}より速い\u{1f}\u{1f}Szybciej\u{1f}Mais Rápido\u{1f}Быстрее\u{1f}เร\u{e47}วกว\u{e48}า\u{1f}Daha Hızlı\u{1f}较快\u{1f}較快",
            "Am schnellsten\u{1f}Fastest\u{1f}El más rápido\u{1f}Mucho más rápido\u{1f}Le plus rapide\u{1f}Velocissimo\u{1f}最速\u{1f}\u{1f}Najszybciej\u{1f}O Mais Rápido\u{1f}Очень быстро\u{1f}เร\u{e47}วท\u{e35}\u{e48}ส\u{e38}ด\u{1f}En Hızlı\u{1f}最快\u{1f}最快",
            "Fehler\u{1f}Fault\u{1f}Falta\u{1f}Falla\u{1f}Faute\u{1f}Colpa\u{1f}失敗\u{1f}\u{1f}Uskok\u{1f}Falta\u{1f}Дефект\u{1f}ข\u{e49}อบกพร\u{e48}อง\u{1f}Hata\u{1f}失误\u{1f}錯誤",
            "Schuld\u{1f}Faults\u{1f}Faltas\u{1f}Fallas\u{1f}Fautes\u{1f}Colpe\u{1f}失敗\u{1f}\u{1f}Uskoki\u{1f}Faltas\u{1f}Дефекты\u{1f}ข\u{e49}อบกพร\u{e48}อง\u{1f}Hatalar\u{1f}失误\u{1f}錯誤",
            "Zuletzt\u{1f}Final\u{1f}\u{1f}\u{1f}\u{1f}Finale\u{1f}最終\u{1f}\u{1f}Finalny\u{1f}\u{1f}Конечный\u{1f}ส\u{e38}ดท\u{e49}าย\u{1f}Son\u{1f}最终\u{1f}最終",
            "Letzte Verbündete\u{1f}Final Allies\u{1f}Aliados finales\u{1f}Aliados finales\u{1f}Alliés finaux\u{1f}Alleati Finali\u{1f}最終の味方\u{1f}\u{1f}Finalni sojusznicy\u{1f}Aliados Finais\u{1f}Конечные союзники\u{1f}เพ\u{e37}\u{e48}อนร\u{e48}วมท\u{e35}มส\u{e38}ดท\u{e49}าย\u{1f}Son Müttefikler\u{1f}最终盟友\u{1f}最終盟友",
            "Letzter Verbündeter\u{1f}Final Ally\u{1f}Aliado final\u{1f}Aliado final\u{1f}Allié final\u{1f}Alleato Finale\u{1f}最終の味方\u{1f}\u{1f}Finalny sojusznik\u{1f}Aliado Final\u{1f}Конечный союзник\u{1f}เพ\u{e37}\u{e48}อนร\u{e48}วมท\u{e35}มส\u{e38}ดท\u{e49}าย\u{1f}Son Müttefik\u{1f}最终盟友\u{1f}最終盟友",
            "Letzter Versuch\u{1f}Final Attempt\u{1f}Intento final\u{1f}Intento final\u{1f}Tentative finale\u{1f}Tentativo Finale\u{1f}最終挑戦\u{1f}\u{1f}Finalna próba\u{1f}Tentativa Final\u{1f}Конечная попытка\u{1f}ความพยายามคร\u{e31}\u{e49}งส\u{e38}ดท\u{e49}าย\u{1f}Son Deneme\u{1f}最终尝试\u{1f}最終機會",
            "Letzter Checkpoint\u{1f}Final Checkpoint\u{1f}Punto de control final\u{1f}Punto de control final\u{1f}Point de contrôle final\u{1f}Checkpoint Finale\u{1f}最終チェックポイント\u{1f}\u{1f}Finalny punkt kontrolny\u{1f}Ponto de Verificação Final\u{1f}Конечная контрольная точка\u{1f}เช\u{e47}คพอยต\u{e4c}ส\u{e38}ดท\u{e49}าย\u{1f}Son Kontrol Noktası\u{1f}最终检查点\u{1f}最終檢查點",
            "Letzte Gegner\u{1f}Final Enemies\u{1f}Enemigos finales\u{1f}Enemigos finales\u{1f}Ennemis finaux\u{1f}Nemici Finali\u{1f}最終の敵\u{1f}\u{1f}Finalni wrogowie\u{1f}Inimigos Finais\u{1f}Конечные враги\u{1f}ศ\u{e31}ตร\u{e39}ส\u{e38}ดท\u{e49}าย\u{1f}Son Düşmanlar\u{1f}最终敌人\u{1f}最終敵人",
            "Letzter Gegner\u{1f}Final Enemy\u{1f}Enemigo final\u{1f}Enemigo final\u{1f}Ennemi final\u{1f}Nemico Finale\u{1f}最終の敵\u{1f}\u{1f}Finalny wróg\u{1f}Inimigo Final\u{1f}Конечный враг\u{1f}ศ\u{e31}ตร\u{e39}ส\u{e38}ดท\u{e49}าย\u{1f}Son Düşman\u{1f}最终敌人\u{1f}最終敵人",
            "Letzte Form\u{1f}Final Form\u{1f}Forma final\u{1f}Forma final\u{1f}Forme finale\u{1f}Forma Finale\u{1f}最終フォーム\u{1f}\u{1f}Finalna postać\u{1f}Forma Final\u{1f}Конечная форма\u{1f}ร\u{e48}างส\u{e38}ดท\u{e49}าย\u{1f}Son Biçim\u{1f}最终形态\u{1f}最終型態",
            "Letztes Spiel\u{1f}Final Game\u{1f}Partida final\u{1f}Partida final\u{1f}Partie finale\u{1f}Partita Finale\u{1f}最終マッチ\u{1f}\u{1f}Finalna gra\u{1f}Jogo Final\u{1f}Конечный матч\u{1f}เกมส\u{e38}ดท\u{e49}าย\u{1f}Son Oyun\u{1f}最终游戏\u{1f}最終遊戲",
            "Letzter Held\u{1f}Final Hero\u{1f}Héroe final\u{1f}Héroe final\u{1f}Héros final\u{1f}Eroe Finale\u{1f}最終ヒーロー\u{1f}\u{1f}Finalny bohater\u{1f}Herói Final\u{1f}Конечный герой\u{1f}ฮ\u{e35}โร\u{e48}ส\u{e38}ดท\u{e49}าย\u{1f}Son Kahraman\u{1f}最终英雄\u{1f}最終英雄",
            "Letzte Helden\u{1f}Final Heroes\u{1f}Héroes finales\u{1f}Héroes finales\u{1f}Héros finaux\u{1f}Eroi Finali\u{1f}最終ヒーロー\u{1f}\u{1f}Finalni bohaterowie\u{1f}Heróis Finais\u{1f}Конечные герои\u{1f}ฮ\u{e35}โร\u{e48}ส\u{e38}ดท\u{e49}าย\u{1f}Son Kahramanlar\u{1f}最终英雄\u{1f}最終英雄",
            "Letzte Geisel\u{1f}Final Hostage\u{1f}Rehén final\u{1f}Rehén final\u{1f}Otage final\u{1f}Ostaggio Finale\u{1f}最終人質\u{1f}\u{1f}Finalny zakładnik\u{1f}Refém Final\u{1f}Конечный заложник\u{1f}ต\u{e31}วประก\u{e31}นส\u{e38}ดท\u{e49}าย\u{1f}Son Rehine\u{1f}最终人质\u{1f}最終人質",
            "Letzte Geiseln\u{1f}Final Hostages\u{1f}Rehenes finales\u{1f}Rehenes finales\u{1f}Otages finaux\u{1f}Ostaggi Finali\u{1f}最終人質\u{1f}\u{1f}Finalni zakładnicy\u{1f}Reféns Finais\u{1f}Конечные заложники\u{1f}ต\u{e31}วประก\u{e31}นส\u{e38}ดท\u{e49}าย\u{1f}Son Rehineler\u{1f}最终人质\u{1f}最終人質",
            "Letzter Gegenstand\u{1f}Final Item\u{1f}Objeto final\u{1f}Objeto final\u{1f}Article final\u{1f}Oggetto Finale\u{1f}最終アイテム\u{1f}\u{1f}Finalny element\u{1f}Item Final\u{1f}Конечный предмет\u{1f}ไอเท\u{e47}มส\u{e38}ดท\u{e49}าย\u{1f}Son Eşya\u{1f}最终物品\u{1f}最終物品",
            "Letzter Level\u{1f}Final Level\u{1f}Nivel final\u{1f}Nivel final\u{1f}Niveau final\u{1f}Livello Finale\u{1f}最終レベル\u{1f}\u{1f}Finalny poziom\u{1f}Nível Final\u{1f}Конечный уровень\u{1f}เลเวลส\u{e38}ดท\u{e49}าย\u{1f}Son Seviye\u{1f}最终等级\u{1f}最終等級",
            "Letzte Mission\u{1f}Final Mission\u{1f}Misión final\u{1f}Misión final\u{1f}Mission finale\u{1f}Missione Finale\u{1f}最終任務\u{1f}\u{1f}Finalna misja\u{1f}Missão Final\u{1f}Конечное задание\u{1f}ภารก\u{e34}จส\u{e38}ดท\u{e49}าย\u{1f}Son Görev\u{1f}最终任务\u{1f}最終任務",
            "Letztes Objekt\u{1f}Final Object\u{1f}Objeto final\u{1f}Objeto final\u{1f}Objet final\u{1f}Oggetto Finale\u{1f}最終オブジェクト\u{1f}\u{1f}Finalny obiekt\u{1f}Objeto Final\u{1f}Конечный объект\u{1f}ว\u{e31}ตถ\u{e38}ส\u{e38}ดท\u{e49}าย\u{1f}Son Nesne\u{1f}最终对象\u{1f}最終目標",
            "Letzter Zielpunkt\u{1f}Final Objective\u{1f}Objetivo final\u{1f}Objetivo final\u{1f}Objectif final\u{1f}Obiettivo Finale\u{1f}最終目標\u{1f}\u{1f}Finalne zadanie\u{1f}Objetivo Final\u{1f}Конечная задача\u{1f}เป\u{e49}าหมายส\u{e38}ดท\u{e49}าย\u{1f}Son Hedef\u{1f}最终目标点\u{1f}最終目標",
            "Letzte Objekte\u{1f}Final Objects\u{1f}Objetos finales\u{1f}Objetos finales\u{1f}Objets finaux\u{1f}Oggetti Finali\u{1f}最終オブジェクト\u{1f}\u{1f}Finalne obiekty\u{1f}Objetos Finais\u{1f}Конечные объекты\u{1f}ว\u{e31}ตถ\u{e38}ส\u{e38}ดท\u{e49}าย\u{1f}Son Nesneler\u{1f}最终对象\u{1f}最終目標",
            "Letzte Phase\u{1f}Final Phase\u{1f}Fase final\u{1f}Fase final\u{1f}Phase finale\u{1f}Fase Finale\u{1f}最終フェーズ\u{1f}\u{1f}Finalna faza\u{1f}Fase Final\u{1f}Конечная фаза\u{1f}เฟสส\u{e38}ดท\u{e49}าย\u{1f}Son Safha\u{1f}最终阶段\u{1f}最終階段",
            "Letzter Spieler\u{1f}Final Player\u{1f}Jugador final\u{1f}Jugador final\u{1f}Joueur final\u{1f}Giocatore Finale\u{1f}最終プレイヤー\u{1f}\u{1f}Finalny gracz\u{1f}Jogador Final\u{1f}Конечный игрок\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นส\u{e38}ดท\u{e49}าย\u{1f}Son Oyuncu\u{1f}最终玩家\u{1f}最終玩家",
            "Letzte Spieler\u{1f}Final Players\u{1f}Jugadores finales\u{1f}Jugadores finales\u{1f}Joueurs finaux\u{1f}Giocatori Finali\u{1f}最終プレイヤー\u{1f}\u{1f}Finalni gracze\u{1f}Jogadores Finais\u{1f}Конечные игроки\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นส\u{e38}ดท\u{e49}าย\u{1f}Son Oyuncular\u{1f}最终玩家\u{1f}最終玩家",
            "Letzte Runde\u{1f}Final Round\u{1f}Ronda final\u{1f}Ronda final\u{1f}Manche finale\u{1f}Round Finale\u{1f}最終ラウンド\u{1f}\u{1f}Finalna runda\u{1f}Rodada Final\u{1f}Конечный раунд\u{1f}รอบส\u{e38}ดท\u{e49}าย\u{1f}Son Tur\u{1f}最终回合\u{1f}最終回合",
            "Letztes Ziel\u{1f}Final Target\u{1f}Objetivo final\u{1f}Objetivo final\u{1f}Cible finale\u{1f}Bersaglio Finale\u{1f}最終ターゲット\u{1f}\u{1f}Finalny cel\u{1f}Alvo Final\u{1f}Конечная цель\u{1f}เป\u{e49}าหมายส\u{e38}ดท\u{e49}าย\u{1f}Son Hedef\u{1f}最终目标\u{1f}最終目標",
            "Letzte Ziele\u{1f}Final Targets\u{1f}Objetivos finales\u{1f}Objetivos finales\u{1f}Cibles finales\u{1f}Bersagli Finali\u{1f}最終ターゲット\u{1f}\u{1f}Finalne cele\u{1f}Alvos Finais\u{1f}Конечные цели\u{1f}เป\u{e49}าหมายส\u{e38}ดท\u{e49}าย\u{1f}Son Hedefler\u{1f}最终目标\u{1f}最終目標",
            "Finale Zeit\u{1f}Final Time\u{1f}Tiempo final\u{1f}Tiempo final\u{1f}Temps final\u{1f}Tempo Finale\u{1f}最終時間\u{1f}\u{1f}Finalny czas\u{1f}Tempo Final\u{1f}Конечное время\u{1f}เวลาส\u{e38}ดท\u{e49}าย\u{1f}Son Zaman\u{1f}最终时间\u{1f}最終時間",
            "Letztes Upgrade\u{1f}Final Upgrade\u{1f}Mejora final\u{1f}Mejora final\u{1f}Amélioration finale\u{1f}Miglioramento Finale\u{1f}最終アップグレード\u{1f}\u{1f}Finalne ulepszenie\u{1f}Melhoria Final\u{1f}Конечное улучшение\u{1f}การอ\u{e31}พเกรดส\u{e38}ดท\u{e49}าย\u{1f}Son Geliştirme\u{1f}最终升级\u{1f}最終升級",
            "Finden\u{1f}Find\u{1f}Buscar\u{1f}Encontrar\u{1f}Trouver\u{1f}Trova\u{1f}検索\u{1f}\u{1f}Znajdź\u{1f}Encontrar\u{1f}Найти\u{1f}หา\u{1f}Bulma\u{1f}寻找\u{1f}尋找",
            "Fund\u{1f}Finding\u{1f}Buscando\u{1f}Encontrando\u{1f}Découverte\u{1f}Ritrovamento in Corso\u{1f}検索中\u{1f}\u{1f}Znajdywanie\u{1f}Encontrando\u{1f}Находит\u{1f}กำล\u{e31}งหา\u{1f}Bulan\u{1f}正在寻找\u{1f}正在尋找",
            "Beenden\u{1f}Finish\u{1f}Acabar\u{1f}Terminar\u{1f}Finir\u{1f}Concludi\u{1f}終了\u{1f}\u{1f}Zakończ\u{1f}Concluir\u{1f}Завершать\u{1f}เสร\u{e47}จส\u{e34}\u{e49}น\u{1f}Bitir\u{1f}结束\u{1f}完成",
            "Beendet\u{1f}Finished\u{1f}Acabado\u{1f}Terminado\u{1f}Fini\u{1f}Concluso\u{1f}終了した\u{1f}\u{1f}Zakończono\u{1f}Concluído\u{1f}Завершил\u{1f}เสร\u{e47}จส\u{e34}\u{e49}นแล\u{e49}ว\u{1f}Biten\u{1f}已结束\u{1f}已完成",
            "Ende\u{1f}Finishing\u{1f}Acabando\u{1f}Terminando\u{1f}Fin\u{1f}Conclusione in Corso\u{1f}終了中\u{1f}\u{1f}Kończenie\u{1f}Concluindo\u{1f}Завершает\u{1f}กำล\u{e31}งเสร\u{e47}จส\u{e34}\u{e49}น\u{1f}Bitiren\u{1f}正在结束\u{1f}正在完成",
            "Geflogen\u{1f}Flown\u{1f}Volado\u{1f}Voló\u{1f}Volé\u{1f}Volato\u{1f}飛ばされた\u{1f}\u{1f}Poleciano\u{1f}Em Voo\u{1f}Улетел\u{1f}บ\u{e34}นแล\u{e49}ว\u{1f}Uçurulan\u{1f}已飞行\u{1f}已飛行",
            "Fliegen\u{1f}Fly\u{1f}Volar\u{1f}Volar\u{1f}Voler\u{1f}Vola\u{1f}飛ぶ\u{1f}\u{1f}Leć\u{1f}Voar\u{1f}Лететь\u{1f}บ\u{e34}น\u{1f}Uç\u{1f}飞行\u{1f}飛行",
            "Flug\u{1f}Flying\u{1f}Volando\u{1f}Volando\u{1f}Vole\u{1f}Volo in Corso\u{1f}飛んでいる\u{1f}\u{1f}Latanie\u{1f}Voando\u{1f}Летит\u{1f}กำล\u{e31}งบ\u{e34}น\u{1f}Uçan\u{1f}正在飞行\u{1f}正在飛行",
            "Aussteigen\u{1f}Fold\u{1f}Retirarse\u{1f}Retirarse\u{1f}Se coucher\u{1f}Lascia\u{1f}フォールドする\u{1f}\u{1f}Złóż\u{1f}Fugir\u{1f}Сбросить\u{1f}พ\u{e31}บ\u{1f}Katla\u{1f}收起\u{1f}蓋牌",
            "Ausgestiegen\u{1f}Folded\u{1f}Retirado\u{1f}Se retiró\u{1f}S’est couché\u{1f}Lasciato\u{1f}フォールドされた\u{1f}\u{1f}Złożono\u{1f}Fugiu\u{1f}Сбросил\u{1f}พ\u{e31}บแล\u{e49}ว\u{1f}Katlanan\u{1f}已收起\u{1f}蓋牌",
            "Passen\u{1f}Folding\u{1f}Retirándose\u{1f}Retirándose\u{1f}Se couche\u{1f}Abbandono in Corso\u{1f}フォールドしている\u{1f}\u{1f}Składania\u{1f}Fugindo\u{1f}Сбрасывает\u{1f}กำล\u{e31}งพ\u{e31}บ\u{1f}Katlayan\u{1f}正在收起\u{1f}蓋牌",
            "\u{1f}Form\u{1f}Forma\u{1f}Forma\u{1f}Forme\u{1f}Forma\u{1f}フォーム\u{1f}\u{1f}Forma\u{1f}Forma\u{1f}Форма\u{1f}ร\u{e48}าง\u{1f}Biçim\u{1f}表格\u{1f}型態",
            "Formen\u{1f}Forms\u{1f}Forma\u{1f}Formas\u{1f}Formes\u{1f}Forme\u{1f}フォーム\u{1f}\u{1f}Formy\u{1f}Formas\u{1f}Формы\u{1f}ร\u{e48}าง\u{1f}Biçimler\u{1f}表格\u{1f}型態",
            "Vorwärts\u{1f}Forward\u{1f}Hacia adelante\u{1f}Adelante\u{1f}Avant\u{1f}Avanti\u{1f}前方\u{1f}\u{1f}Do przodu\u{1f}Para a Frente\u{1f}Вперед\u{1f}ไปข\u{e49}างหน\u{e49}า\u{1f}İleri\u{1f}前\u{1f}前進",
            "Gefunden\u{1f}Found\u{1f}Encontrado\u{1f}Encontrado\u{1f}Trouvé\u{1f}Trovato\u{1f}検索した\u{1f}\u{1f}Znaleziono\u{1f}Encontrado\u{1f}Нашел\u{1f}หาเจอ\u{1f}Bulunan\u{1f}已找到\u{1f}被發現",
            "Einfrieren\u{1f}Freeze\u{1f}Congelar\u{1f}Congelar\u{1f}Geler\u{1f}Congela\u{1f}フリーズ\u{1f}\u{1f}Zamroź\u{1f}Congelar\u{1f}Заморозить\u{1f}แช\u{e48}แข\u{e47}ง\u{1f}Donma\u{1f}冰冻\u{1f}冰凍",
            "Frost\u{1f}Freezing\u{1f}Congelando\u{1f}Congelando\u{1f}Gel\u{1f}Congelamento in Corso\u{1f}フリーズ中\u{1f}\u{1f}Zamrażanie\u{1f}Congelando\u{1f}Замораживает\u{1f}กำล\u{e31}งแช\u{e48}แข\u{e47}ง\u{1f}Buz Gibi\u{1f}正在冰冻\u{1f}正在冰凍",
            "Eingefroren\u{1f}Frozen\u{1f}Congelado\u{1f}Congelado\u{1f}Gelé\u{1f}Congelato\u{1f}凍らされた\u{1f}\u{1f}Zamrożono\u{1f}Congelado\u{1f}Заморозил\u{1f}แช\u{e48}แข\u{e47}งแล\u{e49}ว\u{1f}Donan\u{1f}已冰冻\u{1f}受到冰凍",
            "\u{1f}GG\u{1f}\u{1f}Buen juego\u{1f}BRAVO\u{1f}\u{1f}\u{1f}\u{1f}Dobra gra\u{1f}\u{1f}ГГ",
            "Spiel\u{1f}Game\u{1f}Partida\u{1f}Partida\u{1f}Partie\u{1f}Partita\u{1f}マッチ\u{1f}\u{1f}Gra\u{1f}Jogo\u{1f}Матч\u{1f}เกม\u{1f}Oyun\u{1f}游戏\u{1f}遊戲",
            "Spiele\u{1f}Games\u{1f}Partidas\u{1f}Partidas\u{1f}Parties\u{1f}Partite\u{1f}マッチ\u{1f}\u{1f}Gry\u{1f}Jogos\u{1f}Матчи\u{1f}เกม\u{1f}Oyunlar\u{1f}游戏\u{1f}遊戲",
            "Spiele verloren\u{1f}Games Lost\u{1f}Partidas perdidas\u{1f}Partidas perdidas\u{1f}Parties perdues\u{1f}Partite Perse\u{1f}マッチ敗北\u{1f}\u{1f}Przegrane gry\u{1f}Jogos Perdidos\u{1f}Проиграно матчей\u{1f}เกมท\u{e35}\u{e48}แพ\u{e49}\u{1f}Kaybedilen Oyunlar\u{1f}战败场数\u{1f}敗場",
            "Spiele gewonnen\u{1f}Games Won\u{1f}Partidas ganadas\u{1f}Partidas ganadas\u{1f}Parties gagnées\u{1f}Partite Vinte\u{1f}マッチ勝利\u{1f}\u{1f}Wygrane gry\u{1f}Jogos Vencidos\u{1f}Выиграно матчей\u{1f}เกมท\u{e35}\u{e48}ชนะ\u{1f}Kazanılan Oyunlar\u{1f}胜利场数\u{1f}勝場",
            "Gehen\u{1f}Go\u{1f}Ir\u{1f}Vamos\u{1f}Aller\u{1f}Vai\u{1f}行く\u{1f}\u{1f}Idź\u{1f}Ir\u{1f}Вперед\u{1f}ไป\u{1f}Git\u{1f}前往\u{1f}前往",
            "Ziel\u{1f}Goal\u{1f}Objetivo\u{1f}Objetivo\u{1f}But\u{1f}Obiettivo\u{1f}目標\u{1f}목표\u{1f}Wytyczna\u{1f}Meta\u{1f}Цель\u{1f}ประต\u{e39}\u{1f}Amaç\u{1f}目标\u{1f}目標",
            "Ziele\u{1f}Goals\u{1f}Objetivos\u{1f}Objetivos\u{1f}Buts\u{1f}Obiettivi\u{1f}目標\u{1f}\u{1f}Wytyczne\u{1f}Metas\u{1f}Голы\u{1f}ประต\u{e39}\u{1f}Gol\u{1f}目标\u{1f}目標",
            "Los\u{1f}Going\u{1f}Yendo\u{1f}En camino\u{1f}Aller\u{1f}Movimento in Corso\u{1f}行く\u{1f}\u{1f}Idzie\u{1f}Indo\u{1f}Идет\u{1f}กำล\u{e31}งไป\u{1f}Giden\u{1f}正在前往\u{1f}正在前往",
            "Gut\u{1f}Good\u{1f}Bueno\u{1f}Bien\u{1f}Bon\u{1f}Buono\u{1f}良い\u{1f}\u{1f}Dobrze\u{1f}Bom\u{1f}Хорошо\u{1f}ด\u{e35}\u{1f}İyi\u{1f}好\u{1f}好",
            "Viel Glück\u{1f}Good Luck\u{1f}Buena suerte\u{1f}Buena suerte\u{1f}Bonne chance\u{1f}Buona Fortuna\u{1f}グッド・ラック\u{1f}\u{1f}Powodzenia\u{1f}Boa Sorte\u{1f}Удачи\u{1f}โชคด\u{e35}นะ\u{1f}Boş Şans\u{1f}好运\u{1f}祝好運",
            "Tschüss\u{1f}Goodbye\u{1f}Adiós\u{1f}Adiós\u{1f}Au revoir\u{1f}Arrivederci\u{1f}さようなら\u{1f}\u{1f}Na razie\u{1f}Tchau\u{1f}До свидания\u{1f}ลาก\u{e48}อน\u{1f}Görüşürüz\u{1f}再见\u{1f}再見",
            "Grün\u{1f}Green\u{1f}Verde\u{1f}Verde\u{1f}Vert\u{1f}Verde\u{1f}緑\u{1f}\u{1f}Zielony\u{1f}Verde\u{1f}Зеленый\u{1f}เข\u{e35}ยว\u{1f}Yeşil\u{1f}绿色\u{1f}綠",
            "Schuldig\u{1f}Guilty\u{1f}Culpable\u{1f}Culpable\u{1f}Coupable\u{1f}Colpevole\u{1f}有罪\u{1f}\u{1f}Winny\u{1f}Culpado\u{1f}Виновен\u{1f}ม\u{e35}ความผ\u{e34}ด\u{1f}Suçlu\u{1f}有罪\u{1f}有罪",
            "Hacken\u{1f}Hack\u{1f}Hackear\u{1f}Hackear\u{1f}Pirater\u{1f}Sabota\u{1f}ハッキング\u{1f}\u{1f}Zhakuj\u{1f}Hackear\u{1f}Взломать\u{1f}แฮ\u{e47}ค\u{1f}\u{1f}入侵\u{1f}駭入",
            "Gehackt\u{1f}Hacked\u{1f}Hackeado\u{1f}Hackeado\u{1f}Piraté\u{1f}Sabotato\u{1f}ハックされている\u{1f}\u{1f}Zhakowano\u{1f}Hackeado\u{1f}Взломал\u{1f}ถ\u{e39}กแฮ\u{e47}ค\u{1f}Hack'lendi\u{1f}已入侵\u{1f}被駭入",
            "Hack\u{1f}Hacking\u{1f}Hackeando\u{1f}Hackeando\u{1f}Piratage\u{1f}Sabotaggio in Corso\u{1f}ハッキング中\u{1f}\u{1f}Hakowanie\u{1f}Hackeando\u{1f}Взламывает\u{1f}กำล\u{e31}งแฮ\u{e47}ค\u{1f}Sisteme Sızma\u{1f}正在入侵\u{1f}正在駭入",
            "\u{1f}Hand\u{1f}Mano\u{1f}Mano\u{1f}Main\u{1f}Mano\u{1f}手\u{1f}\u{1f}Dłoń\u{1f}Mão\u{1f}Рука\u{1f}ม\u{e37}อ\u{1f}El\u{1f}手牌\u{1f}手牌",
            "Hände\u{1f}Hands\u{1f}Manos\u{1f}Manos\u{1f}Mains\u{1f}Mani\u{1f}手\u{1f}\u{1f}Dłonie\u{1f}Mãos\u{1f}Руки\u{1f}ม\u{e37}อ\u{1f}El\u{1f}手牌\u{1f}手牌",
            "Heilen\u{1f}Heal\u{1f}Sanar\u{1f}Sanar\u{1f}Soigner\u{1f}Cura\u{1f}回復する\u{1f}\u{1f}Wylecz\u{1f}Curar\u{1f}Лечить\u{1f}ฮ\u{e35}ล\u{1f}İyileştir\u{1f}治疗\u{1f}治療",
            "Geheilt\u{1f}Healed\u{1f}Sanado\u{1f}Sanado\u{1f}Soigné\u{1f}Curato\u{1f}回復された\u{1f}\u{1f}Wyleczono\u{1f}Curado\u{1f}Вылечен\u{1f}ฮ\u{e35}ลแล\u{e49}ว\u{1f}İyileştirilen\u{1f}已治疗\u{1f}受到治療",
            "Heiler\u{1f}Healer\u{1f}Sanador\u{1f}Sanador\u{1f}Soigneur\u{1f}Guaritore\u{1f}ヒーラー\u{1f}\u{1f}Leczący\u{1f}Curandeiro\u{1f}Медик\u{1f}ฮ\u{e35}ลเลอร\u{e4c}\u{1f}Şifacı\u{1f}治疗者\u{1f}治療者",
            "Sanitäter\u{1f}Healers\u{1f}Sanadores\u{1f}Sanadores\u{1f}Soigneurs\u{1f}Guaritori\u{1f}ヒーラー\u{1f}\u{1f}Leczący\u{1f}Curandeiros\u{1f}Медики\u{1f}ฮ\u{e35}ลเลอร\u{e4c}\u{1f}Şifacılar\u{1f}治疗者\u{1f}治療者",
            "Heilung\u{1f}Healing\u{1f}Sanando\u{1f}Sanando\u{1f}Soigne\u{1f}Cura in Corso\u{1f}回復している\u{1f}\u{1f}Leczenie\u{1f}Curando\u{1f}Лечит\u{1f}กำล\u{e31}งฮ\u{e35}ล\u{1f}İyileştirme\u{1f}正在治疗\u{1f}正在治療",
            "Herz\u{1f}Heart\u{1f}Corazón\u{1f}Corazón\u{1f}Cœur\u{1f}Cuore\u{1f}ハート\u{1f}\u{1f}Kier\u{1f}Copas\u{1f}Червы\u{1f}โพแดง\u{1f}Kupa\u{1f}红桃\u{1f}紅心",
            "Herz\u{1f}Hearts\u{1f}Corazones\u{1f}Corazones\u{1f}Cœur\u{1f}Cuori\u{1f}ハート\u{1f}\u{1f}Kier\u{1f}Copas\u{1f}Червы\u{1f}โพแดง\u{1f}Kupa\u{1f}心\u{1f}紅心",
            "Höhe\u{1f}Height\u{1f}Altura\u{1f}Altura\u{1f}Hauteur\u{1f}Altezza\u{1f}高さ\u{1f}\u{1f}Wysokość\u{1f}Altura\u{1f}Высота\u{1f}ความส\u{e39}ง\u{1f}Yükseklik\u{1f}高度\u{1f}高度",
            "Hallo\u{1f}Hello\u{1f}Hola\u{1f}Hola\u{1f}Bonjour\u{1f}Ciao\u{1f}こんにちは\u{1f}\u{1f}Witaj\u{1f}Olá\u{1f}Привет\u{1f}ด\u{e35}จ\u{e49}า\u{1f}Merhaba\u{1f}问候\u{1f}哈囉",
            "Hilfe\u{1f}Help\u{1f}Ayuda\u{1f}Ayuda\u{1f}Aide\u{1f}Aiuto\u{1f}助けて\u{1f}\u{1f}Pomocy\u{1f}Socorro\u{1f}На помощь\u{1f}ช\u{e48}วยด\u{e49}วย\u{1f}İmdat\u{1f}求助\u{1f}幫我",
            "Hier\u{1f}Here\u{1f}Aquí\u{1f}Aquí\u{1f}Ici\u{1f}Qui\u{1f}ここ\u{1f}\u{1f}Tutaj\u{1f}Aqui\u{1f}Сюда\u{1f}ท\u{e35}\u{e48}น\u{e35}\u{e48}\u{1f}Burada\u{1f}此处\u{1f}這裡",
            "Held\u{1f}Hero\u{1f}Héroe\u{1f}Héroe\u{1f}Personnage\u{1f}Eroe\u{1f}ヒーロー\u{1f}\u{1f}Bohater\u{1f}Herói\u{1f}Герой\u{1f}ฮ\u{e35}โร\u{e48}\u{1f}Kahraman\u{1f}英雄\u{1f}英雄",
            "Helden\u{1f}Heroes\u{1f}Héroes\u{1f}Héroes\u{1f}Personnages\u{1f}Eroi\u{1f}ヒーロー\u{1f}\u{1f}Bohaterowie\u{1f}Heróis\u{1f}Герои\u{1f}ฮ\u{e35}โร\u{e48}\u{1f}Kahramanlar\u{1f}英雄\u{1f}英雄",
            "Versteckt\u{1f}Hidden\u{1f}Ocultado\u{1f}Oculto\u{1f}Caché\u{1f}Nascosto\u{1f}非表示にした\u{1f}\u{1f}Ukryto\u{1f}Escondido\u{1f}Скрытый\u{1f}ซ\u{e48}อนแล\u{e49}ว\u{1f}Gizli\u{1f}已隐藏\u{1f}躲藏",
            "Verstecken\u{1f}Hide\u{1f}Ocultar\u{1f}Ocultar\u{1f}Se cacher\u{1f}Nascondi\u{1f}非表示\u{1f}\u{1f}Ukryj\u{1f}Esconder\u{1f}Скрыть\u{1f}ซ\u{e48}อน\u{1f}Gizle\u{1f}隐藏\u{1f}躲藏",
            "Versteck\u{1f}Hiding\u{1f}Ocultando\u{1f}Ocultando\u{1f}Cache\u{1f}Nascondimento in Corso\u{1f}非表示にしている\u{1f}\u{1f}Ukrywanie\u{1f}Escondendo\u{1f}Скрывается\u{1f}กำล\u{e31}งซ\u{e48}อน\u{1f}Gizleyen\u{1f}正在隐藏\u{1f}正在躲藏",
            "Highscore\u{1f}High Score\u{1f}Mejor puntuación\u{1f}Puntuación alta\u{1f}Meilleur score\u{1f}Punteggio Migliore\u{1f}ハイスコア\u{1f}\u{1f}Najwyższy wynik\u{1f}Pontuação Máxima\u{1f}Рекорд\u{1f}คะแนนส\u{e39}งส\u{e38}ด\u{1f}Yüksek Skor\u{1f}最高得分\u{1f}最高分",
            "Highscores\u{1f}High Scores\u{1f}Mejores puntuaciones\u{1f}Puntuaciones altas\u{1f}Meilleurs scores\u{1f}Punteggi Migliori\u{1f}ハイスコア\u{1f}\u{1f}Najwyższe wyniki\u{1f}Pontuações Máximas\u{1f}Рекорды\u{1f}คะแนนส\u{e39}งส\u{e38}ด\u{1f}Yüksek Skor\u{1f}最高得分\u{1f}最高分",
            "Treffen\u{1f}Hit\u{1f}Golpear\u{1f}Golpear\u{1f}Toucher\u{1f}Colpisci\u{1f}ヒットする\u{1f}\u{1f}Uderz\u{1f}Acertar\u{1f}Попадание\u{1f}โจมต\u{e35}\u{1f}Vur\u{1f}击中\u{1f}擊中",
            "Treffer\u{1f}Hitting\u{1f}Golpeando\u{1f}Golpeando\u{1f}Touche\u{1f}Colpo in Corso\u{1f}ヒットしている\u{1f}\u{1f}Uderzanie\u{1f}Acertando\u{1f}Попадает\u{1f}กำล\u{e31}งโจมต\u{e35}\u{1f}Vuran\u{1f}正在击中\u{1f}擊中",
            "\u{1f}Hmmm\u{1f}Mmm\u{1f}\u{1f}\u{1f}Mmm\u{1f}ふーむ\u{1f}\u{1f}Hm\u{1f}\u{1f}Хм-м-м\u{1f}อ\u{e37}มมม\u{1f}Hımm\u{1f}嗯\u{1f}嗯",
            "Geisel\u{1f}Hostage\u{1f}Rehén\u{1f}Rehén\u{1f}Otage\u{1f}Ostaggio\u{1f}人質\u{1f}\u{1f}Zakładnik\u{1f}Refém\u{1f}Заложник\u{1f}ต\u{e31}วประก\u{e31}น\u{1f}Rehine\u{1f}人质\u{1f}人質",
            "Geiseln\u{1f}Hostages\u{1f}Rehenes\u{1f}Rehenes\u{1f}Otages\u{1f}Ostaggi\u{1f}人質\u{1f}\u{1f}Zakładnicy\u{1f}Reféns\u{1f}Заложники\u{1f}ต\u{e31}วประก\u{e31}น\u{1f}Rehineler\u{1f}人质\u{1f}人質",
            "Hä\u{1f}Huh\u{1f}Ein\u{1f}¿Eh?\u{1f}Euh\u{1f}Eh\u{1f}ふうん\u{1f}\u{1f}Hę\u{1f}Hã\u{1f}Хех\u{1f}ฮะ\u{1f}Hah\u{1f}哈\u{1f}哼",
            "Jagen\u{1f}Hunt\u{1f}Dar caza\u{1f}Cazar\u{1f}Chasser\u{1f}Caccia\u{1f}狩り\u{1f}\u{1f}Upoluj\u{1f}Caçar\u{1f}Охотиться\u{1f}ล\u{e48}า\u{1f}Avla\u{1f}狩猎\u{1f}狩獵",
            "Gejagt\u{1f}Hunted\u{1f}Cazado\u{1f}Cazado\u{1f}Pourchassé\u{1f}Cacciato\u{1f}狩りをした\u{1f}\u{1f}Upolowano\u{1f}Caçado\u{1f}Жертва\u{1f}ล\u{e48}าแล\u{e49}ว\u{1f}Avlanan\u{1f}已狩猎 \u{1f}遭到狩獵",
            "Jäger\u{1f}Hunter\u{1f}Cazadora\u{1f}Cazador\u{1f}Chasseur\u{1f}Cacciatore\u{1f}ハンター\u{1f}\u{1f}Łowca\u{1f}Caçador\u{1f}Охотник\u{1f}น\u{e31}กล\u{e48}า\u{1f}Avcı\u{1f}猎人\u{1f}獵人",
            "Verfolger\u{1f}Hunters\u{1f}Cazadoras\u{1f}Cazadores\u{1f}Chasseurs\u{1f}Cacciatori\u{1f}ハンター\u{1f}\u{1f}Łowcy\u{1f}Caçadores\u{1f}Охотники\u{1f}น\u{e31}กล\u{e48}า\u{1f}Avcılar\u{1f}猎人\u{1f}獵人",
            "Jagd\u{1f}Hunting\u{1f}Dando caza\u{1f}Cazando\u{1f}Chasse\u{1f}Caccia in Corso\u{1f}狩り中\u{1f}\u{1f}Polowanie\u{1f}Caçando\u{1f}Охотится\u{1f}กำล\u{e31}งล\u{e48}า\u{1f}Avlayan\u{1f}正在狩猎\u{1f}正在狩獵",
            "Ich gebe auf\u{1f}I Give Up\u{1f}Me rindo\u{1f}Me rindo\u{1f}J’abandonne\u{1f}Mi Arrendo\u{1f}もう諦めた\u{1f}\u{1f}Poddaję się\u{1f}Desisto\u{1f}Я сдаюсь\u{1f}ฉ\u{e31}นยอมแพ\u{e49}\u{1f}Pes Ediyorum\u{1f}我放弃了\u{1f}放棄",
            "Ich hab's versucht\u{1f}I Tried\u{1f}Lo he intentado\u{1f}Lo intenté\u{1f}J’ai essayé\u{1f}Ho Provato\u{1f}やるだけやった\u{1f}\u{1f}Próbowałem\u{1f}Eu Tentei\u{1f}Попытка – не пытка\u{1f}ฉ\u{e31}นเหน\u{e37}\u{e48}อย\u{1f}Denedim Olmadı\u{1f}我尽力了\u{1f}努力了",
            "In Sicht\u{1f}In View\u{1f}A la vista\u{1f}A la vista\u{1f}En vue\u{1f}In Visuale\u{1f}表示内\u{1f}\u{1f}W polu widzenia\u{1f}Na Visão\u{1f}В поле зрения\u{1f}ในม\u{e38}มมอง\u{1f}Görüntülenen\u{1f}在视野中\u{1f}在視野中",
            "Einkommen\u{1f}Income\u{1f}Ingresos\u{1f}Ingreso\u{1f}Revenu\u{1f}Reddito\u{1f}受ける\u{1f}\u{1f}Dochód\u{1f}A caminho\u{1f}Доход\u{1f}รายได\u{e49}\u{1f}Gelir\u{1f}收入\u{1f}收入",
            "Eingehend\u{1f}Incoming\u{1f}Entrante\u{1f}Enemigos aproximándose\u{1f}En approche\u{1f}In Arrivo\u{1f}受信\u{1f}\u{1f}Przychodzące\u{1f}Chegando\u{1f}Опасность\u{1f}ขาเข\u{e49}า\u{1f}Yola Çıktı\u{1f}正在前来\u{1f}來襲",
            "Zuerst\u{1f}Initial\u{1f}Inicial\u{1f}Inicial\u{1f}\u{1f}Iniziale\u{1f}初期\u{1f}\u{1f}Wstępne\u{1f}Inicial\u{1f}Начальный\u{1f}แรกเร\u{e34}\u{e48}ม\u{1f}İlk\u{1f}初始\u{1f}初始",
            "Erste Verbündete\u{1f}Initial Allies\u{1f}Aliados iniciales\u{1f}Aliados iniciales\u{1f}Alliés initiaux\u{1f}Alleati Iniziali\u{1f}初期の味方\u{1f}\u{1f}Wstępni sojusznicy\u{1f}Aliados Iniciais\u{1f}Начальные союзники\u{1f}เพ\u{e37}\u{e48}อนร\u{e48}วมท\u{e35}ม\u{1f}Başlangıç Müttefikleri\u{1f}初始盟友\u{1f}初始盟友",
            "Erster Verbündeter\u{1f}Initial Ally\u{1f}Aliado inicial\u{1f}Aliado inicial\u{1f}Allié initial\u{1f}Alleato Iniziale\u{1f}初期の味方\u{1f}\u{1f}Wstępny sojusznik\u{1f}Aliado Inicial\u{1f}Начальный союзник\u{1f}เพ\u{e37}\u{e48}อนร\u{e48}วมท\u{e35}ม\u{1f}Başlangıç Müttefiki\u{1f}初始盟友\u{1f}初始盟友",
            "Erster Versuch\u{1f}Initial Attempt\u{1f}Intento inicial\u{1f}Intento inicial\u{1f}Tentative initiale\u{1f}Tentativo Iniziale\u{1f}初期挑戦\u{1f}\u{1f}Wstępna próba\u{1f}Tentativa Inicial\u{1f}Начальная попытка\u{1f}ความพยายามเร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Denemesi\u{1f}初始尝试\u{1f}初次機會",
            "Erster Checkpoint\u{1f}Initial Checkpoint\u{1f}Punto de control inicial\u{1f}Punto de control inicial\u{1f}Point de contrôle initial\u{1f}Checkpoint Iniziale\u{1f}初期チェックポイント\u{1f}\u{1f}Wstępny punkt kontrolny\u{1f}Ponto de Verificação Inicial\u{1f}Начальная контрольная точка\u{1f}เช\u{e47}คพอยต\u{e4c}เร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Kontrol Noktası\u{1f}初始检查点\u{1f}初始檢查點",
            "Erste Gegner\u{1f}Initial Enemies\u{1f}Enemigos iniciales\u{1f}Enemigos iniciales\u{1f}Ennemis initiaux\u{1f}Nemici Iniziali\u{1f}初期の敵\u{1f}\u{1f}Wstępni wrogowie\u{1f}Inimigos Iniciais\u{1f}Начальные враги\u{1f}ศ\u{e31}ตร\u{e39}เร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Düşmanları\u{1f}初始敌人\u{1f}初始敵人",
            "Erster Gegner\u{1f}Initial Enemy\u{1f}Enemigo inicial\u{1f}Enemigo inicial\u{1f}Ennemi initial\u{1f}Nemico Iniziale\u{1f}初期の敵\u{1f}\u{1f}Wstępny wróg\u{1f}Inimigo Inicial\u{1f}Начальный враг\u{1f}ศ\u{e31}ตร\u{e39}เร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Düşmanı\u{1f}初始敌人\u{1f}初始敵人",
            "Erste Form\u{1f}Initial Form\u{1f}Forma inicial\u{1f}Forma inicial\u{1f}Forme initiale\u{1f}Forma Iniziale\u{1f}初期フォーム\u{1f}\u{1f}Wstępna forma\u{1f}Forma Inicial\u{1f}Начальная форма\u{1f}ร\u{e48}างแรก\u{1f}Başlangıç Biçimi\u{1f}初始形态\u{1f}初始型態",
            "Erstes Spiel\u{1f}Initial Game\u{1f}Partida inicial\u{1f}Partida inicial\u{1f}Partie initiale\u{1f}Partita Iniziale\u{1f}初期マッチ\u{1f}\u{1f}Wstępna gra\u{1f}Jogo Inicial\u{1f}Начальный матч\u{1f}เกมเร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Oyunu\u{1f}初始游戏\u{1f}初始遊戲",
            "Erster Held\u{1f}Initial Hero\u{1f}Héroe inicial\u{1f}Héroe inicial\u{1f}Héros initial\u{1f}Eroe Iniziale\u{1f}初期ヒーロー\u{1f}\u{1f}Wstępny bohater\u{1f}Herói Inicial\u{1f}Начальный герой\u{1f}ฮ\u{e35}โร\u{e48}แรกเร\u{e34}\u{e48}ม\u{1f}Başlangıç Kahramanı\u{1f}初始英雄\u{1f}初始英雄",
            "Erste Helden\u{1f}Initial Heroes\u{1f}Héroes iniciales\u{1f}Héroes iniciales\u{1f}Héros initiaux\u{1f}Eroi Iniziali\u{1f}初期ヒーロー\u{1f}\u{1f}Wstępni bohaterowie\u{1f}Heróis Iniciais\u{1f}Начальные герои\u{1f}ฮ\u{e35}โร\u{e48}เร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Kahramanları\u{1f}初始英雄\u{1f}初始英雄",
            "Erste Geisel\u{1f}Initial Hostage\u{1f}Rehén inicial\u{1f}Rehén inicial\u{1f}Otage initial\u{1f}Ostaggio Iniziale\u{1f}初期の人質\u{1f}\u{1f}Wstępny zakładnik\u{1f}Refém Inicial\u{1f}Начальный заложник\u{1f}ต\u{e31}วประก\u{e31}นเร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Rehinesi\u{1f}初始人质\u{1f}初始人質",
            "Erster Level\u{1f}Initial Level\u{1f}Nivel inicial\u{1f}Nivel inicial\u{1f}Niveau initial\u{1f}Livello Iniziale\u{1f}初期レベル\u{1f}\u{1f}Wstępny poziom\u{1f}Nível Inicial\u{1f}Начальный уровень\u{1f}เลเวลเร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Seviyesi\u{1f}初始等级\u{1f}初始等級",
            "Erste Mission\u{1f}Initial Mission\u{1f}Misión inicial\u{1f}Misión inicial\u{1f}Mission initiale\u{1f}Missione Iniziale\u{1f}初期任務\u{1f}\u{1f}Wstępna misja\u{1f}Missão Inicial\u{1f}Начальное задание\u{1f}ภารก\u{e34}จเร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Görevi\u{1f}初始任务\u{1f}初始任務",
            "Erstes Objekt\u{1f}Initial Object\u{1f}Objeto inicial\u{1f}Objeto inicial\u{1f}Objet initial\u{1f}Oggetto Iniziale\u{1f}初期オブジェクト\u{1f}\u{1f}Wstępny obiekt\u{1f}Objeto Inicial\u{1f}Начальный объект\u{1f}ว\u{e31}ตถ\u{e38}เร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Nesnesi\u{1f}初始对象\u{1f}初始目標",
            "Erster Zielpunkt\u{1f}Initial Objective\u{1f}Objetivo inicial\u{1f}Objetivo inicial\u{1f}Objectif initial\u{1f}Obiettivo Iniziale\u{1f}初期目標\u{1f}\u{1f}Wstępne zadanie\u{1f}Objetivo Inicial\u{1f}Начальная задача\u{1f}เป\u{e49}าหมายเร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Hedefi\u{1f}初始目标点\u{1f}初始目標",
            "Erste Objekte\u{1f}Initial Objects\u{1f}Objetos iniciales\u{1f}Objetos iniciales\u{1f}Objets initiaux\u{1f}Oggetti Iniziali\u{1f}初期オブジェクト\u{1f}\u{1f}Wstępne obiekty\u{1f}Objetos Iniciais\u{1f}Начальные объекты\u{1f}ว\u{e31}ตถ\u{e38}เร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Nesneleri\u{1f}初始对象\u{1f}初始目標",
            "Erste Phase\u{1f}Initial Phase\u{1f}Fase inicial\u{1f}Fase inicial\u{1f}Phase initiale\u{1f}Fase Iniziale\u{1f}初期フェーズ\u{1f}\u{1f}Wstępna faza\u{1f}Fase Inicial\u{1f}Начальная фаза\u{1f}เฟสเร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Safhası\u{1f}初始阶段\u{1f}初始階段",
            "Erster Spieler\u{1f}Initial Player\u{1f}Jugador inicial\u{1f}Jugador inicial\u{1f}Joueur initial\u{1f}Giocatore Iniziale\u{1f}初期プレイヤー\u{1f}\u{1f}Wstępny gracz\u{1f}Jogador Inicial\u{1f}Начальный игрок\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นเร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Oyuncusu\u{1f}初始玩家\u{1f}初始玩家",
            "Erste Spieler\u{1f}Initial Players\u{1f}Jugadores iniciales\u{1f}Jugadores iniciales\u{1f}Joueurs initiaux\u{1f}Giocatori Iniziali\u{1f}初期プレイヤー\u{1f}\u{1f}Wstępni gracze\u{1f}Jogadores Iniciais\u{1f}Начальные игроки\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นเร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Oyuncuları\u{1f}初始玩家\u{1f}初始玩家",
            "Erste Runde\u{1f}Initial Round\u{1f}Ronda inicial\u{1f}Ronda inicial\u{1f}Manche initiale\u{1f}Round Iniziale\u{1f}初期ラウンド\u{1f}\u{1f}Wstępna runda\u{1f}Rodada Inicial\u{1f}Начальный раунд\u{1f}รอบเร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Turu\u{1f}初始回合\u{1f}初始回合",
            "Erstes Ziel\u{1f}Initial Target\u{1f}Objetivo inicial\u{1f}Objetivo inicial\u{1f}Cible initiale\u{1f}Bersaglio Iniziale\u{1f}初期ターゲット\u{1f}\u{1f}Wstępny cel\u{1f}Alvo Inicial\u{1f}Начальная цель\u{1f}เป\u{e49}าหมายเร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Hedefi\u{1f}初始目标\u{1f}初始目標",
            "Erste Ziele\u{1f}Initial Targets\u{1f}Objetivos iniciales\u{1f}Objetivos iniciales\u{1f}Cibles initiales\u{1f}Bersagli Iniziali\u{1f}初期ターゲット\u{1f}\u{1f}Wstępne cele\u{1f}Alvos Iniciais\u{1f}Начальные цели\u{1f}เป\u{e49}าหมายเร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Hedefleri\u{1f}初始目标\u{1f}初始目標",
            "Erstes Upgrade\u{1f}Initial Upgrade\u{1f}Mejora inicial\u{1f}Mejora inicial\u{1f}Amélioration initiale\u{1f}Miglioramento Iniziale\u{1f}初期アップグレード\u{1f}\u{1f}Wstępne ulepszenie\u{1f}Melhoria Inicial\u{1f}Начальное улучшение\u{1f}การอ\u{e31}พเกรดเร\u{e34}\u{e48}มต\u{e49}น\u{1f}Başlangıç Yükseltmesi\u{1f}初始升级\u{1f}初始升級",
            "Unschuldig\u{1f}Innocent\u{1f}Inocente\u{1f}Inocente\u{1f}\u{1f}Innocente\u{1f}無罪\u{1f}\u{1f}Niewinny\u{1f}Inocente\u{1f}Невиновен\u{1f}บร\u{e34}ส\u{e38}ทธ\u{e34}\u{e4c}\u{1f}Masum\u{1f}无罪\u{1f}無辜",
            "Innerhalb\u{1f}Inside\u{1f}Dentro\u{1f}Adentro\u{1f}À l’intérieur\u{1f}Dentro\u{1f}中\u{1f}\u{1f}Wewnątrz\u{1f}Dentro\u{1f}Внутри\u{1f}ข\u{e49}างใน\u{1f}İç\u{1f}内\u{1f}內部",
            "Intelligenz\u{1f}Intelligence\u{1f}Inteligencia\u{1f}Inteligencia\u{1f}\u{1f}Intelligenza\u{1f}知力\u{1f}\u{1f}Inteligencja\u{1f}Inteligência\u{1f}Интеллект\u{1f}ความฉลาด\u{1f}Zekâ\u{1f}智力\u{1f}智力",
            "Interagieren\u{1f}Interact\u{1f}Interactuar\u{1f}Interactuar\u{1f}Interagir\u{1f}Interagisci\u{1f}インタラクト\u{1f}\u{1f}Interakcja\u{1f}Interagir\u{1f}Взаимодействие\u{1f}โต\u{e49}ตอบ\u{1f}Etkileşim\u{1f}互动\u{1f}互動",
            "Unsichtbar\u{1f}Invisible\u{1f}\u{1f}\u{1f}\u{1f}Invisibile\u{1f}表示されない\u{1f}\u{1f}Niewidzialne\u{1f}Invisível\u{1f}Невидимый\u{1f}มองไม\u{e48}เห\u{e47}น\u{1f}Görünmez\u{1f}不可见\u{1f}隱形",
            "Gegenstand\u{1f}Item\u{1f}Objeto\u{1f}Objeto\u{1f}Article\u{1f}Oggetto\u{1f}アイテム\u{1f}\u{1f}Element\u{1f}\u{1f}Предмет\u{1f}ไอเท\u{e47}ม\u{1f}Eşya\u{1f}物品\u{1f}物品",
            "Gegenstände\u{1f}Items\u{1f}Objetos\u{1f}Objetos\u{1f}Articles\u{1f}Oggetti\u{1f}アイテム\u{1f}\u{1f}Elementy\u{1f}Itens\u{1f}Предметы\u{1f}ไอเท\u{e47}ม\u{1f}Eşyalar\u{1f}物品\u{1f}物品",
            "Beitreten\u{1f}Join\u{1f}Unirse\u{1f}Unirse\u{1f}Rejoindre\u{1f}Unisce\u{1f}参加する\u{1f}\u{1f}Dołącz\u{1f}Entrar\u{1f}Присоединиться\u{1f}เข\u{e49}าร\u{e48}วม\u{1f}Katıl\u{1f}加入\u{1f}加入",
            "Beigetreten\u{1f}Joined\u{1f}Unido\u{1f}Se unió\u{1f}A rejoint\u{1f}Unito\u{1f}参加した\u{1f}\u{1f}Dołączono\u{1f}Entrou\u{1f}Присоединился\u{1f}เข\u{e49}าร\u{e48}วมแล\u{e49}ว\u{1f}Katılınan\u{1f}已加入\u{1f}已加入",
            "Beitritt\u{1f}Joining\u{1f}Uniéndose\u{1f}Uniéndose\u{1f}Rejoint\u{1f}Unione in Corso\u{1f}参加している\u{1f}\u{1f}Dołączanie\u{1f}Entrando\u{1f}Присоединяется\u{1f}กำล\u{e31}งเข\u{e49}าร\u{e48}วม\u{1f}Katılan\u{1f}正在加入\u{1f}正在加入",
            "Springen\u{1f}Jump\u{1f}Saltar\u{1f}Saltar\u{1f}Sauter\u{1f}Salta\u{1f}ジャンプ\u{1f}\u{1f}Skocz\u{1f}Pular\u{1f}Прыгать\u{1f}กระโดด\u{1f}Zıpla\u{1f}跳跃\u{1f}跳躍",
            "Sprung\u{1f}Jumping\u{1f}Saltando\u{1f}Saltando\u{1f}Saut\u{1f}Salto in Corso\u{1f}ジャンプ中\u{1f}\u{1f}Skakanie\u{1f}Pulando\u{1f}Прыгает\u{1f}กำล\u{e31}งกระโดด\u{1f}Zıplayan\u{1f}正在跳跃\u{1f}正在跳躍",
            "\u{1f}Kill\u{1f}Asesinato\u{1f}Matar\u{1f}Victime\u{1f}Uccisione\u{1f}キル\u{1f}\u{1f}Zabójstwo\u{1f}Abater\u{1f}Убийство\u{1f}ฆ\u{e48}า\u{1f}Öldürme\u{1f}击杀\u{1f}擊殺",
            "\u{1f}Kills\u{1f}Asesinatos\u{1f}Muertes\u{1f}Victimes\u{1f}Uccisioni\u{1f}キル\u{1f}\u{1f}Zabójstwa\u{1f}Abates\u{1f}Убийства\u{1f}ฆ\u{e48}า\u{1f}Öldürüş\u{1f}消灭\u{1f}擊殺",
            "\u{1f}Killstreak\u{1f}Racha de asesinatos\u{1f}Racha de muertes\u{1f}Série de victimes\u{1f}Serie di Uccisioni\u{1f}連続キル\u{1f}\u{1f}Seria likwidacji\u{1f}Série de Abates\u{1f}Серия убийств\u{1f}ฆ\u{e48}าต\u{e48}อเน\u{e37}\u{e48}อง\u{1f}Öldürüş Serisi\u{1f}连杀\u{1f}連殺",
            "\u{1f}Killstreaks\u{1f}Rachas de asesinatos\u{1f}Rachas de muertes\u{1f}Séries de victimes\u{1f}Serie di Uccisioni\u{1f}連続キル\u{1f}\u{1f}Serie likwidacji\u{1f}Séries de Abates\u{1f}Серии убийств\u{1f}ฆ\u{e48}าต\u{e48}อเน\u{e37}\u{e48}อง\u{1f}Öldürüş Serisi\u{1f}连杀\u{1f}連殺",
            "Kommandant\u{1f}Leader\u{1f}Líder\u{1f}Líder\u{1f}Meneur\u{1f}\u{1f}リーダー\u{1f}\u{1f}Lider\u{1f}Líder\u{1f}Лидер\u{1f}ผ\u{e39}\u{e49}นำ\u{1f}Lider\u{1f}队长\u{1f}領先者",
            "Anführer\u{1f}Leaders\u{1f}Líderes\u{1f}Líderes\u{1f}Meneurs\u{1f}Leader\u{1f}リーダー\u{1f}\u{1f}Liderzy\u{1f}Líderes\u{1f}Лидеры\u{1f}ผ\u{e39}\u{e49}นำ\u{1f}Liderler\u{1f}队长\u{1f}領先者",
            "Am wenigsten\u{1f}Least\u{1f}Mínimo\u{1f}Mínimo\u{1f}Le moins\u{1f}Il Meno\u{1f}最低\u{1f}\u{1f}Najmniej\u{1f}O Menor\u{1f}Меньше всего\u{1f}น\u{e49}อยท\u{e35}\u{e48}ส\u{e38}ด\u{1f}En Az\u{1f}最少\u{1f}最少",
            "Links\u{1f}Left\u{1f}Izquierda\u{1f}Izquierda\u{1f}Gauche\u{1f}Sinistra\u{1f}左\u{1f}\u{1f}Lewo\u{1f}Esquerda\u{1f}Влево\u{1f}ซ\u{e49}าย\u{1f}Sol\u{1f}左\u{1f}左",
            "Weniger\u{1f}Less\u{1f}Menos\u{1f}Menos\u{1f}Moins\u{1f}Meno\u{1f}少ない\u{1f}\u{1f}Mniej\u{1f}Menos\u{1f}Меньше\u{1f}น\u{e49}อยกว\u{e48}า\u{1f}Daha Az\u{1f}更少\u{1f}較少",
            "\u{1f}Level\u{1f}Nivel\u{1f}Nivel\u{1f}Niveau\u{1f}Livello\u{1f}レベル\u{1f}\u{1f}Poziom\u{1f}Nível\u{1f}Уровень\u{1f}เลเวล\u{1f}Seviye\u{1f}等级\u{1f}等級",
            "Stufe abgestiegen\u{1f}Level Down\u{1f}Bajar nivel\u{1f}Bajar de nivel\u{1f}Perte de niveau\u{1f}Livello inferiore\u{1f}レベルダウン\u{1f}\u{1f}Degradacja\u{1f}Desceu de Nível\u{1f}Понижение уровня\u{1f}เลเวลลด\u{1f}Seviye Azalışı\u{1f}等级下降\u{1f}降級",
            "Stufe aufgestiegen\u{1f}Level Up\u{1f}Subir nivel\u{1f}Subir de nivel\u{1f}Gain de niveau\u{1f}Livello superiore\u{1f}レベルアップ\u{1f}\u{1f}Awans\u{1f}Subiu de Nível\u{1f}Новый уровень\u{1f}เลเวลอ\u{e31}พ\u{1f}Seviye Atlama\u{1f}等级提升\u{1f}升級",
            "\u{1f}Levels\u{1f}Niveles\u{1f}Niveles\u{1f}Niveaux\u{1f}Livelli\u{1f}レベル\u{1f}\u{1f}Poziomy\u{1f}Níveis\u{1f}Уровни\u{1f}เลเวล\u{1f}Seviyeler\u{1f}等级\u{1f}等級",
            "Trefferpunkte\u{1f}Life\u{1f}Vida\u{1f}Vida\u{1f}Vie\u{1f}Vita\u{1f}ライフ\u{1f}\u{1f}Życie\u{1f}Vida\u{1f}Жизнь\u{1f}ช\u{e35}ว\u{e34}ต\u{1f}Can\u{1f}生命\u{1f}生命",
            "Begrenzt\u{1f}Limited\u{1f}Limitado\u{1f}Limitado\u{1f}Limité\u{1f}Limitato\u{1f}制限\u{1f}\u{1f}Ograniczone\u{1f}Limitado\u{1f}Ограничен\u{1f}จำก\u{e31}ด\u{1f}Sınırlı\u{1f}受限\u{1f}有限制",
            "Leben\u{1f}Lives\u{1f}Vidas\u{1f}Vidas\u{1f}Vies\u{1f}Vite\u{1f}ライフ\u{1f}\u{1f}Życia\u{1f}Vidas\u{1f}Жизни\u{1f}ช\u{e35}ว\u{e34}ต\u{1f}Canlar\u{1f}生命\u{1f}生命",
            "Ladung\u{1f}Load\u{1f}Cargar\u{1f}Cargar\u{1f}Charger\u{1f}Carica\u{1f}ロード\u{1f}\u{1f}Wczytaj\u{1f}Carregar\u{1f}Загрузить\u{1f}โหลด\u{1f}Yükle\u{1f}读取\u{1f}讀取",
            "Geladen\u{1f}Loaded\u{1f}Cargado\u{1f}Cargado\u{1f}Chargé\u{1f}Caricato\u{1f}ロードした\u{1f}\u{1f}Wczytano\u{1f}Carregado\u{1f}Загружен\u{1f}โหลดแล\u{e49}ว\u{1f}Yüklendi\u{1f}已读取\u{1f}已讀取",
            "Laden\u{1f}Loading\u{1f}Cargando\u{1f}Cargando\u{1f}Chargement\u{1f}Caricamento in Corso\u{1f}ロード中\u{1f}\u{1f}Wczytywanie\u{1f}Carregando\u{1f}Загружает\u{1f}กำล\u{e31}งโหลด\u{1f}Yükleniyor\u{1f}正在读取\u{1f}正在讀取",
            "Ort\u{1f}Location\u{1f}Ubicación\u{1f}Ubicación\u{1f}Lieu\u{1f}Luogo\u{1f}ロケーション\u{1f}\u{1f}Lokalizacja\u{1f}Local\u{1f}Местоположение\u{1f}สถานท\u{e35}\u{e48}\u{1f}Konum\u{1f}坐标\u{1f}地點",
            "Sperren\u{1f}Lock\u{1f}Bloquear\u{1f}Bloquear\u{1f}Verrouiller\u{1f}Blocca\u{1f}ロック\u{1f}\u{1f}Zablokuj\u{1f}Bloquear\u{1f}Закреплять\u{1f}ล\u{e47}อค\u{1f}Kilitle\u{1f}锁定\u{1f}鎖定",
            "Gesperrt\u{1f}Locked\u{1f}Bloqueado\u{1f}Bloqueado\u{1f}Verrouillé\u{1f}Bloccato\u{1f}ロックした\u{1f}\u{1f}Zablokowano\u{1f}Bloqueado\u{1f}Закрепил\u{1f}ล\u{e47}อคแล\u{e49}ว\u{1f}Kilitli\u{1f}未解锁\u{1f}受到鎖定",
            "Sperre\u{1f}Locking\u{1f}Bloqueando\u{1f}Bloqueando\u{1f}Verrouillage\u{1f}Blocco in Corso\u{1f}ロック中\u{1f}\u{1f}Blokowanie\u{1f}Bloqueando\u{1f}Закрепляет\u{1f}กำล\u{e31}งล\u{e47}อค\u{1f}Kilitleyen\u{1f}正在锁定\u{1f}正在鎖定",
            "Verlierer\u{1f}Loser\u{1f}Perdedor\u{1f}Perdedor\u{1f}Perdant\u{1f}Perdente\u{1f}敗者\u{1f}\u{1f}Przegrany\u{1f}Perdedor\u{1f}Проигравший\u{1f}ผ\u{e39}\u{e49}แพ\u{e49}\u{1f}Kaybeden\u{1f}败者\u{1f}輸家",
            "Unterlegen\u{1f}Losers\u{1f}Perdedores\u{1f}Perdedores\u{1f}Perdants\u{1f}Perdenti\u{1f}敗者\u{1f}\u{1f}Przegrani\u{1f}Perdedores\u{1f}Проигравшие\u{1f}ผ\u{e39}\u{e49}แพ\u{e49}\u{1f}Kaybedenler\u{1f}败者\u{1f}輸家",
            "Verloren\u{1f}Loss\u{1f}Derrota\u{1f}Derrota\u{1f}Défaite\u{1f}Sconfitta\u{1f}敗北\u{1f}\u{1f}Przegrana\u{1f}Derrota\u{1f}Поражение\u{1f}แพ\u{e49}\u{1f}Mağlubiyet\u{1f}失败\u{1f}落敗",
            "Niederlagen\u{1f}Losses\u{1f}Derrotas\u{1f}Derrotas\u{1f}Défaites\u{1f}Sconfitte\u{1f}敗北\u{1f}\u{1f}Przegrane\u{1f}Derrotas\u{1f}Поражения\u{1f}แพ\u{e49}\u{1f}Mağlubiyet\u{1f}失败\u{1f}落敗",
            "Max.\u{1f}Max\u{1f}Máx\u{1f}Máximo\u{1f}Maximum\u{1f}Massimo\u{1f}最大\u{1f}\u{1f}\u{1f}Máx.\u{1f}Макс.\u{1f}ส\u{e39}งส\u{e38}ด\u{1f}Maks.\u{1f}较大\u{1f}最大",
            "\u{1f}Mild\u{1f}Leve\u{1f}Leve\u{1f}Modéré\u{1f}Leggero\u{1f}易しい\u{1f}\u{1f}Łagodnie\u{1f}Brando\u{1f}Мягкий\u{1f}น\u{e49}อย\u{1f}Hafif\u{1f}轻微\u{1f}輕微",
            "Min.\u{1f}Min\u{1f}Mín\u{1f}Mínimo\u{1f}Minimum\u{1f}Minimo\u{1f}最小\u{1f}\u{1f}\u{1f}Mín.\u{1f}Мин.\u{1f}ต\u{e48}ำส\u{e38}ด\u{1f}Min.\u{1f}较小\u{1f}最小",
            "\u{1f}Mission\u{1f}Misión\u{1f}Misión\u{1f}\u{1f}Missione\u{1f}任務\u{1f}\u{1f}Misja\u{1f}Missão\u{1f}Задание\u{1f}ภารก\u{e34}จ\u{1f}Görev\u{1f}任务\u{1f}任務",
            "Mission abgebrochen\u{1f}Mission Aborted\u{1f}Misión abortada\u{1f}Misión abortada\u{1f}Mission annulée\u{1f}Missione Annullata\u{1f}任務中止\u{1f}\u{1f}Misja przerwana\u{1f}Missão Anulada\u{1f}Задание отменено\u{1f}ภารก\u{e34}จถ\u{e39}กยกเล\u{e34}ก\u{1f}Görev İptal Edildi\u{1f}放弃的任务\u{1f}任務中止",
            "Mission erfüllt\u{1f}Mission Accomplished\u{1f}Misión cumplida\u{1f}Misión cumplida\u{1f}Mission accomplie\u{1f}Missione Compiuta\u{1f}任務完了\u{1f}\u{1f}Misja wykonana\u{1f}Missão Cumprida\u{1f}Задание выполнено\u{1f}ภารก\u{e34}จสำเร\u{e47}จ\u{1f}Görev Başarılı\u{1f}任务完成\u{1f}任務完成",
            "Mission gescheitert\u{1f}Mission Failed\u{1f}Misión fracasada\u{1f}Misión fallida\u{1f}Échec de la mission\u{1f}Missione Fallita\u{1f}任務失敗\u{1f}\u{1f}Misja nieudana\u{1f}Missão Falhou\u{1f}Задание провалено\u{1f}ภารก\u{e34}จล\u{e49}มเหลว\u{1f}Görev Başarısız\u{1f}失败的任务\u{1f}任務失敗",
            "Missionen\u{1f}Missions\u{1f}Misiones\u{1f}Misiones\u{1f}\u{1f}Missioni\u{1f}任務\u{1f}\u{1f}Misje\u{1f}Missões\u{1f}Задания\u{1f}ภารก\u{e34}จ\u{1f}Görevler\u{1f}任务\u{1f}任務",
            "Moderat\u{1f}Moderate\u{1f}Moderado\u{1f}Moderado\u{1f}Modéré\u{1f}Moderato\u{1f}中程度\u{1f}\u{1f}Umiarkowanie\u{1f}Moderado\u{1f}Умеренный\u{1f}ปานกลาง\u{1f}Makul\u{1f}适中\u{1f}中等",
            "Geld\u{1f}Money\u{1f}Dinero\u{1f}Dinero\u{1f}Argent\u{1f}Denaro\u{1f}金\u{1f}\u{1f}Pieniądze\u{1f}Dinheiro\u{1f}Деньги\u{1f}เง\u{e34}น\u{1f}Para\u{1f}金钱\u{1f}金錢",
            "\u{1f}Monster\u{1f}Monstruo\u{1f}Monstruo\u{1f}Monstre\u{1f}Mostro\u{1f}モンスター\u{1f}\u{1f}Potwór\u{1f}Monstro\u{1f}Монстр\u{1f}ส\u{e31}ตว\u{e4c}ประหลาด\u{1f}Canavar\u{1f}怪物\u{1f}怪物",
            "Ungeheuer\u{1f}Monsters\u{1f}Monstruos\u{1f}Monstruos\u{1f}Monstres\u{1f}Mostri\u{1f}モンスター\u{1f}\u{1f}Potwory\u{1f}Monstros\u{1f}Монстры\u{1f}ส\u{e31}ตว\u{e4c}ประหลาด\u{1f}Canavarlar\u{1f}怪物\u{1f}怪物",
            "Mehr\u{1f}More\u{1f}Más\u{1f}Más\u{1f}Plus\u{1f}Più\u{1f}多い\u{1f}\u{1f}Więcej\u{1f}Mais\u{1f}Больше\u{1f}มากกว\u{e48}า\u{1f}Daha Fazla\u{1f}更多\u{1f}較多",
            "Am meisten\u{1f}Most\u{1f}Máximo\u{1f}Máximo\u{1f}Le plus\u{1f}Il Più\u{1f}最高\u{1f}\u{1f}Najwięcej\u{1f}O Maior\u{1f}Больше всего\u{1f}มากท\u{e35}\u{e48}ส\u{e38}ด\u{1f}En Fazla\u{1f}最多\u{1f}最多",
            "Mein Fehler\u{1f}My Mistake\u{1f}Mi culpa\u{1f}Fue mi error\u{1f}C’est de ma faute\u{1f}Errore Mio\u{1f}申し訳ない\u{1f}\u{1f}Mój błąd\u{1f}Erro Meu\u{1f}Моя ошибка\u{1f}ความผ\u{e34}ดพลาดของฉ\u{e31}น\u{1f}Pardon\u{1f}我的错\u{1f}是我不好",
            "Nahe\u{1f}Near\u{1f}Cerca\u{1f}Cerca\u{1f}Près\u{1f}Vicino\u{1f}近く\u{1f}\u{1f}Blisko\u{1f}Perto\u{1f}Рядом\u{1f}ใกล\u{e49}\u{1f}Yakın\u{1f}近\u{1f}近",
            "Neuer Highscore\u{1f}New High Score\u{1f}Nueva mejor puntuación\u{1f}Nueva puntuación alta\u{1f}Nouveau meilleur score\u{1f}Nuovo Punteggio Migliore\u{1f}ハイスコア更新\u{1f}\u{1f}Nowy najwyższy wynik\u{1f}Nova Pontuação Máxima\u{1f}Новый рекорд\u{1f}คะแนนส\u{e39}งส\u{e38}ดใหม\u{e48}\u{1f}Yeni Yüksek Skor\u{1f}最新高分\u{1f}全新最高分",
            "Neuer Rekord\u{1f}New Record\u{1f}Nuevo récord\u{1f}Nuevo récord\u{1f}Nouveau record\u{1f}Nuovo Record\u{1f}新記録\u{1f}\u{1f}Nowy rekord\u{1f}Novo Recorde\u{1f}Новая запись\u{1f}สถ\u{e34}ต\u{e34}ใหม\u{e48}\u{1f}Yeni Rekor\u{1f}最新记录\u{1f}全新紀錄",
            "Nächster\u{1f}Next\u{1f}Siguiente\u{1f}Siguiente\u{1f}Suivant\u{1f}Successivo\u{1f}次へ\u{1f}\u{1f}Dalej\u{1f}Próximo\u{1f}Следующий\u{1f}ถ\u{e31}ดไป\u{1f}Sıradaki\u{1f}下一个\u{1f}下一個",
            "Nächste Verbündete\u{1f}Next Allies\u{1f}Siguientes objetivos\u{1f}Siguientes aliados\u{1f}Alliés suivants\u{1f}Alleati Successivi\u{1f}次の味方\u{1f}\u{1f}Następni sojusznicy\u{1f}Próximos Aliados\u{1f}Следующие союзники\u{1f}เพ\u{e37}\u{e48}อนร\u{e48}วมท\u{e35}มถ\u{e31}ดไป\u{1f}Sıradaki Müttefikler\u{1f}下一批盟友\u{1f}下個盟友",
            "Nächster Verbündeter\u{1f}Next Ally\u{1f}Siguiente aliado\u{1f}Siguiente aliado\u{1f}Allié suivant\u{1f}Alleato Successivo\u{1f}次の味方\u{1f}\u{1f}Następny sojusznik\u{1f}Próximo Aliado\u{1f}Следующий союзник\u{1f}เพ\u{e37}\u{e48}อนร\u{e48}วมท\u{e35}มถ\u{e31}ดไป\u{1f}Sıradaki Müttefik\u{1f}下一个盟友\u{1f}下個盟友",
            "Nächster Versuch\u{1f}Next Attempt\u{1f}Siguiente intento\u{1f}Siguiente intento\u{1f}Tentative suivante\u{1f}Tentativo Successivo\u{1f}次の挑戦\u{1f}\u{1f}Następna próba\u{1f}Próxima Tentativa\u{1f}Следующая попытка\u{1f}ความพยายามถ\u{e31}ดไป\u{1f}Sonraki Deneme\u{1f}下一次尝试\u{1f}下次機會",
            "Nächster Checkpoint\u{1f}Next Checkpoint\u{1f}Siguiente punto de control\u{1f}Siguiente punto de control\u{1f}Point de contrôle suivant\u{1f}Checkpoint Successivo\u{1f}次のチェックポイント\u{1f}\u{1f}Następny punkt kontrolny\u{1f}Próximo Ponto de Verificação\u{1f}Следующая контрольная точка\u{1f}เช\u{e47}คพอยต\u{e4c}ถ\u{e31}ดไป\u{1f}Sonraki Kontrol Noktası\u{1f}下一个检查点\u{1f}下個檢查點",
            "Nächste Gegner\u{1f}Next Enemies\u{1f}Siguientes enemigos\u{1f}Siguientes enemigos\u{1f}Ennemis suivants\u{1f}Nemici Successivi\u{1f}次の敵\u{1f}\u{1f}Następni wrogowie\u{1f}Próximos Inimigos\u{1f}Следующие враги\u{1f}ศ\u{e31}ตร\u{e39}ถ\u{e31}ดไป\u{1f}Sonraki Düşmanlar\u{1f}下一批敌人\u{1f}下個敵人",
            "Nächster Gegner\u{1f}Next Enemy\u{1f}Siguiente enemigo\u{1f}Siguiente enemigo\u{1f}Ennemi suivant\u{1f}Nemico Successivo\u{1f}次の敵\u{1f}\u{1f}Następny wróg\u{1f}Próximo Inimigo\u{1f}Следующий враг\u{1f}ศ\u{e31}ตร\u{e39}ถ\u{e31}ดไป\u{1f}Sıradaki Düşman\u{1f}下一个敌人\u{1f}下個敵人",
            "Nächste Form\u{1f}Next Form\u{1f}Siguiente forma\u{1f}Siguiente forma\u{1f}Forme suivante\u{1f}Forma Successiva\u{1f}次のフォーム\u{1f}\u{1f}Następna forma\u{1f}Próximo Forma\u{1f}Следующая форма\u{1f}ร\u{e48}างถ\u{e31}ดไป\u{1f}Sonraki Form\u{1f}下一个表格\u{1f}下個型態",
            "Nächstes Spiel\u{1f}Next Game\u{1f}Siguiente partida\u{1f}Siguiente partida\u{1f}Partie suivante\u{1f}Partita Successiva\u{1f}次のマッチ\u{1f}\u{1f}Następna gra\u{1f}Próximo Jogo\u{1f}Следующий матч\u{1f}เกมถ\u{e31}ดไป\u{1f}Sonraki Oyun\u{1f}下一个游戏\u{1f}下場遊戲",
            "Nächster Held\u{1f}Next Hero\u{1f}Siguiente héroe\u{1f}Siguiente héroe\u{1f}Héros suivant\u{1f}Eroe Successivo\u{1f}次のヒーロー\u{1f}\u{1f}Następny bohater\u{1f}Próximo Herói\u{1f}Следующий герой\u{1f}ฮ\u{e35}โร\u{e48}ถ\u{e31}ดไป\u{1f}Sıradaki Kahraman\u{1f}下一个英雄\u{1f}下個英雄",
            "Nächste Helden\u{1f}Next Heroes\u{1f}Siguientes héroes\u{1f}Siguientes héroes\u{1f}Héros suivants\u{1f}Eroi Successivi\u{1f}次のヒーロー\u{1f}\u{1f}Następni bohaterowie\u{1f}Próximos Heróis\u{1f}Следующие герои\u{1f}ฮ\u{e35}โร\u{e48}ถ\u{e31}ดไป\u{1f}Sıradaki Kahramanlar\u{1f}下一批英雄\u{1f}下個英雄",
            "Nächste Geisel\u{1f}Next Hostage\u{1f}Siguiente rehén\u{1f}Siguiente rehén\u{1f}Otage suivant\u{1f}Ostaggio Successivo\u{1f}次の人質\u{1f}\u{1f}Następny zakładnik\u{1f}Próximo Refém\u{1f}Следующий заложник\u{1f}ต\u{e31}วประก\u{e31}นถ\u{e31}ดไป\u{1f}Sıradaki Rehine\u{1f}下一个人质\u{1f}下個人質",
            "Nächste Geiseln\u{1f}Next Hostages\u{1f}Siguientes rehenes\u{1f}Siguientes rehenes\u{1f}Otages suivants\u{1f}Ostaggi Successivi\u{1f}次の人質\u{1f}\u{1f}Następni zakładnicy\u{1f}Próximos Reféns\u{1f}Следующие заложники\u{1f}ต\u{e31}วประก\u{e31}นถ\u{e31}ดไป\u{1f}Sıradaki Rehineler\u{1f}下一批人质\u{1f}下個人質",
            "Nächster Level\u{1f}Next Level\u{1f}Siguiente nivel\u{1f}Siguiente nivel\u{1f}Niveau suivant\u{1f}Livello Successivo\u{1f}次のレベル\u{1f}\u{1f}Następny poziom\u{1f}Próximo Nível\u{1f}Следующий уровень\u{1f}เลเวลถ\u{e31}ดไป\u{1f}Sonraki Seviye\u{1f}下一个等级\u{1f}下個等級",
            "Nächste Mission\u{1f}Next Mission\u{1f}Siguiente misión\u{1f}Siguiente misión\u{1f}Mission suivante\u{1f}Missione Successiva\u{1f}次の任務\u{1f}\u{1f}Następna misja\u{1f}Próxima Missão\u{1f}Следующее задание\u{1f}ภารก\u{e34}จถ\u{e31}ดไป\u{1f}Sonraki Görev\u{1f}下一个任务\u{1f}下個任務",
            "Nächstes Objekt\u{1f}Next Object\u{1f}Siguiente objeto\u{1f}Siguiente objeto\u{1f}Objet suivant\u{1f}Oggetto Successivo\u{1f}次のオブジェクト\u{1f}\u{1f}Następny obiekt\u{1f}Próximo Objeto\u{1f}Следующий объект\u{1f}ว\u{e31}ตถ\u{e38}ถ\u{e31}ดไป\u{1f}Sıradaki Nesne\u{1f}下一批对象\u{1f}下個目標",
            "Nächster Zielpunkt\u{1f}Next Objective\u{1f}Siguiente objetivo\u{1f}Siguiente objetivo\u{1f}Objectif suivant\u{1f}Obiettivo Successivo\u{1f}次の目標\u{1f}\u{1f}Następne zadanie\u{1f}Próximo Objetivo\u{1f}Следующая задача\u{1f}เป\u{e49}าหมายถ\u{e31}ดไป\u{1f}Sıradaki Hedef\u{1f}下一个目标点\u{1f}下個目標",
            "Nächste Objekte\u{1f}Next Objects\u{1f}Siguientes objetos\u{1f}Siguientes objetos\u{1f}Objets suivants\u{1f}Oggetti Successivi\u{1f}次のオブジェクト\u{1f}\u{1f}Następne obiekty\u{1f}Próximos Objetos\u{1f}Следующие объекты\u{1f}ว\u{e31}ตถ\u{e38}ถ\u{e31}ดไป\u{1f}Sıradaki Nesneler\u{1f}下一个对象\u{1f}下個目標",
            "Nächste Phase\u{1f}Next Phase\u{1f}Siguiente fase\u{1f}Siguiente fase\u{1f}Phase suivante\u{1f}Fase Successiva\u{1f}次のフェーズ \u{1f}\u{1f}Następna faza\u{1f}Próxima Fase\u{1f}Следующая фаза\u{1f}เฟสถ\u{e31}ดไป\u{1f}Sonraki Safha\u{1f}下一个阶段\u{1f}下個階段",
            "Nächster Spieler\u{1f}Next Player\u{1f}Siguiente jugador\u{1f}Siguiente jugador\u{1f}Joueur suivant\u{1f}Giocatore Successivo\u{1f}次のプレイヤー\u{1f}\u{1f}Następny gracz\u{1f}Próximo Jogador\u{1f}Следующий игрок\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นถ\u{e31}ดไป\u{1f}Sıradaki Oyuncu\u{1f}下一个玩家\u{1f}下個玩家",
            "Nächste Spieler\u{1f}Next Players\u{1f}Siguientes jugadores\u{1f}Siguientes jugadores\u{1f}Joueurs suivants\u{1f}Giocatori Successivi\u{1f}次のプレイヤー\u{1f}\u{1f}Następni gracze\u{1f}Próximos Jogadores\u{1f}Следующие игроки\u{1f}ผ\u{e39}\u{e49}เล\u{e48}นถ\u{e31}ดไป\u{1f}Sıradaki Oyuncular\u{1f}下一批玩家\u{1f}下個玩家",
            "Nächste Runde\u{1f}Next Round\u{1f}Siguiente ronda\u{1f}Siguiente ronda\u{1f}Manche suivante\u{1f}Round Successivo\u{1f}次のラウンド\u{1f}\u{1f}Następna runda\u{1f}Próxima Rodada\u{1f}Следующий раунд\u{1f}รอบถ\u{e31}ดไป\u{1f}Sıradaki Tur\u{1f}下一个回合\u{1f}下個回合",
            "Nächstes Ziel\u{1f}Next Target\u{1f}Siguiente objetivo\u{1f}Siguiente objetivo\u{1f}Cible suivante\u{1f}Bersaglio Successivo\u{1f}次のターゲット\u{1f}\u{1f}Następny cel\u{1f}Próximo Alvo\u{1f}Следующая цель\u{1f}เป\u{e49}าถ\u{e31}ดไป\u{1f}Sıradaki Hedef\u{1f}下一个目标\u{1f}下個目標",
            "Nächste Ziele\u{1f}Next Targets\u{1f}Siguientes objetivos\u{1f}Siguientes objetivos\u{1f}Cibles suivantes\u{1f}Bersagli Successivi\u{1f}次のターゲット\u{1f}\u{1f}Następne cele\u{1f}Próximos Alvos\u{1f}Следующие цели\u{1f}เป\u{e49}าถ\u{e31}ดไป\u{1f}Sıradaki Hedefler\u{1f}下一批目标\u{1f}下個目標",
            "Nächstes Upgrade\u{1f}Next Upgrade\u{1f}Siguiente mejora\u{1f}Siguiente mejora\u{1f}Amélioration suivante\u{1f}Miglioramento Successivo\u{1f}次のアップグレード\u{1f}\u{1f}Następne ulepszenie\u{1f}Próxima Melhoria\u{1f}Следующее улучшение\u{1f}การอ\u{e31}พเกรดถ\u{e31}ดไป\u{1f}Sonraki Yükseltme\u{1f}下一个升级 \u{1f}下個升級",
            "Netter Versuch\u{1f}Nice Try\u{1f}Buen intento\u{1f}Buen intento\u{1f}Bien essayé\u{1f}Bel Tentativo\u{1f}ドンマイ\u{1f}\u{1f}Niezła próba\u{1f}Bela Tentativa\u{1f}Хорошая попытка\u{1f}เก\u{e37}อบด\u{e35}แล\u{e49}ว\u{1f}Yemezler\u{1f}表现不错\u{1f}差一點",
            "Nein\u{1f}No\u{1f}\u{1f}\u{1f}Non\u{1f}\u{1f}いいえ\u{1f}\u{1f}Nie\u{1f}Não\u{1f}Нет\u{1f}ไม\u{e48}ใช\u{e48}\u{1f}Hayır\u{1f}否\u{1f}否",
            "Nein danke\u{1f}No Thanks\u{1f}No, gracias\u{1f}No, gracias\u{1f}Non merci\u{1f}No Grazie\u{1f}結構です\u{1f}\u{1f}Nie, dzięki\u{1f}Não, Obrigado\u{1f}Нет, спасибо\u{1f}ไม\u{e48} ขอบค\u{e38}ณ\u{1f}Yok Sağ Ol\u{1f}不用了\u{1f}不，謝謝",
            "Nichts\u{1f}None\u{1f}Nada\u{1f}Ninguno\u{1f}Aucun\u{1f}Nessuno\u{1f}なし\u{1f}\u{1f}Brak\u{1f}Nenhum\u{1f}Нет\u{1f}ไม\u{e48}ม\u{e35}\u{1f}Yok\u{1f}无\u{1f}無",
            "\u{1f}Normal\u{1f}\u{1f}\u{1f}\u{1f}Normale\u{1f}ノーマル\u{1f}\u{1f}Normalne\u{1f}\u{1f}Нормальный\u{1f}ปกต\u{e34}\u{1f}\u{1f}普通\u{1f}普通",
            "Norden\u{1f}North\u{1f}Norte\u{1f}Norte\u{1f}Nord\u{1f}Nord\u{1f}北\u{1f}\u{1f}Północ\u{1f}Norte\u{1f}Север\u{1f}เหน\u{e37}อ\u{1f}Kuzey\u{1f}北\u{1f}北",
            "Nordosten\u{1f}Northeast\u{1f}Noreste\u{1f}Noreste\u{1f}Nord-est\u{1f}Nord-est\u{1f}北東\u{1f}\u{1f}Północny wschód\u{1f}Nordeste\u{1f}Северо-восток\u{1f}ตะว\u{e31}นออกเฉ\u{e35}ยงเหน\u{e37}อ\u{1f}Kuzeydoğu\u{1f}东北\u{1f}東北",
            "Nordwesten\u{1f}Northwest\u{1f}Noroeste\u{1f}Noroeste\u{1f}Nord-ouest\u{1f}Nord-ovest\u{1f}北西\u{1f}\u{1f}Północny zachód\u{1f}Noroeste\u{1f}Северо-запад\u{1f}ตะว\u{e31}นตกเฉ\u{e35}ยงเหน\u{e37}อ\u{1f}Kuzeybatı\u{1f}西北\u{1f}西北",
            "Nicht heute\u{1f}Not Today\u{1f}Esta vez no\u{1f}Hoy no\u{1f}Pas aujourd’hui\u{1f}Non Oggi\u{1f}やめておこう\u{1f}\u{1f}Nie dzisiaj\u{1f}Hoje Não\u{1f}Не сегодня\u{1f}ไม\u{e48}ใช\u{e48}ว\u{e31}นน\u{e35}\u{e49}\u{1f}Bugün Değil\u{1f}算了吧\u{1f}下次再說",
            "Objekt\u{1f}Object\u{1f}Objeto\u{1f}Objeto\u{1f}Objet\u{1f}Oggetto\u{1f}オブジェクト\u{1f}\u{1f}Obiekt\u{1f}Objeto\u{1f}Объект\u{1f}ว\u{e31}ตถ\u{e38}\u{1f}Nesne\u{1f}对象\u{1f}目標",
            "Zielpunkt\u{1f}Objective\u{1f}Objetivo\u{1f}Objetivo\u{1f}Objectif\u{1f}Obiettivo\u{1f}目標\u{1f}\u{1f}Zadanie\u{1f}Objetivo\u{1f}Задача\u{1f}เป\u{e49}าหมาย\u{1f}Hedef\u{1f}目标点\u{1f}目標",
            "Zielpunkte\u{1f}Objectives\u{1f}Objetivos\u{1f}Objetivos\u{1f}Objectifs\u{1f}Obiettivi\u{1f}目標\u{1f}\u{1f}Zadania\u{1f}Objetivos\u{1f}Задачи\u{1f}เป\u{e49}าหมาย\u{1f}Hedefler\u{1f}目标点\u{1f}目標",
            "Objekte\u{1f}Objects\u{1f}Objetos\u{1f}Objetos\u{1f}Objets\u{1f}Oggetti\u{1f}オブジェクト\u{1f}\u{1f}Obiekty\u{1f}Objetos\u{1f}Объекты\u{1f}ว\u{e31}ตถ\u{e38}\u{1f}Nesneler\u{1f}对象\u{1f}目標",
            "Erhalte\u{1f}Obtain\u{1f}Obtener\u{1f}Obtener\u{1f}Obtenir\u{1f}Ottieni\u{1f}取得\u{1f}필요 조건\u{1f}Zdobądź\u{1f}Obter\u{1f}Получить\u{1f}ได\u{e49}ร\u{e31}บ\u{1f}Gereken:\u{1f}获取\u{1f}取得",
            "Erlangt\u{1f}Obtained\u{1f}Obtenido\u{1f}Obtenido\u{1f}Obtenu\u{1f}Ottenuto\u{1f}取得した\u{1f}획득\u{1f}Zdobyto\u{1f}Obtido\u{1f}Получил\u{1f}ได\u{e49}ร\u{e31}บแล\u{e49}ว\u{1f}Alınan\u{1f}已获取\u{1f}取得",
            "Erhält\u{1f}Obtaining\u{1f}Obteniendo\u{1f}Obteniendo\u{1f}Obtention\u{1f}Ottenimento in Corso\u{1f}取得中\u{1f}획득\u{1f}Zdobywanie\u{1f}Obtendo\u{1f}Получает\u{1f}กำล\u{e31}งได\u{e49}ร\u{e31}บ\u{1f}Alan\u{1f}正在获取\u{1f}正在取得",
            "Aus\u{1f}Off\u{1f}Desactivado\u{1f}No\u{1f}Désactivé\u{1f}\u{1f}OFF\u{1f}\u{1f}Wył.\u{1f}Desligado\u{1f}Выкл.\u{1f}ป\u{e34}ด\u{1f}Kapalı\u{1f}关闭\u{1f}關閉",
            "Ein\u{1f}On\u{1f}Activado\u{1f}Activado\u{1f}Activé\u{1f}\u{1f}ON\u{1f}\u{1f}Wł.\u{1f}Ligado\u{1f}Вкл.\u{1f}เป\u{e34}ด\u{1f}Açık\u{1f}开启\u{1f}開啟",
            "Uff\u{1f}Oof\u{1f}Uf\u{1f}Uff\u{1f}Aïe\u{1f}Ooof\u{1f}うーん\u{1f}\u{1f}Uch\u{1f}Uff\u{1f}Ох\u{1f}อ\u{e4a}ะ\u{1f}Ayy\u{1f}噢\u{1f}噢",
            "Ups\u{1f}Oops\u{1f}Ay\u{1f}Ups\u{1f}Oups\u{1f}Ops\u{1f}おっと\u{1f}\u{1f}Ups\u{1f}Ops\u{1f}Упс\u{1f}อ\u{e38}\u{e4a}ปส\u{e4c}\u{1f}Tüh\u{1f}啊呀\u{1f}哎呀",
            "\u{1f}Optimal\u{1f}Óptimo\u{1f}Óptimo\u{1f}\u{1f}Ottimale\u{1f}最適\u{1f}\u{1f}Optymalne\u{1f}Ideal\u{1f}Оптимальный\u{1f}ต\u{e31}วเล\u{e37}อกท\u{e35}\u{e48}ด\u{e35}ส\u{e38}ด\u{1f}Standart\u{1f}最佳\u{1f}最佳",
            "Optimieren\u{1f}Optimize\u{1f}Optimizar\u{1f}Optimizar\u{1f}Optimiser\u{1f}Ottimizza\u{1f}最適化\u{1f}\u{1f}Zoptymalizuj\u{1f}Otimizar\u{1f}Оптимизировать\u{1f}เพ\u{e34}\u{e48}มประส\u{e34}ทธ\u{e34}ภาพ\u{1f}Optimize Et\u{1f}优化\u{1f}最佳化",
            "Optimiert\u{1f}Optimized\u{1f}Optimizado\u{1f}Optimizado\u{1f}Optimisé\u{1f}Ottimizzato\u{1f}最適化した\u{1f}\u{1f}Zoptymalizowano\u{1f}Otimizado\u{1f}Оптимизирован\u{1f}เพ\u{e34}\u{e48}มประส\u{e34}ทธ\u{e34}ภาพแล\u{e49}ว\u{1f}Optimize Edilen\u{1f}已优化\u{1f}獲得最佳化",
            "Optimierung\u{1f}Optimizing\u{1f}Optimizando\u{1f}Optimizando\u{1f}Optimisation\u{1f}Ottimizzazione in Corso\u{1f}最適化中\u{1f}\u{1f}Optymalizowanie\u{1f}Otimizando\u{1f}Оптимизирует\u{1f}กำล\u{e31}งเพ\u{e34}\u{e48}มประส\u{e34}ทธ\u{e34}ภาพ\u{1f}Optimize Eden\u{1f}正在优化\u{1f}正在最佳化",
            "Nicht in Sicht\u{1f}Out of View\u{1f}Fuera de la vista\u{1f}Fuera de vista\u{1f}Hors de la vue\u{1f}Fuori Visuale\u{1f}表示外\u{1f}\u{1f}Poza pole widzenia\u{1f}Fora da Visão\u{1f}Вне поля зрения\u{1f}นอกม\u{e38}มมอง\u{1f}Görüş Dışı\u{1f}不在视野中\u{1f}不在視野中",
            "Ausgehend\u{1f}Outgoing\u{1f}Saliente\u{1f}En camino\u{1f}Sortant\u{1f}In Uscita\u{1f}送信\u{1f}\u{1f}Wychodzące\u{1f}Saindo\u{1f}Исходящий\u{1f}ขาออก\u{1f}Giden\u{1f}正在远去\u{1f}離開",
            "Außerhalb\u{1f}Outside\u{1f}Fuera\u{1f}Afuera\u{1f}Dehors\u{1f}Fuori\u{1f}外\u{1f}\u{1f}Zewnątrz\u{1f}Fora\u{1f}Снаружи\u{1f}ด\u{e49}านนอก\u{1f}Dış\u{1f}外\u{1f}外部",
            "Über\u{1f}Over\u{1f}Encima\u{1f}Sobre\u{1f}Sur\u{1f}Sopra\u{1f}上\u{1f}\u{1f}Nad\u{1f}Acima\u{1f}Более\u{1f}เหน\u{e37}อ\u{1f}Üst\u{1f}上方\u{1f}超過",
            "Verlängerung\u{1f}Overtime\u{1f}Tiempo extra\u{1f}Tiempo extra\u{1f}Prolongations\u{1f}Tempo supplementare\u{1f}オーバータイム\u{1f}\u{1f}Dogrywka\u{1f}Prorrogação\u{1f}Дополнительное время\u{1f}ต\u{e48}อเวลา\u{1f}Uzatma\u{1f}加时\u{1f}延長賽",
            "Teilnehmer\u{1f}Participant\u{1f}Participante\u{1f}Participante\u{1f}\u{1f}Partecipante\u{1f}参加者\u{1f}\u{1f}Uczestnik\u{1f}Participante\u{1f}Участник\u{1f}ผ\u{e39}\u{e49}เข\u{e49}าร\u{e48}วม\u{1f}Katılımcı\u{1f}参与者\u{1f}參與者",
            "Teilnehmer\u{1f}Participants\u{1f}Participantes\u{1f}Participantes\u{1f}\u{1f}Partecipanti\u{1f}参加者\u{1f}\u{1f}Uczestnicy\u{1f}Participantes\u{1f}Участники\u{1f}ผ\u{e39}\u{e49}เข\u{e49}าร\u{e48}วม\u{1f}Katılımcılar\u{1f}参与者\u{1f}參與者",
            "Fracht\u{1f}Payload\u{1f}Carga\u{1f}Carga\u{1f}Convoi\u{1f}Carico\u{1f}ペイロード\u{1f}\u{1f}Ładunek\u{1f}Carga\u{1f}Груз\u{1f}เพย\u{e4c}โหลด\u{1f}Navlun\u{1f}运载目标\u{1f}護送目標",
            "Frachten\u{1f}Payloads\u{1f}Cargas\u{1f}Cargas\u{1f}Convois\u{1f}Carichi\u{1f}ペイロード\u{1f}\u{1f}Ładunki\u{1f}Cargas\u{1f}Грузы\u{1f}เพย\u{e4c}โหลด\u{1f}Navlunlar\u{1f}运载目标\u{1f}護送目標",
            "\u{1f}Phase\u{1f}Fase\u{1f}Fase\u{1f}\u{1f}Fase\u{1f}フェーズ\u{1f}\u{1f}Faza\u{1f}Fase\u{1f}Фаза\u{1f}เฟส\u{1f}Safha\u{1f}阶段\u{1f}階段",
            "Phasen\u{1f}Phases\u{1f}Fases\u{1f}Fases\u{1f}\u{1f}Fasi\u{1f}フェーズ\u{1f}\u{1f}Fazy\u{1f}Fases\u{1f}Фазы\u{1f}เฟส\u{1f}Safhalar\u{1f}阶段\u{1f}階段",
            "Wählen\u{1f}Pick\u{1f}Elegir\u{1f}Elegir\u{1f}Prendre\u{1f}Scegli\u{1f}選択\u{1f}\u{1f}Wybierz\u{1f}Escolher\u{1f}Поднять\u{1f}หย\u{e34}บ\u{1f}Al\u{1f}选取\u{1f}挑選",
            "Gewählt\u{1f}Picked\u{1f}Elegido\u{1f}Elegido\u{1f}Pris\u{1f}Scelto\u{1f}選択した\u{1f}\u{1f}Wybrano\u{1f}Escolhido\u{1f}Поднял\u{1f}หย\u{e34}บแล\u{e49}ว\u{1f}Alınan\u{1f}已选取\u{1f}受到挑選",
            "Wahl\u{1f}Picking\u{1f}Eligiendo\u{1f}Eligiendo\u{1f}Prise\u{1f}Scelta in Corso\u{1f}選択中\u{1f}\u{1f}Wybieranie\u{1f}Escolhendo\u{1f}Поднимает\u{1f}กำล\u{e31}งหย\u{e34}บ\u{1f}Alan\u{1f}正在选取\u{1f}正在挑選",
            "Stapel\u{1f}Pile\u{1f}Pila\u{1f}Pila\u{1f}\u{1f}Pila\u{1f}パイル\u{1f}\u{1f}Stosik\u{1f}Bolo\u{1f}Отбить\u{1f}กอง\u{1f}Yığın\u{1f}堆\u{1f}牌堆",
            "Haufen\u{1f}Piles\u{1f}Pilas\u{1f}Pilas\u{1f}\u{1f}Pile\u{1f}パイル\u{1f}\u{1f}Stosiki\u{1f}Bolos\u{1f}Отбивает\u{1f}กอง\u{1f}Yığınlar\u{1f}堆\u{1f}牌堆",
            "Spielen\u{1f}Play\u{1f}Jugar\u{1f}Jugar\u{1f}Jouer\u{1f}Gioca\u{1f}プレイする\u{1f}\u{1f}Zagraj\u{1f}Jogar\u{1f}Разыграть\u{1f}เล\u{e48}น\u{1f}Oyna\u{1f}打出\u{1f}遊玩",
            "Gespielt\u{1f}Played\u{1f}Jugado\u{1f}Jugado\u{1f}Joué\u{1f}Giocato\u{1f}プレイされた\u{1f}\u{1f}Zagrano\u{1f}Jogada\u{1f}Разыграл\u{1f}เล\u{e48}นแล\u{e49}ว\u{1f}Oynanan\u{1f}已打出\u{1f}遊玩",
            "Spieler\u{1f}Player\u{1f}Jugador\u{1f}Jugador\u{1f}Joueur\u{1f}Giocatore\u{1f}プレイヤー\u{1f}\u{1f}Gracz\u{1f}Jogador\u{1f}Игрок\u{1f}ผ\u{e39}\u{e49}เล\u{e48}น\u{1f}Oyuncu\u{1f}玩家\u{1f}玩家",
            "Spieler\u{1f}Players\u{1f}Jugadores\u{1f}Jugadores\u{1f}Joueurs\u{1f}Giocatori\u{1f}プレイヤー\u{1f}\u{1f}Gracze\u{1f}Jogadores\u{1f}Игроки\u{1f}ผ\u{e39}\u{e49}เล\u{e48}น\u{1f}Oyuncular\u{1f}玩家\u{1f}玩家",
            "Spiel\u{1f}Playing\u{1f}Jugando\u{1f}Jugando\u{1f}Joue\u{1f}Gioco in Corso\u{1f}プレイしている\u{1f}\u{1f}Zagranie\u{1f}Jogando\u{1f}Разыгрывает\u{1f}กำล\u{e31}งเล\u{e48}น\u{1f}Oynayan\u{1f}正在打出\u{1f}遊玩",
            "Punkt\u{1f}Point\u{1f}Punto\u{1f}Punto\u{1f}\u{1f}Punto\u{1f}ポイント\u{1f}\u{1f}Punkt\u{1f}Ponto\u{1f}Очко\u{1f}คะแนน\u{1f}Puan\u{1f}点\u{1f}分",
            "Punkte\u{1f}Points\u{1f}Puntos\u{1f}Puntos\u{1f}\u{1f}Punti\u{1f}ポイント\u{1f}\u{1f}Punkty\u{1f}Pontos\u{1f}Очки\u{1f}คะแนน\u{1f}Puan\u{1f}点\u{1f}分",
            "Punkte erhalten\u{1f}Points Earned\u{1f}Puntos ganados\u{1f}Puntos obtenidos\u{1f}Points gagnés\u{1f}Punti Guadagnati\u{1f}獲得ポイント\u{1f}\u{1f}Zdobyte punkty\u{1f}Pontos Ganhos\u{1f}Очков заработано\u{1f}คะแนนท\u{e35}\u{e48}ได\u{e49}ร\u{e31}บ\u{1f}Kazanılan Puanlar\u{1f}获得分数\u{1f}獲得的分數",
            "Punkte verloren\u{1f}Points Lost\u{1f}Puntos perdidos\u{1f}Puntos perdidos\u{1f}Points perdus\u{1f}Punti Persi\u{1f}損失ポイント\u{1f}\u{1f}Stracone punkty\u{1f}Pontos Perdidos\u{1f}Очков потеряно\u{1f}คะแนนท\u{e35}\u{e48}เส\u{e35}ย\u{1f}Kaybedilen Puanlar\u{1f}失去分数\u{1f}失去的點",
            "\u{1f}Position\u{1f}Posición\u{1f}Posición\u{1f}\u{1f}Posizione\u{1f}位置\u{1f}\u{1f}Pozycja\u{1f}Posição\u{1f}Положение\u{1f}ตำแหน\u{e48}ง\u{1f}Konum\u{1f}位置\u{1f}位置",
            "Macht\u{1f}Power\u{1f}Poder\u{1f}Poder\u{1f}Puissance\u{1f}Potenza\u{1f}パワー\u{1f}\u{1f}Moc\u{1f}Poder\u{1f}Энергия\u{1f}พล\u{e31}ง\u{1f}Güç\u{1f}能量\u{1f}能量",
            "\u{1f}Power-up\u{1f}Potenciador\u{1f}Potenciador\u{1f}Bonus\u{1f}Potenziamento\u{1f}パワーアップ\u{1f}\u{1f}Doładowanie\u{1f}\u{1f}Усиление\u{1f}เพ\u{e34}\u{e48}มพล\u{e31}ง\u{1f}Güçlendirme\u{1f}强化\u{1f}強化",
            "\u{1f}Power-ups\u{1f}Potenciadores\u{1f}Potenciadores\u{1f}Bonus\u{1f}Potenziamenti\u{1f}パワーアップ\u{1f}\u{1f}Doładowania\u{1f}\u{1f}Усиления\u{1f}เพ\u{e34}\u{e48}มพล\u{e31}ง\u{1f}Güçlendirmeler\u{1f}强化\u{1f}強化",
            "Preis\u{1f}Price\u{1f}Precio\u{1f}Precio\u{1f}Prix\u{1f}Prezzo\u{1f}価格\u{1f}\u{1f}Cena\u{1f}Preço\u{1f}Цена\u{1f}ราคา\u{1f}Fiyat\u{1f}价格\u{1f}價格",
            "Primärer Feuermodus\u{1f}Primary Fire\u{1f}Disparo principal\u{1f}Disparo principal\u{1f}Tir principal\u{1f}Fuoco Primario\u{1f}メイン攻撃\u{1f}\u{1f}Atak podstawowy\u{1f}Disparo Primário\u{1f}Основной режим огня\u{1f}การย\u{e34}งร\u{e39}ปแบบหล\u{e31}ก\u{1f}Birincil Saldırı\u{1f}主要攻击模式\u{1f}主要攻擊",
            "Projektil\u{1f}Projectile\u{1f}Proyectil\u{1f}Proyectil\u{1f}\u{1f}Proiettile\u{1f}投射物\u{1f}\u{1f}Pocisk\u{1f}Projétil\u{1f}Снаряд\u{1f}โพรเจกไทล\u{e4c}\u{1f}Atılır Cisim\u{1f}飞弹\u{1f}拋射物",
            "Projektile\u{1f}Projectiles\u{1f}Proyectiles\u{1f}Proyectiles\u{1f}\u{1f}Proiettili\u{1f}投射物\u{1f}\u{1f}Pociski\u{1f}Projéteis\u{1f}Снаряды\u{1f}โพรเจกไทล\u{e4c}\u{1f}Atılır Cisimler\u{1f}飞弹\u{1f}拋射物",
            "Schützen\u{1f}Protect\u{1f}Proteger\u{1f}Proteger\u{1f}Protéger\u{1f}Proteggi\u{1f}守る\u{1f}\u{1f}Ochroń\u{1f}Proteger\u{1f}Защищать\u{1f}ป\u{e49}องก\u{e31}น\u{1f}Koru\u{1f}保护\u{1f}保護",
            "Geschützt\u{1f}Protected\u{1f}Protegido\u{1f}Protegido\u{1f}Protégé\u{1f}Protetto\u{1f}守った\u{1f}\u{1f}Ochroniono\u{1f}Protegido\u{1f}Защитил\u{1f}ป\u{e49}องก\u{e31}นแล\u{e49}ว\u{1f}Korunan\u{1f}已保护\u{1f}受到保護",
            "Schutz\u{1f}Protecting\u{1f}Protegiendo\u{1f}Protegiendo\u{1f}Protection\u{1f}Protezione in Corso\u{1f}守っている\u{1f}\u{1f}Chronienie\u{1f}Protegendo\u{1f}Защищает\u{1f}กำล\u{e31}งป\u{e49}องก\u{e31}น\u{1f}Koruyan\u{1f}正在保护\u{1f}正在保護",
            "Gereinigt\u{1f}Purified\u{1f}Purificado\u{1f}Purificado\u{1f}Purifié\u{1f}Purificato\u{1f}浄化した\u{1f}\u{1f}Wyczyszczono\u{1f}Purificado\u{1f}Очистил\u{1f}ทำให\u{e49}บร\u{e34}ส\u{e38}ทธ\u{e34}\u{e4c}แล\u{e49}ว\u{1f}Arındırılan\u{1f}已净化\u{1f}受到淨化",
            "Reinigen\u{1f}Purify\u{1f}Purificar\u{1f}Purificar\u{1f}Purifier\u{1f}Purifica\u{1f}浄化\u{1f}\u{1f}Wyczyść\u{1f}Purificar\u{1f}Очистить\u{1f}ทำให\u{e49}บร\u{e34}ส\u{e38}ทธ\u{e34}\u{e4c}\u{1f}Arındır\u{1f}净化\u{1f}淨化",
            "Reinigung\u{1f}Purifying\u{1f}Purificando\u{1f}Purificando\u{1f}Purification\u{1f}Purificazione in Corso\u{1f}浄化中\u{1f}\u{1f}Czyszczenie\u{1f}Purificando\u{1f}Очищает\u{1f}กำล\u{e31}งทำให\u{e49}บร\u{e34}ส\u{e38}ทธ\u{e34}\u{e4c}\u{1f}Temizleniyor\u{1f}正在净化\u{1f}正在淨化",
            "Violett\u{1f}Purple\u{1f}Morado\u{1f}Morado\u{1f}Violet\u{1f}Viola\u{1f}紫\u{1f}\u{1f}Fioletowy\u{1f}Roxo\u{1f}Лиловый\u{1f}ม\u{e48}วง\u{1f}Mor\u{1f}亮紫色\u{1f}紫",
            "Erhöhen\u{1f}Raise\u{1f}Levantar\u{1f}Levantar\u{1f}Augmenter\u{1f}Alza\u{1f}上昇\u{1f}\u{1f}Wznieś\u{1f}Elevar\u{1f}Поднять\u{1f}ยกข\u{e36}\u{e49}น\u{1f}Kaldır\u{1f}复活\u{1f}提高",
            "Erhöht\u{1f}Raised\u{1f}Levantado\u{1f}Levantado\u{1f}Augmenté\u{1f}Alzato\u{1f}上昇した\u{1f}\u{1f}Wzniesiono\u{1f}Elevado\u{1f}Поднял\u{1f}ยกข\u{e36}\u{e49}นแล\u{e49}ว\u{1f}Kaldırılan\u{1f}已复活\u{1f}獲得提高",
            "Erhöhung\u{1f}Raising\u{1f}Levantando\u{1f}Levantando\u{1f}Augmentation\u{1f}Innalzamento in Corso\u{1f}上昇中\u{1f}\u{1f}Wznoszenie\u{1f}Elevando\u{1f}Поднимает\u{1f}กำล\u{e31}งยกข\u{e36}\u{e49}น\u{1f}Kaldıran\u{1f}正在复活\u{1f}正在提高",
            "Rang\u{1f}Rank\u{1f}Rango\u{1f}Categoría\u{1f}Rang\u{1f}Grado\u{1f}ランク\u{1f}\u{1f}Ranga\u{1f}Ranque\u{1f}Ранг\u{1f}อ\u{e31}นด\u{e31}บ\u{1f}Derece\u{1f}排名\u{1f}等級",
            "Rang A\u{1f}Rank A\u{1f}Rango A\u{1f}Categoría A\u{1f}Rang A\u{1f}Grado A\u{1f}ランクA\u{1f}\u{1f}Ranga A\u{1f}Ranque A\u{1f}Ранг A\u{1f}อ\u{e31}นด\u{e31}บ A\u{1f}A Derecesi\u{1f}等级 A\u{1f}等級A",
            "Rang B\u{1f}Rank B\u{1f}Rango B\u{1f}Categoría B\u{1f}Rang B\u{1f}Grado B\u{1f}ランクB\u{1f}\u{1f}Ranga B\u{1f}Ranque B\u{1f}Ранг B\u{1f}อ\u{e31}นด\u{e31}บ B\u{1f}B Derecesi\u{1f}等级 B\u{1f}等級B",
            "Rang C\u{1f}Rank C\u{1f}Rango C\u{1f}Categoría C\u{1f}Rang C\u{1f}Grado C\u{1f}ランクC\u{1f}\u{1f}Ranga C\u{1f}Ranque C\u{1f}Ранг C\u{1f}อ\u{e31}นด\u{e31}บ C\u{1f}C Derecesi\u{1f}等级 C\u{1f}等級C",
            "Rang D\u{1f}Rank D\u{1f}Rango D\u{1f}Categoría D\u{1f}Rang D\u{1f}Grado D\u{1f}ランクD\u{1f}\u{1f}Ranga D\u{1f}Ranque D\u{1f}Ранг D\u{1f}อ\u{e31}นด\u{e31}บ D\u{1f}D Derecesi\u{1f}等级 D\u{1f}等級D",
            "Rang E\u{1f}Rank E\u{1f}Rango E\u{1f}Categoría E\u{1f}Rang E\u{1f}Grado E\u{1f}ランクE\u{1f}\u{1f}Ranga E\u{1f}Ranque E\u{1f}Ранг E\u{1f}อ\u{e31}นด\u{e31}บ E\u{1f}E Derecesi\u{1f}等级 E\u{1f}等級E",
            "Rang F\u{1f}Rank F\u{1f}Rango F\u{1f}Categoría F\u{1f}Rang F\u{1f}Grado F\u{1f}ランクF\u{1f}\u{1f}Ranga F\u{1f}Ranque F\u{1f}Ранг F\u{1f}อ\u{e31}นด\u{e31}บ F\u{1f}F Derecesi\u{1f}等级 F\u{1f}等級F",
            "Rang S\u{1f}Rank S\u{1f}Rango S\u{1f}Categoría S\u{1f}Rang S\u{1f}Grado S\u{1f}ランクS\u{1f}\u{1f}Ranga S\u{1f}Ranque S\u{1f}Ранг S\u{1f}อ\u{e31}นด\u{e31}บ S\u{1f}S Derecesi\u{1f}等级 S\u{1f}等級S",
            "Erreichen\u{1f}Reach\u{1f}Alcanzar\u{1f}Alcanzar\u{1f}Atteindre\u{1f}Raggiungi\u{1f}到達する\u{1f}\u{1f}Sięgnij\u{1f}Chegar\u{1f}Достигнуть\u{1f}ไปถ\u{e36}ง\u{1f}Ulaş\u{1f}前往\u{1f}前往",
            "Erreicht\u{1f}Reached\u{1f}Alcanzado\u{1f}Alcanzado\u{1f}Atteint\u{1f}Raggiunto\u{1f}到達した\u{1f}\u{1f}Sięgnięto\u{1f}Chegou\u{1f}Достиг\u{1f}ไปถ\u{e36}งแล\u{e49}ว\u{1f}Ulaşıldı\u{1f}已抵达\u{1f}抵達",
            "Reichweite\u{1f}Reaching\u{1f}Alcanzando\u{1f}Alcanzando\u{1f}Atteinte\u{1f}Raggiungimento in Corso\u{1f}到達している\u{1f}\u{1f}Sięganie\u{1f}Chegando\u{1f}Достигает\u{1f}กำล\u{e31}งไปถ\u{e36}ง\u{1f}Ulaşılıyor\u{1f}正在前往\u{1f}正在前往",
            "Bereit\u{1f}Ready\u{1f}Listo\u{1f}Listo\u{1f}Prêt(e)\u{1f}Pronto\u{1f}準備完了\u{1f}\u{1f}Przygotuj\u{1f}Pronto\u{1f}Готов\u{1f}พร\u{e49}อม\u{1f}Hazır\u{1f}就绪\u{1f}就緒",
            "Rekord\u{1f}Record\u{1f}Récord\u{1f}Récord\u{1f}\u{1f}\u{1f}記録\u{1f}\u{1f}Rekord\u{1f}Recorde\u{1f}Запись\u{1f}บ\u{e31}นท\u{e36}ก\u{1f}Rekor\u{1f}记录\u{1f}紀錄",
            "Rekorde\u{1f}Records\u{1f}Récords\u{1f}Récords\u{1f}\u{1f}Più Record\u{1f}記録\u{1f}\u{1f}Rekordy\u{1f}Recordes\u{1f}Записи\u{1f}บ\u{e31}นท\u{e36}ก\u{1f}Rekor\u{1f}记录\u{1f}紀錄",
            "Wiederherstellen\u{1f}Recover\u{1f}Recuperar\u{1f}Recuperar\u{1f}Récupérer\u{1f}Recupera\u{1f}復元\u{1f}\u{1f}Zregeneruj\u{1f}Recuperar\u{1f}Восстановить\u{1f}ฟ\u{e37}\u{e49}นฟ\u{e39}\u{1f}İyileşme\u{1f}恢复\u{1f}恢復",
            "Wiederhergestellt\u{1f}Recovered\u{1f}Recuperado\u{1f}Recuperado\u{1f}Récupéré\u{1f}Recuperato\u{1f}復元した\u{1f}\u{1f}Zregenerowano\u{1f}Recuperado\u{1f}Восстановил\u{1f}ฟ\u{e37}\u{e49}นฟ\u{e39}แล\u{e49}ว\u{1f}İyileşen\u{1f}已恢复\u{1f}獲得恢復",
            "Wiederherstellung\u{1f}Recovering\u{1f}Recuperando\u{1f}Recuperándose\u{1f}Récupération\u{1f}Recupero in Corso\u{1f}復元中\u{1f}\u{1f}Regeneracja\u{1f}Recuperando\u{1f}Восстанавливает\u{1f}กำล\u{e31}งฟ\u{e37}\u{e49}นฟ\u{e39}\u{1f}İyileştiren\u{1f}正在恢复\u{1f}正在恢復",
            "Rot\u{1f}Red\u{1f}Rojo\u{1f}Rojo\u{1f}Rouge\u{1f}Rosso\u{1f}赤\u{1f}빨간색\u{1f}Czerwony\u{1f}Vermelho\u{1f}Красный\u{1f}แดง\u{1f}Kırmızı\u{1f}红色\u{1f}紅",
            "Bleiben\u{1f}Remain\u{1f}Quedar\u{1f}Restar\u{1f}Rester\u{1f}Restante\u{1f}残る\u{1f}\u{1f}Pozostań\u{1f}Restam\u{1f}Остаться\u{1f}เหล\u{e37}อ\u{1f}Kal\u{1f}剩余\u{1f}剩餘",
            "Verbleibend\u{1f}Remaining\u{1f}Restante\u{1f}Restando\u{1f}Restant\u{1f}Restanti\u{1f}残っている\u{1f}\u{1f}Pozostawanie\u{1f}Restantes\u{1f}Осталось\u{1f}ท\u{e35}\u{e48}เหล\u{e37}ออย\u{e39}\u{e48}\u{1f}Kalan\u{1f}剩余\u{1f}剩餘",
            "Retten\u{1f}Rescue\u{1f}Rescatar\u{1f}Rescatar\u{1f}Secourir\u{1f}Salva\u{1f}レスキュー\u{1f}\u{1f}Ratuj\u{1f}Resgatar\u{1f}Спасать\u{1f}ช\u{e48}วยเหล\u{e37}อ\u{1f}Kurtar\u{1f}营救\u{1f}救援",
            "Gerettet\u{1f}Rescued\u{1f}Rescatado\u{1f}Rescatado\u{1f}Secouru\u{1f}Salvato\u{1f}レスキューした\u{1f}\u{1f}Uratowano\u{1f}Resgatado\u{1f}Спасен\u{1f}ช\u{e48}วยเหล\u{e37}อแล\u{e49}ว\u{1f}Kurtarılan\u{1f}已营救 \u{1f}獲得救援",
            "Rettung\u{1f}Rescuing\u{1f}Rescatando\u{1f}Rescatando\u{1f}Sauvetage\u{1f}Salvataggio in Corso\u{1f}レスキュー中\u{1f}\u{1f}Ratowanie\u{1f}Resgatando\u{1f}Спасает\u{1f}กำล\u{e31}งช\u{e48}วยเหล\u{e37}อ\u{1f}Kurtaran\u{1f}正在营救\u{1f}正在救援",
            "Ressource\u{1f}Resource\u{1f}Recurso\u{1f}Recurso\u{1f}Ressource\u{1f}Risorsa\u{1f}リソース\u{1f}\u{1f}Zasób systemowy\u{1f}Recurso\u{1f}Ресурс\u{1f}ทร\u{e31}พยากร\u{1f}Kaynak\u{1f}资源\u{1f}資源",
            "Ressourcen\u{1f}Resources\u{1f}Recursos\u{1f}Recursos\u{1f}Ressources \u{1f}Risorse\u{1f}リソース\u{1f}\u{1f}Zasoby systemowe\u{1f}Recursos\u{1f}Ресурсы\u{1f}ทร\u{e31}พยากร\u{1f}Kaynaklar\u{1f}资源\u{1f}資源",
            "Wiederbeleben\u{1f}Resurrect\u{1f}Resurrección\u{1f}Resucitar\u{1f}Ressusciter\u{1f}Resuscita\u{1f}蘇生\u{1f}\u{1f}Wskrześ\u{1f}Ressuscitar\u{1f}Воскресить\u{1f}ช\u{e38}บช\u{e35}ว\u{e34}ต\u{1f}Diriliş\u{1f}复活\u{1f}復活",
            "Wiederbelebt\u{1f}Resurrected\u{1f}Resucitado\u{1f}Resucitado\u{1f}Ressuscité\u{1f}Resuscitato\u{1f}蘇生した\u{1f}\u{1f}Wskrzeszono\u{1f}Ressuscitado\u{1f}Воскресил\u{1f}ช\u{e38}บช\u{e35}ว\u{e34}ตแล\u{e49}ว\u{1f}Canlandırılan\u{1f}已复活\u{1f}獲得復活",
            "Wiederbelebung\u{1f}Resurrecting\u{1f}Resucitando\u{1f}Resucitando\u{1f}Résurrection\u{1f}Resurrezione in Corso\u{1f}蘇生中\u{1f}\u{1f}Wskrzeszanie\u{1f}Ressuscitando\u{1f}Воскрешает\u{1f}กำล\u{e31}งช\u{e38}บช\u{e35}ว\u{e34}ต\u{1f}Dirilten\u{1f}正在复活\u{1f}正在復活",
            "Enttarnen\u{1f}Reveal\u{1f}Revelar\u{1f}Revelar\u{1f}Révéler\u{1f}Rivela\u{1f}開示\u{1f}\u{1f}Ujawnij\u{1f}Revelar\u{1f}Обнаруживать\u{1f}เป\u{e34}ดเผย\u{1f}Gösterim\u{1f}揭示\u{1f}現身",
            "Enttarnt\u{1f}Revealed\u{1f}Revelado\u{1f}Revelado\u{1f}Révélé\u{1f}Rivelato\u{1f}表示した\u{1f}\u{1f}Ujawniono\u{1f}Revelado\u{1f}Обнаружил\u{1f}เป\u{e34}ดเผยแล\u{e49}ว\u{1f}Gösterilen\u{1f}已揭示\u{1f}已現身",
            "Enttarnung\u{1f}Revealing\u{1f}Revelando\u{1f}Revelando\u{1f}Révélation\u{1f}Rivelazione in Corso\u{1f}表示中\u{1f}\u{1f}Ujawnianie\u{1f}Revelando\u{1f}Обнаруживает\u{1f}กำล\u{e31}งเป\u{e34}ดเผย\u{1f}Gösteren\u{1f}正在揭示\u{1f}正在現身",
            "Umkehren\u{1f}Reverse\u{1f}Invertir\u{1f}Invertir\u{1f}Inverser\u{1f}Inverti\u{1f}反転する\u{1f}\u{1f}Odwróć\u{1f}Inverter\u{1f}Перевернуть\u{1f}กล\u{e31}บด\u{e49}าน\u{1f}Ters Çevir\u{1f}撤销\u{1f}反轉",
            "Umgekehrt\u{1f}Reversed\u{1f}Invertido\u{1f}Invertido\u{1f}Inversé\u{1f}Invertito\u{1f}反転した\u{1f}\u{1f}Odwrócono\u{1f}Invertido\u{1f}Перевернул\u{1f}กล\u{e31}บด\u{e49}านแล\u{e49}ว\u{1f}Ters Çevrilen\u{1f}已撤销\u{1f}反轉",
            "Umdrehen\u{1f}Reversing\u{1f}Invirtiendo\u{1f}Invirtiendo\u{1f}Inverse\u{1f}Inversione in Corso\u{1f}反転している\u{1f}\u{1f}Odwracanie\u{1f}Invertendo\u{1f}Переворачивает\u{1f}กำล\u{e31}งกล\u{e31}บด\u{e49}าน\u{1f}Ters Çeviren\u{1f}正在撤销\u{1f}反轉",
            "Rechts\u{1f}Right\u{1f}Derecha\u{1f}Derecha\u{1f}Droite\u{1f}Destra\u{1f}右\u{1f}\u{1f}Prawo\u{1f}Direita\u{1f}Вправо\u{1f}ขวา\u{1f}Sağ\u{1f}右\u{1f}右",
            "Runde\u{1f}Round\u{1f}Ronda\u{1f}Ronda\u{1f}Manche\u{1f}\u{1f}ラウンド\u{1f}\u{1f}Runda\u{1f}Rodada\u{1f}Раунд\u{1f}รอบ\u{1f}Tur\u{1f}回合\u{1f}回合",
            "Runde {0}\u{1f}Round {0}\u{1f}Ronda {0}\u{1f}Ronda {0}\u{1f}Manche {0}\u{1f}\u{1f}ラウンド{0}\u{1f}\u{1f}Runda {0}\u{1f}Rodada {0}\u{1f}{0}-й раунд\u{1f}รอบท\u{e35}\u{e48} {0}\u{1f}{0}. Tur\u{1f}第{0}回合\u{1f}第{0}回合",
            "Runden\u{1f}Rounds\u{1f}Rondas\u{1f}Rondas\u{1f}Manches\u{1f}Round\u{1f}ラウンド\u{1f}\u{1f}Rundy\u{1f}Rodadas\u{1f}Раунды\u{1f}รอบ\u{1f}Turlar\u{1f}回合\u{1f}回合",
            "Runden verloren\u{1f}Rounds Lost\u{1f}Rondas perdidas\u{1f}Rondas perdidas\u{1f}Manches perdues\u{1f}Round Persi\u{1f}ラウンド敗北\u{1f}\u{1f}Przegrane rundy\u{1f}Rodadas Perdidas\u{1f}Проиграно раундов\u{1f}รอบท\u{e35}\u{e48}แพ\u{e49}\u{1f}Kaybedilen Turlar\u{1f}战败回合数\u{1f}落敗回合",
            "Runden gewonnen\u{1f}Rounds Won\u{1f}Rondas ganadas\u{1f}Rondas ganadas\u{1f}Manches gagnées\u{1f}Round Vinti\u{1f}ラウンド勝利\u{1f}\u{1f}Wygrane rundy\u{1f}Rodadas Vencidas\u{1f}Выиграно раундов\u{1f}รอบท\u{e35}\u{e48}ชนะ\u{1f}Kazanılan Turlar\u{1f}胜利回合数\u{1f}獲勝回合",
            "Rennen\u{1f}Run\u{1f}Correr\u{1f}Correr\u{1f}Courir\u{1f}Corri\u{1f}走行\u{1f}\u{1f}Biegnij\u{1f}Correr\u{1f}Бежать\u{1f}ว\u{e34}\u{e48}ง\u{1f}Koş\u{1f}运行\u{1f}快跑",
            "Renne\u{1f}Running\u{1f}Corriendo\u{1f}Corriendo\u{1f}Course\u{1f}Corsa in Corso\u{1f}走行中\u{1f}\u{1f}Biegnie\u{1f}Correndo\u{1f}Бежит\u{1f}กำล\u{e31}งว\u{e34}\u{e48}ง\u{1f}Koşan\u{1f}正在运行\u{1f}正在跑",
            "Sicher\u{1f}Safe\u{1f}A salvo\u{1f}Seguro\u{1f}Sans danger\u{1f}Sicuro\u{1f}安全\u{1f}\u{1f}Bezpieczne\u{1f}Seguro\u{1f}В безопасности\u{1f}ปลอดภ\u{e31}ย\u{1f}Güvenli\u{1f}安全\u{1f}安全",
            "Speichern\u{1f}Save\u{1f}Guardar\u{1f}Guardar\u{1f}Sauver\u{1f}Salva\u{1f}保存\u{1f}\u{1f}Zapisz\u{1f}Salvar\u{1f}Сохранить\u{1f}บ\u{e31}นท\u{e36}ก\u{1f}Kaydet\u{1f}保存\u{1f}儲存",
            "Gespeichert\u{1f}Saved\u{1f}Guardado\u{1f}Guardado\u{1f}Sauvé\u{1f}Salvato\u{1f}保存した\u{1f}\u{1f}Zapisano\u{1f}Salvo\u{1f}Сохранил\u{1f}บ\u{e31}นท\u{e36}กแล\u{e49}ว\u{1f}Kayıtlı\u{1f}已保存\u{1f}已儲存",
            "Speichern\u{1f}Saving\u{1f}Guardando\u{1f}Guardando\u{1f}Sauvegarde\u{1f}Salvataggio in Corso\u{1f}保存中\u{1f}\u{1f}Zapisywanie\u{1f}Salvando\u{1f}Сохраняет\u{1f}กำล\u{e31}งบ\u{e31}นท\u{e36}ก\u{1f}Kaydediliyor\u{1f}正在保存\u{1f}正在儲存",
            "Punktestand\u{1f}Score\u{1f}Puntuación\u{1f}Puntuación\u{1f}Points\u{1f}Punteggio\u{1f}スコア\u{1f}\u{1f}Wynik\u{1f}Pontuação\u{1f}Счет\u{1f}คะแนน\u{1f}Skor\u{1f}分数\u{1f}分數",
            "Punktestände\u{1f}Scores\u{1f}Puntuaciones\u{1f}Puntuaciones\u{1f}Points\u{1f}Punteggi\u{1f}スコア\u{1f}\u{1f}Wyniki\u{1f}Pontuações\u{1f}Счет\u{1f}คะแนน\u{1f}Skor\u{1f}分数\u{1f}分數",
            "Sekundärer Feuermodus\u{1f}Secondary Fire\u{1f}Disparo secundario\u{1f}Disparo secundario\u{1f}Tir secondaire\u{1f}Fuoco Secondario\u{1f}サブ攻撃\u{1f}\u{1f}Atak alternatywny\u{1f}Disparo Secundário\u{1f}Дополнительный режим огня\u{1f}การย\u{e34}งร\u{e39}ปแบบรอง\u{1f}İkincil Saldırı\u{1f}辅助攻击模式\u{1f}次要攻擊",
            "Sichern\u{1f}Secure\u{1f}Tomar\u{1f}Asegurar\u{1f}Sécuriser\u{1f}Metti al Sicuro\u{1f}確保\u{1f}\u{1f}Zabezpiecz\u{1f}Tomar\u{1f}Брать под контроль\u{1f}ร\u{e31}กษาความปลอดภ\u{e31}ย\u{1f}Emniyete Al\u{1f}保护\u{1f}佔領",
            "Gesichert\u{1f}Secured\u{1f}Tomado\u{1f}Asegurado\u{1f}Sécurisé\u{1f}Messo al Sicuro\u{1f}確保した\u{1f}\u{1f}Zabezpieczono\u{1f}Tomado\u{1f}Взял под контроль\u{1f}ร\u{e31}กษาความปลอดภ\u{e31}ยแล\u{e49}ว\u{1f}Emniyete Alınan\u{1f}已保护\u{1f}遭到佔領",
            "Sicherung\u{1f}Securing\u{1f}Tomando\u{1f}Asegurando\u{1f}Sécurisation\u{1f}Messa in Sicurezza in Corso\u{1f}確保中\u{1f}\u{1f}Zabezpieczanie\u{1f}Tomando\u{1f}Берет под контроль\u{1f}กำล\u{e31}งร\u{e31}กษาความปลอดภ\u{e31}ย\u{1f}Emniyete Alan\u{1f}正在保护\u{1f}正在佔領",
            "Auswählen\u{1f}Select\u{1f}Seleccionar\u{1f}Seleccionar\u{1f}Sélectionner\u{1f}Seleziona\u{1f}選択\u{1f}\u{1f}Wybierz\u{1f}Selecionar\u{1f}Выбрать\u{1f}เล\u{e37}อก\u{1f}Seç\u{1f}选择\u{1f}選擇",
            "Ausgewählt\u{1f}Selected\u{1f}Seleccionado\u{1f}Seleccionado\u{1f}Sélectionné\u{1f}Selezionato\u{1f}選択された\u{1f}\u{1f}Wybrano\u{1f}Seleção\u{1f}Выбрал\u{1f}เล\u{e37}อกแล\u{e49}ว\u{1f}Seçilen\u{1f}已选择\u{1f}選取",
            "Auswahl\u{1f}Selecting\u{1f}Seleccionando\u{1f}Seleccionando\u{1f}Sélectionne\u{1f}Selezionamento in Corso\u{1f}選択している\u{1f}\u{1f}Wybieranie\u{1f}Selecionando\u{1f}Выбирает\u{1f}กำล\u{e31}งเล\u{e37}อก\u{1f}Seçen\u{1f}正在选择\u{1f}選取",
            "Verkaufen\u{1f}Sell\u{1f}Vender\u{1f}Vender\u{1f}Vendre\u{1f}Vendi\u{1f}売却\u{1f}\u{1f}Sprzedaj\u{1f}Vender\u{1f}Продать\u{1f}ขาย\u{1f}Sat\u{1f}出售\u{1f}出售",
            "Verkauf\u{1f}Selling\u{1f}Vendiendo\u{1f}Vendiendo\u{1f}Vente\u{1f}Vendita in Corso\u{1f}売却中\u{1f}\u{1f}Sprzedawanie\u{1f}Vendendo\u{1f}Продает\u{1f}กำล\u{e31}งขาย\u{1f}Satan\u{1f}正在出售\u{1f}正在出售",
            "Serverauslastung\u{1f}Server Load\u{1f}Carga del servidor\u{1f}Uso del servidor\u{1f}Charge du serveur\u{1f}Carico del server\u{1f}サーバー負荷\u{1f}\u{1f}Obciążenie serwera\u{1f}Uso do Servidor\u{1f}Загрузка сервера\u{1f}เซ\u{e34}ร\u{e4c}ฟเวอร\u{e4c}โหลด\u{1f}Sunucu Doluluğu\u{1f}服务器负载\u{1f}伺服器負載",
            "Durchschnittliche Serverauslastung\u{1f}Server Load Average\u{1f}Carga media del servidor\u{1f}Uso promedio del servidor\u{1f}Charge moyenne du serveur\u{1f}Carico medio del server\u{1f}サーバー負荷平均\u{1f}\u{1f}Średnie obciążenie serwera\u{1f}Média de Uso do Servidor\u{1f}Средняя загрузка сервера\u{1f}เซ\u{e34}ร\u{e4c}ฟเวอร\u{e4c}โหลดเฉล\u{e35}\u{e48}ย\u{1f}Sunucu Doluluk Ortalaması\u{1f}服务器负载平均值\u{1f}伺服器平均負載",
            "Maximale Serverauslastung\u{1f}Server Load Peak\u{1f}Carga máxima del servidor\u{1f}Uso máximo del servidor\u{1f}Pic de charge du serveur\u{1f}Carico massimo del server\u{1f}サーバー負荷ピーク\u{1f}\u{1f}Szczytowe obciążenie serwera\u{1f}Pico de Uso do Servidor\u{1f}Пиковая загрузка сервера\u{1f}เซ\u{e34}ร\u{e4c}ฟเวอร\u{e4c}โหลดส\u{e39}งส\u{e38}ด\u{1f}Sunucu Doluluk En Üst Noktası\u{1f}服务器负载峰值\u{1f}伺服器最高負載",
            "Trennen\u{1f}Sever\u{1f}Cortar\u{1f}Cortar\u{1f}Couper\u{1f}Taglia\u{1f}切断\u{1f}\u{1f}Oddziel\u{1f}Separar\u{1f}Отсечь\u{1f}ต\u{e31}ด\u{1f}Ayır\u{1f}撕裂\u{1f}截斷",
            "Schlimm\u{1f}Severe\u{1f}Fuerte\u{1f}Grave\u{1f}Grave\u{1f}Grave\u{1f}難しい\u{1f}\u{1f}Poważnie\u{1f}Severo\u{1f}Жесткий\u{1f}ร\u{e38}นแรง\u{1f}Ciddi\u{1f}严重\u{1f}嚴重",
            "Getrennt\u{1f}Severed\u{1f}Cortado\u{1f}Cortado\u{1f}Coupé\u{1f}Tagliato\u{1f}切断した\u{1f}\u{1f}Oddzielono\u{1f}Separado\u{1f}Отсек\u{1f}ร\u{e38}นแรง\u{1f}Ayrılan\u{1f}已撕裂\u{1f}受到截斷",
            "Trennung\u{1f}Severing\u{1f}Cortando\u{1f}Cortando\u{1f}Coupure\u{1f}Taglio in Corso\u{1f}切断中\u{1f}\u{1f}Oddzielanie\u{1f}Separando\u{1f}Отсекает\u{1f}กำล\u{e31}งต\u{e31}ด\u{1f}Ayıran\u{1f}正在撕裂\u{1f}正在截斷",
            "\u{1f}Shop\u{1f}Tienda\u{1f}Tienda\u{1f}Boutique\u{1f}Negozio\u{1f}ショップ\u{1f}상점\u{1f}Sklep\u{1f}Loja\u{1f}Магазин\u{1f}ร\u{e49}านค\u{e49}า\u{1f}Mağaza\u{1f}商店\u{1f}商店",
            "\u{1f}Shops\u{1f}Tiendas\u{1f}Tiendas\u{1f}Boutiques\u{1f}Negozi\u{1f}ショップ\u{1f}\u{1f}Sklepy\u{1f}Lojas\u{1f}Магазины\u{1f}ร\u{e49}านค\u{e49}า\u{1f}Mağazalar\u{1f}商店\u{1f}商店",
            "Mischen\u{1f}Shuffle\u{1f}Barajar\u{1f}Mezclar\u{1f}Mélanger\u{1f}Mescola\u{1f}シャッフルする\u{1f}\u{1f}Przetasuj\u{1f}Embaralhar\u{1f}Замешать\u{1f}ส\u{e31}บเปล\u{e35}\u{e48}ยน\u{1f}Karıştır\u{1f}洗牌\u{1f}洗牌",
            "Gemischt\u{1f}Shuffled\u{1f}Barajado\u{1f}Mezclado\u{1f}Mélangé\u{1f}Mescolato\u{1f}シャッフルされた\u{1f}\u{1f}Przetasowano\u{1f}Embaralhado\u{1f}Замешал\u{1f}ส\u{e31}บเปล\u{e35}\u{e48}ยนแล\u{e49}ว\u{1f}Karıştırılan\u{1f}已洗牌\u{1f}洗牌",
            "Versenken\u{1f}Sink\u{1f}Hundir\u{1f}Hundir\u{1f}Noyer\u{1f}Affonda\u{1f}下降\u{1f}\u{1f}Zatop\u{1f}Afundar\u{1f}Тонуть\u{1f}จม\u{1f}Batır\u{1f}击沉\u{1f}沉沒",
            "Versenkung\u{1f}Sinking\u{1f}Hundiendo\u{1f}Hundiéndose\u{1f}Noyade\u{1f}Affondamento in Corso\u{1f}下降中\u{1f}\u{1f}Zatapianie\u{1f}Afundando\u{1f}Тонет\u{1f}กำล\u{e31}งจม\u{1f}Batıran\u{1f}正在击沉\u{1f}正在沉沒",
            "Überspringen\u{1f}Skip\u{1f}Saltar\u{1f}Omitir\u{1f}Passer\u{1f}Salta\u{1f}スキップする\u{1f}\u{1f}Pomiń\u{1f}Passar\u{1f}Пропустить\u{1f}ข\u{e49}าม\u{1f}Atla\u{1f}跳过\u{1f}跳過",
            "Übersprungen\u{1f}Skipped\u{1f}Saltado\u{1f}Omitido\u{1f}Passé\u{1f}Saltato\u{1f}スキップされた\u{1f}\u{1f}Pominięto\u{1f}Passou\u{1f}Пропустил\u{1f}ข\u{e49}ามแล\u{e49}ว\u{1f}Atlanan\u{1f}已跳过\u{1f}跳過",
            "Auslassen\u{1f}Skipping\u{1f}Saltando\u{1f}Omitiendo\u{1f}Passe\u{1f}Salto in Corso\u{1f}スキップしている\u{1f}\u{1f}Pomijanie\u{1f}Passando\u{1f}Пропускает\u{1f}กำล\u{e31}งข\u{e49}าม\u{1f}Atlayan\u{1f}正在跳过\u{1f}跳過",
            "Einschläfern\u{1f}Sleep\u{1f}Dormir\u{1f}Dormir\u{1f}Sommeil\u{1f}Addormenta\u{1f}眠る\u{1f}\u{1f}Śpij\u{1f}Dormir\u{1f}Спать\u{1f}หล\u{e31}บ\u{1f}Uyku\u{1f}沉睡\u{1f}睡眠",
            "Schlaf\u{1f}Sleeping\u{1f}Durmiendo\u{1f}Durmiendo\u{1f}Dort\u{1f}Dormita in Corso\u{1f}眠っている\u{1f}\u{1f}Spanie\u{1f}Dormindo\u{1f}Спит\u{1f}กำล\u{e31}งหล\u{e31}บ\u{1f}Uyuyor\u{1f}正在沉睡\u{1f}正在睡眠",
            "Eingeschläfert\u{1f}Slept\u{1f}Dormido\u{1f}Dormido\u{1f}Dormi\u{1f}Addormentato\u{1f}眠った\u{1f}\u{1f}Spano\u{1f}Adormecido\u{1f}Усыплен\u{1f}หล\u{e31}บแล\u{e49}ว\u{1f}Uyutulan\u{1f}已沉睡\u{1f}已睡著",
            "Langsam\u{1f}Slow\u{1f}Lento\u{1f}Lento\u{1f}Lent\u{1f}Lento\u{1f}遅い\u{1f}\u{1f}Wolno\u{1f}Lento\u{1f}Медленно\u{1f}ช\u{e49}า\u{1f}Yavaş\u{1f}慢\u{1f}慢",
            "Langsamer\u{1f}Slower\u{1f}Más lento\u{1f}Más lento\u{1f}Plus lent\u{1f}Più Lento\u{1f}より遅い\u{1f}\u{1f}Wolniej\u{1f}Mais Lento\u{1f}Медленнее\u{1f}ช\u{e49}ากว\u{e48}า\u{1f}Daha Yavaş\u{1f}较慢\u{1f}較慢",
            "Am langsamsten\u{1f}Slowest\u{1f}El más lento\u{1f}Mucho más lento\u{1f}Le plus lent\u{1f}Lentissimo\u{1f}最遅\u{1f}\u{1f}Najwolniej\u{1f}O Mais Lento\u{1f}Очень медленно\u{1f}ช\u{e49}าท\u{e35}\u{e48}ส\u{e38}ด\u{1f}En Yavaş\u{1f}最慢\u{1f}最慢",
            "Verkauft\u{1f}Sold\u{1f}Vendido\u{1f}Vendido\u{1f}Vendu\u{1f}Venduto\u{1f}売却した\u{1f}\u{1f}Sprzedano\u{1f}Vendido\u{1f}Продал\u{1f}ขายแล\u{e49}ว\u{1f}Satılan\u{1f}已出售\u{1f}遭到出售",
            "Tut mir leid\u{1f}Sorry\u{1f}Lo siento\u{1f}Lo siento\u{1f}Désolé\u{1f}Mi Spiace\u{1f}謝ればいいのかしら\u{1f}\u{1f}Wybacz\u{1f}Desculpe\u{1f}Извини\u{1f}ขอโทษ\u{1f}Kusura bakma\u{1f}抱歉\u{1f}抱歉",
            "Süden\u{1f}South\u{1f}Sur\u{1f}Sur\u{1f}Sud\u{1f}Sud\u{1f}南\u{1f}\u{1f}Południe\u{1f}Sul\u{1f}Юг\u{1f}ใต\u{e49}\u{1f}Güney\u{1f}南\u{1f}南",
            "Südosten\u{1f}Southeast\u{1f}Sureste\u{1f}Sudeste\u{1f}Sud-est\u{1f}Sud-est\u{1f}南東\u{1f}\u{1f}Południowy wschód\u{1f}Sudeste\u{1f}Юго-восток\u{1f}ตะว\u{e31}นออกเฉ\u{e35}ยงใต\u{e49}\u{1f}Güneydoğu\u{1f}东南\u{1f}東南",
            "Südwesten\u{1f}Southwest\u{1f}Suroeste\u{1f}Sudoeste\u{1f}Sud-ouest\u{1f}Sud-ovest\u{1f}南西\u{1f}\u{1f}Południowy zachód\u{1f}Sudoeste\u{1f}Юго-запад\u{1f}ตะว\u{e31}นตกเฉ\u{e35}ยงใต\u{e49}\u{1f}Güneybatı\u{1f}西南\u{1f}西南",
            "Pik\u{1f}Spade\u{1f}Pica\u{1f}Pica\u{1f}Pique\u{1f}Picca\u{1f}スペード\u{1f}\u{1f}Pik\u{1f}Espadas\u{1f}Пики\u{1f}โพดำ\u{1f}Maça\u{1f}黑桃\u{1f}黑桃",
            "Pik\u{1f}Spades\u{1f}Picas\u{1f}Picas\u{1f}Pique\u{1f}Picche\u{1f}スペード\u{1f}\u{1f}Pik\u{1f}Espadas\u{1f}Пики\u{1f}โพดำ\u{1f}Maça\u{1f}黑桃\u{1f}黑桃",
            "Glitzer\u{1f}Sparkles\u{1f}Destellos\u{1f}Chispas\u{1f}Étincelles\u{1f}Scintille\u{1f}スパークル\u{1f}\u{1f}Iskry\u{1f}Faíscas\u{1f}Искры\u{1f}ประกายไฟ\u{1f}Pırıltı\u{1f}火花\u{1f}火花",
            "Spawnen\u{1f}Spawn\u{1f}Engendrar\u{1f}Aparecer\u{1f}Apparaître\u{1f}Genera\u{1f}スポーン\u{1f}\u{1f}Odrodź\u{1f}Surgir\u{1f}Порождать\u{1f}เก\u{e34}ด\u{1f}Doğ\u{1f}生成\u{1f}重生",
            "Gespawnt\u{1f}Spawned\u{1f}Engendrado\u{1f}Aparecido\u{1f}Apparu\u{1f}Generato\u{1f}スポーンした\u{1f}\u{1f}Odrodzono\u{1f}Surgido\u{1f}Породил\u{1f}เก\u{e34}ดแล\u{e49}ว\u{1f}Doğan\u{1f}已生成\u{1f}得到重生",
            "Spawn\u{1f}Spawning\u{1f}Engendrando\u{1f}Apareciendo\u{1f}Apparition\u{1f}Generazione in Corso\u{1f}スポーン中\u{1f}\u{1f}Odradzanie\u{1f}Surgindo\u{1f}Порождает\u{1f}กำล\u{e31}งเก\u{e34}ด\u{1f}Doğan\u{1f}正在生成\u{1f}正在重生",
            "Geschwindigkeit\u{1f}Speed\u{1f}Velocidad\u{1f}Velocidad\u{1f}Vitesse\u{1f}Velocità Non Direzionale\u{1f}速さ\u{1f}\u{1f}Szybkość\u{1f}Velocidade\u{1f}Скорость\u{1f}ความเร\u{e47}ว\u{1f}Hız\u{1f}速度\u{1f}速度",
            "Tempo\u{1f}Speeds\u{1f}Velocidades\u{1f}Velocidades\u{1f}Vitesses\u{1f}Velocità Non Direzionali\u{1f}速さ\u{1f}\u{1f}Szybkości\u{1f}Velocidades\u{1f}Скорости\u{1f}ความเร\u{e47}ว\u{1f}Hız\u{1f}速度\u{1f}速度",
            "Sphäre\u{1f}Sphere\u{1f}Esfera\u{1f}Esfera\u{1f}Sphère\u{1f}Sfera\u{1f}球体\u{1f}\u{1f}Sfera\u{1f}Esfera\u{1f}Сфера\u{1f}ทรงกลม\u{1f}Küre\u{1f}球体\u{1f}球體",
            "Sphären\u{1f}Spheres\u{1f}Esferas\u{1f}Esferas\u{1f}Sphères\u{1f}Sfere\u{1f}球体\u{1f}\u{1f}Sfery\u{1f}Esferas\u{1f}Сферы\u{1f}ทรงกลม\u{1f}Küre\u{1f}球形\u{1f}球體",
            "Stabilisieren\u{1f}Stabilize\u{1f}Estabilizar\u{1f}Estabilizar\u{1f}Stabiliser\u{1f}Stabilizza\u{1f}安定化\u{1f}\u{1f}Ustabilizuj\u{1f}Estabilizar\u{1f}Стабилизировать\u{1f}ทำให\u{e49}เสถ\u{e35}ยร\u{1f}Dengeleme\u{1f}稳定\u{1f}穩定",
            "Stabilisiert\u{1f}Stabilized\u{1f}Estabilizado\u{1f}Estabilizado\u{1f}Stabilisé\u{1f}Stabilizzato\u{1f}安定化した\u{1f}\u{1f}Ustabilizowano\u{1f}Estabilizado\u{1f}Стабилизировал\u{1f}ทำให\u{e49}เสถ\u{e35}ยรแล\u{e49}ว\u{1f}Dengede\u{1f}已稳定\u{1f}獲得穩定",
            "Stabilisierung\u{1f}Stabilizing\u{1f}Estabilizando\u{1f}Estabilizando\u{1f}Stabilisation\u{1f}Stabilizzazione in Corso\u{1f}安定化中\u{1f}\u{1f}Stabilizowanie\u{1f}Estabilizando\u{1f}Стабилизирует\u{1f}กำล\u{e31}งทำให\u{e49}เสถ\u{e35}ยร\u{1f}Dengeleyen\u{1f}正在稳定\u{1f}正在穩定",
            "Stabil\u{1f}Stable\u{1f}Estable\u{1f}Estable\u{1f}\u{1f}Stabile\u{1f}安定\u{1f}\u{1f}Stabilne\u{1f}Estável\u{1f}Стабильный\u{1f}ม\u{e31}\u{e48}นคง\u{1f}Dengeli\u{1f}存放宠物\u{1f}穩定",
            "Stern\u{1f}Star\u{1f}Estrella\u{1f}Estrella\u{1f}Étoile\u{1f}Stella\u{1f}スター\u{1f}\u{1f}Gwiazdka\u{1f}Estrela\u{1f}Звезда\u{1f}ดาว\u{1f}Yıldız\u{1f}星星\u{1f}星星",
            "Sterne\u{1f}Stars\u{1f}Estrellas\u{1f}Estrellas\u{1f}Étoiles\u{1f}Stelle\u{1f}スター\u{1f}\u{1f}Gwiazdy\u{1f}Estrelas\u{1f}Звезды\u{1f}ดาว\u{1f}Yıldızlar\u{1f}群星\u{1f}星星",
            "Starten\u{1f}Start\u{1f}Iniciar\u{1f}Comenzar\u{1f}Commencer\u{1f}Avvio\u{1f}開始\u{1f}\u{1f}Rozpocznij\u{1f}Iniciar\u{1f}Начинать\u{1f}เร\u{e34}\u{e48}ม\u{1f}Başla\u{1f}开始\u{1f}開始",
            "Gestartet\u{1f}Started\u{1f}Iniciado\u{1f}Comenzado\u{1f}Commencé\u{1f}Avviato\u{1f}開始した\u{1f}\u{1f}Rozpoczęto\u{1f}Iniciado\u{1f}Начал\u{1f}เร\u{e34}\u{e48}มแล\u{e49}ว\u{1f}Başlamış\u{1f}已开始\u{1f}已開始",
            "Start\u{1f}Starting\u{1f}Iniciando\u{1f}Comenzando\u{1f}Commencement\u{1f}Avvio in Corso\u{1f}開始している\u{1f}\u{1f}Rozpoczynanie\u{1f}Iniciando\u{1f}Начинает\u{1f}กำล\u{e31}งเร\u{e34}\u{e48}ม\u{1f}Başlayan\u{1f}正在开始\u{1f}正在開始",
            "\u{1f}Status\u{1f}Estado\u{1f}Estado\u{1f}Statut\u{1f}Stato\u{1f}ステータス\u{1f}\u{1f}\u{1f}\u{1f}Статус\u{1f}สถานะ\u{1f}Durum\u{1f}状态\u{1f}狀態",
            "Bleiben\u{1f}Stay\u{1f}Quedarse\u{1f}Quedarse\u{1f}Suivre\u{1f}Continua\u{1f}ステイする\u{1f}\u{1f}Zostań\u{1f}Permanecer\u{1f}Остаться\u{1f}อย\u{e39}\u{e48}\u{1f}Bırakma\u{1f}停止\u{1f}不再要牌",
            "Bleib weg\u{1f}Stay Away\u{1f}Aléjate\u{1f}Aléjate de mí\u{1f}Éloignez-vous\u{1f}Stai alla Larga\u{1f}離れていろ\u{1f}\u{1f}Trzymaj się z dala\u{1f}Fique Longe\u{1f}Не подходи\u{1f}อย\u{e39}\u{e48}ห\u{e48}างๆ\u{1f}Git Başımdan\u{1f}走开\u{1f}走開",
            "Geblieben\u{1f}Stayed\u{1f}Quedado\u{1f}Se quedó\u{1f}Suivi\u{1f}Continuato\u{1f}ステイされた\u{1f}\u{1f}Pozostano\u{1f}Permaneceu\u{1f}Остался\u{1f}อย\u{e39}\u{e48}แล\u{e49}ว\u{1f}Bırakılan\u{1f}已停止 \u{1f}不再要牌",
            "Verbleiben\u{1f}Staying\u{1f}Quedándose\u{1f}Quedándose\u{1f}Suit\u{1f}Continuazione in Corso\u{1f}ステイしている\u{1f}\u{1f}Zostawanie\u{1f}Permanecendo\u{1f}Остается\u{1f}กำล\u{e31}งอย\u{e39}\u{e48}\u{1f}Bırakan\u{1f}正在停止\u{1f}不再要牌",
            "Stoppen\u{1f}Stop\u{1f}Parar\u{1f}Detener\u{1f}Arrêter\u{1f}Interrompi\u{1f}停止\u{1f}\u{1f}Zatrzymaj\u{1f}Parar\u{1f}Остановить\u{1f}หย\u{e38}ด\u{1f}Dur\u{1f}阻止\u{1f}停止",
            "Gestoppt\u{1f}Stopped\u{1f}Parado\u{1f}Detenido\u{1f}Arrêté\u{1f}Interrotto\u{1f}停止した\u{1f}\u{1f}Zatrzymano\u{1f}Parado\u{1f}Остановил\u{1f}หย\u{e38}ดแล\u{e49}ว\u{1f}Durdurulan\u{1f}已阻止\u{1f}已停止",
            "Stopp\u{1f}Stopping\u{1f}Parando\u{1f}Deteniendo\u{1f}Arrêt\u{1f}Interruzione in Corso\u{1f}停止中\u{1f}\u{1f}Zatrzymywanie\u{1f}Parando\u{1f}Останавливает\u{1f}กำล\u{e31}งหย\u{e38}ด\u{1f}Durduran\u{1f}正在阻止\u{1f}正在停止",
            "Stärke\u{1f}Strength\u{1f}Fuerza\u{1f}Fuerza\u{1f}Force\u{1f}Forza\u{1f}腕力\u{1f}\u{1f}Siła\u{1f}Força\u{1f}Сила\u{1f}ความแข\u{e47}งแกร\u{e48}ง\u{1f}Kuvvet\u{1f}力量\u{1f}力量",
            "Betäubung\u{1f}Stun\u{1f}Aturdir\u{1f}Aturdir\u{1f}Étourdir\u{1f}Stordisce\u{1f}スタンする\u{1f}\u{1f}Ogłusz\u{1f}Atordoar\u{1f}Оглушить\u{1f}สต\u{e31}น\u{1f}Sersemlet\u{1f}击晕\u{1f}擊昏",
            "Betäubt\u{1f}Stunned\u{1f}Aturdido\u{1f}Aturdido\u{1f}Étourdi\u{1f}Stordito\u{1f}スタンされている\u{1f}\u{1f}Ogłuszono\u{1f}Atordoado\u{1f}Оглушил\u{1f}ถ\u{e39}กสต\u{e31}น\u{1f}Sersemletilen\u{1f}已击晕\u{1f}遭到擊昏",
            "Betäuben\u{1f}Stunning\u{1f}Aturdiendo\u{1f}Aturdiendo\u{1f}Étourdissant\u{1f}Stordimento in Corso\u{1f}スタンしている\u{1f}\u{1f}Ogłuszanie\u{1f}Atordoando\u{1f}Оглушает\u{1f}กำล\u{e31}งสต\u{e31}น\u{1f}Sersemleten\u{1f}正在击晕\u{1f}正在擊昏",
            "\u{1f}Suboptimal\u{1f}Subóptimo\u{1f}Casi óptimo\u{1f}Sous-optimal\u{1f}Subottimale\u{1f}準最適\u{1f}\u{1f}Nieoptymalne\u{1f}Abaixo do Ideal\u{1f}Неоптимальный\u{1f}ไม\u{e48}ใช\u{e48}ต\u{e31}วเล\u{e37}อกท\u{e35}\u{e48}ด\u{e35}ส\u{e38}ด\u{1f}Standart Altı\u{1f}次佳\u{1f}次佳",
            "Erfolg\u{1f}Success\u{1f}Éxito\u{1f}Éxito\u{1f}Réussite\u{1f}Successo\u{1f}完了\u{1f}\u{1f}Sukces\u{1f}Sucesso\u{1f}Успех\u{1f}ความสำเร\u{e47}จ\u{1f}Başarı\u{1f}成功\u{1f}成功",
            "K.o.-Runde\u{1f}Sudden Death\u{1f}Muerte súbita\u{1f}Muerte súbita\u{1f}Mort subite\u{1f}Scontro decisivo\u{1f}サドンデス\u{1f}\u{1f}Decydujące starcie\u{1f}Morte Súbita\u{1f}Внезапная смерть\u{1f}รอบต\u{e31}ดส\u{e34}นป\u{e49}องก\u{e31}นการเสมอก\u{e31}น\u{1f}Ani Ölüm\u{1f}绝杀局\u{1f}殊死戰",
            "Versenkt\u{1f}Sunk\u{1f}Hundido\u{1f}Hundido\u{1f}Noyé\u{1f}Affondato\u{1f}下降した\u{1f}\u{1f}Zatopiono\u{1f}Afundado\u{1f}Погрузил\u{1f}จมแล\u{e49}ว\u{1f}Batan\u{1f}已击沉\u{1f}遭到沉沒",
            "Super\u{1f}Superb\u{1f}Soberbio\u{1f}Magnífico\u{1f}Superbe\u{1f}Superbo\u{1f}見事\u{1f}\u{1f}Super\u{1f}Estupendo\u{1f}Превосходно\u{1f}ส\u{e38}ดยอด\u{1f}Müthiş\u{1f}强烈\u{1f}非常",
            "Überleben\u{1f}Survive\u{1f}Sobrevivir\u{1f}Sobrevivir\u{1f}Survivre\u{1f}Sopravvivi\u{1f}生存\u{1f}\u{1f}Przetrwaj\u{1f}Sobreviver\u{1f}Выживать\u{1f}รอด\u{1f}Kurtul\u{1f}生存\u{1f}存活",
            "Überlebt\u{1f}Survived\u{1f}Sobrevivido\u{1f}Sobrevivido\u{1f}Survécu\u{1f}Sopravvissuto\u{1f}生存した\u{1f}\u{1f}Przetrwano\u{1f}Sobreviveu\u{1f}Выжил\u{1f}รอดแล\u{e49}ว\u{1f}Kurtulan\u{1f}已生存\u{1f}生還",
            "Überlebe\u{1f}Surviving\u{1f}Sobreviviendo\u{1f}Sobreviviendo\u{1f}Survie\u{1f}Sopravvivenza in Corso\u{1f}生存中\u{1f}\u{1f}Przetrwanie\u{1f}Sobrevivendo\u{1f}Выживает\u{1f}กำล\u{e31}งเอาต\u{e31}วรอด\u{1f}Kurtulan\u{1f}正在生存\u{1f}存活中",
            "Ziel\u{1f}Target\u{1f}Objetivo\u{1f}Objetivo\u{1f}Cible\u{1f}Bersaglio\u{1f}ターゲット\u{1f}\u{1f}Cel\u{1f}Alvo\u{1f}Цель\u{1f}เป\u{e49}าหมาย\u{1f}Hedef\u{1f}目标\u{1f}目標",
            "Ziele\u{1f}Targets\u{1f}Objetivos\u{1f}Objetivos\u{1f}Cibles\u{1f}Bersagli\u{1f}ターゲット\u{1f}\u{1f}Cele\u{1f}Alvos\u{1f}Цели\u{1f}เป\u{e49}าหมาย\u{1f}Hedefler\u{1f}目标\u{1f}目標",
            "\u{1f}Team\u{1f}Equipo\u{1f}Equipo\u{1f}Équipe\u{1f}Squadra\u{1f}チーム\u{1f}\u{1f}Drużyna\u{1f}Equipe\u{1f}Команда\u{1f}ท\u{e35}ม\u{1f}Takım\u{1f}队伍\u{1f}隊伍",
            "Teammitglied\u{1f}Teammate\u{1f}Compañero de equipo\u{1f}Compañero de equipo\u{1f}Équipier\u{1f}Compagno di squadra\u{1f}チームメイト\u{1f}\u{1f}Członek drużyny\u{1f}Aliado\u{1f}Союзник\u{1f}เพ\u{e37}\u{e48}อนร\u{e48}วมท\u{e35}ม\u{1f}Takım Arkadaşı\u{1f}队友\u{1f}隊友",
            "Teammitglieder\u{1f}Teammates\u{1f}Compañeros de equipo\u{1f}Compañeros de equipo\u{1f}Équipiers\u{1f}Compagni di squadra\u{1f}チームメイト\u{1f}\u{1f}Członkowie drużyny\u{1f}Aliados\u{1f}Союзники\u{1f}เพ\u{e37}\u{e48}อนร\u{e48}วมท\u{e35}ม\u{1f}Takım Arkadaşı\u{1f}队友\u{1f}隊友",
            "\u{1f}Teams\u{1f}Equipos\u{1f}Equipos\u{1f}Équipes\u{1f}Squadre\u{1f}チーム\u{1f}\u{1f}Drużyny\u{1f}Equipes\u{1f}Команды\u{1f}ท\u{e35}ม\u{1f}Takımlar\u{1f}队伍\u{1f}隊伍",
            "Sehr schlecht\u{1f}Terrible\u{1f}\u{1f}Pésimo\u{1f}\u{1f}Terribile\u{1f}最悪\u{1f}\u{1f}Okropnie\u{1f}Terrível\u{1f}Ужасно\u{1f}ไม\u{e48}ด\u{e35}มากๆ\u{1f}Korkunç\u{1f}极差\u{1f}差",
            "Vielen Dank\u{1f}Thank You\u{1f}Te lo agradezco\u{1f}Gracias\u{1f}Merci\u{1f}Ti Ringrazio\u{1f}ありがとう\u{1f}\u{1f}Dziękuję\u{1f}Obrigado\u{1f}Благодарю\u{1f}ขอบค\u{e38}ณ\u{1f}Teşekkürler\u{1f}衷心感谢\u{1f}謝謝你",
            "Danke\u{1f}Thanks\u{1f}Gracias\u{1f}Gracias\u{1f}Remerciements\u{1f}Grazie\u{1f}ありがとう\u{1f}\u{1f}Dzięki\u{1f}Valeu\u{1f}Спасибо\u{1f}ขอบค\u{e38}ณ\u{1f}Sağ Ol\u{1f}感谢\u{1f}謝謝",
            "Das war der Hammer\u{1f}That Was Awesome\u{1f}Ha sido genial\u{1f}Eso estuvo increíble\u{1f}C’était génial\u{1f}È Stato Incredibile\u{1f}百点満点！\u{1f}\u{1f}To było niesamowite\u{1f}Essa Foi Demais\u{1f}Это было потрясно\u{1f}น\u{e31}\u{e48}นม\u{e31}นเย\u{e35}\u{e48}ยมไปเลย\u{1f}Muhteşemdi\u{1f}太棒了\u{1f}真厲害",
            "Bedrohung\u{1f}Threat\u{1f}Amenaza\u{1f}Amenaza\u{1f}Menace\u{1f}Minaccia\u{1f}脅威\u{1f}\u{1f}Zagrożenie\u{1f}Ameaça\u{1f}Угроза\u{1f}ภ\u{e31}ยค\u{e38}กคาม\u{1f}Tehdit\u{1f}威胁\u{1f}威脅",
            "Bedrohungsstufe\u{1f}Threat Level\u{1f}Nivel de amenaza\u{1f}Nivel de amenaza\u{1f}Niveau de menace\u{1f}Livello Minaccia\u{1f}脅威レベル\u{1f}\u{1f}Poziom zagrożenia\u{1f}Nível de Ameaça\u{1f}Уровень угрозы\u{1f}ระด\u{e31}บภ\u{e31}ยค\u{e38}กคาม\u{1f}Tehdit Seviyesi\u{1f}威胁等级\u{1f}威脅等級",
            "Bedrohungsstufen\u{1f}Threat Levels\u{1f}Niveles de amenaza\u{1f}Niveles de amenaza\u{1f}Niveaux de menace\u{1f}Livelli Minaccia\u{1f}脅威レベル\u{1f}\u{1f}Poziomy zagrożenia\u{1f}Níveis de Ameaça\u{1f}Уровни угрозы\u{1f}ระด\u{e31}บภ\u{e31}ยค\u{e38}กคาม\u{1f}Tehdit Seviyeleri\u{1f}威胁等级\u{1f}威脅等級",
            "Bedrohungen\u{1f}Threats\u{1f}Amenazas\u{1f}Amenazas\u{1f}Menaces\u{1f}Minacce\u{1f}脅威\u{1f}\u{1f}Zagrożenia\u{1f}Ameaças\u{1f}Угрозы\u{1f}ภ\u{e31}ยค\u{e38}กคาม\u{1f}Tehditler\u{1f}威胁\u{1f}威脅",
            "Nächster Punkt entscheidet\u{1f}Tiebreaker\u{1f}Desempate\u{1f}Desempate\u{1f}Décisif\u{1f}Spareggio\u{1f}タイブレーカー\u{1f}\u{1f}Rozstrzygnięcie\u{1f}Desempate\u{1f}Тай-брейк\u{1f}แมตช\u{e4c}ไทเบรก\u{1f}Uzatma\u{1f}绝杀局\u{1f}決勝賽局",
            "Zeit\u{1f}Time\u{1f}Tiempo\u{1f}Tiempo\u{1f}Temps\u{1f}Tempo\u{1f}時間\u{1f}\u{1f}Czas\u{1f}Tempo\u{1f}Время\u{1f}เวลา\u{1f}Süre\u{1f}时间\u{1f}時間",
            "Zeiten\u{1f}Times\u{1f}Tiempos\u{1f}Tiempos\u{1f}Temps\u{1f}Tempi\u{1f}時間\u{1f}\u{1f}Czasy\u{1f}Tempos\u{1f}Результаты\u{1f}เวลา\u{1f}Süre\u{1f}时间\u{1f}時間",
            "Insgesamt\u{1f}Total\u{1f}\u{1f}\u{1f}\u{1f}Totale\u{1f}合計\u{1f}\u{1f}Suma\u{1f}\u{1f}Всего\u{1f}ท\u{e31}\u{e49}งหมด\u{1f}Toplam\u{1f}总计\u{1f}總計",
            "Tauschen\u{1f}Trade\u{1f}Comerciar\u{1f}Intercambiar\u{1f}Échanger\u{1f}Commercia\u{1f}取引\u{1f}\u{1f}Wymień\u{1f}Trocar\u{1f}Торговать\u{1f}แลกเปล\u{e35}\u{e48}ยน\u{1f}Alışveriş Yap\u{1f}交易\u{1f}交易",
            "Getauscht\u{1f}Traded\u{1f}Comerciado\u{1f}Intercambiado\u{1f}Échangé\u{1f}Commerciato\u{1f}取引した\u{1f}\u{1f}Wymieniono\u{1f}Trocado\u{1f}Провел торговлю\u{1f}แลกเปล\u{e35}\u{e48}ยนแล\u{e49}ว\u{1f}Alışveriş Yapılan\u{1f}已交易\u{1f}得到交易",
            "Tausch\u{1f}Trading\u{1f}Comerciando\u{1f}Intercambiando\u{1f}Échange\u{1f}Commercio in Corso\u{1f}取引中\u{1f}\u{1f}Wymienianie\u{1f}Trocando\u{1f}Торгует\u{1f}กำล\u{e31}งแลกเปล\u{e35}\u{e48}ยน\u{1f}Alışveriş Yapan\u{1f}正在交易\u{1f}正在交易",
            "Verräter\u{1f}Traitor\u{1f}Traidor\u{1f}Traidor\u{1f}Traître\u{1f}Traditore\u{1f}反逆者\u{1f}\u{1f}Zdrajca\u{1f}Traidor\u{1f}Предатель\u{1f}คนทรยศ\u{1f}Hain\u{1f}叛徒\u{1f}叛徒",
            "Spione\u{1f}Traitors\u{1f}Traidores\u{1f}Traidores\u{1f}Traîtres\u{1f}Traditori\u{1f}反逆者\u{1f}\u{1f}Zdrajcy\u{1f}Traidores\u{1f}Предатели\u{1f}คนทรยศ\u{1f}Hainler\u{1f}叛徒\u{1f}叛徒",
            "Transferieren\u{1f}Transfer\u{1f}Transferir\u{1f}Transferir\u{1f}Transférer\u{1f}Trasferisci\u{1f}移動\u{1f}\u{1f}Transferuj\u{1f}Transferir\u{1f}Передавать\u{1f}ถ\u{e48}ายโอน\u{1f}Transfer Et\u{1f}转移\u{1f}轉移",
            "Transferiert\u{1f}Transferred\u{1f}Transferido\u{1f}Transferido\u{1f}Transféré\u{1f}Trasferito\u{1f}移動した\u{1f}\u{1f}Przetransferowano\u{1f}Transferido\u{1f}Передал\u{1f}ถ\u{e48}ายโอนแล\u{e49}ว\u{1f}Transfer Edilen\u{1f}已转移\u{1f}受到轉移",
            "Transfer\u{1f}Transferring\u{1f}Transfiriendo\u{1f}Transfiriendo\u{1f}Transfert\u{1f}Trasferimento in Corso\u{1f}移動中\u{1f}\u{1f}Transferowanie\u{1f}Transferindo\u{1f}Передает\u{1f}กำล\u{e31}งถ\u{e48}ายโอน\u{1f}Transfer Eden\u{1f}正在转移\u{1f}正在轉移",
            "Neuer Versuch\u{1f}Try Again\u{1f}Vuelve a intentarlo\u{1f}Inténtalo de nuevo\u{1f}Veuillez réessayer\u{1f}Riprovaci\u{1f}もう一回\u{1f}\u{1f}Spróbuj ponownie\u{1f}Tente de Novo\u{1f}Еще раз\u{1f}ลองอ\u{e35}กคร\u{e31}\u{e49}ง\u{1f}Tekrar Dene\u{1f}再来试试\u{1f}再一次",
            "Geschütz\u{1f}Turret\u{1f}Torreta\u{1f}Torreta\u{1f}Tourelle\u{1f}Torretta\u{1f}タレット\u{1f}\u{1f}Wieżyczka\u{1f}Torre\u{1f}Турель\u{1f}ป\u{e49}อมย\u{e34}ง\u{1f}Taret\u{1f}炮台\u{1f}砲塔",
            "Geschütze\u{1f}Turrets\u{1f}Torretas\u{1f}Torretas\u{1f}Tourelles\u{1f}Torrette\u{1f}タレット\u{1f}\u{1f}Wieżyczki\u{1f}Torres\u{1f}Турели\u{1f}ป\u{e49}อมย\u{e34}ง\u{1f}Taretler\u{1f}炮台\u{1f}砲塔",
            "Argh\u{1f}Ugh\u{1f}Argh\u{1f}Agh\u{1f}Arg\u{1f}\u{1f}あー\u{1f}\u{1f}Uch\u{1f}\u{1f}Фу\u{1f}ย\u{e35}\u{e49}\u{1f}Öf\u{1f}呃\u{1f}呃",
            "Ultimative Fähigkeit\u{1f}Ultimate Ability\u{1f}Habilidad definitiva\u{1f}Habilidad máxima\u{1f}Capacité ultime\u{1f}Abilità Ultra\u{1f}アルティメット・アビリティ\u{1f}\u{1f}Superzdolność\u{1f}Habilidade Suprema\u{1f}Суперспособность\u{1f}อ\u{e31}ลต\u{e34}เมท\u{1f}Ulti Kabiliyet\u{1f}终极技能\u{1f}絕招",
            "Unter\u{1f}Under\u{1f}Debajo\u{1f}Debajo\u{1f}Sous\u{1f}Sotto\u{1f}下\u{1f}\u{1f}Pod\u{1f}Abaixo\u{1f}Менее\u{1f}ใต\u{e49}\u{1f}Alt\u{1f}下方\u{1f}下",
            "Unbekannt\u{1f}Unknown\u{1f}Desconocido\u{1f}Desconocido\u{1f}Inconnu\u{1f}Sconosciuto\u{1f}不明\u{1f}\u{1f}Nieznane\u{1f}Desconhecido\u{1f}Неизвестно\u{1f}ไม\u{e48}ทราบ\u{1f}Bilinmiyor\u{1f}未知\u{1f}未知",
            "Unbegrenzt\u{1f}Unlimited\u{1f}Ilimitado\u{1f}Ilimitado\u{1f}Illimité\u{1f}Illimitato\u{1f}無制限\u{1f}\u{1f}Nieograniczone\u{1f}Ilimitado\u{1f}Неограниченно\u{1f}ไม\u{e48}จำก\u{e31}ด\u{1f}Sınırsız\u{1f}不受限\u{1f}無限制",
            "Freischalten\u{1f}Unlock\u{1f}Desbloquear\u{1f}Desbloquear\u{1f}Déverrouiller\u{1f}Sblocca\u{1f}アンロック\u{1f}\u{1f}Odblokuj\u{1f}Desbloquear\u{1f}Разблокировать\u{1f}ปลดล\u{e47}อค\u{1f}Aç\u{1f}解锁\u{1f}解鎖",
            "Freigeschaltet\u{1f}Unlocked\u{1f}Desbloqueado\u{1f}Desbloqueado\u{1f}Déverrouillé\u{1f}Sbloccato\u{1f}アンロックした\u{1f}\u{1f}Odblokowano\u{1f}Desbloqueado\u{1f}Разблокировал\u{1f}ปลดล\u{e47}อคแล\u{e49}ว\u{1f}Açıldı\u{1f}已解锁\u{1f}得到解鎖",
            "Freischaltung\u{1f}Unlocking\u{1f}Desbloqueando\u{1f}Desbloqueando\u{1f}Déverrouillage\u{1f}Sblocco in Corso\u{1f}アンロック中\u{1f}잠금 해제\u{1f}Odblokowanie\u{1f}Desbloqueando\u{1f}Активация\u{1f}กำล\u{e31}งปลดล\u{e47}อค\u{1f}Açılıyor\u{1f}正在解锁\u{1f}正在解鎖",
            "Unsicher\u{1f}Unsafe\u{1f}Inseguro\u{1f}Inseguro\u{1f}Dangereux\u{1f}Non Sicuro\u{1f}危険\u{1f}\u{1f}Niebezpieczne\u{1f}Inseguro\u{1f}Небезопасный\u{1f}ไม\u{e48}ปลอดภ\u{e31}ย\u{1f}Güvensiz\u{1f}不安全\u{1f}不安全",
            "Instabil\u{1f}Unstable\u{1f}Inestable\u{1f}Inestable\u{1f}Instable\u{1f}Instabile\u{1f}不安定\u{1f}\u{1f}Niestabilne\u{1f}Instável\u{1f}Нестабильный\u{1f}ไม\u{e48}ม\u{e31}\u{e48}นคง\u{1f}Dengesiz\u{1f}不稳定\u{1f}不穩定",
            "Nach oben\u{1f}Up\u{1f}Arriba\u{1f}Arriba\u{1f}Haut\u{1f}Su\u{1f}上\u{1f}\u{1f}Góra\u{1f}Cima\u{1f}Вверх\u{1f}ด\u{e49}านบน\u{1f}At\u{1f}上\u{1f}上",
            "\u{1f}Upgrade\u{1f}Mejora\u{1f}Mejora\u{1f}Amélioration\u{1f}Miglioramento\u{1f}アップグレード\u{1f}\u{1f}Ulepszenie\u{1f}Melhoria\u{1f}Улучшение\u{1f}อ\u{e31}พเกรด\u{1f}Geliştirme\u{1f}升级\u{1f}升級",
            "\u{1f}Upgrades\u{1f}Mejoras\u{1f}Mejoras\u{1f}Améliorations\u{1f}Miglioramenti\u{1f}アップグレード\u{1f}\u{1f}Ulepszenia\u{1f}Melhorias\u{1f}Улучшения\u{1f}อ\u{e31}พเกรด\u{1f}Geliştirmeler\u{1f}升级\u{1f}升級",
            "Hochladen\u{1f}Upload\u{1f}Subir\u{1f}Cargar\u{1f}Envoyer\u{1f}\u{1f}アップロード\u{1f}\u{1f}Wyślij\u{1f}Enviar\u{1f}Загружать\u{1f}อ\u{e31}พโหลด\u{1f}Yükle\u{1f}上传\u{1f}上傳",
            "Hochgeladen\u{1f}Uploaded\u{1f}Subido\u{1f}Cargado\u{1f}Envoyé\u{1f}Upload Effettuato\u{1f}アップロードした\u{1f}\u{1f}Wysłano\u{1f}Enviado\u{1f}Загружен\u{1f}อ\u{e31}พโหลดแล\u{e49}ว\u{1f}Yüklenen\u{1f}已上传\u{1f}已上傳",
            "Upload\u{1f}Uploading\u{1f}Subiendo\u{1f}Cargando\u{1f}Envoi\u{1f}Upload in Corso\u{1f}アップロード中\u{1f}\u{1f}Wysyłanie\u{1f}Enviando\u{1f}Загружает\u{1f}กำล\u{e31}งอ\u{e31}พโหลด\u{1f}Yükleyen\u{1f}正在上传\u{1f}正在上傳",
            "Fähigkeit 1 einsetzen\u{1f}Use Ability 1\u{1f}Usar habilidad 1\u{1f}Usar habilidad 1\u{1f}Utilisation de la capacité 1\u{1f}Usa Abilità 1\u{1f}アビリティ1を使う\u{1f}\u{1f}Użyj Zdolności 1\u{1f}Usar Habilidade 1\u{1f}Использовать способность 1\u{1f}ใช\u{e49}ความสามารถ 1\u{1f}1. Kabiliyeti Kullan\u{1f}使用技能 1\u{1f}使用技能1",
            "Fähigkeit 2 einsetzen\u{1f}Use Ability 2\u{1f}Usar habilidad 2\u{1f}Usar habilidad 2\u{1f}Utilisation de la capacité 2\u{1f}Usa Abilità 2\u{1f}アビリティ2を使う\u{1f}\u{1f}Użyj Zdolności 2\u{1f}Usar Habilidade 2\u{1f}Использовать способность 2\u{1f}ใช\u{e49}ความสามารถ 2\u{1f}2. Kabiliyeti Kullan\u{1f}使用技能 2\u{1f}使用技能2",
            "Ultimative Fähigkeit einsetzen\u{1f}Use Ultimate Ability\u{1f}Usar habilidad definitiva\u{1f}Usar habilidad máxima\u{1f}Utilisation de la capacité ultime\u{1f}Usa Abilità Ultra\u{1f}アルティメット・アビリティを使う\u{1f}\u{1f}Użyj Superzdolności\u{1f}Usar Habilidade Suprema\u{1f}Использовать суперспособность\u{1f}ใช\u{e49}อ\u{e31}ลต\u{e34}เมท\u{1f}Ulti Kabiliyet'i Kullan\u{1f}使用终极技能\u{1f}使用絕招",
            "Sieg\u{1f}Victory\u{1f}Victoria\u{1f}Victoria\u{1f}Victoire\u{1f}Vittoria\u{1f}勝利\u{1f}\u{1f}Zwycięstwo\u{1f}Vitória\u{1f}Победа\u{1f}ช\u{e31}ยชนะ\u{1f}Zafer\u{1f}胜利\u{1f}勝利",
            "Sichtbar\u{1f}Visible\u{1f}\u{1f}\u{1f}\u{1f}Visibile\u{1f}表示される\u{1f}\u{1f}Widzialne\u{1f}Visível\u{1f}Видимый\u{1f}มองเห\u{e47}นได\u{e49}\u{1f}Görülebilir\u{1f}可见\u{1f}可見",
            "\u{1f}Vortex\u{1f}Vórtice\u{1f}Vórtice\u{1f}\u{1f}Vortice\u{1f}渦\u{1f}\u{1f}Wir\u{1f}Vórtice\u{1f}Воронка\u{1f}กระแสน\u{e49}ำวน\u{1f}Girdap\u{1f}漩涡\u{1f}旋渦",
            "Wirbel\u{1f}Vortices\u{1f}Vórtices\u{1f}Vórtices\u{1f}Vortex\u{1f}Vortici\u{1f}渦\u{1f}\u{1f}Wiry\u{1f}Vórtices\u{1f}Воронки\u{1f}กระแสน\u{e49}ำวน\u{1f}Girdap\u{1f}漩涡\u{1f}旋渦",
            "Warten\u{1f}Wait\u{1f}Esperar\u{1f}Esperar\u{1f}Attendre\u{1f}Attendi\u{1f}待機する\u{1f}\u{1f}Czekaj\u{1f}Esperar\u{1f}Ждать\u{1f}รอ\u{1f}Bekle\u{1f}等待\u{1f}等待",
            "Wartezeit\u{1f}Waiting\u{1f}Esperando\u{1f}Esperando\u{1f}Attente\u{1f}In attesa\u{1f}待機している\u{1f}\u{1f}Oczekiwanie\u{1f}Aguardando\u{1f}Ждет\u{1f}กำล\u{e31}งรอ\u{1f}Bekleniyor\u{1f}正在等待\u{1f}正在等待",
            "Wand\u{1f}Wall\u{1f}Muro\u{1f}Muro\u{1f}Mur\u{1f}Muro\u{1f}壁\u{1f}\u{1f}Ściana\u{1f}Parede\u{1f}Стена\u{1f}กำแพง\u{1f}Duvar\u{1f}墙壁\u{1f}牆壁",
            "Wände\u{1f}Walls\u{1f}Muros\u{1f}Muros\u{1f}Murs\u{1f}Muri\u{1f}壁\u{1f}\u{1f}Ściany\u{1f}Paredes\u{1f}Стены\u{1f}กำแพง\u{1f}Duvar\u{1f}墙壁\u{1f}牆壁",
            "Warnung\u{1f}Warning\u{1f}Aviso\u{1f}Advertencia\u{1f}Avertissement\u{1f}Attenzione\u{1f}警告\u{1f}\u{1f}Ostrzeżenie\u{1f}Aviso\u{1f}Предупреждение\u{1f}คำเต\u{e37}อน\u{1f}Uyarı\u{1f}警告\u{1f}警告",
            "Willkommen\u{1f}Welcome\u{1f}¡Hola!\u{1f}Bienvenido\u{1f}Bienvenue\u{1f}Benvenuto\u{1f}WELCOME\u{1f}\u{1f}Witaj\u{1f}Bem-vindos\u{1f}Добро пожаловать\u{1f}ย\u{e34}นด\u{e35}ต\u{e49}อนร\u{e31}บ\u{1f}Hoş Geldin\u{1f}欢迎\u{1f}歡迎",
            "Gut gespielt\u{1f}Well Played\u{1f}Bien jugado\u{1f}Buena jugada\u{1f}Bien joué\u{1f}Ben Fatto\u{1f}お見事\u{1f}\u{1f}Niezłe zagranie\u{1f}Boa Jogo\u{1f}Неплохо сыграно\u{1f}เล\u{e48}นได\u{e49}ด\u{e35}\u{1f}İyi Oynadın\u{1f}称赞\u{1f}打得不錯",
            "Westen\u{1f}West\u{1f}Oeste\u{1f}Oeste\u{1f}Ouest\u{1f}Ovest\u{1f}西\u{1f}\u{1f}Zachód\u{1f}Oeste\u{1f}Запад\u{1f}ตะว\u{e31}นตก\u{1f}Batı\u{1f}西\u{1f}西",
            "Weiß\u{1f}White\u{1f}Blanco\u{1f}Blanco\u{1f}Blanc\u{1f}Bianco\u{1f}白\u{1f}\u{1f}Biały\u{1f}Branco\u{1f}Белый\u{1f}ขาว\u{1f}Beyaz\u{1f}白色\u{1f}白",
            "\u{1f}Wild\u{1f}Salvaje\u{1f}Salvaje\u{1f}Libre\u{1f}Jolly\u{1f}ワイルド\u{1f}\u{1f}Dzicz\u{1f}Curinga\u{1f}Дикий\u{1f}ป\u{e48}า\u{1f}Joker\u{1f}关键牌\u{1f}關鍵牌",
            "Sieg: %1$s EP\u{1f}Win: %1$s XP\u{1f}Victoria: %1$s PE\u{1f}Victoria: %1$s EXP\u{1f}Victoire : %1$s EXP\u{1f}Vittoria: %1$s PE\u{1f}勝利: %1$s XP\u{1f}승리: %1$s 경험치\u{1f}Wygrana: %1$s PD\u{1f}Vitória: %1$s de EXP\u{1f}Победа: %1$s ед. опыта\u{1f}ชนะ: %1$s XP\u{1f}Galibiyet: %1$s TP\u{1f}胜利：%1$s 经验\u{1f}勝利：%1$s點經驗值",
            "Sieger\u{1f}Winner\u{1f}Vencedor\u{1f}Ganador\u{1f}Gagnant\u{1f}Vincitore\u{1f}勝者\u{1f}\u{1f}Wygrany\u{1f}Vencedor\u{1f}Победитель\u{1f}ผ\u{e39}\u{e49}ชนะ\u{1f}Kazanan\u{1f}胜者\u{1f}贏家",
            "Gewinner\u{1f}Winners\u{1f}Vencedores\u{1f}Ganadores\u{1f}Vainqueurs\u{1f}Vincitori\u{1f}勝者\u{1f}\u{1f}Wygrani\u{1f}Vencedores\u{1f}Победители\u{1f}ผ\u{e39}\u{e49}ชนะ\u{1f}Kazananlar\u{1f}胜者\u{1f}贏家",
            "Siege\u{1f}Wins\u{1f}Victorias\u{1f}Victorias\u{1f}Victoires\u{1f}Vittorie\u{1f}勝利\u{1f}\u{1f}Wygrane\u{1f}Vitórias\u{1f}Победы\u{1f}ชนะ\u{1f}Galibiyetler\u{1f}胜利\u{1f}勝利",
            "Weisheit\u{1f}Wisdom\u{1f}Sabiduría\u{1f}Sabiduría\u{1f}Sagesse\u{1f}Saggezza\u{1f}知恵\u{1f}\u{1f}Mądrość\u{1f}Sabedoria\u{1f}Мудрость\u{1f}ป\u{e31}ญญา\u{1f}Bilgelik\u{1f}感知\u{1f}智慧",
            "Schlechter\u{1f}Worse\u{1f}Peor\u{1f}Peor\u{1f}Pire\u{1f}Peggio\u{1f}より悪い\u{1f}\u{1f}Gorzej\u{1f}Pior\u{1f}Хуже\u{1f}แย\u{e48}กว\u{e48}า\u{1f}Daha Kötü\u{1f}较差\u{1f}較差",
            "Am schlechtesten\u{1f}Worst\u{1f}El peor\u{1f}El peor\u{1f}Le pire\u{1f}Peggiore\u{1f}最悪\u{1f}\u{1f}Najgorzej\u{1f}O Pior\u{1f}Худший\u{1f}แย\u{e48}ท\u{e35}\u{e48}ส\u{e38}ด\u{1f}En Kötüsü\u{1f}最差\u{1f}最差",
            "\u{1f}Wow\u{1f}Guau\u{1f}Guau\u{1f}Waouh\u{1f}\u{1f}感動\u{1f}\u{1f}\u{1f}Uau\u{1f}Ух ты!\u{1f}ว\u{e49}าว\u{1f}Vay Canına\u{1f}哇哦\u{1f}哇",
            "Gelb\u{1f}Yellow\u{1f}Amarillo\u{1f}Amarillo\u{1f}Jaune\u{1f}Giallo\u{1f}黄色\u{1f}\u{1f}Żółty\u{1f}Amarelo\u{1f}Желтый\u{1f}เหล\u{e37}อง\u{1f}Sarı\u{1f}黄色\u{1f}黃",
            "Ja\u{1f}Yes\u{1f}Sí\u{1f}Sí\u{1f}Oui\u{1f}Sì\u{1f}はい\u{1f}\u{1f}Tak\u{1f}Sim\u{1f}Да\u{1f}ใช\u{e48}\u{1f}Evet\u{1f}是\u{1f}是",
            "Du\u{1f}You\u{1f}Tú\u{1f}Tú\u{1f}Vous\u{1f}Tu\u{1f}あなた\u{1f}\u{1f}Ty\u{1f}Você\u{1f}Вы\u{1f}ค\u{e38}ณ\u{1f}Sen\u{1f}你\u{1f}你",
            "Du hast verloren\u{1f}You Lose\u{1f}Pierdes\u{1f}Has perdido\u{1f}Vous avez perdu\u{1f}Hai Perso\u{1f}敗北\u{1f}\u{1f}Przegrywasz\u{1f}Você Perdeu\u{1f}Вы проиграли\u{1f}ค\u{e38}ณแพ\u{e49}\u{1f}Kaybettin\u{1f}你失败了\u{1f}你輸了",
            "Du hast gewonnen\u{1f}You Win\u{1f}Ganas\u{1f}Has ganado\u{1f}Vous avez gagné\u{1f}Hai Vinto\u{1f}勝利\u{1f}\u{1f}Wygrywasz\u{1f}Você Venceu\u{1f}Вы победили\u{1f}ค\u{e38}ณชนะ\u{1f}Kazandın\u{1f}你胜利了\u{1f}你贏了",
            "\u{1f}Zone\u{1f}Zona\u{1f}Zona\u{1f}\u{1f}Zona\u{1f}ゾーン\u{1f}\u{1f}Strefa\u{1f}Zona\u{1f}Зона\u{1f}โซน\u{1f}Bölge\u{1f}区域\u{1f}範圍",
            "Zonen\u{1f}Zones\u{1f}Zonas\u{1f}Zonas\u{1f}\u{1f}Zone\u{1f}ゾーン\u{1f}\u{1f}Strefy\u{1f}Zonas\u{1f}Зоны\u{1f}โซน\u{1f}Bölgeler\u{1f}区域\u{1f}範圍",
            "\u{1f}{0} != {1}",
            "\u{1f}{0} * {1}",
            "\u{1f}{0} + {1}",
            "\u{1f}{0} - {1}",
            "\u{1f}{0} - {1} - {2}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}{0} – {1} – {2}",
            "\u{1f}{0} ->",
            "\u{1f}{0} -> {1}",
            "\u{1f}{0} / {1}",
            "\u{1f}{0} : {1} : {2}",
            "\u{1f}{0} < {1}",
            "\u{1f}{0} <-",
            "\u{1f}{0} <- {1}",
            "\u{1f}{0} <->",
            "\u{1f}{0} <-> {1}",
            "\u{1f}{0} <= {1}",
            "\u{1f}{0} = {1}",
            "\u{1f}{0} == {1}",
            "\u{1f}{0} > {1}",
            "\u{1f}{0} >= {1}",
            "{0} und {1}\u{1f}{0} and {1}\u{1f}{0} y {1}\u{1f}{0} y {1}\u{1f}{0} et {1}\u{1f}{0} e {1}\u{1f}{0}と{1}\u{1f}\u{1f}{0} i {1}\u{1f}{0} e {1}\u{1f}{0} и {1}\u{1f}{0} และ {1}\u{1f}{0} ve {1}\u{1f}{0}和{1}\u{1f}{0}以及{1}",
            "\u{1f}{0} m\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}{0}メートル\u{1f}\u{1f}\u{1f}\u{1f}{0} м\u{1f}\u{1f}\u{1f}{0} 米\u{1f}{0}公尺",
            "\u{1f}{0} m/s\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}{0}メートル/秒\u{1f}\u{1f}\u{1f}\u{1f}{0} м/сек.\u{1f}\u{1f}{0} m/sn\u{1f}{0} 米/秒\u{1f}{0}公尺/秒",
            "{0} Sek.\u{1f}{0} sec\u{1f}{0} s\u{1f}{0} seg.\u{1f}{0} s\u{1f}{0} s\u{1f}{0}秒\u{1f}\u{1f}{0} sek.\u{1f}{0} s\u{1f}{0} сек.\u{1f}{0} ว\u{e34}นาท\u{e35}\u{1f}{0} sn.\u{1f}{0}秒\u{1f}{0}秒",
            "\u{1f}{0} vs {1}\u{1f}{0} contra {1}\u{1f}{0} vs. {1}\u{1f}\u{1f}\u{1f}{0}対{1}\u{1f}\u{1f}\u{1f}{0} vs. {1}\u{1f}{0} х {1}\u{1f}\u{1f}{0} / {1}\u{1f}{0}对阵{1}",
            "\u{1f}{0} {1}",
            "\u{1f}{0} {1} {2}",
            "\u{1f}{0}!\u{1f}¡{0}!\u{1f}\u{1f}{0} !\u{1f}\u{1f}{0}！\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}{0}！",
            "\u{1f}{0}!!\u{1f}¡¡{0}!!\u{1f}\u{1f}{0} !!\u{1f}\u{1f}{0}！！\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}{0}！！",
            "\u{1f}{0}!!!\u{1f}¡¡¡{0}!!!\u{1f}\u{1f}{0} !!!\u{1f}\u{1f}{0}！！！\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}{0}！！！",
            "\u{1f}{0}%\u{1f}\u{1f}\u{1f}{0} %\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}%{0}",
            "\u{1f}{0}, {1}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}{0}、{1}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}{0}，{1}",
            "{0}, {1} und {2}\u{1f}{0}, {1}, and {2}\u{1f}{0}, {1} y {2}\u{1f}{0}, {1} y {2}\u{1f}{0}, {1} et {2}\u{1f}{0}, {1}, e {2}\u{1f}{0}、{1}、{2}\u{1f}\u{1f}{0}, {1} i {2}\u{1f}{0}, {1} e {2}\u{1f}{0}, {1} и {2}\u{1f}{0}, {1}, และ {2}\u{1f}{0}, {1} ve {2}\u{1f}{0}，{1}和{2}\u{1f}{0}, {1}, 以及{2}",
            "\u{1f}{0}:\u{1f}\u{1f}\u{1f}{0} :\u{1f}\u{1f}{0}: ",
            "\u{1f}{0}: {1}\u{1f}\u{1f}\u{1f}{0} : {1}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}{0}：{1}",
            "{0}: {1} und {2}\u{1f}{0}: {1} and {2}\u{1f}{0}: {1} y {2}\u{1f}{0}: {1} y {2}\u{1f}{0} : {1} et {2}\u{1f}{0}: {1} e {2}\u{1f}{0}: {1}と{2}\u{1f}\u{1f}{0}: {1} i {2}\u{1f}{0}: {1} e {2}\u{1f}{0}: {1} и {2}\u{1f}{0}: {1} และ {2}\u{1f}{0}: {1} ve {2}\u{1f}{0}：{1}和{2}\u{1f}{0}: {1} 以及{2}",
            "\u{1f}{0}:{1}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}{0}: {1}",
            "\u{1f}{0}?\u{1f}¿{0}?\u{1f}\u{1f}{0} ?\u{1f}\u{1f}{0}？\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}{0}？",
            "\u{1f}{0}??\u{1f}¿¿{0}??\u{1f}\u{1f}{0} ??\u{1f}\u{1f}{0}？？\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}{0}？？",
            "\u{1f}{0}???\u{1f}¿¿¿{0}???\u{1f}\u{1f}{0} ???\u{1f}\u{1f}{0}？？？\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}{0}？？？",
            "\u{1f}¡{0}!",
            "\u{1f}¿{0}?",
        ];
        localized(NAMES[self as usize], lang)
    }
}
impl LocalizedString {
    /// The phrase with this English text, if the game has one.
    pub fn from_en(text: &str) -> Option<Self> {
        Self::ALL
            .binary_search_by(|phrase| phrase.text(Lang::EnUs).cmp(text))
            .ok()
            .map(|i| Self::ALL[i])
    }
}
