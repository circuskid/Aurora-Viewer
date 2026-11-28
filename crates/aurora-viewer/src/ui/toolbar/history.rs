//! Undo and redo while the interface is edited. An Aurora addition: the
//! Legacy Viewer's toolbar window (LLFloaterToybox,
//! indra/newview/llfloatertoybox.cpp) has none, and a wrong drop or a wrong
//! interface loaded could only be put right by hand.
//!
//! A step is the whole layout as it was: the bars' settings and where the
//! floating blocks stand (a window's place in `ui_layout.ron`, not a
//! setting). It is taken by comparing the layout at the end of each edited
//! frame with the one before, so every way of changing it counts: the
//! edits, Restore defaults, Clear all and the interfaces loaded alike. A
//! drag, a slider or a typed name is one step, kept once no pointer is down
//! and no field has the keyboard.
//!
//! What also changes outside editing is no step and stays as it is on undo:
//! the lock, the floating blocks shown or hidden, the fixed blocks their
//! shortcut hid, and the one-time migrations. The history lasts while the
//! viewer runs; a layout changed while the editor was closed (an interface
//! loaded from its block) starts it again.

use super::{ToolbarSettings, ToolbarUi, floating};
use crate::keybinds::EditHistory;
use crate::ui::widgets;
use egui::Pos2;
use std::collections::BTreeMap;

/// The steps kept each way.
pub(super) const DEPTH: usize = 100;

/// Below this, a floating block has not moved (anchoring rounds its place).
const MOVE_TOLERANCE: f32 = 0.5;

/// Where each floating block stands, by its drawer id.
pub(super) type Places = BTreeMap<String, Pos2>;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Snapshot {
    pub settings: ToolbarSettings,
    pub places: Places,
}

#[derive(Default)]
pub(crate) struct History {
    undo: Vec<Snapshot>,
    redo: Vec<Snapshot>,
    /// The layout last seen, and its comparable form (`layout_only`).
    seen: Option<(Snapshot, ToolbarSettings)>,
    /// The layout before the change under way (a drag not released yet).
    before: Option<Snapshot>,
    /// The editor was open on the last frame seen.
    editing: bool,
}

impl History {
    /// One edited frame's layout. `settled`: no pointer is down and no
    /// field has the keyboard, so the change under way is one step now.
    pub(super) fn observe(&mut self, settings: &ToolbarSettings, places: &Places, settled: bool) {
        let resumed = !std::mem::replace(&mut self.editing, true);
        let key = layout_only(settings);
        match self.seen.as_mut() {
            Some((seen, seen_key)) if *seen_key == key && !moved(&seen.places, places) => {
                // a block drawn for the first time gives its place
                for (id, pos) in places {
                    seen.places.entry(id.clone()).or_insert(*pos);
                }
            }
            Some(_) if resumed => {
                self.undo.clear();
                self.redo.clear();
                self.before = None;
                self.seen = Some((snapshot(settings, places), key));
            }
            Some(_) => {
                if let Some((seen, _)) = self.seen.take() {
                    self.before.get_or_insert(seen);
                }
                self.redo.clear();
                self.seen = Some((snapshot(settings, places), key));
            }
            None => self.seen = Some((snapshot(settings, places), key)),
        }
        if settled {
            self.settle();
        }
    }

    /// A frame with the editor closed: the change under way is kept.
    pub(super) fn closed(&mut self) {
        if std::mem::replace(&mut self.editing, false) {
            self.settle();
        }
    }

    /// The layout to go back (or forward) to, the one shown now kept for
    /// the other way.
    pub(super) fn step(&mut self, history: EditHistory) -> Option<Snapshot> {
        self.settle();
        let (from, to) = match history {
            EditHistory::Undo => (&mut self.undo, &mut self.redo),
            EditHistory::Redo => (&mut self.redo, &mut self.undo),
        };
        let target = from.pop()?;
        if let Some((current, _)) = self.seen.take() {
            push(to, current);
        }
        self.seen = Some((target.clone(), layout_only(&target.settings)));
        Some(target)
    }

    #[cfg(test)]
    pub(super) fn depth(&self) -> (usize, usize) {
        (self.undo.len(), self.redo.len())
    }

