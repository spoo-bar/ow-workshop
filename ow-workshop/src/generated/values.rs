//! Typed values generated from the signature tables.
use crate::expr::Expr;
use crate::support;
use crate::ty::*;
use ow_workshop_data::ValueId;
/// Values generated here.
pub(crate) const GENERATED_VALUES: &[ValueId] = &[
    ValueId::VectorBackward,
    ValueId::VectorDown,
    ValueId::VectorForward,
    ValueId::VectorLeft,
    ValueId::VectorRight,
    ValueId::VectorUp,
    ValueId::AbilityIconString,
    ValueId::Abs,
    ValueId::Acos,
    ValueId::AcosDeg,
    ValueId::AngleBetweenVectors,
    ValueId::AngleDifference,
    ValueId::Asin,
    ValueId::AsinDeg,
    ValueId::Atan2,
    ValueId::Atan2Deg,
    ValueId::CharAt,
    ValueId::Concat,
    ValueId::Cos,
    ValueId::CosDeg,
    ValueId::CreateWorkshopSettingBool,
    ValueId::CreateWorkshopSettingEnum,
    ValueId::CreateWorkshopSettingFloat,
    ValueId::CreateWorkshopSettingHero,
    ValueId::CreateWorkshopSettingInt,
    ValueId::CrossProduct,
    ValueId::DirectionFromAngles,
    ValueId::DirectionTowards,
    ValueId::Distance,
    ValueId::DotProduct,
    ValueId::EntityExists,
    ValueId::EvalOnce,
    ValueId::Exclude,
    ValueId::GetAbilityCharge,
    ValueId::GetAbilityCooldown,
    ValueId::GetAbilityResource,
    ValueId::GetAllHeroes,
    ValueId::GetAllowedHeroes,
    ValueId::GetAltitude,
    ValueId::GetAmmo,
    ValueId::GetAverageServerLoad,
    ValueId::GetCapturePercentage,
    ValueId::GetClosestPlayer,
    ValueId::GetControlScorePercentage,
    ValueId::GetControlScoringTeam,
    ValueId::GetCurrentGamemode,
    ValueId::GetCurrentObjective,
    ValueId::GetCurrentWeapon,
    ValueId::GetDamageHeroes,
    ValueId::GetDeadPlayers,
    ValueId::GetEyePosition,
    ValueId::GetFacingDirection,
    ValueId::GetFarthestPlayer,
    ValueId::GetFlagCarrier,
    ValueId::GetFlagPosition,
    ValueId::GetHealth,
    ValueId::GetHealthOfType,
    ValueId::GetHero,
    ValueId::GetHeroOfDuplication,
    ValueId::GetHeroStatistic,
    ValueId::GetHorizontalFacingAngle,
    ValueId::GetHorizontalSpeed,
    ValueId::GetLastAssistId,
    ValueId::GetLastCreatedEntity,
    ValueId::GetLastCreatedHealthPool,
    ValueId::GetLastCreatedText,
    ValueId::GetLastDamageModification,
    ValueId::GetLastDamageOverTimeId,
    ValueId::GetLastHealingModification,
    ValueId::GetLastHealingOverTimeId,
    ValueId::GetLivingPlayers,
    ValueId::GetMatchRound,
    ValueId::GetMatchTime,
    ValueId::GetMaxAmmo,
    ValueId::GetMaxHealth,
    ValueId::GetMaxHealthOfType,
    ValueId::GetNormalizedHealth,
    ValueId::GetNumberOfAssistIds,
    ValueId::GetNumberOfDamageModificationIds,
    ValueId::GetNumberOfDamageOverTimeIds,
    ValueId::GetNumberOfDeadPlayers,
    ValueId::GetNumberOfDeaths,
    ValueId::GetNumberOfElims,
    ValueId::GetNumberOfEntityIds,
    ValueId::GetNumberOfFinalBlows,
    ValueId::GetNumberOfHealingModificationIds,
    ValueId::GetNumberOfHealingOverTimeIds,
    ValueId::GetNumberOfHeroes,
    ValueId::GetNumberOfLivingPlayers,
    ValueId::GetNumberOfPlayers,
    ValueId::GetNumberOfPlayersOnObjective,
    ValueId::GetNumberOfSlots,
    ValueId::GetNumberOfTextIds,
    ValueId::GetObjectivePosition,
    ValueId::GetOppositeTeam,
    ValueId::GetPayloadPosition,
    ValueId::GetPayloadProgressPercentage,
    ValueId::GetPeakServerLoad,
    ValueId::GetPlayerClosestToReticle,
    ValueId::GetPlayers,
    ValueId::GetPlayersInRadius,
    ValueId::GetPlayersInSlot,
    ValueId::GetPlayersInViewAngle,
    ValueId::GetPlayersNotOnObjective,
    ValueId::GetPlayersOnHero,
    ValueId::GetPlayersOnObjective,
    ValueId::GetPosition,
    ValueId::GetScore,
    ValueId::GetServerLoad,
    ValueId::GetSlot,
    ValueId::GetSpawnPoints,
    ValueId::GetSpeed,
    ValueId::GetSpeedInDirection,
    ValueId::GetStatistic,
    ValueId::GetSupportHeroes,
    ValueId::GetTankHeroes,
    ValueId::GetTeam,
    ValueId::GetThrottle,
    ValueId::GetTotalTimeElapsed,
    ValueId::GetUltCharge,
    ValueId::GetVelocity,
    ValueId::GetVerticalFacingAngle,
    ValueId::GetVerticalSpeed,
    ValueId::HasSpawned,
    ValueId::HasStatus,
    ValueId::HeroIcon,
    ValueId::HorizontalAngleOfDirection,
    ValueId::HorizontalAngleTowards,
    ValueId::HostPlayer,
    ValueId::IconString,
    ValueId::Index,
    ValueId::InputBindingString,
    ValueId::IsAlive,
    ValueId::IsAssemblingHeroes,
    ValueId::IsCommunicating,
    ValueId::IsCommunicatingAnything,
    ValueId::IsCommunicatingEmote,
    ValueId::IsCommunicatingSpray,
    ValueId::IsCommunicatingVoiceline,
    ValueId::IsControlPointLocked,
    ValueId::IsCrouching,
    ValueId::IsDead,
    ValueId::IsDummy,
    ValueId::IsDuplicatingAHero,
    ValueId::IsFiringPrimaryFire,
    ValueId::IsFiringSecondaryFire,
    ValueId::IsFlagAtBase,
    ValueId::IsFlagBeingCarried,
    ValueId::IsGameInProgress,
    ValueId::IsHeroBeingPlayed,
    ValueId::IsHoldingButton,
    ValueId::IsInAir,
    ValueId::IsInAlternateForm,
    ValueId::IsInLoS,
    ValueId::IsInSetup,
    ValueId::IsInSpawnRoom,
    ValueId::IsInSuddenDeath,
    ValueId::IsInViewAngle,
    ValueId::IsJumping,
    ValueId::IsMatchBetweenRounds,
    ValueId::IsMatchComplete,
    ValueId::IsMeleeing,
    ValueId::IsMoving,
    ValueId::IsObjectiveComplete,
    ValueId::IsOnFire,
    ValueId::IsOnGround,
    ValueId::IsOnObjective,
    ValueId::IsOnWall,
    ValueId::IsReloading,
    ValueId::IsStanding,
    ValueId::IsTeamOnDefense,
    ValueId::IsTeamOnOffense,
    ValueId::IsUsingAbility1,
    ValueId::IsUsingAbility2,
    ValueId::IsUsingUltimate,
    ValueId::IsWaitingForPlayers,
    ValueId::Last,
    ValueId::Len,
    ValueId::LocalPlayer,
    ValueId::LocalVector,
    ValueId::Magnitude,
    ValueId::Max,
    ValueId::Min,
    ValueId::NearestWalkablePosition,
    ValueId::Normalize,
    ValueId::RandomChoice,
    ValueId::RandomRandint,
    ValueId::RandomShuffle,
    ValueId::RandomUniform,
    ValueId::Replace,
    ValueId::Rgb,
    ValueId::Sin,
    ValueId::SinDeg,
    ValueId::Slice,
    ValueId::Split,
    ValueId::Sqrt,
    ValueId::StrContains,
    ValueId::StrIndex,
    ValueId::StrLen,
    ValueId::Substring,
    ValueId::Tan,
    ValueId::TanDeg,
    ValueId::TeamScore,
    ValueId::UpdateEveryFrame,
    ValueId::VectorTowards,
    ValueId::VerticalAngleOfDirection,
    ValueId::VerticalAngleTowards,
    ValueId::WorldVector,
];
/// Values with a hand-written API instead.
pub(crate) const HAND_WRITTEN_VALUES: &[ValueId] = &[
    ValueId::Attacker,
    ValueId::Victim,
    ValueId::Healer,
    ValueId::Healee,
    ValueId::EventPlayer,
    ValueId::EventDamage,
    ValueId::EventHealing,
    ValueId::EventAbility,
    ValueId::EventDirection,
    ValueId::EventWasCriticalHit,
    ValueId::EventWasEnvironment,
    ValueId::EventWasHealthPack,
    ValueId::True,
    ValueId::False,
    ValueId::Null,
    ValueId::Vect,
    ValueId::RaycastHitNormal,
    ValueId::RaycastHitPlayer,
    ValueId::RaycastHitPosition,
];
/// `Backward`.
#[track_caller]
#[must_use]
pub fn vector_backward() -> Expr<Direction> {
    support::value(ValueId::VectorBackward, vec![])
}
/// `Down`.
#[track_caller]
#[must_use]
pub fn vector_down() -> Expr<Direction> {
    support::value(ValueId::VectorDown, vec![])
}
/// `Forward`.
#[track_caller]
#[must_use]
pub fn vector_forward() -> Expr<Direction> {
    support::value(ValueId::VectorForward, vec![])
}
/// `Left`.
#[track_caller]
#[must_use]
pub fn vector_left() -> Expr<Direction> {
    support::value(ValueId::VectorLeft, vec![])
}
/// `Right`.
#[track_caller]
#[must_use]
pub fn vector_right() -> Expr<Direction> {
    support::value(ValueId::VectorRight, vec![])
}
/// `Up`.
#[track_caller]
#[must_use]
pub fn vector_up() -> Expr<Direction> {
    support::value(ValueId::VectorUp, vec![])
}
/// `Ability Icon String`.
#[track_caller]
#[must_use]
pub fn ability_icon_string(hero: impl IntoExpr<Hero>, button: impl IntoExpr<Button>) -> Expr<Str> {
    support::value(
        ValueId::AbilityIconString,
        vec![hero.into_expr().into_ir(), button.into_expr().into_ir()],
    )
}
/// `Absolute Value`.
#[track_caller]
#[must_use]
pub fn abs(value: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(ValueId::Abs, vec![value.into_expr().into_ir()])
}
/// `Arccosine In Radians`.
#[track_caller]
#[must_use]
pub fn acos(value: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(ValueId::Acos, vec![value.into_expr().into_ir()])
}
/// `Arccosine In Degrees`.
#[track_caller]
#[must_use]
pub fn acos_deg(value: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(ValueId::AcosDeg, vec![value.into_expr().into_ir()])
}
/// `Angle Between Vectors`.
#[track_caller]
#[must_use]
pub fn angle_between_vectors(
    vector_1: impl IntoExpr<Direction>,
    vector_2: impl IntoExpr<Direction>,
) -> Expr<Num> {
    support::value(
        ValueId::AngleBetweenVectors,
        vec![
            vector_1.into_expr().into_ir(),
            vector_2.into_expr().into_ir(),
        ],
    )
}
/// `Angle Difference`.
#[track_caller]
#[must_use]
pub fn angle_difference(angle_1: impl IntoExpr<Num>, angle_2: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(
        ValueId::AngleDifference,
        vec![angle_1.into_expr().into_ir(), angle_2.into_expr().into_ir()],
    )
}
/// `Arcsine In Radians`.
#[track_caller]
#[must_use]
pub fn asin(value: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(ValueId::Asin, vec![value.into_expr().into_ir()])
}
/// `Arcsine In Degrees`.
#[track_caller]
#[must_use]
pub fn asin_deg(value: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(ValueId::AsinDeg, vec![value.into_expr().into_ir()])
}
/// `Arctangent In Radians`.
#[track_caller]
#[must_use]
pub fn atan2(numerator: impl IntoExpr<Num>, denominator: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(
        ValueId::Atan2,
        vec![
            numerator.into_expr().into_ir(),
            denominator.into_expr().into_ir(),
        ],
    )
}
/// `Arctangent In Degrees`.
#[track_caller]
#[must_use]
pub fn atan2_deg(numerator: impl IntoExpr<Num>, denominator: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(
        ValueId::Atan2Deg,
        vec![
            numerator.into_expr().into_ir(),
            denominator.into_expr().into_ir(),
        ],
    )
}
/// `Cosine From Radians`.
#[track_caller]
#[must_use]
pub fn cos(angle: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(ValueId::Cos, vec![angle.into_expr().into_ir()])
}
/// `Cosine From Degrees`.
#[track_caller]
#[must_use]
pub fn cos_deg(angle: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(ValueId::CosDeg, vec![angle.into_expr().into_ir()])
}
/// `Workshop Setting Toggle`.
#[track_caller]
#[must_use]
pub fn create_workshop_setting_bool(
    category: impl IntoExpr<Str>,
    name: impl IntoExpr<Str>,
    default: impl IntoExpr<Bool>,
) -> Expr<Bool> {
    create_workshop_setting_bool_with(
        category,
        name,
        default,
        CreateWorkshopSettingBoolArgs::default(),
    )
}
/// `Workshop Setting Toggle`.
#[track_caller]
#[must_use]
pub fn create_workshop_setting_bool_with(
    category: impl IntoExpr<Str>,
    name: impl IntoExpr<Str>,
    default: impl IntoExpr<Bool>,
    opts: CreateWorkshopSettingBoolArgs,
) -> Expr<Bool> {
    support::value(
        ValueId::CreateWorkshopSettingBool,
        vec![
            category.into_expr().into_ir(),
            name.into_expr().into_ir(),
            default.into_expr().into_ir(),
            opts.sort_order.into_ir(),
        ],
    )
}
/// `Workshop Setting Combo`.
#[track_caller]
#[must_use]
pub fn create_workshop_setting_enum(
    category: impl IntoExpr<Str>,
    name: impl IntoExpr<Str>,
    default: impl IntoExpr<Num>,
    options: impl IntoExpr<Array<Str>>,
) -> Expr<Num> {
    create_workshop_setting_enum_with(
        category,
        name,
        default,
        options,
        CreateWorkshopSettingEnumArgs::default(),
    )
}
/// `Workshop Setting Combo`.
#[track_caller]
#[must_use]
pub fn create_workshop_setting_enum_with(
    category: impl IntoExpr<Str>,
    name: impl IntoExpr<Str>,
    default: impl IntoExpr<Num>,
    options: impl IntoExpr<Array<Str>>,
    opts: CreateWorkshopSettingEnumArgs,
) -> Expr<Num> {
    support::value(
        ValueId::CreateWorkshopSettingEnum,
        vec![
            category.into_expr().into_ir(),
            name.into_expr().into_ir(),
            default.into_expr().into_ir(),
            options.into_expr().into_ir(),
            opts.sort_order.into_ir(),
        ],
    )
}
/// `Workshop Setting Real`.
#[track_caller]
#[must_use]
pub fn create_workshop_setting_float(
    category: impl IntoExpr<Str>,
    name: impl IntoExpr<Str>,
    default: impl IntoExpr<Num>,
    min: impl IntoExpr<Num>,
    max: impl IntoExpr<Num>,
) -> Expr<Num> {
    create_workshop_setting_float_with(
        category,
        name,
        default,
        min,
        max,
        CreateWorkshopSettingFloatArgs::default(),
    )
}
/// `Workshop Setting Real`.
#[track_caller]
#[must_use]
pub fn create_workshop_setting_float_with(
    category: impl IntoExpr<Str>,
    name: impl IntoExpr<Str>,
    default: impl IntoExpr<Num>,
    min: impl IntoExpr<Num>,
    max: impl IntoExpr<Num>,
    opts: CreateWorkshopSettingFloatArgs,
) -> Expr<Num> {
    support::value(
        ValueId::CreateWorkshopSettingFloat,
        vec![
            category.into_expr().into_ir(),
            name.into_expr().into_ir(),
            default.into_expr().into_ir(),
            min.into_expr().into_ir(),
            max.into_expr().into_ir(),
            opts.sort_order.into_ir(),
        ],
    )
}
/// `Workshop Setting Hero`.
#[track_caller]
#[must_use]
pub fn create_workshop_setting_hero(
    category: impl IntoExpr<Str>,
    name: impl IntoExpr<Str>,
    default: impl IntoExpr<Hero>,
) -> Expr<Hero> {
    create_workshop_setting_hero_with(
        category,
        name,
        default,
        CreateWorkshopSettingHeroArgs::default(),
    )
}
/// `Workshop Setting Hero`.
#[track_caller]
#[must_use]
pub fn create_workshop_setting_hero_with(
    category: impl IntoExpr<Str>,
    name: impl IntoExpr<Str>,
    default: impl IntoExpr<Hero>,
    opts: CreateWorkshopSettingHeroArgs,
) -> Expr<Hero> {
    support::value(
        ValueId::CreateWorkshopSettingHero,
        vec![
            category.into_expr().into_ir(),
            name.into_expr().into_ir(),
            default.into_expr().into_ir(),
            opts.sort_order.into_ir(),
        ],
    )
}
/// `Workshop Setting Integer`.
#[track_caller]
#[must_use]
pub fn create_workshop_setting_int(
    category: impl IntoExpr<Str>,
    name: impl IntoExpr<Str>,
    default: impl IntoExpr<Num>,
    min: impl IntoExpr<Num>,
    max: impl IntoExpr<Num>,
) -> Expr<Num> {
    create_workshop_setting_int_with(
        category,
        name,
        default,
        min,
        max,
        CreateWorkshopSettingIntArgs::default(),
    )
}
/// `Workshop Setting Integer`.
#[track_caller]
#[must_use]
pub fn create_workshop_setting_int_with(
    category: impl IntoExpr<Str>,
    name: impl IntoExpr<Str>,
    default: impl IntoExpr<Num>,
    min: impl IntoExpr<Num>,
    max: impl IntoExpr<Num>,
    opts: CreateWorkshopSettingIntArgs,
) -> Expr<Num> {
    support::value(
        ValueId::CreateWorkshopSettingInt,
        vec![
            category.into_expr().into_ir(),
            name.into_expr().into_ir(),
            default.into_expr().into_ir(),
            min.into_expr().into_ir(),
            max.into_expr().into_ir(),
            opts.sort_order.into_ir(),
        ],
    )
}
/// `Cross Product`.
#[track_caller]
#[must_use]
pub fn cross_product(
    value_1: impl IntoExpr<Vector>,
    value_2: impl IntoExpr<Vector>,
) -> Expr<Vector> {
    support::value(
        ValueId::CrossProduct,
        vec![value_1.into_expr().into_ir(), value_2.into_expr().into_ir()],
    )
}
/// `Direction From Angles`.
#[track_caller]
#[must_use]
pub fn direction_from_angles(
    horizontal_angle: impl IntoExpr<Num>,
    vertical_angle: impl IntoExpr<Num>,
) -> Expr<Direction> {
    support::value(
        ValueId::DirectionFromAngles,
        vec![
            horizontal_angle.into_expr().into_ir(),
            vertical_angle.into_expr().into_ir(),
        ],
    )
}
/// `Direction Towards`.
#[track_caller]
#[must_use]
pub fn direction_towards(
    start_pos: impl IntoExpr<Position>,
    end_pos: impl IntoExpr<Position>,
) -> Expr<Direction> {
    support::value(
        ValueId::DirectionTowards,
        vec![
            start_pos.into_expr().into_ir(),
            end_pos.into_expr().into_ir(),
        ],
    )
}
/// `Distance Between`.
#[track_caller]
#[must_use]
pub fn distance(start_pos: impl IntoExpr<Position>, end_pos: impl IntoExpr<Position>) -> Expr<Num> {
    support::value(
        ValueId::Distance,
        vec![
            start_pos.into_expr().into_ir(),
            end_pos.into_expr().into_ir(),
        ],
    )
}
/// `Dot Product`.
#[track_caller]
#[must_use]
pub fn dot_product(value_1: impl IntoExpr<Vector>, value_2: impl IntoExpr<Vector>) -> Expr<Num> {
    support::value(
        ValueId::DotProduct,
        vec![value_1.into_expr().into_ir(), value_2.into_expr().into_ir()],
    )
}
/// `Entity Exists`.
#[track_caller]
#[must_use]
pub fn entity_exists(entity: impl IntoExpr<EntityOrPlayer>) -> Expr<Bool> {
    support::value(ValueId::EntityExists, vec![entity.into_expr().into_ir()])
}
/// `Evaluate Once`.
#[track_caller]
#[must_use]
pub fn eval_once(input_value: impl IntoExpr<Any>) -> Expr<Any> {
    support::value(ValueId::EvalOnce, vec![input_value.into_expr().into_ir()])
}
/// `All Heroes`.
#[track_caller]
#[must_use]
pub fn all_heroes() -> Expr<Array<Hero>> {
    support::value(ValueId::GetAllHeroes, vec![])
}
/// `Server Load Average`.
#[track_caller]
#[must_use]
pub fn average_server_load() -> Expr<Num> {
    support::value(ValueId::GetAverageServerLoad, vec![])
}
/// `Point Capture Percentage`.
#[track_caller]
#[must_use]
pub fn capture_percentage() -> Expr<Num> {
    support::value(ValueId::GetCapturePercentage, vec![])
}
/// `Closest Player To`.
#[track_caller]
#[must_use]
pub fn closest_player(center: impl IntoExpr<Position>, team: impl IntoExpr<Team>) -> Expr<Player> {
    support::value(
        ValueId::GetClosestPlayer,
        vec![center.into_expr().into_ir(), team.into_expr().into_ir()],
    )
}
/// `Control Mode Scoring Percentage`.
#[track_caller]
#[must_use]
pub fn control_score_percentage(team: impl IntoExpr<Team>) -> Expr<Num> {
    support::value(
        ValueId::GetControlScorePercentage,
        vec![team.into_expr().into_ir()],
    )
}
/// `Control Mode Scoring Team`.
#[track_caller]
#[must_use]
pub fn control_scoring_team() -> Expr<Team> {
    support::value(ValueId::GetControlScoringTeam, vec![])
}
/// `Current Game Mode`.
#[track_caller]
#[must_use]
pub fn current_gamemode() -> Expr<Gamemode> {
    support::value(ValueId::GetCurrentGamemode, vec![])
}
/// `Objective Index`.
#[track_caller]
#[must_use]
pub fn current_objective() -> Expr<Num> {
    support::value(ValueId::GetCurrentObjective, vec![])
}
/// `All Damage Heroes`.
#[track_caller]
#[must_use]
pub fn damage_heroes() -> Expr<Array<Hero>> {
    support::value(ValueId::GetDamageHeroes, vec![])
}
/// `All Dead Players`.
#[track_caller]
#[must_use]
pub fn dead_players() -> Expr<Array<Player>> {
    dead_players_with(GetDeadPlayersArgs::default())
}
/// `All Dead Players`.
#[track_caller]
#[must_use]
pub fn dead_players_with(opts: GetDeadPlayersArgs) -> Expr<Array<Player>> {
    support::value(ValueId::GetDeadPlayers, vec![opts.team.into_ir()])
}
/// `Farthest Player From`.
#[track_caller]
#[must_use]
pub fn farthest_player(center: impl IntoExpr<Position>, team: impl IntoExpr<Team>) -> Expr<Player> {
    support::value(
        ValueId::GetFarthestPlayer,
        vec![center.into_expr().into_ir(), team.into_expr().into_ir()],
    )
}
/// `Player Carrying Flag`.
#[track_caller]
#[must_use]
pub fn flag_carrier(team: impl IntoExpr<Team>) -> Expr<Player> {
    support::value(ValueId::GetFlagCarrier, vec![team.into_expr().into_ir()])
}
/// `Flag Position`.
#[track_caller]
#[must_use]
pub fn flag_position(team: impl IntoExpr<Team>) -> Expr<Position> {
    support::value(ValueId::GetFlagPosition, vec![team.into_expr().into_ir()])
}
/// `Last Assist ID`.
#[track_caller]
#[must_use]
pub fn last_assist_id() -> Expr<AssistId> {
    support::value(ValueId::GetLastAssistId, vec![])
}
/// `Last Created Entity`.
#[track_caller]
#[must_use]
pub fn last_created_entity() -> Expr<EntityId> {
    support::value(ValueId::GetLastCreatedEntity, vec![])
}
/// `Last Created Health Pool`.
#[track_caller]
#[must_use]
pub fn last_created_health_pool() -> Expr<HealthPoolId> {
    support::value(ValueId::GetLastCreatedHealthPool, vec![])
}
/// `Last Text ID`.
#[track_caller]
#[must_use]
pub fn last_created_text() -> Expr<TextId> {
    support::value(ValueId::GetLastCreatedText, vec![])
}
/// `Last Damage Modification ID`.
#[track_caller]
#[must_use]
pub fn last_damage_modification() -> Expr<DamageModId> {
    support::value(ValueId::GetLastDamageModification, vec![])
}
/// `Last Damage Over Time ID`.
#[track_caller]
#[must_use]
pub fn last_damage_over_time_id() -> Expr<DotId> {
    support::value(ValueId::GetLastDamageOverTimeId, vec![])
}
/// `Last Healing Modification ID`.
#[track_caller]
#[must_use]
pub fn last_healing_modification() -> Expr<HealingModId> {
    support::value(ValueId::GetLastHealingModification, vec![])
}
/// `Last Heal Over Time ID`.
#[track_caller]
#[must_use]
pub fn last_healing_over_time_id() -> Expr<HotId> {
    support::value(ValueId::GetLastHealingOverTimeId, vec![])
}
/// `All Living Players`.
#[track_caller]
#[must_use]
pub fn living_players() -> Expr<Array<Player>> {
    living_players_with(GetLivingPlayersArgs::default())
}
/// `All Living Players`.
#[track_caller]
#[must_use]
pub fn living_players_with(opts: GetLivingPlayersArgs) -> Expr<Array<Player>> {
    support::value(ValueId::GetLivingPlayers, vec![opts.team.into_ir()])
}
/// `Match Round`.
#[track_caller]
#[must_use]
pub fn match_round() -> Expr<Num> {
    support::value(ValueId::GetMatchRound, vec![])
}
/// `Match Time`.
#[track_caller]
#[must_use]
pub fn match_time() -> Expr<Num> {
    support::value(ValueId::GetMatchTime, vec![])
}
/// `Assist Count`.
#[track_caller]
#[must_use]
pub fn number_of_assist_ids() -> Expr<Num> {
    support::value(ValueId::GetNumberOfAssistIds, vec![])
}
/// `Damage Modification Count`.
#[track_caller]
#[must_use]
pub fn number_of_damage_modification_ids() -> Expr<Num> {
    support::value(ValueId::GetNumberOfDamageModificationIds, vec![])
}
/// `Damage Over Time Count`.
#[track_caller]
#[must_use]
pub fn number_of_damage_over_time_ids() -> Expr<Num> {
    support::value(ValueId::GetNumberOfDamageOverTimeIds, vec![])
}
/// `Number Of Dead Players`.
#[track_caller]
#[must_use]
pub fn number_of_dead_players() -> Expr<Num> {
    number_of_dead_players_with(GetNumberOfDeadPlayersArgs::default())
}
/// `Number Of Dead Players`.
#[track_caller]
#[must_use]
pub fn number_of_dead_players_with(opts: GetNumberOfDeadPlayersArgs) -> Expr<Num> {
    support::value(ValueId::GetNumberOfDeadPlayers, vec![opts.team.into_ir()])
}
/// `Entity Count`.
#[track_caller]
#[must_use]
pub fn number_of_entity_ids() -> Expr<Num> {
    support::value(ValueId::GetNumberOfEntityIds, vec![])
}
/// `Healing Modification Count`.
#[track_caller]
#[must_use]
pub fn number_of_healing_modification_ids() -> Expr<Num> {
    support::value(ValueId::GetNumberOfHealingModificationIds, vec![])
}
/// `Heal Over Time Count`.
#[track_caller]
#[must_use]
pub fn number_of_healing_over_time_ids() -> Expr<Num> {
    support::value(ValueId::GetNumberOfHealingOverTimeIds, vec![])
}
/// `Number Of Heroes`.
#[track_caller]
#[must_use]
pub fn number_of_heroes(hero: impl IntoExpr<Hero>) -> Expr<Num> {
    number_of_heroes_with(hero, GetNumberOfHeroesArgs::default())
}
/// `Number Of Heroes`.
#[track_caller]
#[must_use]
pub fn number_of_heroes_with(hero: impl IntoExpr<Hero>, opts: GetNumberOfHeroesArgs) -> Expr<Num> {
    support::value(
        ValueId::GetNumberOfHeroes,
        vec![hero.into_expr().into_ir(), opts.team.into_ir()],
    )
}
/// `Number Of Living Players`.
#[track_caller]
#[must_use]
pub fn number_of_living_players() -> Expr<Num> {
    number_of_living_players_with(GetNumberOfLivingPlayersArgs::default())
}
/// `Number Of Living Players`.
#[track_caller]
#[must_use]
pub fn number_of_living_players_with(opts: GetNumberOfLivingPlayersArgs) -> Expr<Num> {
    support::value(ValueId::GetNumberOfLivingPlayers, vec![opts.team.into_ir()])
}
/// `Number Of Players`.
#[track_caller]
#[must_use]
pub fn number_of_players() -> Expr<Num> {
    number_of_players_with(GetNumberOfPlayersArgs::default())
}
/// `Number Of Players`.
#[track_caller]
#[must_use]
pub fn number_of_players_with(opts: GetNumberOfPlayersArgs) -> Expr<Num> {
    support::value(ValueId::GetNumberOfPlayers, vec![opts.team.into_ir()])
}
/// `Number Of Players On Objective`.
#[track_caller]
#[must_use]
pub fn number_of_players_on_objective() -> Expr<Num> {
    number_of_players_on_objective_with(GetNumberOfPlayersOnObjectiveArgs::default())
}
/// `Number Of Players On Objective`.
#[track_caller]
#[must_use]
pub fn number_of_players_on_objective_with(opts: GetNumberOfPlayersOnObjectiveArgs) -> Expr<Num> {
    support::value(
        ValueId::GetNumberOfPlayersOnObjective,
        vec![opts.team.into_ir()],
    )
}
/// `Number Of Slots`.
#[track_caller]
#[must_use]
pub fn number_of_slots() -> Expr<Num> {
    number_of_slots_with(GetNumberOfSlotsArgs::default())
}
/// `Number Of Slots`.
#[track_caller]
#[must_use]
pub fn number_of_slots_with(opts: GetNumberOfSlotsArgs) -> Expr<Num> {
    support::value(ValueId::GetNumberOfSlots, vec![opts.team.into_ir()])
}
/// `Text Count`.
#[track_caller]
#[must_use]
pub fn number_of_text_ids() -> Expr<Num> {
    support::value(ValueId::GetNumberOfTextIds, vec![])
}
/// `Objective Position`.
#[track_caller]
#[must_use]
pub fn objective_position(number: impl IntoExpr<Num>) -> Expr<Position> {
    support::value(
        ValueId::GetObjectivePosition,
        vec![number.into_expr().into_ir()],
    )
}
/// `Opposite Team Of`.
#[track_caller]
#[must_use]
pub fn opposite_team(team: impl IntoExpr<Team>) -> Expr<Team> {
    support::value(ValueId::GetOppositeTeam, vec![team.into_expr().into_ir()])
}
/// `Payload Position`.
#[track_caller]
#[must_use]
pub fn payload_position() -> Expr<Position> {
    support::value(ValueId::GetPayloadPosition, vec![])
}
/// `Payload Progress Percentage`.
#[track_caller]
#[must_use]
pub fn payload_progress_percentage() -> Expr<Num> {
    support::value(ValueId::GetPayloadProgressPercentage, vec![])
}
/// `Server Load Peak`.
#[track_caller]
#[must_use]
pub fn peak_server_load() -> Expr<Num> {
    support::value(ValueId::GetPeakServerLoad, vec![])
}
/// `All Players`.
#[track_caller]
#[must_use]
pub fn all_players() -> Expr<Array<Player>> {
    all_players_with(GetPlayersArgs::default())
}
/// `All Players`.
#[track_caller]
#[must_use]
pub fn all_players_with(opts: GetPlayersArgs) -> Expr<Array<Player>> {
    support::value(ValueId::GetPlayers, vec![opts.team.into_ir()])
}
/// `Players Within Radius`.
#[track_caller]
#[must_use]
pub fn players_in_radius(
    center: impl IntoExpr<Position>,
    radius: impl IntoExpr<Num>,
) -> Expr<Array<Player>> {
    players_in_radius_with(center, radius, GetPlayersInRadiusArgs::default())
}
/// `Players Within Radius`.
#[track_caller]
#[must_use]
pub fn players_in_radius_with(
    center: impl IntoExpr<Position>,
    radius: impl IntoExpr<Num>,
    opts: GetPlayersInRadiusArgs,
) -> Expr<Array<Player>> {
    support::value(
        ValueId::GetPlayersInRadius,
        vec![
            center.into_expr().into_ir(),
            radius.into_expr().into_ir(),
            opts.team.into_ir(),
            opts.los_check.into_ir(),
        ],
    )
}
/// `Players In Slot`.
#[track_caller]
#[must_use]
pub fn players_in_slot(slot: impl IntoExpr<Num>) -> Expr<Any> {
    players_in_slot_with(slot, GetPlayersInSlotArgs::default())
}
/// `Players In Slot`.
#[track_caller]
#[must_use]
pub fn players_in_slot_with(slot: impl IntoExpr<Num>, opts: GetPlayersInSlotArgs) -> Expr<Any> {
    support::value(
        ValueId::GetPlayersInSlot,
        vec![slot.into_expr().into_ir(), opts.team.into_ir()],
    )
}
/// `All Players Not On Objective`.
#[track_caller]
#[must_use]
pub fn players_not_on_objective() -> Expr<Array<Player>> {
    players_not_on_objective_with(GetPlayersNotOnObjectiveArgs::default())
}
/// `All Players Not On Objective`.
#[track_caller]
#[must_use]
pub fn players_not_on_objective_with(opts: GetPlayersNotOnObjectiveArgs) -> Expr<Array<Player>> {
    support::value(ValueId::GetPlayersNotOnObjective, vec![opts.team.into_ir()])
}
/// `Players On Hero`.
#[track_caller]
#[must_use]
pub fn players_on_hero(hero: impl IntoExpr<Hero>) -> Expr<Array<Player>> {
    players_on_hero_with(hero, GetPlayersOnHeroArgs::default())
}
/// `Players On Hero`.
#[track_caller]
#[must_use]
pub fn players_on_hero_with(
    hero: impl IntoExpr<Hero>,
    opts: GetPlayersOnHeroArgs,
) -> Expr<Array<Player>> {
    support::value(
        ValueId::GetPlayersOnHero,
        vec![hero.into_expr().into_ir(), opts.team.into_ir()],
    )
}
/// `All Players On Objective`.
#[track_caller]
#[must_use]
pub fn players_on_objective() -> Expr<Array<Player>> {
    players_on_objective_with(GetPlayersOnObjectiveArgs::default())
}
/// `All Players On Objective`.
#[track_caller]
#[must_use]
pub fn players_on_objective_with(opts: GetPlayersOnObjectiveArgs) -> Expr<Array<Player>> {
    support::value(ValueId::GetPlayersOnObjective, vec![opts.team.into_ir()])
}
/// `Server Load`.
#[track_caller]
#[must_use]
pub fn server_load() -> Expr<Num> {
    support::value(ValueId::GetServerLoad, vec![])
}
/// `Spawn Points`.
#[track_caller]
#[must_use]
pub fn spawn_points(team: impl IntoExpr<Team>) -> Expr<Array<Position>> {
    support::value(ValueId::GetSpawnPoints, vec![team.into_expr().into_ir()])
}
/// `All Support Heroes`.
#[track_caller]
#[must_use]
pub fn support_heroes() -> Expr<Array<Hero>> {
    support::value(ValueId::GetSupportHeroes, vec![])
}
/// `All Tank Heroes`.
#[track_caller]
#[must_use]
pub fn tank_heroes() -> Expr<Array<Hero>> {
    support::value(ValueId::GetTankHeroes, vec![])
}
/// `Total Time Elapsed`.
#[track_caller]
#[must_use]
pub fn total_time_elapsed() -> Expr<Num> {
    support::value(ValueId::GetTotalTimeElapsed, vec![])
}
/// `Hero Icon String`.
#[track_caller]
#[must_use]
pub fn hero_icon(value: impl IntoExpr<Hero>) -> Expr<Str> {
    support::value(ValueId::HeroIcon, vec![value.into_expr().into_ir()])
}
/// `Horizontal Angle From Direction`.
#[track_caller]
#[must_use]
pub fn horizontal_angle_of_direction(direction: impl IntoExpr<Direction>) -> Expr<Num> {
    support::value(
        ValueId::HorizontalAngleOfDirection,
        vec![direction.into_expr().into_ir()],
    )
}
/// `Horizontal Angle Towards`.
#[track_caller]
#[must_use]
pub fn horizontal_angle_towards(
    player: impl IntoExpr<Player>,
    position: impl IntoExpr<Position>,
) -> Expr<Num> {
    support::value(
        ValueId::HorizontalAngleTowards,
        vec![player.into_expr().into_ir(), position.into_expr().into_ir()],
    )
}
/// `Host Player`.
#[track_caller]
#[must_use]
pub fn host_player() -> Expr<Player> {
    support::value(ValueId::HostPlayer, vec![])
}
/// `Icon String`.
#[track_caller]
#[must_use]
pub fn icon_string(icon: impl IntoExpr<Icon>) -> Expr<Str> {
    support::value(ValueId::IconString, vec![icon.into_expr().into_ir()])
}
/// `Input Binding String`.
#[track_caller]
#[must_use]
pub fn input_binding_string(button: impl IntoExpr<Button>) -> Expr<Str> {
    support::value(
        ValueId::InputBindingString,
        vec![button.into_expr().into_ir()],
    )
}
/// `Is Assembling Heroes`.
#[track_caller]
#[must_use]
pub fn is_assembling_heroes() -> Expr<Bool> {
    support::value(ValueId::IsAssemblingHeroes, vec![])
}
/// `Is Control Mode Point Locked`.
#[track_caller]
#[must_use]
pub fn is_control_point_locked() -> Expr<Bool> {
    support::value(ValueId::IsControlPointLocked, vec![])
}
/// `Is Flag At Base`.
#[track_caller]
#[must_use]
pub fn is_flag_at_base(team: impl IntoExpr<Team>) -> Expr<Bool> {
    support::value(ValueId::IsFlagAtBase, vec![team.into_expr().into_ir()])
}
/// `Is Flag Being Carried`.
#[track_caller]
#[must_use]
pub fn is_flag_being_carried(team: impl IntoExpr<Team>) -> Expr<Bool> {
    support::value(
        ValueId::IsFlagBeingCarried,
        vec![team.into_expr().into_ir()],
    )
}
/// `Is Game In Progress`.
#[track_caller]
#[must_use]
pub fn is_game_in_progress() -> Expr<Bool> {
    support::value(ValueId::IsGameInProgress, vec![])
}
/// `Is Hero Being Played`.
#[track_caller]
#[must_use]
pub fn is_hero_being_played(hero: impl IntoExpr<Hero>, team: impl IntoExpr<Team>) -> Expr<Bool> {
    support::value(
        ValueId::IsHeroBeingPlayed,
        vec![hero.into_expr().into_ir(), team.into_expr().into_ir()],
    )
}
/// `Is In Line of Sight`.
#[track_caller]
#[must_use]
pub fn is_in_los(
    start_pos: impl IntoExpr<PlayerOrPosition>,
    end_pos: impl IntoExpr<PlayerOrPosition>,
) -> Expr<Bool> {
    is_in_los_with(start_pos, end_pos, IsInLoSArgs::default())
}
/// `Is In Line of Sight`.
#[track_caller]
#[must_use]
pub fn is_in_los_with(
    start_pos: impl IntoExpr<PlayerOrPosition>,
    end_pos: impl IntoExpr<PlayerOrPosition>,
    opts: IsInLoSArgs,
) -> Expr<Bool> {
    support::value(
        ValueId::IsInLoS,
        vec![
            start_pos.into_expr().into_ir(),
            end_pos.into_expr().into_ir(),
            opts.barriers.into_ir(),
        ],
    )
}
/// `Is In Setup`.
#[track_caller]
#[must_use]
pub fn is_in_setup() -> Expr<Bool> {
    support::value(ValueId::IsInSetup, vec![])
}
/// `Is CTF Mode In Sudden Death`.
#[track_caller]
#[must_use]
pub fn is_in_sudden_death() -> Expr<Bool> {
    support::value(ValueId::IsInSuddenDeath, vec![])
}
/// `Is Between Rounds`.
#[track_caller]
#[must_use]
pub fn is_match_between_rounds() -> Expr<Bool> {
    support::value(ValueId::IsMatchBetweenRounds, vec![])
}
/// `Is Match Complete`.
#[track_caller]
#[must_use]
pub fn is_match_complete() -> Expr<Bool> {
    support::value(ValueId::IsMatchComplete, vec![])
}
/// `Is Objective Complete`.
#[track_caller]
#[must_use]
pub fn is_objective_complete(number: impl IntoExpr<Num>) -> Expr<Bool> {
    support::value(
        ValueId::IsObjectiveComplete,
        vec![number.into_expr().into_ir()],
    )
}
/// `Is Team On Defense`.
#[track_caller]
#[must_use]
pub fn is_team_on_defense(team: impl IntoExpr<Team>) -> Expr<Bool> {
    support::value(ValueId::IsTeamOnDefense, vec![team.into_expr().into_ir()])
}
/// `Is Team On Offense`.
#[track_caller]
#[must_use]
pub fn is_team_on_offense(team: impl IntoExpr<Team>) -> Expr<Bool> {
    support::value(ValueId::IsTeamOnOffense, vec![team.into_expr().into_ir()])
}
/// `Is Waiting For Players`.
#[track_caller]
#[must_use]
pub fn is_waiting_for_players() -> Expr<Bool> {
    support::value(ValueId::IsWaitingForPlayers, vec![])
}
/// `Count Of`.
#[track_caller]
#[must_use]
pub fn len(array: impl IntoExpr<Array<Any>>) -> Expr<Num> {
    support::value(ValueId::Len, vec![array.into_expr().into_ir()])
}
/// `Local Player`.
#[track_caller]
#[must_use]
pub fn local_player() -> Expr<Player> {
    support::value(ValueId::LocalPlayer, vec![])
}
/// `Local Vector Of`.
#[track_caller]
#[must_use]
pub fn local_vector(
    world_vector: impl IntoExpr<Vector>,
    relative_player: impl IntoExpr<Player>,
    transformation: impl IntoExpr<Transform>,
) -> Expr<Vector> {
    support::value(
        ValueId::LocalVector,
        vec![
            world_vector.into_expr().into_ir(),
            relative_player.into_expr().into_ir(),
            transformation.into_expr().into_ir(),
        ],
    )
}
/// `Magnitude Of`.
#[track_caller]
#[must_use]
pub fn magnitude(vector: impl IntoExpr<Vector>) -> Expr<Num> {
    support::value(ValueId::Magnitude, vec![vector.into_expr().into_ir()])
}
/// `Max`.
#[track_caller]
#[must_use]
pub fn max(value_1: impl IntoExpr<Num>, value_2: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(
        ValueId::Max,
        vec![value_1.into_expr().into_ir(), value_2.into_expr().into_ir()],
    )
}
/// `Min`.
#[track_caller]
#[must_use]
pub fn min(value_1: impl IntoExpr<Num>, value_2: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(
        ValueId::Min,
        vec![value_1.into_expr().into_ir(), value_2.into_expr().into_ir()],
    )
}
/// `Nearest Walkable Position`.
#[track_caller]
#[must_use]
pub fn nearest_walkable_position(position: impl IntoExpr<Position>) -> Expr<Position> {
    support::value(
        ValueId::NearestWalkablePosition,
        vec![position.into_expr().into_ir()],
    )
}
/// `Normalize`.
#[track_caller]
#[must_use]
pub fn normalize(vector: impl IntoExpr<Vector>) -> Expr<Vector> {
    support::value(ValueId::Normalize, vec![vector.into_expr().into_ir()])
}
/// `Custom Color`.
#[track_caller]
#[must_use]
pub fn rgb(
    red: impl IntoExpr<Num>,
    green: impl IntoExpr<Num>,
    blue: impl IntoExpr<Num>,
) -> Expr<Color> {
    rgb_with(red, green, blue, RgbArgs::default())
}
/// `Custom Color`.
#[track_caller]
#[must_use]
pub fn rgb_with(
    red: impl IntoExpr<Num>,
    green: impl IntoExpr<Num>,
    blue: impl IntoExpr<Num>,
    opts: RgbArgs,
) -> Expr<Color> {
    support::value(
        ValueId::Rgb,
        vec![
            red.into_expr().into_ir(),
            green.into_expr().into_ir(),
            blue.into_expr().into_ir(),
            opts.alpha.into_ir(),
        ],
    )
}
/// `Sine From Radians`.
#[track_caller]
#[must_use]
pub fn sin(angle: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(ValueId::Sin, vec![angle.into_expr().into_ir()])
}
/// `Sine From Degrees`.
#[track_caller]
#[must_use]
pub fn sin_deg(angle: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(ValueId::SinDeg, vec![angle.into_expr().into_ir()])
}
/// `Square Root`.
#[track_caller]
#[must_use]
pub fn sqrt(value: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(ValueId::Sqrt, vec![value.into_expr().into_ir()])
}
/// `String Contains`.
#[track_caller]
#[must_use]
pub fn str_contains(string: impl IntoExpr<Str>, substring: impl IntoExpr<Str>) -> Expr<Bool> {
    support::value(
        ValueId::StrContains,
        vec![
            string.into_expr().into_ir(),
            substring.into_expr().into_ir(),
        ],
    )
}
/// `String Length`.
#[track_caller]
#[must_use]
pub fn str_len(string: impl IntoExpr<Str>) -> Expr<Num> {
    support::value(ValueId::StrLen, vec![string.into_expr().into_ir()])
}
/// `Tangent From Radians`.
#[track_caller]
#[must_use]
pub fn tan(angle: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(ValueId::Tan, vec![angle.into_expr().into_ir()])
}
/// `Tangent From Degrees`.
#[track_caller]
#[must_use]
pub fn tan_deg(angle: impl IntoExpr<Num>) -> Expr<Num> {
    support::value(ValueId::TanDeg, vec![angle.into_expr().into_ir()])
}
/// `Team Score`.
#[track_caller]
#[must_use]
pub fn team_score(team: impl IntoExpr<Team>) -> Expr<Num> {
    support::value(ValueId::TeamScore, vec![team.into_expr().into_ir()])
}
/// `Update Every Frame`.
#[track_caller]
#[must_use]
pub fn update_every_frame(value: impl IntoExpr<Any>) -> Expr<Any> {
    support::value(ValueId::UpdateEveryFrame, vec![value.into_expr().into_ir()])
}
/// `Vector Towards`.
#[track_caller]
#[must_use]
pub fn vector_towards(
    start_pos: impl IntoExpr<Position>,
    end_pos: impl IntoExpr<Position>,
) -> Expr<Direction> {
    support::value(
        ValueId::VectorTowards,
        vec![
            start_pos.into_expr().into_ir(),
            end_pos.into_expr().into_ir(),
        ],
    )
}
/// `Vertical Angle From Direction`.
#[track_caller]
#[must_use]
pub fn vertical_angle_of_direction(direction: impl IntoExpr<Direction>) -> Expr<Num> {
    support::value(
        ValueId::VerticalAngleOfDirection,
        vec![direction.into_expr().into_ir()],
    )
}
/// `Vertical Angle Towards`.
#[track_caller]
#[must_use]
pub fn vertical_angle_towards(
    player: impl IntoExpr<Player>,
    position: impl IntoExpr<Position>,
) -> Expr<Num> {
    support::value(
        ValueId::VerticalAngleTowards,
        vec![player.into_expr().into_ir(), position.into_expr().into_ir()],
    )
}
/// `World Vector Of`.
#[track_caller]
#[must_use]
pub fn world_vector(
    local_vector: impl IntoExpr<Vector>,
    relative_player: impl IntoExpr<Player>,
    transformation: impl IntoExpr<Transform>,
) -> Expr<Vector> {
    support::value(
        ValueId::WorldVector,
        vec![
            local_vector.into_expr().into_ir(),
            relative_player.into_expr().into_ir(),
            transformation.into_expr().into_ir(),
        ],
    )
}
/// Functions of the `random` group.
pub mod random {
    use super::*;
    /// `Random Value In Array`.
    #[track_caller]
    #[must_use]
    pub fn choice(array: impl IntoExpr<Array<Any>>) -> Expr<Any> {
        support::value(ValueId::RandomChoice, vec![array.into_expr().into_ir()])
    }
    /// `Random Integer`.
    #[track_caller]
    #[must_use]
    pub fn int(min: impl IntoExpr<Num>, max: impl IntoExpr<Num>) -> Expr<Num> {
        support::value(
            ValueId::RandomRandint,
            vec![min.into_expr().into_ir(), max.into_expr().into_ir()],
        )
    }
    /// `Randomized Array`.
    #[track_caller]
    #[must_use]
    pub fn shuffle(array: impl IntoExpr<Array<Any>>) -> Expr<Array<Any>> {
        support::value(ValueId::RandomShuffle, vec![array.into_expr().into_ir()])
    }
    /// `Random Real`.
    #[track_caller]
    #[must_use]
    pub fn real(min: impl IntoExpr<Num>, max: impl IntoExpr<Num>) -> Expr<Num> {
        support::value(
            ValueId::RandomUniform,
            vec![min.into_expr().into_ir(), max.into_expr().into_ir()],
        )
    }
}
/// Values that take a `Player` as their first argument.
pub trait PlayerExt {
    /// `Ability Charge`.
    #[track_caller]
    fn ability_charge(&self, button: impl IntoExpr<Button>) -> Expr<Num>;
    /// `Ability Cooldown`.
    #[track_caller]
    fn ability_cooldown(&self, button: impl IntoExpr<Button>) -> Expr<Num>;
    /// `Ability Resource`.
    #[track_caller]
    fn ability_resource(&self, button: impl IntoExpr<Button>) -> Expr<Num>;
    /// `Allowed Heroes`.
    #[track_caller]
    fn allowed_heroes(&self) -> Expr<Array<Hero>>;
    /// `Altitude Of`.
    #[track_caller]
    fn altitude(&self) -> Expr<Num>;
    /// `Ammo`.
    #[track_caller]
    fn ammo(&self) -> Expr<Num> {
        self.ammo_with(GetAmmoArgs::default())
    }
    /// `Ammo`.
    #[track_caller]
    fn ammo_with(&self, opts: GetAmmoArgs) -> Expr<Num>;
    /// `Weapon`.
    #[track_caller]
    fn current_weapon(&self) -> Expr<Num>;
    /// `Eye Position`.
    #[track_caller]
    fn eye_position(&self) -> Expr<Position>;
    /// `Facing Direction Of`.
    #[track_caller]
    fn facing_direction(&self) -> Expr<Direction>;
    /// `Health`.
    #[track_caller]
    fn health(&self) -> Expr<Num>;
    /// `Health Of Type`.
    #[track_caller]
    fn health_of_type(&self, health: impl IntoExpr<Health>) -> Expr<Num>;
    /// `Hero Of`.
    #[track_caller]
    fn hero(&self) -> Expr<Hero>;
    /// `Hero Being Duplicated`.
    #[track_caller]
    fn hero_of_duplication(&self) -> Expr<Hero>;
    /// `Player Hero Stat`.
    #[track_caller]
    fn hero_statistic(&self, hero: impl IntoExpr<Hero>, stat: impl IntoExpr<HeroStat>)
    -> Expr<Num>;
    /// `Horizontal Facing Angle Of`.
    #[track_caller]
    fn horizontal_facing_angle(&self) -> Expr<Num>;
    /// `Horizontal Speed Of`.
    #[track_caller]
    fn horizontal_speed(&self) -> Expr<Num>;
    /// `Max Ammo`.
    #[track_caller]
    fn max_ammo(&self) -> Expr<Num> {
        self.max_ammo_with(GetMaxAmmoArgs::default())
    }
    /// `Max Ammo`.
    #[track_caller]
    fn max_ammo_with(&self, opts: GetMaxAmmoArgs) -> Expr<Num>;
    /// `Max Health`.
    #[track_caller]
    fn max_health(&self) -> Expr<Num>;
    /// `Max Health Of Type`.
    #[track_caller]
    fn max_health_of_type(&self, health: impl IntoExpr<Health>) -> Expr<Num>;
    /// `Normalized Health`.
    #[track_caller]
    fn normalized_health(&self) -> Expr<Num>;
    /// `Number Of Deaths`.
    #[track_caller]
    fn number_of_deaths(&self) -> Expr<Num>;
    /// `Number Of Eliminations`.
    #[track_caller]
    fn number_of_elims(&self) -> Expr<Num>;
    /// `Number Of Final Blows`.
    #[track_caller]
    fn number_of_final_blows(&self) -> Expr<Num>;
    /// `Player Closest To Reticle`.
    #[track_caller]
    fn player_closest_to_reticle(&self, team: impl IntoExpr<Team>) -> Expr<Player>;
    /// `Players in View Angle`.
    #[track_caller]
    fn players_in_view_angle(
        &self,
        team: impl IntoExpr<Team>,
        view_angle: impl IntoExpr<Num>,
    ) -> Expr<Array<Player>>;
    /// `Position Of`.
    #[track_caller]
    fn position(&self) -> Expr<Position>;
    /// `Score Of`.
    #[track_caller]
    fn score(&self) -> Expr<Num>;
    /// `Slot Of`.
    #[track_caller]
    fn slot(&self) -> Expr<Num>;
    /// `Speed Of`.
    #[track_caller]
    fn speed(&self) -> Expr<Num>;
    /// `Speed Of In Direction`.
    #[track_caller]
    fn speed_in_direction(&self, direction: impl IntoExpr<Direction>) -> Expr<Num>;
    /// `Player Stat`.
    #[track_caller]
    fn statistic(&self, statistic: impl IntoExpr<Stat>) -> Expr<Num>;
    /// `Team Of`.
    #[track_caller]
    fn team(&self) -> Expr<Team>;
    /// `Throttle Of`.
    #[track_caller]
    fn throttle(&self) -> Expr<Direction>;
    /// `Ultimate Charge Percent`.
    #[track_caller]
    fn ult_charge(&self) -> Expr<Num>;
    /// `Velocity Of`.
    #[track_caller]
    fn velocity(&self) -> Expr<Velocity>;
    /// `Vertical Facing Angle Of`.
    #[track_caller]
    fn vertical_facing_angle(&self) -> Expr<Num>;
    /// `Vertical Speed Of`.
    #[track_caller]
    fn vertical_speed(&self) -> Expr<Num>;
    /// `Has Spawned`.
    #[track_caller]
    fn has_spawned(&self) -> Expr<Bool>;
    /// `Has Status`.
    #[track_caller]
    fn has_status(&self, status: impl IntoExpr<Status>) -> Expr<Bool>;
    /// `Is Alive`.
    #[track_caller]
    fn is_alive(&self) -> Expr<Bool>;
    /// `Is Communicating`.
    #[track_caller]
    fn is_communicating(&self, type_: impl IntoExpr<Comms>) -> Expr<Bool>;
    /// `Is Communicating Any`.
    #[track_caller]
    fn is_communicating_anything(&self) -> Expr<Bool>;
    /// `Is Communicating Any Emote`.
    #[track_caller]
    fn is_communicating_emote(&self) -> Expr<Bool>;
    /// `Is Communicating Any Spray`.
    #[track_caller]
    fn is_communicating_spray(&self) -> Expr<Bool>;
    /// `Is Communicating Any Voice line`.
    #[track_caller]
    fn is_communicating_voiceline(&self) -> Expr<Bool>;
    /// `Is Crouching`.
    #[track_caller]
    fn is_crouching(&self) -> Expr<Bool>;
    /// `Is Dead`.
    #[track_caller]
    fn is_dead(&self) -> Expr<Bool>;
    /// `Is Dummy Bot`.
    #[track_caller]
    fn is_dummy(&self) -> Expr<Bool>;
    /// `Is Duplicating`.
    #[track_caller]
    fn is_duplicating_a_hero(&self) -> Expr<Bool>;
    /// `Is Firing Primary`.
    #[track_caller]
    fn is_firing_primary_fire(&self) -> Expr<Bool>;
    /// `Is Firing Secondary`.
    #[track_caller]
    fn is_firing_secondary_fire(&self) -> Expr<Bool>;
    /// `Is Button Held`.
    #[track_caller]
    fn is_holding_button(&self, button: impl IntoExpr<Button>) -> Expr<Bool>;
    /// `Is In Air`.
    #[track_caller]
    fn is_in_air(&self) -> Expr<Bool>;
    /// `Is In Alternate Form`.
    #[track_caller]
    fn is_in_alternate_form(&self) -> Expr<Bool>;
    /// `Is In Spawn Room`.
    #[track_caller]
    fn is_in_spawn_room(&self) -> Expr<Bool>;
    /// `Is In View Angle`.
    #[track_caller]
    fn is_in_view_angle(
        &self,
        location: impl IntoExpr<Position>,
        view_angle: impl IntoExpr<Num>,
    ) -> Expr<Bool>;
    /// `Is Jumping`.
    #[track_caller]
    fn is_jumping(&self) -> Expr<Bool>;
    /// `Is Meleeing`.
    #[track_caller]
    fn is_meleeing(&self) -> Expr<Bool>;
    /// `Is Moving`.
    #[track_caller]
    fn is_moving(&self) -> Expr<Bool>;
    /// `Is Portrait On Fire`.
    #[track_caller]
    fn is_on_fire(&self) -> Expr<Bool>;
    /// `Is On Ground`.
    #[track_caller]
    fn is_on_ground(&self) -> Expr<Bool>;
    /// `Is On Objective`.
    #[track_caller]
    fn is_on_objective(&self) -> Expr<Bool>;
    /// `Is On Wall`.
    #[track_caller]
    fn is_on_wall(&self) -> Expr<Bool>;
    /// `Is Reloading`.
    #[track_caller]
    fn is_reloading(&self) -> Expr<Bool>;
    /// `Is Standing`.
    #[track_caller]
    fn is_standing(&self) -> Expr<Bool>;
    /// `Is Using Ability 1`.
    #[track_caller]
    fn is_using_ability1(&self) -> Expr<Bool>;
    /// `Is Using Ability 2`.
    #[track_caller]
    fn is_using_ability2(&self) -> Expr<Bool>;
    /// `Is Using Ultimate`.
    #[track_caller]
    fn is_using_ultimate(&self) -> Expr<Bool>;
}
impl<T: Is<Player>> PlayerExt for Expr<T> {
    #[track_caller]
    fn ability_charge(&self, button: impl IntoExpr<Button>) -> Expr<Num> {
        support::value(
            ValueId::GetAbilityCharge,
            vec![self.ir().clone(), button.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn ability_cooldown(&self, button: impl IntoExpr<Button>) -> Expr<Num> {
        support::value(
            ValueId::GetAbilityCooldown,
            vec![self.ir().clone(), button.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn ability_resource(&self, button: impl IntoExpr<Button>) -> Expr<Num> {
        support::value(
            ValueId::GetAbilityResource,
            vec![self.ir().clone(), button.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn allowed_heroes(&self) -> Expr<Array<Hero>> {
        support::value(ValueId::GetAllowedHeroes, vec![self.ir().clone()])
    }
    #[track_caller]
    fn altitude(&self) -> Expr<Num> {
        support::value(ValueId::GetAltitude, vec![self.ir().clone()])
    }
    #[track_caller]
    fn ammo_with(&self, opts: GetAmmoArgs) -> Expr<Num> {
        support::value(
            ValueId::GetAmmo,
            vec![self.ir().clone(), opts.clip.into_ir()],
        )
    }
    #[track_caller]
    fn current_weapon(&self) -> Expr<Num> {
        support::value(ValueId::GetCurrentWeapon, vec![self.ir().clone()])
    }
    #[track_caller]
    fn eye_position(&self) -> Expr<Position> {
        support::value(ValueId::GetEyePosition, vec![self.ir().clone()])
    }
    #[track_caller]
    fn facing_direction(&self) -> Expr<Direction> {
        support::value(ValueId::GetFacingDirection, vec![self.ir().clone()])
    }
    #[track_caller]
    fn health(&self) -> Expr<Num> {
        support::value(ValueId::GetHealth, vec![self.ir().clone()])
    }
    #[track_caller]
    fn health_of_type(&self, health: impl IntoExpr<Health>) -> Expr<Num> {
        support::value(
            ValueId::GetHealthOfType,
            vec![self.ir().clone(), health.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn hero(&self) -> Expr<Hero> {
        support::value(ValueId::GetHero, vec![self.ir().clone()])
    }
    #[track_caller]
    fn hero_of_duplication(&self) -> Expr<Hero> {
        support::value(ValueId::GetHeroOfDuplication, vec![self.ir().clone()])
    }
    #[track_caller]
    fn hero_statistic(
        &self,
        hero: impl IntoExpr<Hero>,
        stat: impl IntoExpr<HeroStat>,
    ) -> Expr<Num> {
        support::value(
            ValueId::GetHeroStatistic,
            vec![
                self.ir().clone(),
                hero.into_expr().into_ir(),
                stat.into_expr().into_ir(),
            ],
        )
    }
    #[track_caller]
    fn horizontal_facing_angle(&self) -> Expr<Num> {
        support::value(ValueId::GetHorizontalFacingAngle, vec![self.ir().clone()])
    }
    #[track_caller]
    fn horizontal_speed(&self) -> Expr<Num> {
        support::value(ValueId::GetHorizontalSpeed, vec![self.ir().clone()])
    }
    #[track_caller]
    fn max_ammo_with(&self, opts: GetMaxAmmoArgs) -> Expr<Num> {
        support::value(
            ValueId::GetMaxAmmo,
            vec![self.ir().clone(), opts.clip.into_ir()],
        )
    }
    #[track_caller]
    fn max_health(&self) -> Expr<Num> {
        support::value(ValueId::GetMaxHealth, vec![self.ir().clone()])
    }
    #[track_caller]
    fn max_health_of_type(&self, health: impl IntoExpr<Health>) -> Expr<Num> {
        support::value(
            ValueId::GetMaxHealthOfType,
            vec![self.ir().clone(), health.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn normalized_health(&self) -> Expr<Num> {
        support::value(ValueId::GetNormalizedHealth, vec![self.ir().clone()])
    }
    #[track_caller]
    fn number_of_deaths(&self) -> Expr<Num> {
        support::value(ValueId::GetNumberOfDeaths, vec![self.ir().clone()])
    }
    #[track_caller]
    fn number_of_elims(&self) -> Expr<Num> {
        support::value(ValueId::GetNumberOfElims, vec![self.ir().clone()])
    }
    #[track_caller]
    fn number_of_final_blows(&self) -> Expr<Num> {
        support::value(ValueId::GetNumberOfFinalBlows, vec![self.ir().clone()])
    }
    #[track_caller]
    fn player_closest_to_reticle(&self, team: impl IntoExpr<Team>) -> Expr<Player> {
        support::value(
            ValueId::GetPlayerClosestToReticle,
            vec![self.ir().clone(), team.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn players_in_view_angle(
        &self,
        team: impl IntoExpr<Team>,
        view_angle: impl IntoExpr<Num>,
    ) -> Expr<Array<Player>> {
        support::value(
            ValueId::GetPlayersInViewAngle,
            vec![
                self.ir().clone(),
                team.into_expr().into_ir(),
                view_angle.into_expr().into_ir(),
            ],
        )
    }
    #[track_caller]
    fn position(&self) -> Expr<Position> {
        support::value(ValueId::GetPosition, vec![self.ir().clone()])
    }
    #[track_caller]
    fn score(&self) -> Expr<Num> {
        support::value(ValueId::GetScore, vec![self.ir().clone()])
    }
    #[track_caller]
    fn slot(&self) -> Expr<Num> {
        support::value(ValueId::GetSlot, vec![self.ir().clone()])
    }
    #[track_caller]
    fn speed(&self) -> Expr<Num> {
        support::value(ValueId::GetSpeed, vec![self.ir().clone()])
    }
    #[track_caller]
    fn speed_in_direction(&self, direction: impl IntoExpr<Direction>) -> Expr<Num> {
        support::value(
            ValueId::GetSpeedInDirection,
            vec![self.ir().clone(), direction.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn statistic(&self, statistic: impl IntoExpr<Stat>) -> Expr<Num> {
        support::value(
            ValueId::GetStatistic,
            vec![self.ir().clone(), statistic.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn team(&self) -> Expr<Team> {
        support::value(ValueId::GetTeam, vec![self.ir().clone()])
    }
    #[track_caller]
    fn throttle(&self) -> Expr<Direction> {
        support::value(ValueId::GetThrottle, vec![self.ir().clone()])
    }
    #[track_caller]
    fn ult_charge(&self) -> Expr<Num> {
        support::value(ValueId::GetUltCharge, vec![self.ir().clone()])
    }
    #[track_caller]
    fn velocity(&self) -> Expr<Velocity> {
        support::value(ValueId::GetVelocity, vec![self.ir().clone()])
    }
    #[track_caller]
    fn vertical_facing_angle(&self) -> Expr<Num> {
        support::value(ValueId::GetVerticalFacingAngle, vec![self.ir().clone()])
    }
    #[track_caller]
    fn vertical_speed(&self) -> Expr<Num> {
        support::value(ValueId::GetVerticalSpeed, vec![self.ir().clone()])
    }
    #[track_caller]
    fn has_spawned(&self) -> Expr<Bool> {
        support::value(ValueId::HasSpawned, vec![self.ir().clone()])
    }
    #[track_caller]
    fn has_status(&self, status: impl IntoExpr<Status>) -> Expr<Bool> {
        support::value(
            ValueId::HasStatus,
            vec![self.ir().clone(), status.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn is_alive(&self) -> Expr<Bool> {
        support::value(ValueId::IsAlive, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_communicating(&self, type_: impl IntoExpr<Comms>) -> Expr<Bool> {
        support::value(
            ValueId::IsCommunicating,
            vec![self.ir().clone(), type_.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn is_communicating_anything(&self) -> Expr<Bool> {
        support::value(ValueId::IsCommunicatingAnything, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_communicating_emote(&self) -> Expr<Bool> {
        support::value(ValueId::IsCommunicatingEmote, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_communicating_spray(&self) -> Expr<Bool> {
        support::value(ValueId::IsCommunicatingSpray, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_communicating_voiceline(&self) -> Expr<Bool> {
        support::value(ValueId::IsCommunicatingVoiceline, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_crouching(&self) -> Expr<Bool> {
        support::value(ValueId::IsCrouching, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_dead(&self) -> Expr<Bool> {
        support::value(ValueId::IsDead, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_dummy(&self) -> Expr<Bool> {
        support::value(ValueId::IsDummy, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_duplicating_a_hero(&self) -> Expr<Bool> {
        support::value(ValueId::IsDuplicatingAHero, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_firing_primary_fire(&self) -> Expr<Bool> {
        support::value(ValueId::IsFiringPrimaryFire, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_firing_secondary_fire(&self) -> Expr<Bool> {
        support::value(ValueId::IsFiringSecondaryFire, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_holding_button(&self, button: impl IntoExpr<Button>) -> Expr<Bool> {
        support::value(
            ValueId::IsHoldingButton,
            vec![self.ir().clone(), button.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn is_in_air(&self) -> Expr<Bool> {
        support::value(ValueId::IsInAir, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_in_alternate_form(&self) -> Expr<Bool> {
        support::value(ValueId::IsInAlternateForm, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_in_spawn_room(&self) -> Expr<Bool> {
        support::value(ValueId::IsInSpawnRoom, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_in_view_angle(
        &self,
        location: impl IntoExpr<Position>,
        view_angle: impl IntoExpr<Num>,
    ) -> Expr<Bool> {
        support::value(
            ValueId::IsInViewAngle,
            vec![
                self.ir().clone(),
                location.into_expr().into_ir(),
                view_angle.into_expr().into_ir(),
            ],
        )
    }
    #[track_caller]
    fn is_jumping(&self) -> Expr<Bool> {
        support::value(ValueId::IsJumping, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_meleeing(&self) -> Expr<Bool> {
        support::value(ValueId::IsMeleeing, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_moving(&self) -> Expr<Bool> {
        support::value(ValueId::IsMoving, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_on_fire(&self) -> Expr<Bool> {
        support::value(ValueId::IsOnFire, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_on_ground(&self) -> Expr<Bool> {
        support::value(ValueId::IsOnGround, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_on_objective(&self) -> Expr<Bool> {
        support::value(ValueId::IsOnObjective, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_on_wall(&self) -> Expr<Bool> {
        support::value(ValueId::IsOnWall, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_reloading(&self) -> Expr<Bool> {
        support::value(ValueId::IsReloading, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_standing(&self) -> Expr<Bool> {
        support::value(ValueId::IsStanding, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_using_ability1(&self) -> Expr<Bool> {
        support::value(ValueId::IsUsingAbility1, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_using_ability2(&self) -> Expr<Bool> {
        support::value(ValueId::IsUsingAbility2, vec![self.ir().clone()])
    }
    #[track_caller]
    fn is_using_ultimate(&self) -> Expr<Bool> {
        support::value(ValueId::IsUsingUltimate, vec![self.ir().clone()])
    }
}
/// Values that take a `Str` as their first argument.
pub trait StrExt {
    /// `Char In String`.
    #[track_caller]
    fn char_at(&self, index: impl IntoExpr<Num>) -> Expr<Str>;
    /// `String Replace`.
    #[track_caller]
    fn replace(&self, pattern: impl IntoExpr<Str>, replacement: impl IntoExpr<Str>) -> Expr<Str>;
    /// `String Split`.
    #[track_caller]
    fn split(&self, separator: impl IntoExpr<Str>) -> Expr<Array<Str>>;
    /// `Index Of String Char`.
    #[track_caller]
    fn find(&self, character: impl IntoExpr<Str>) -> Expr<Num>;
    /// `String Slice`.
    #[track_caller]
    fn slice(&self, substring_start_index: impl IntoExpr<Num>) -> Expr<Str> {
        self.slice_with(substring_start_index, SubstringArgs::default())
    }
    /// `String Slice`.
    #[track_caller]
    fn slice_with(
        &self,
        substring_start_index: impl IntoExpr<Num>,
        opts: SubstringArgs,
    ) -> Expr<Str>;
}
impl<T: Is<Str>> StrExt for Expr<T> {
    #[track_caller]
    fn char_at(&self, index: impl IntoExpr<Num>) -> Expr<Str> {
        support::value(
            ValueId::CharAt,
            vec![self.ir().clone(), index.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn replace(&self, pattern: impl IntoExpr<Str>, replacement: impl IntoExpr<Str>) -> Expr<Str> {
        support::value(
            ValueId::Replace,
            vec![
                self.ir().clone(),
                pattern.into_expr().into_ir(),
                replacement.into_expr().into_ir(),
            ],
        )
    }
    #[track_caller]
    fn split(&self, separator: impl IntoExpr<Str>) -> Expr<Array<Str>> {
        support::value(
            ValueId::Split,
            vec![self.ir().clone(), separator.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn find(&self, character: impl IntoExpr<Str>) -> Expr<Num> {
        support::value(
            ValueId::StrIndex,
            vec![self.ir().clone(), character.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn slice_with(
        &self,
        substring_start_index: impl IntoExpr<Num>,
        opts: SubstringArgs,
    ) -> Expr<Str> {
        support::value(
            ValueId::Substring,
            vec![
                self.ir().clone(),
                substring_start_index.into_expr().into_ir(),
                opts.substring_length.into_ir(),
            ],
        )
    }
}
/// Values that take a `Array` as their first argument.
pub trait ArrayExt {
    /// `Append To Array`.
    #[track_caller]
    fn concat(&self, value: impl IntoExpr<Any>) -> Expr<Array<Any>>;
    /// `Remove From Array`.
    #[track_caller]
    fn exclude(&self, value: impl IntoExpr<Any>) -> Expr<Array<Any>>;
    /// `Index Of Array Value`.
    #[track_caller]
    fn index(&self, value: impl IntoExpr<Any>) -> Expr<Num>;
    /// `Last Of`.
    #[track_caller]
    fn last(&self) -> Expr<Any>;
    /// `Array Slice`.
    #[track_caller]
    fn slice(&self, start_index: impl IntoExpr<Num>) -> Expr<Array<Any>> {
        self.slice_with(start_index, SliceArgs::default())
    }
    /// `Array Slice`.
    #[track_caller]
    fn slice_with(&self, start_index: impl IntoExpr<Num>, opts: SliceArgs) -> Expr<Array<Any>>;
}
impl<T: Is<Array<Any>>> ArrayExt for Expr<T> {
    #[track_caller]
    fn concat(&self, value: impl IntoExpr<Any>) -> Expr<Array<Any>> {
        support::value(
            ValueId::Concat,
            vec![self.ir().clone(), value.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn exclude(&self, value: impl IntoExpr<Any>) -> Expr<Array<Any>> {
        support::value(
            ValueId::Exclude,
            vec![self.ir().clone(), value.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn index(&self, value: impl IntoExpr<Any>) -> Expr<Num> {
        support::value(
            ValueId::Index,
            vec![self.ir().clone(), value.into_expr().into_ir()],
        )
    }
    #[track_caller]
    fn last(&self) -> Expr<Any> {
        support::value(ValueId::Last, vec![self.ir().clone()])
    }
    #[track_caller]
    fn slice_with(&self, start_index: impl IntoExpr<Num>, opts: SliceArgs) -> Expr<Array<Any>> {
        support::value(
            ValueId::Slice,
            vec![
                self.ir().clone(),
                start_index.into_expr().into_ir(),
                opts.count.into_ir(),
            ],
        )
    }
}
/// Optional arguments of `Workshop Setting Toggle`.
#[derive(Clone, Debug)]
pub struct CreateWorkshopSettingBoolArgs {
    /// An optional argument.
    pub sort_order: Expr<Num>,
}
impl Default for CreateWorkshopSettingBoolArgs {
    fn default() -> Self {
        Self {
            sort_order: Expr::from_ir(support::num(0f64)),
        }
    }
}
/// Optional arguments of `Workshop Setting Combo`.
#[derive(Clone, Debug)]
pub struct CreateWorkshopSettingEnumArgs {
    /// An optional argument.
    pub sort_order: Expr<Num>,
}
impl Default for CreateWorkshopSettingEnumArgs {
    fn default() -> Self {
        Self {
            sort_order: Expr::from_ir(support::num(0f64)),
        }
    }
}
/// Optional arguments of `Workshop Setting Real`.
#[derive(Clone, Debug)]
pub struct CreateWorkshopSettingFloatArgs {
    /// An optional argument.
    pub sort_order: Expr<Num>,
}
impl Default for CreateWorkshopSettingFloatArgs {
    fn default() -> Self {
        Self {
            sort_order: Expr::from_ir(support::num(0f64)),
        }
    }
}
/// Optional arguments of `Workshop Setting Hero`.
#[derive(Clone, Debug)]
pub struct CreateWorkshopSettingHeroArgs {
    /// An optional argument.
    pub sort_order: Expr<Num>,
}
impl Default for CreateWorkshopSettingHeroArgs {
    fn default() -> Self {
        Self {
            sort_order: Expr::from_ir(support::num(0f64)),
        }
    }
}
/// Optional arguments of `Workshop Setting Integer`.
#[derive(Clone, Debug)]
pub struct CreateWorkshopSettingIntArgs {
    /// An optional argument.
    pub sort_order: Expr<Num>,
}
impl Default for CreateWorkshopSettingIntArgs {
    fn default() -> Self {
        Self {
            sort_order: Expr::from_ir(support::num(0f64)),
        }
    }
}
/// Optional arguments of `Ammo`.
#[derive(Clone, Debug)]
pub struct GetAmmoArgs {
    /// An optional argument.
    pub clip: Expr<Num>,
}
impl Default for GetAmmoArgs {
    fn default() -> Self {
        Self {
            clip: Expr::from_ir(support::num(0f64)),
        }
    }
}
/// Optional arguments of `All Dead Players`.
#[derive(Clone, Debug)]
pub struct GetDeadPlayersArgs {
    /// An optional argument.
    pub team: Expr<Team>,
}
impl Default for GetDeadPlayersArgs {
    fn default() -> Self {
        Self {
            team: Expr::from_ir(support::constant("Team", "ALL")),
        }
    }
}
/// Optional arguments of `All Living Players`.
#[derive(Clone, Debug)]
pub struct GetLivingPlayersArgs {
    /// An optional argument.
    pub team: Expr<Team>,
}
impl Default for GetLivingPlayersArgs {
    fn default() -> Self {
        Self {
            team: Expr::from_ir(support::constant("Team", "ALL")),
        }
    }
}
/// Optional arguments of `Max Ammo`.
#[derive(Clone, Debug)]
pub struct GetMaxAmmoArgs {
    /// An optional argument.
    pub clip: Expr<Num>,
}
impl Default for GetMaxAmmoArgs {
    fn default() -> Self {
        Self {
            clip: Expr::from_ir(support::num(0f64)),
        }
    }
}
/// Optional arguments of `Number Of Dead Players`.
#[derive(Clone, Debug)]
pub struct GetNumberOfDeadPlayersArgs {
    /// An optional argument.
    pub team: Expr<Team>,
}
impl Default for GetNumberOfDeadPlayersArgs {
    fn default() -> Self {
        Self {
            team: Expr::from_ir(support::constant("Team", "ALL")),
        }
    }
}
/// Optional arguments of `Number Of Heroes`.
#[derive(Clone, Debug)]
pub struct GetNumberOfHeroesArgs {
    /// An optional argument.
    pub team: Expr<Team>,
}
impl Default for GetNumberOfHeroesArgs {
    fn default() -> Self {
        Self {
            team: Expr::from_ir(support::constant("Team", "ALL")),
        }
    }
}
/// Optional arguments of `Number Of Living Players`.
#[derive(Clone, Debug)]
pub struct GetNumberOfLivingPlayersArgs {
    /// An optional argument.
    pub team: Expr<Team>,
}
impl Default for GetNumberOfLivingPlayersArgs {
    fn default() -> Self {
        Self {
            team: Expr::from_ir(support::constant("Team", "ALL")),
        }
    }
}
/// Optional arguments of `Number Of Players`.
#[derive(Clone, Debug)]
pub struct GetNumberOfPlayersArgs {
    /// An optional argument.
    pub team: Expr<Team>,
}
impl Default for GetNumberOfPlayersArgs {
    fn default() -> Self {
        Self {
            team: Expr::from_ir(support::constant("Team", "ALL")),
        }
    }
}
/// Optional arguments of `Number Of Players On Objective`.
#[derive(Clone, Debug)]
pub struct GetNumberOfPlayersOnObjectiveArgs {
    /// An optional argument.
    pub team: Expr<Team>,
}
impl Default for GetNumberOfPlayersOnObjectiveArgs {
    fn default() -> Self {
        Self {
            team: Expr::from_ir(support::constant("Team", "ALL")),
        }
    }
}
/// Optional arguments of `Number Of Slots`.
#[derive(Clone, Debug)]
pub struct GetNumberOfSlotsArgs {
    /// An optional argument.
    pub team: Expr<Team>,
}
impl Default for GetNumberOfSlotsArgs {
    fn default() -> Self {
        Self {
            team: Expr::from_ir(support::constant("Team", "ALL")),
        }
    }
}
/// Optional arguments of `All Players`.
#[derive(Clone, Debug)]
pub struct GetPlayersArgs {
    /// An optional argument.
    pub team: Expr<Team>,
}
impl Default for GetPlayersArgs {
    fn default() -> Self {
        Self {
            team: Expr::from_ir(support::constant("Team", "ALL")),
        }
    }
}
/// Optional arguments of `Players Within Radius`.
#[derive(Clone, Debug)]
pub struct GetPlayersInRadiusArgs {
    /// An optional argument.
    pub team: Expr<Team>,
    /// An optional argument.
    pub los_check: Expr<LosCheck>,
}
impl Default for GetPlayersInRadiusArgs {
    fn default() -> Self {
        Self {
            team: Expr::from_ir(support::constant("Team", "ALL")),
            los_check: Expr::from_ir(support::constant("LosCheck", "OFF")),
        }
    }
}
/// Optional arguments of `Players In Slot`.
#[derive(Clone, Debug)]
pub struct GetPlayersInSlotArgs {
    /// An optional argument.
    pub team: Expr<Team>,
}
impl Default for GetPlayersInSlotArgs {
    fn default() -> Self {
        Self {
            team: Expr::from_ir(support::constant("Team", "ALL")),
        }
    }
}
/// Optional arguments of `All Players Not On Objective`.
#[derive(Clone, Debug)]
pub struct GetPlayersNotOnObjectiveArgs {
    /// An optional argument.
    pub team: Expr<Team>,
}
impl Default for GetPlayersNotOnObjectiveArgs {
    fn default() -> Self {
        Self {
            team: Expr::from_ir(support::constant("Team", "ALL")),
        }
    }
}
/// Optional arguments of `Players On Hero`.
#[derive(Clone, Debug)]
pub struct GetPlayersOnHeroArgs {
    /// An optional argument.
    pub team: Expr<Team>,
}
impl Default for GetPlayersOnHeroArgs {
    fn default() -> Self {
        Self {
            team: Expr::from_ir(support::constant("Team", "ALL")),
        }
    }
}
/// Optional arguments of `All Players On Objective`.
#[derive(Clone, Debug)]
pub struct GetPlayersOnObjectiveArgs {
    /// An optional argument.
    pub team: Expr<Team>,
}
impl Default for GetPlayersOnObjectiveArgs {
    fn default() -> Self {
        Self {
            team: Expr::from_ir(support::constant("Team", "ALL")),
        }
    }
}
/// Optional arguments of `Is In Line of Sight`.
#[derive(Clone, Debug)]
pub struct IsInLoSArgs {
    /// An optional argument.
    pub barriers: Expr<BarrierLos>,
}
impl Default for IsInLoSArgs {
    fn default() -> Self {
        Self {
            barriers: Expr::from_ir(support::constant("BarrierLos", "PASS_THROUGH_BARRIERS")),
        }
    }
}
/// Optional arguments of `Custom Color`.
#[derive(Clone, Debug)]
pub struct RgbArgs {
    /// An optional argument.
    pub alpha: Expr<Num>,
}
impl Default for RgbArgs {
    fn default() -> Self {
        Self {
            alpha: Expr::from_ir(support::num(255f64)),
        }
    }
}
/// Optional arguments of `Array Slice`.
#[derive(Clone, Debug)]
pub struct SliceArgs {
    /// An optional argument.
    pub count: Expr<Num>,
}
impl Default for SliceArgs {
    fn default() -> Self {
        Self {
            count: Expr::from_ir(support::num(999999999999f64)),
        }
    }
}
/// Optional arguments of `String Slice`.
#[derive(Clone, Debug)]
pub struct SubstringArgs {
    /// An optional argument.
    pub substring_length: Expr<Num>,
}
impl Default for SubstringArgs {
    fn default() -> Self {
        Self {
            substring_length: Expr::from_ir(support::num(999999999999f64)),
        }
    }
}
