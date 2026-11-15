/// Where on the screen a debug line is shown.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HudLocation {
    /// The left side.
    #[default]
    Left,
    /// The top centre.
    Top,
    /// The right side.
    Right,
}

impl HudLocation {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Left => "Left",
            Self::Top => "Top",
            Self::Right => "Right",
        }
    }
}

/// How an item is shown in debug builds. Release builds ignore it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DebugOpts {
    /// The text shown next to the value. Defaults to the item's name.
    pub label: Option<String>,
    /// Where on the HUD it is shown.
    pub hud: HudLocation,
    /// Position among other HUD lines at the same location.
    pub sort_order: i32,
    /// Also write a line to the workshop inspector.
    pub log_to_inspector: bool,
}

/// Declares how an item is shown in debug builds.
///
/// Derive it with `#[derive(WorkshopDebug)]` and an optional
/// `#[debug(hud = Left, label = "..", sort_order = 1, log_to_inspector)]`.
/// Without the attribute the item has no debug display.
pub trait WorkshopDebug {
    /// The debug display options, or `None` for no display.
    fn debug_opts(&self) -> Option<DebugOpts> {
        None
    }
}