    fn settle(&mut self) {
        let Some(before) = self.before.take() else { return };
        let Some((seen, seen_key)) = &self.seen else { return };
        // a block dragged back where it was leaves no step
        if layout_only(&before.settings) != *seen_key || moved(&before.places, &seen.places) {
            push(&mut self.undo, before);
        }
    }
}

/// The windows with a history of their own: Ctrl+Z is theirs while one is
/// in front of the editor.
const OWN_HISTORY: [&str; 4] = [
    crate::build::ui::INSPECTOR_ID,
    crate::ik_edit::poser::FULL_WINDOW,
    crate::ik_edit::poser::COMPACT_WINDOW,
    crate::ui::director::WINDOW,
];

impl ToolbarUi {
    /// After the frame's edits (`commit_with_menus`, the shortcuts'
    /// fallback): the layout noted, and an undo or redo asked for, by the
    /// editor's title buttons or Ctrl+Z / Ctrl+Y, applied. Returns whether
    /// the settings changed, to be saved.
    pub fn track_history(&mut self, ctx: &egui::Context, settings: &mut ToolbarSettings) -> bool {
        if !self.open {
            self.history.closed();
            self.history_request = None;
            return false;
        }
        let now = places(ctx, settings);
        let settled = !ctx.input(|i| i.pointer.any_down()) && !ctx.egui_wants_keyboard_input() && !Self::dragging(ctx);
        self.history.observe(settings, &now, settled);
        let Some(request) = self.history_request.take().or_else(|| self.history_key(ctx)) else {
            return false;
        };
        let Some(target) = self.history.step(request) else {
            return false;
        };
        let mut restored = target.settings;
        // the frame's settings, the lock set aside included: `restore_lock`
        // puts it back as for any edit
        carry_live(&mut restored, settings);
        *settings = restored;
        place(ctx, &target.places, &now);
        // the selection and the open settings windows may name what is gone
        self.profile_applied(ctx);
        true
    }

    /// Ctrl+Z, Ctrl+Y or Ctrl+Shift+Z, as the poser takes them
    /// (`ik_edit::poser::ui`): none while a field keeps its own history, a
    /// menu or a dialog is open, a button is dragged, a shortcut is being
    /// recorded, or a window with a history of its own is in front.
    fn history_key(&self, ctx: &egui::Context) -> Option<EditHistory> {
        if self.shortcut_editing()
            || Self::dragging(ctx)
            || ctx.egui_wants_keyboard_input()
            || egui::Popup::is_any_open(ctx)
            || ctx.memory(|m| m.top_modal_layer()).is_some()
            || OWN_HISTORY
                .iter()
                .any(|window| widgets::floater_above(ctx, window, super::editor::WINDOW))
        {
            return None;
        }
        let (key, modifiers, history) = ctx.input(|i| {
            i.events.iter().find_map(|e| match *e {
                egui::Event::Key {
                    key,
                    pressed: true,
                    modifiers: m,
                    ..
                } => {
                    let mods = crate::keybinds::Mods {
                        ctrl: m.command || m.ctrl,
                        shift: m.shift,
                        alt: m.alt,
                    };
                    EditHistory::from_key(key, mods).map(|h| (key, m, h))
                }
                _ => None,
            })
        })?;
        ctx.input_mut(|i| i.consume_key(modifiers, key));
        Some(history)
    }
}

fn snapshot(settings: &ToolbarSettings, places: &Places) -> Snapshot {
    Snapshot {
        settings: settings.clone(),
        places: places.clone(),
    }
}

fn push(steps: &mut Vec<Snapshot>, step: Snapshot) {
    steps.push(step);
    if steps.len() > DEPTH {
        steps.remove(0);
    }
}

/// A block standing in both that is not where it was.
fn moved(before: &Places, now: &Places) -> bool {
    before
        .iter()
        .any(|(id, a)| now.get(id).is_some_and(|b| (*a - *b).length() > MOVE_TOLERANCE))
}

/// The settings with what changes outside editing set aside, to compare.
fn layout_only(settings: &ToolbarSettings) -> ToolbarSettings {
    let mut key = settings.clone();
    carry_live(&mut key, &ToolbarSettings::default());
    for floating in key.drawers.values_mut().filter_map(|drawer| drawer.floating.as_mut()) {
        floating.shown = true;
    }
    key
}

