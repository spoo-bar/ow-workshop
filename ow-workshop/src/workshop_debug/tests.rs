use crate::ir::{ExprKind, StrExpr, StrToken, VarId};
use crate::prelude::*;
use crate::workshop_debug::Scope;
use ow_workshop_data::ValueId;

#[derive(GlobalVars)]
struct Globals {
    wave: Global<Num>,
    #[ow(debug_label = "Boss HP")]
    boss_health: Global<Num>,
    #[ow(debug = false)]
    #[allow(dead_code)]
    secret: Global<Num>,
    #[ow(debug_with = hex)]
    mask: Global<Num>,
    names: Global<Array<Str>>,
    title: Global<Str>,
    side: Global<Team>,
}

#[derive(PlayerVars)]
struct Players {
    kills: PlayerVar<Num>,
    #[ow(name = "renamed", debug_label = "Alive?")]
    alive: PlayerVar<Bool>,
}

/// A custom renderer: `0x{0}`.
fn hex(value: Expr<Num>) -> Expr<Str> {
    ws_fmt!("0x{}", value)
}

#[derive(Clone, Copy, WorkshopEnum)]
enum State {
    Lobby,
    Playing,
    Over,
}

#[derive(Clone, Copy, WorkshopEnum)]
enum Wave {
    Calm = 3,
    Boss = 10,
}

#[derive(Clone, Copy, WorkshopEnum)]
#[ow(ty = Team)]
enum Side {
    #[ow(value = Team::One)]
    Red,
    #[ow(value = Team::Two)]
    Blue,
}

#[derive(GlobalVars)]
struct WithEnum {
    #[ow(debug_with = State::debug_render)]
    state: Global<Num>,
}

fn declare<G: GlobalVars, P: PlayerVars>() -> (G, P) {
    (
        G::declare(&mut VarRegistry::default()),
        P::declare(&mut VarRegistry::default()),
    )
}

fn labels(fields: &[DebugField]) -> Vec<&str> {
    fields.iter().map(|f| f.label).collect()
}

/// The template text and the arguments of a rendered `Custom String`.
fn custom(text: &Expr<Str>) -> (String, Vec<crate::ir::Expr>) {
    let ExprKind::Str(StrExpr::Custom { tokens, args }) = text.ir().kind() else {
        panic!("not a custom string: {:?}", text.ir().kind());
    };
    let template = tokens
        .iter()
        .map(|token| match token {
            StrToken::Text(text) => text.clone(),
            StrToken::Arg(i) => format!("{{{i}}}"),
        })
        .collect();
    (template, args.clone())
}

fn player() -> Expr<Player> {
    Expr::<Player>::from_ir(crate::ir::Expr::here(ExprKind::Value {
        id: ow_workshop_data::ValueId::EventPlayer,
        args: vec![],
    }))
}

#[test]
fn labels_default_to_the_field_name_and_can_be_overridden() {
    let (globals, players) = declare::<Globals, Players>();
    assert_eq!(
        labels(&globals.debug_fields()),
        ["wave", "Boss HP", "mask", "names", "title", "side"]
    );
    assert_eq!(labels(&players.debug_fields()), ["kills", "Alive?"]);
}

#[test]
fn a_field_can_be_hidden() {
    let (globals, _) = declare::<Globals, Players>();
    assert!(!labels(&globals.debug_fields()).contains(&"secret"));
}

#[test]
fn fields_know_their_scope_and_variable() {
    let (globals, players) = declare::<Globals, Players>();
    let global = globals.debug_fields();
    let player = players.debug_fields();
    assert!(global.iter().all(|f| f.scope == Scope::Global));
    assert!(player.iter().all(|f| f.scope == Scope::Player));
    // Variables are numbered in declaration order, hidden ones included.
    assert_eq!(global[0].var, VarId(0));
    assert_eq!(global[1].var, VarId(1));
    assert_eq!(global[2].var, VarId(3));
    assert_eq!(player[1].var, VarId(1));
}

#[test]
fn the_unit_type_has_no_fields() {
    assert_eq!(().debug_fields(), vec![]);
}

#[test]
fn numbers_are_rendered_as_themselves() {
    let (globals, _) = declare::<Globals, Players>();
    let field = &globals.debug_fields()[0];
    let text = (field.value)(player());
    let (template, args) = custom(&text);
    assert_eq!(template, "{0}");
    assert!(matches!(args[0].kind(), ExprKind::GlobalVar(_)));
}

#[test]
fn strings_are_quoted() {
    let (globals, _) = declare::<Globals, Players>();
    let field = &globals.debug_fields()[4];
    assert_eq!(field.label, "title");
    assert_eq!(custom(&(field.value)(player())).0, "\"{0}\"");
}

#[test]
fn constants_are_rendered_as_themselves() {
    let (globals, _) = declare::<Globals, Players>();
    let field = &globals.debug_fields()[5];
    assert_eq!(field.label, "side");
    assert_eq!(custom(&(field.value)(player())).0, "{0}");
}

