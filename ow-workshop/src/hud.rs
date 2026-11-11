//! HUD shortcuts and `print`.

use crate::block::Block;
use crate::expr::{Expr, null};
use crate::generated::api::HudTextArgs;
use crate::math::LOTS_OF_SPACES;
use crate::ty::{Any, Color, HudPosition, HudReeval, IntoExpr, Num, Players, SpecVisibility};

/// Optional arguments of the HUD shortcuts.
#[derive(Clone, Debug)]
pub struct HudArgs {
    /// Who sees the text; default: everyone.
    pub visible_to: Expr<Players>,
    /// Where on the screen; default: left.
    pub location: Expr<HudPosition>,
    /// Position among the other HUD texts of that location; default: 0.
    pub sort_order: Expr<Num>,
    /// The color of the text; default: white.
    pub color: Expr<Color>,
    /// What the game re-reads while the text is shown; default: everything that can change.
    pub reevaluation: Expr<HudReeval>,
    /// Whether spectators see it; default: as the game decides.
    pub spec_visibility: Expr<SpecVisibility>,
}

impl Default for HudArgs {
    fn default() -> Self {
        let d = HudTextArgs::default();
        Self {
            visible_to: d.visible_to,
            location: d.location,
            sort_order: d.sort_order,
            color: d.header_color,
            reevaluation: d.reevaluation,
            spec_visibility: d.spec_visibility,
        }
    }
}

/// Which of the three text slots of `Create HUD Text` a shortcut fills.
#[derive(Clone, Copy)]
enum Slot {
    Header,
    Subheader,
    Text,
}

impl Block {
    #[track_caller]
    fn hud(&mut self, slot: Slot, text: Expr<Any>, opts: HudArgs) {
        let (mut header, mut subheader, mut body) = (null(), null(), null());
        let (mut header_color, mut subheader_color, mut text_color) = (null(), null(), null());
        match slot {
            Slot::Header => (header, header_color) = (text, opts.color),
            Slot::Subheader => (subheader, subheader_color) = (text, opts.color),
            Slot::Text => (body, text_color) = (text, opts.color),
        }
        self.hud_text_with(HudTextArgs {
            visible_to: opts.visible_to,
            header,
            subheader,
            text: body,
            location: opts.location,
            sort_order: opts.sort_order,
            header_color,
            subheader_color,
            text_color,
            reevaluation: opts.reevaluation,
            spec_visibility: opts.spec_visibility,
        });
    }

    /// A HUD text with only a header.
    #[track_caller]
    pub fn hud_header(&mut self, text: impl IntoExpr<Any>) {
        self.hud_header_with(text, HudArgs::default());
    }

    /// [`hud_header`](Self::hud_header) with explicit optional arguments.
    #[track_caller]
    pub fn hud_header_with(&mut self, text: impl IntoExpr<Any>, opts: HudArgs) {
        self.hud(Slot::Header, text.into_expr(), opts);
    }

    /// A HUD text with only a subheader.
    #[track_caller]
    pub fn hud_subheader(&mut self, text: impl IntoExpr<Any>) {
        self.hud_subheader_with(text, HudArgs::default());
    }

    /// [`hud_subheader`](Self::hud_subheader) with explicit optional arguments.
    #[track_caller]
    pub fn hud_subheader_with(&mut self, text: impl IntoExpr<Any>, opts: HudArgs) {
        self.hud(Slot::Subheader, text.into_expr(), opts);
    }

    /// A HUD text with only body text.
    #[track_caller]
    pub fn hud_subtext(&mut self, text: impl IntoExpr<Any>) {
        self.hud_subtext_with(text, HudArgs::default());
    }

    /// [`hud_subtext`](Self::hud_subtext) with explicit optional arguments.
    #[track_caller]
    pub fn hud_subtext_with(&mut self, text: impl IntoExpr<Any>, opts: HudArgs) {
        self.hud(Slot::Text, text.into_expr(), opts);
    }

    /// Shows `text` to everyone, in orange at the very top of the left HUD.
    ///
    /// The wide subheader keeps the box at a fixed width so the text does not jump around.
    #[track_caller]
    ///
    /// A debug-only call: a release build leaves it out (see [`Block::debug_only`]).
    pub fn print(&mut self, text: impl IntoExpr<Any>) {
        let start = self.recorded();
        self.hud_text_with(HudTextArgs {
            header: text.into_expr(),
            subheader: LOTS_OF_SPACES.into_expr(),
            sort_order: (-9999).into(),
            header_color: Color::Orange.into(),
            subheader_color: null(),
            text_color: null(),
            reevaluation: HudReeval::VisibilityAndString.into(),
            ..HudTextArgs::default()
        });
        self.mark_debug_only(start);
    }
}

#[cfg(test)]
mod tests;