/// What changes outside editing, taken from `live` into a restored layout.
/// A floating block `live` does not have keeps the restored state.
pub(super) fn carry_live(restored: &mut ToolbarSettings, live: &ToolbarSettings) {
    restored.locked = live.locked;
    restored.vr_button_migrated = live.vr_button_migrated;
    restored.director_button_migrated = live.director_button_migrated;
    restored.import_bar_filled = live.import_bar_filled;
    restored.import_shortcut_given = live.import_shortcut_given;
    restored.composed = live.composed;
    restored.chat_composed = live.chat_composed;
    restored.hidden_blocks = live.hidden_blocks.clone();
    for (id, drawer) in &mut restored.drawers {
        if let (Some(floating), Some(shown)) = (
            drawer.floating.as_mut(),
            live.drawers.get(id).and_then(|drawer| drawer.floating.as_ref()).map(|f| f.shown),
        ) {
            floating.shown = shown;
        }
    }
}

/// Where the floating blocks drawn so far stand.
pub(super) fn places(ctx: &egui::Context, settings: &ToolbarSettings) -> Places {
    ctx.memory(|m| {
        settings
            .drawers
            .iter()
            .filter(|(_, drawer)| drawer.floating.is_some())
            .filter_map(|(id, _)| {
                let rect = m.area_rect(egui::Id::unique(floating::window_id(id).as_str()))?;
                Some((id.clone(), rect.min))
            })
            .collect()
    })
}