#[test]
fn arrays_are_spelled_out() {
    let (globals, _) = declare::<Globals, Players>();
    let field = &globals.debug_fields()[3];
    assert_eq!(field.label, "names");
    // `array_to_string` builds a string out of several workshop calls, not one template.
    let text = (field.value)(player());
    assert!(
        matches!(text.ir().kind(), ExprKind::Value { id, .. } if *id == ValueId::MappedArray),
        "the array is not spelled out"
    );
}

#[test]
fn a_custom_renderer_replaces_the_default() {
    let (globals, _) = declare::<Globals, Players>();
    let field = &globals.debug_fields()[2];
    assert_eq!(field.label, "mask");
    assert_eq!(custom(&(field.value)(player())).0, "0x{0}");
}

#[test]
fn player_variables_are_read_from_the_player_given() {
    let (_, players) = declare::<Globals, Players>();
    let field = &players.debug_fields()[0];
    let (template, args) = custom(&(field.value)(player()));
    assert_eq!(template, "{0}");
    assert!(matches!(args[0].kind(), ExprKind::PlayerVar { .. }));
}

#[test]
fn registered_things_follow_the_variable_structs() {
    struct Mode;

    struct Extra(Global<Num>);

    impl WorkshopDebug for Extra {
        fn debug_fields(&self) -> Vec<DebugField> {
            vec![DebugField::new(self.0, "extra")]
        }
    }

    impl Workshop for Mode {
        fn mode_name(&self) -> &'static str {
            "Test"
        }

        fn description(&self) -> &'static str {
            "Test mode"
        }

        fn settings(&self) -> crate::settings::Settings {
            crate::settings::Settings::unchecked()
        }

        type Globals = Globals;
        type Players = Players;

        fn build(&self, ws: &mut Builder, globals: &Globals, _: &Players) {
            ws.debug(&Extra(globals.wave));
        }
    }

    let mut registry = VarRegistry::default();
    let globals = Globals::declare(&mut registry);
    let mut ws = Builder::new();
    Mode.build(&mut ws, &globals, &declare::<Globals, Players>().1);
    let (program, _) = ws.finish(crate::ir::Program::default());
    assert_eq!(labels(&program.debug_fields), ["extra"]);
}

fn names_of(array: &crate::ir::Expr) -> Vec<String> {
    let ExprKind::Array(items) = array.kind() else {
        panic!("not an array: {:?}", array.kind());
    };
    items
        .iter()
        .map(|item| match item.kind() {
            ExprKind::Str(StrExpr::Custom { tokens, .. }) => match tokens.as_slice() {
                [StrToken::Text(text)] => text.clone(),
                other => panic!("unexpected {other:?}"),
            },
            other => panic!("unexpected {other:?}"),
        })
        .collect()
}

#[test]
fn an_enum_with_contiguous_values_indexes_its_names() {
    let text = State::debug_render(Expr::<Num>::from(State::Playing));
    let ExprKind::Index { array, index } = text.ir().kind() else {
        panic!("not an index: {:?}", text.ir().kind());
    };
    assert_eq!(names_of(array), ["Lobby", "Playing", "Over"]);
    assert!(matches!(index.kind(), ExprKind::Num(n) if (*n - 1.0).abs() < f64::EPSILON));
}

#[test]
fn an_enum_with_gaps_looks_the_value_up() {
    let text = Wave::debug_render(Expr::<Num>::from(Wave::Boss));
    let ExprKind::Index { array, index } = text.ir().kind() else {
        panic!("not an index: {:?}", text.ir().kind());
    };
    assert_eq!(names_of(array), ["Calm", "Boss"]);
    // `Index Of Array Value(Array(3, 10), v)`
    let ExprKind::Value { args, .. } = index.kind() else {
        panic!("not a call: {:?}", index.kind());
    };
    assert!(matches!(args[0].kind(), ExprKind::Array(values) if values.len() == 2));
}

#[test]
fn a_typed_enum_looks_the_value_up() {
    let text = Side::debug_render(Expr::<Team>::from(Team::Two));
    let ExprKind::Index { array, index } = text.ir().kind() else {
        panic!("not an index: {:?}", text.ir().kind());
    };
    assert_eq!(names_of(array), ["Red", "Blue"]);
    assert!(matches!(index.kind(), ExprKind::Value { .. }));
}

#[test]
fn a_field_can_use_an_enums_renderer() {
    let globals = WithEnum::declare(&mut VarRegistry::default());
    let fields = globals.debug_fields();
    let text = (fields[0].value)(player());
    let ExprKind::Index { array, .. } = text.ir().kind() else {
        panic!("not an index: {:?}", text.ir().kind());
    };
    assert_eq!(names_of(array), ["Lobby", "Playing", "Over"]);
}