/// The floating blocks back where `target` had them.
pub(super) fn place(ctx: &egui::Context, target: &Places, now: &Places) {
    for (id, pos) in target {
        if now.get(id).is_none_or(|at| (*at - *pos).length() > MOVE_TOLERANCE) {
            crate::ui::widgets::move_floater(ctx, &floating::window_id(id), *pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::skin::ToolButton;

    fn layout(commands: &[&str]) -> ToolbarSettings {
        ToolbarSettings {
            top: commands
                .iter()
                .map(|command| ToolButton {
                    command: (*command).into(),
                    icon: String::new(),
                    tooltip: String::new(),
                })
                .collect(),
            ..Default::default()
        }
    }

    fn commands(settings: &ToolbarSettings) -> Vec<&str> {
        settings.top.iter().map(|button| button.command.as_str()).collect()
    }

    #[test]
    fn each_settled_change_is_one_step_back_and_forth() {
        let mut history = History::default();
        let none = Places::new();
        history.observe(&layout(&["a"]), &none, true);
        history.observe(&layout(&["a", "b"]), &none, true);
        history.observe(&layout(&["a", "b", "c"]), &none, true);
        assert_eq!(history.depth(), (2, 0));
        let back = history.step(EditHistory::Undo).expect("a step");
        assert_eq!(commands(&back.settings), ["a", "b"]);
        let back = history.step(EditHistory::Undo).expect("a step");
        assert_eq!(commands(&back.settings), ["a"]);
        assert!(history.step(EditHistory::Undo).is_none(), "nothing before the first layout");
        let forward = history.step(EditHistory::Redo).expect("a step");
        assert_eq!(commands(&forward.settings), ["a", "b"]);
        assert_eq!(history.depth(), (1, 1));
    }

    #[test]
    fn a_change_under_way_is_one_step_once_released() {
        let mut history = History::default();
        let none = Places::new();
        history.observe(&layout(&["a"]), &none, true);
        // a drag (or a typed name) changing the layout on every frame
        history.observe(&layout(&["a", "b"]), &none, false);
        history.observe(&layout(&["b", "a"]), &none, false);
        history.observe(&layout(&["b", "a", "c"]), &none, false);
        assert_eq!(history.depth(), (0, 0), "nothing kept while the pointer is down");
        history.observe(&layout(&["b", "a", "c"]), &none, true);
        assert_eq!(history.depth(), (1, 0));
        let back = history.step(EditHistory::Undo).expect("a step");
        assert_eq!(commands(&back.settings), ["a"]);
    }

    #[test]
    fn a_change_dropped_where_it_started_leaves_no_step() {
        let mut history = History::default();
        let none = Places::new();
        history.observe(&layout(&["a", "b"]), &none, true);
        history.observe(&layout(&["b", "a"]), &none, false);
        history.observe(&layout(&["a", "b"]), &none, true);
        assert_eq!(history.depth(), (0, 0));
    }

    #[test]
    fn a_new_change_forgets_what_could_be_redone() {
        let mut history = History::default();
        let none = Places::new();
        history.observe(&layout(&["a"]), &none, true);
        history.observe(&layout(&["a", "b"]), &none, true);
        let back = history.step(EditHistory::Undo).expect("a step");
        history.observe(&back.settings, &none, true);
        assert_eq!(history.depth(), (0, 1), "the restored layout itself is no change");
        history.observe(&layout(&["a", "c"]), &none, true);
        assert_eq!(history.depth(), (1, 0));
        assert!(history.step(EditHistory::Redo).is_none());
    }

    #[test]
    fn the_lock_and_shown_blocks_are_no_step_and_stay_on_undo() {
        let mut history = History::default();
        let none = Places::new();
        let first = layout(&["a"]);
        history.observe(&first, &none, true);
        let mut locked = first.clone();
        locked.locked = true;
        locked.hidden_blocks.insert("a".into());
        history.observe(&locked, &none, true);
        assert_eq!(history.depth(), (0, 0), "the lock is no step");
        let mut edited = layout(&["a", "b"]);
        edited.locked = true;
        edited.hidden_blocks.insert("a".into());
        history.observe(&edited, &none, true);
        let mut back = history.step(EditHistory::Undo).expect("a step").settings;
        carry_live(&mut back, &edited);
        assert_eq!(commands(&back), ["a"]);
        assert!(back.locked, "the lock stays as it is");
        assert!(back.hidden_blocks.contains("a"));
    }

    #[test]
    fn a_floating_block_moved_is_a_step_and_goes_back() {
        let mut history = History::default();
        let settings = layout(&["a"]);
        let at = |x: f32| Places::from([("drawer_1".to_owned(), egui::pos2(x, 40.0))]);
        history.observe(&settings, &at(100.0), true);
        history.observe(&settings, &at(100.2), true);
        assert_eq!(history.depth(), (0, 0), "anchoring's rounding is no move");
        history.observe(&settings, &at(180.0), false);
        history.observe(&settings, &at(260.0), true);
        assert_eq!(history.depth(), (1, 0));
        let back = history.step(EditHistory::Undo).expect("a step");
        assert_eq!(back.places, at(100.0));
    }

    #[test]
    fn the_history_survives_reopening_unless_the_layout_changed_meanwhile() {
        let mut history = History::default();
        let none = Places::new();
        history.observe(&layout(&["a"]), &none, true);
        history.observe(&layout(&["a", "b"]), &none, true);
        history.closed();
        history.observe(&layout(&["a", "b"]), &none, true);
        assert_eq!(history.depth(), (1, 0), "reopened on the same layout");
        history.closed();
        // an interface loaded from its block, the editor closed
        history.observe(&layout(&["x"]), &none, true);
        assert_eq!(history.depth(), (0, 0));
        assert!(history.step(EditHistory::Undo).is_none());
    }

    #[test]
    fn closing_the_editor_keeps_the_change_under_way() {
        let mut history = History::default();
        let none = Places::new();
        history.observe(&layout(&["a"]), &none, true);
        history.observe(&layout(&["a", "b"]), &none, false);
        history.closed();
        assert_eq!(history.depth(), (1, 0));
    }

    #[test]
    fn the_history_keeps_its_depth() {
        let mut history = History::default();
        let none = Places::new();
        let names: Vec<String> = (0..DEPTH + 20).map(|i| format!("c{i}")).collect();
        for i in 0..names.len() {
            let current: Vec<&str> = names[..=i].iter().map(String::as_str).collect();
            history.observe(&layout(&current), &none, true);
        }
        assert_eq!(history.depth(), (DEPTH, 0));
        let mut oldest = None;
        while let Some(step) = history.step(EditHistory::Undo) {
            oldest = Some(step);
        }
        assert_eq!(oldest.expect("steps").settings.top.len(), names.len() - DEPTH);
    }
}
