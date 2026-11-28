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
//! drag, a slider or a typed name is one step, kept once no pointer is down,
//! no field has the keyboard and a frame went by without a change: what the
//! frame's start completes after an edit (the bars Restore defaults leaves
//! to compose, the gaps made one) belongs to that edit's step.
//!
//! What also changes outside editing is no step and stays as it is on undo:
//! the lock, the floating blocks shown or hidden, the fixed blocks their
//! shortcut hid, the one-time migrations, and a floating block moved without
//! a hand on it (the viewer's window resized, a block kept on the screen).
//! An interface loaded also writes the bars' look and the other Aurora or
//! Legacy styles, and which interface is chosen (`Load`): those go back and
//! forth with its step, each preference only while it keeps what the load
//! gave it. The history lasts while the viewer runs; a layout changed while
//! the editor was closed (an interface loaded from its block, a floating
//! block dragged) starts it again.

use super::{ToolbarSettings, ToolbarUi, floating};
use crate::keybinds::EditHistory;
use crate::settings::Settings;
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

/// What an interface loaded wrote beside the bars (`Preset::apply`,
/// `profile::apply`): the preferences as they `was` and as the load `made`
/// them. Only the fields of `loaded_fields!` are read.
#[derive(Clone, Debug)]
pub(crate) struct Load {
    was: Box<Settings>,
    made: Box<Settings>,
}

/// Every preference an interface loaded writes besides the bars:
/// `options::set_look` (Preferences › Aurora's looks, through
/// `Preset::apply`), the chosen interface, the bug button and the
/// conversations' thumbnails (`Preset::apply`) and the bars' base
/// (`profile::apply`). `a_load_takes_back_every_preference_it_wrote` fails
/// on one written there and missing here.
macro_rules! loaded_fields {
    ($($($field:ident).+),* $(,)?) => {
        impl Load {
            fn changes(&self) -> bool {
                $(self.was.$($field).+ != self.made.$($field).+)||*
            }

            /// The preferences back as they were, each one only while it
            /// keeps what the load gave it.
            pub(crate) fn take_back(&self, settings: &mut Settings) {
                $(back(&mut settings.$($field).+, &self.was.$($field).+, &self.made.$($field).+);)*
            }
        }
    };
}

loaded_fields!(
    chat_style,
    classic_name_tags,
    lookat.style,
    lookat.aim,
    beams.style,
    notifications_style,
    world_map_style,
    classic_bars,
    classic_people,
    classic_profiles,
    photo_window,
    classic_about_land,
    classic_search,
    classic_region_estate,
    classic_inventory,
    classic_outfit_gallery,
    build.legacy_window,
    classic_places,
    destinations_style,
    classic_pay_window,
    legacy_money_tracker,
    classic_controls,
    classic_restart_alert,
    classic_chat_lines,
    classic_rez_effect,
    classic_hover_tips,
    predict_own_motion,
    water_effects.enabled,
    synchronize_animations,
    colored_sky_ambient,
    toolbar_selection,
    show_bug_report,
    chat_thumbnails,
);

fn back<T: PartialEq + Clone>(now: &mut T, was: &T, made: &T) {
    if now == made {
        *now = was.clone();
    }
}

impl Load {
    pub(crate) fn new(was: &Settings, made: &Settings) -> Self {
        Self {
            was: Box::new(was.clone()),
            made: Box::new(made.clone()),
        }
    }

    /// The other way: what the undo took back, made again by the redo.
    fn reversed(&self) -> Self {
        Self {
            was: self.made.clone(),
            made: self.was.clone(),
        }
    }

    /// Two loads in one step: from the first's start to the second's end.
    fn then(self, next: Self) -> Self {
        Self {
            was: self.was,
            made: next.made,
        }
    }
}

/// A layout to go back (or forward) to, and the preferences an interface
/// loaded on the way changed (`Load::take_back` brings back this side's).
#[derive(Clone, Debug)]
struct Step {
    snapshot: Snapshot,
    load: Option<Load>,
}

#[derive(Default)]
pub(crate) struct History {
    undo: Vec<Step>,
    redo: Vec<Step>,
    /// The layout last seen, and its comparable form (`layout_only`).
    seen: Option<(Snapshot, ToolbarSettings)>,
    /// The layout before the change under way (a drag not released yet).
    before: Option<Snapshot>,
    /// What an interface loaded in the change under way wrote besides.
    load: Option<Load>,
    /// The editor was open on the last frame seen.
    editing: bool,
    /// A floating block was dragged while the editor was closed.
    moved_outside: bool,
}

impl History {
    /// One edited frame's layout. `settled`: no pointer is down and no
    /// field has the keyboard. `by_hand`: a pointer is down, so a floating
    /// block that moves is being dragged. The change under way is one step
    /// on the first settled frame that changes nothing.
    pub(super) fn observe(&mut self, settings: &ToolbarSettings, places: &Places, settled: bool, by_hand: bool) {
        let resumed = !std::mem::replace(&mut self.editing, true);
        let moved_outside = std::mem::take(&mut self.moved_outside);
        let key = layout_only(settings);
        match self.seen.as_mut() {
            Some((seen, seen_key)) if *seen_key == key && !(by_hand && moved(&seen.places, places)) && !(resumed && moved_outside) => {
                // nothing edited: what changes outside editing, and the
                // blocks drawn for the first time or moved by the window,
                // follow
                seen.settings.clone_from(settings);
                follow(&mut seen.places, places);
                if settled {
                    self.settle();
                }
            }
            Some(_) if resumed => {
                self.undo.clear();
                self.redo.clear();
                self.before = None;
                self.load = None;
                self.seen = Some((snapshot(settings, places), key));
            }
            Some(_) => {
                if let Some((seen, _)) = self.seen.take() {
                    self.before.get_or_insert(seen);
                }
                self.seen = Some((snapshot(settings, places), key));
            }
            None => self.seen = Some((snapshot(settings, places), key)),
        }
    }

    /// A frame with the editor closed: the change under way is kept, and
    /// a floating block dragged now starts the history again on reopening.
    pub(super) fn closed(&mut self, places: &Places, by_hand: bool) {
        if std::mem::replace(&mut self.editing, false) {
            self.settle();
        }
        if let Some((seen, _)) = self.seen.as_mut()
            && moved(&seen.places, places)
        {
            if by_hand {
                self.moved_outside = true;
            } else {
                follow(&mut seen.places, places);
            }
        }
    }

    /// An interface loaded while editing: what it wrote besides the bars
    /// goes with the change it makes, a step even when the bars stay.
    pub(super) fn loaded(&mut self, load: Load) {
        if !self.editing {
            return;
        }
        if self.before.is_none() {
            self.before = self.seen.as_ref().map(|(seen, _)| seen.clone());
        }
        self.load = Some(match self.load.take() {
            Some(first) => first.then(load),
            None => load,
        });
    }

    /// The layout to go back (or forward) to, the one shown now kept for
    /// the other way, and the load whose preferences to take back.
    pub(super) fn step(&mut self, history: EditHistory) -> Option<(Snapshot, Option<Load>)> {
        self.settle();
        let (from, to) = match history {
            EditHistory::Undo => (&mut self.undo, &mut self.redo),
            EditHistory::Redo => (&mut self.redo, &mut self.undo),
        };
        let target = from.pop()?;
        if let Some((current, _)) = self.seen.take() {
            push(
                to,
                Step {
                    snapshot: current,
                    load: target.load.as_ref().map(Load::reversed),
                },
            );
        }
        self.seen = Some((target.snapshot.clone(), layout_only(&target.snapshot.settings)));
        Some((target.snapshot, target.load))
    }

    #[cfg(test)]
    pub(super) fn depth(&self) -> (usize, usize) {
        (self.undo.len(), self.redo.len())
    }

    /// The change under way kept as one step; a new step forgets what
    /// could be redone.
    fn settle(&mut self) {
        let Some(before) = self.before.take() else { return };
        let load = self.load.take().filter(Load::changes);
        let Some((seen, seen_key)) = &self.seen else { return };
        // a block dragged back where it was leaves no step
        if layout_only(&before.settings) != *seen_key || moved(&before.places, &seen.places) || load.is_some() {
            push(&mut self.undo, Step { snapshot: before, load });
            self.redo.clear();
        }
    }
}

/// The windows with a history of their own: Ctrl+Z is theirs while one is
/// in front of the editor, or has the system's focus in its own window
/// outside the viewer's.
const OWN_HISTORY: [&str; 8] = [
    crate::build::ui::INSPECTOR_ID,
    crate::build::ui::TOOL_WINDOWS[0],
    crate::build::ui::TOOL_WINDOWS[1],
    crate::build::ui::TOOL_WINDOWS[2],
    crate::build::ui::TOOL_WINDOWS[3],
    crate::ik_edit::poser::FULL_WINDOW,
    crate::ik_edit::poser::COMPACT_WINDOW,
    crate::ui::director::WINDOW,
];

impl ToolbarUi {
    /// After the frame's edits (`commit_with_menus`, the shortcuts'
    /// fallback): the layout noted, and an undo or redo asked for, by the
    /// editor's title buttons or Ctrl+Z / Ctrl+Y, applied. Returns whether
    /// the settings changed, to be saved; the preferences of an interface
    /// loaded are the app's to take back (`take_load_back`).
    pub fn track_history(&mut self, ctx: &egui::Context, settings: &mut ToolbarSettings) -> bool {
        let now = places(ctx, settings);
        let by_hand = ctx.input(|i| i.pointer.any_down());
        if !self.open {
            self.history.closed(&now, by_hand);
            self.history_request = None;
            return false;
        }
        let settled = !by_hand && !ctx.egui_wants_keyboard_input() && !Self::dragging(ctx);
        self.history.observe(settings, &now, settled, by_hand);
        let asked = match self.history_request.take() {
            Some(request) => Some((request, None)),
            None => self.history_key(ctx).map(|(request, key)| (request, Some(key))),
        };
        let Some((request, key)) = asked else {
            return false;
        };
        // a key with nothing to undo goes on to what is behind the editor
        let Some((target, load)) = self.history.step(request) else {
            return false;
        };
        if let Some((key, modifiers)) = key {
            ctx.input_mut(|i| i.consume_key(modifiers, key));
        }
        let mut restored = target.settings;
        // the frame's settings, the lock set aside included: `restore_lock`
        // puts it back as for any edit
        carry_live(&mut restored, settings);
        *settings = restored;
        place(ctx, &target.places, &now);
        self.load_back = load;
        // the selection and the open settings windows may name what is gone
        self.profile_applied(ctx);
        true
    }

    /// An interface was loaded (`Preset::apply`, `profile::apply`): the
    /// preferences before and after, undone with its step while editing.
    pub(crate) fn interface_loaded(&mut self, was: &Settings, made: &Settings) {
        self.history.loaded(Load::new(was, made));
    }

    /// The preferences an undo or redo of an interface loaded takes back,
    /// for the app to apply (and save) as a load's.
    pub(crate) fn take_load_back(&mut self) -> Option<Load> {
        self.load_back.take()
    }

    /// Ctrl+Z, Ctrl+Y or Ctrl+Shift+Z, as the poser takes them
    /// (`ik_edit::poser::ui`), not consumed yet: none while a field keeps
    /// its own history, a menu or a dialog is open, a button is dragged, a
    /// shortcut is being recorded, or a window with a history of its own is
    /// in front.
    fn history_key(&self, ctx: &egui::Context) -> Option<(EditHistory, (egui::Key, egui::Modifiers))> {
        if self.shortcut_editing()
            || Self::dragging(ctx)
            || ctx.egui_wants_keyboard_input()
            || egui::Popup::is_any_open(ctx)
            || ctx.memory(|m| m.top_modal_layer()).is_some()
            || OWN_HISTORY.iter().any(|window| {
                let id = egui::Id::unique(window);
                widgets::floater_above(ctx, window, super::editor::WINDOW)
                    || (crate::outside::native(ctx, id) && crate::outside::in_focus(ctx, id))
            })
        {
            return None;
        }
        ctx.input(|i| {
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
                    EditHistory::from_key(key, mods).map(|h| (h, (key, m)))
                }
                _ => None,
            })
        })
    }
}

fn snapshot(settings: &ToolbarSettings, places: &Places) -> Snapshot {
    Snapshot {
        settings: settings.clone(),
        places: places.clone(),
    }
}

fn push(steps: &mut Vec<Step>, step: Step) {
    steps.push(step);
    if steps.len() > DEPTH {
        steps.remove(0);
    }
}

/// A block standing in both that is not where it was.
/// The blocks drawn for the first time, or moved past anchoring's
/// rounding by something other than a hand, taken where they are now.
fn follow(seen: &mut Places, now: &Places) {
    for (id, pos) in now {
        if seen.get(id).is_none_or(|was| (*was - *pos).length() > MOVE_TOLERANCE) {
            seen.insert(id.clone(), *pos);
        }
    }
}

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
    use crate::ui::skin::{Layout, ToolButton};
    use crate::ui::toolbar::drawers::{Drawer, Floating};
    use crate::ui::toolbar::presets::Preset;

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

    /// A change, then the frame after it that changes nothing.
    fn edit(history: &mut History, settings: &ToolbarSettings, places: &Places) {
        history.observe(settings, places, true, false);
        history.observe(settings, places, true, false);
    }

    fn json(settings: &Settings) -> serde_json::Value {
        serde_json::to_value(settings).expect("settings serialize")
    }

    #[test]
    fn each_settled_change_is_one_step_back_and_forth() {
        let mut history = History::default();
        let none = Places::new();
        edit(&mut history, &layout(&["a"]), &none);
        edit(&mut history, &layout(&["a", "b"]), &none);
        edit(&mut history, &layout(&["a", "b", "c"]), &none);
        assert_eq!(history.depth(), (2, 0));
        let (back, _) = history.step(EditHistory::Undo).expect("a step");
        assert_eq!(commands(&back.settings), ["a", "b"]);
        let (back, _) = history.step(EditHistory::Undo).expect("a step");
        assert_eq!(commands(&back.settings), ["a"]);
        assert!(history.step(EditHistory::Undo).is_none(), "nothing before the first layout");
        let (forward, _) = history.step(EditHistory::Redo).expect("a step");
        assert_eq!(commands(&forward.settings), ["a", "b"]);
        assert_eq!(history.depth(), (1, 1));
    }

    #[test]
    fn a_change_under_way_is_one_step_once_released() {
        let mut history = History::default();
        let none = Places::new();
        edit(&mut history, &layout(&["a"]), &none);
        // a drag (or a typed name) changing the layout on every frame
        history.observe(&layout(&["a", "b"]), &none, false, true);
        history.observe(&layout(&["b", "a"]), &none, false, true);
        history.observe(&layout(&["b", "a", "c"]), &none, false, true);
        assert_eq!(history.depth(), (0, 0), "nothing kept while the pointer is down");
        history.observe(&layout(&["b", "a", "c"]), &none, true, false);
        assert_eq!(history.depth(), (1, 0));
        let (back, _) = history.step(EditHistory::Undo).expect("a step");
        assert_eq!(commands(&back.settings), ["a"]);
    }

    #[test]
    fn what_the_next_frame_completes_belongs_to_the_edit() {
        let mut history = History::default();
        let none = Places::new();
        edit(&mut history, &layout(&["a", "b"]), &none);
        // Restore defaults, then the frame's start composing what it left
        history.observe(&layout(&[]), &none, true, false);
        history.observe(&layout(&["x", "y"]), &none, true, false);
        history.observe(&layout(&["x", "y"]), &none, true, false);
        assert_eq!(history.depth(), (1, 0), "one step");
        let (back, _) = history.step(EditHistory::Undo).expect("a step");
        assert_eq!(commands(&back.settings), ["a", "b"]);
        history.observe(&back.settings, &none, true, false);
        let (forward, _) = history.step(EditHistory::Redo).expect("a step");
        assert_eq!(commands(&forward.settings), ["x", "y"], "the composed bars, not the bare ones");
    }

    #[test]
    fn a_change_dropped_where_it_started_leaves_no_step() {
        let mut history = History::default();
        let none = Places::new();
        edit(&mut history, &layout(&["a", "b"]), &none);
        history.observe(&layout(&["b", "a"]), &none, false, true);
        edit(&mut history, &layout(&["a", "b"]), &none);
        assert_eq!(history.depth(), (0, 0));
    }

    #[test]
    fn a_new_change_forgets_what_could_be_redone() {
        let mut history = History::default();
        let none = Places::new();
        edit(&mut history, &layout(&["a"]), &none);
        edit(&mut history, &layout(&["a", "b"]), &none);
        let (back, _) = history.step(EditHistory::Undo).expect("a step");
        edit(&mut history, &back.settings, &none);
        assert_eq!(history.depth(), (0, 1), "the restored layout itself is no change");
        edit(&mut history, &layout(&["a", "c"]), &none);
        assert_eq!(history.depth(), (1, 0));
        assert!(history.step(EditHistory::Redo).is_none());
    }

    #[test]
    fn a_drag_dropped_where_it_started_keeps_what_could_be_redone() {
        let mut history = History::default();
        let settings = layout(&["a"]);
        let at = |x: f32| Places::from([("drawer_1".to_owned(), egui::pos2(x, 40.0))]);
        edit(&mut history, &settings, &at(100.0));
        history.observe(&settings, &at(260.0), false, true);
        edit(&mut history, &settings, &at(260.0));
        let (back, _) = history.step(EditHistory::Undo).expect("a step");
        edit(&mut history, &settings, &back.places);
        history.observe(&settings, &at(140.0), false, true);
        history.observe(&settings, &at(100.0), false, true);
        edit(&mut history, &settings, &at(100.0));
        assert_eq!(history.depth(), (0, 1));
        let (forward, _) = history.step(EditHistory::Redo).expect("the redo kept");
        assert_eq!(forward.places, at(260.0));
    }

    #[test]
    fn the_lock_and_hidden_blocks_are_no_step_and_stay_on_undo() {
        let mut history = History::default();
        let none = Places::new();
        let first = layout(&["a"]);
        edit(&mut history, &first, &none);
        let mut locked = first.clone();
        locked.locked = true;
        locked.hidden_blocks.insert("a".into());
        edit(&mut history, &locked, &none);
        assert_eq!(history.depth(), (0, 0), "the lock is no step");
        let mut edited = layout(&["a", "b"]);
        edited.locked = true;
        edited.hidden_blocks.insert("a".into());
        edit(&mut history, &edited, &none);
        let mut back = history.step(EditHistory::Undo).expect("a step").0.settings;
        carry_live(&mut back, &edited);
        assert_eq!(commands(&back), ["a"]);
        assert!(back.locked, "the lock stays as it is");
        assert!(back.hidden_blocks.contains("a"));
    }

    #[test]
    fn a_floating_block_hidden_meanwhile_stays_hidden_on_undo() {
        let mut history = History::default();
        let none = Places::new();
        let shown = |shown: bool| {
            let mut settings = layout(&["a"]);
            settings.drawers.insert(
                "drawer_1".into(),
                Drawer {
                    floating: Some(Floating {
                        shown,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            );
            settings
        };
        edit(&mut history, &shown(true), &none);
        history.closed(&none, false);
        // hidden from its own button, the editor closed
        edit(&mut history, &shown(false), &none);
        // Clear all: no block left to give its latest state
        edit(&mut history, &layout(&[]), &none);
        let mut back = history.step(EditHistory::Undo).expect("a step").0.settings;
        carry_live(&mut back, &layout(&[]));
        let floating = back.drawers["drawer_1"].floating.as_ref().expect("the floating block back");
        assert!(!floating.shown);
    }

    #[test]
    fn a_floating_block_moved_by_hand_is_a_step_and_goes_back() {
        let mut history = History::default();
        let settings = layout(&["a"]);
        let at = |x: f32| Places::from([("drawer_1".to_owned(), egui::pos2(x, 40.0))]);
        edit(&mut history, &settings, &at(100.0));
        edit(&mut history, &settings, &at(100.2));
        assert_eq!(history.depth(), (0, 0), "anchoring's rounding is no move");
        history.observe(&settings, &at(180.0), false, true);
        history.observe(&settings, &at(260.0), false, true);
        edit(&mut history, &settings, &at(260.0));
        assert_eq!(history.depth(), (1, 0));
        let (back, _) = history.step(EditHistory::Undo).expect("a step");
        assert_eq!(back.places, at(100.0));
    }

    #[test]
    fn a_floating_block_moved_by_the_window_is_no_step() {
        let mut history = History::default();
        let settings = layout(&["a"]);
        let at = |x: f32| Places::from([("drawer_1".to_owned(), egui::pos2(x, 40.0))]);
        edit(&mut history, &layout(&[]), &at(100.0));
        edit(&mut history, &settings, &at(100.0));
        let (back, _) = history.step(EditHistory::Undo).expect("a step");
        // the viewer's window made smaller: the anchored block follows
        edit(&mut history, &back.settings, &at(60.0));
        assert_eq!(history.depth(), (0, 1), "the redo kept");
        let (forward, _) = history.step(EditHistory::Redo).expect("a step");
        assert_eq!(commands(&forward.settings), ["a"]);
        // closed, the window resized again, reopened: the history stays
        history.closed(&at(60.0), false);
        history.closed(&at(30.0), false);
        edit(&mut history, &forward.settings, &at(30.0));
        assert_eq!(history.depth(), (1, 0));
    }

    #[test]
    fn the_history_survives_reopening_unless_the_layout_changed_meanwhile() {
        let mut history = History::default();
        let none = Places::new();
        edit(&mut history, &layout(&["a"]), &none);
        edit(&mut history, &layout(&["a", "b"]), &none);
        history.closed(&none, false);
        edit(&mut history, &layout(&["a", "b"]), &none);
        assert_eq!(history.depth(), (1, 0), "reopened on the same layout");
        history.closed(&none, false);
        // an interface loaded from its block, the editor closed
        edit(&mut history, &layout(&["x"]), &none);
        assert_eq!(history.depth(), (0, 0));
        assert!(history.step(EditHistory::Undo).is_none());
        // a floating block dragged, the editor closed
        let at = |x: f32| Places::from([("drawer_1".to_owned(), egui::pos2(x, 40.0))]);
        edit(&mut history, &layout(&["x", "y"]), &at(100.0));
        assert_eq!(history.depth(), (1, 0));
        history.closed(&at(100.0), false);
        history.closed(&at(300.0), true);
        edit(&mut history, &layout(&["x", "y"]), &at(300.0));
        assert_eq!(history.depth(), (0, 0));
    }

    #[test]
    fn closing_the_editor_keeps_the_change_under_way() {
        let mut history = History::default();
        let none = Places::new();
        edit(&mut history, &layout(&["a"]), &none);
        history.observe(&layout(&["a", "b"]), &none, false, true);
        history.closed(&none, false);
        assert_eq!(history.depth(), (1, 0));
    }

    #[test]
    fn the_history_keeps_its_depth() {
        let mut history = History::default();
        let none = Places::new();
        let names: Vec<String> = (0..DEPTH + 20).map(|i| format!("c{i}")).collect();
        for i in 0..names.len() {
            let current: Vec<&str> = names[..=i].iter().map(String::as_str).collect();
            edit(&mut history, &layout(&current), &none);
        }
        assert_eq!(history.depth(), (DEPTH, 0));
        let mut oldest = None;
        while let Some((step, _)) = history.step(EditHistory::Undo) {
            oldest = Some(step);
        }
        assert_eq!(oldest.expect("steps").settings.top.len(), names.len() - DEPTH);
    }

    /// Applies an undo or redo as `track_history` and the app do.
    fn apply(settings: &mut Settings, (target, load): (Snapshot, Option<Load>)) {
        let mut restored = target.settings;
        carry_live(&mut restored, settings.toolbar.as_ref().expect("bars"));
        settings.toolbar = Some(restored);
        if let Some(load) = load {
            load.take_back(settings);
        }
    }

    #[test]
    fn an_interface_loaded_goes_back_and_forth_with_its_look() {
        let skin = Layout::default();
        let none = Places::new();
        let mut settings = Settings::default();
        Preset::Light.apply(&mut settings, &skin);
        let mut history = History::default();
        edit(&mut history, settings.toolbar.as_ref().expect("bars"), &none);
        let aurora = settings.clone();
        Preset::Legacy.apply(&mut settings, &skin);
        history.loaded(Load::new(&aurora, &settings));
        edit(&mut history, settings.toolbar.as_ref().expect("bars"), &none);
        assert_eq!(history.depth(), (1, 0));
        let legacy = settings.clone();
        assert!(legacy.classic_bars && !aurora.classic_bars);
        apply(&mut settings, history.step(EditHistory::Undo).expect("a step"));
        assert_eq!(json(&settings), json(&aurora), "the bars, the look and the chosen interface back");
        edit(&mut history, settings.toolbar.as_ref().expect("bars"), &none);
        apply(&mut settings, history.step(EditHistory::Redo).expect("a step"));
        assert_eq!(json(&settings), json(&legacy));
    }

    #[test]
    fn an_interface_loaded_with_the_same_bars_is_still_a_step() {
        let none = Places::new();
        let mut settings = Settings {
            toolbar: Some(layout(&["a"])),
            ..Default::default()
        };
        let mut history = History::default();
        edit(&mut history, settings.toolbar.as_ref().expect("bars"), &none);
        let was = settings.clone();
        // a personal interface of the same buttons on the Legacy base
        settings.classic_bars = !settings.classic_bars;
        history.loaded(Load::new(&was, &settings));
        edit(&mut history, settings.toolbar.as_ref().expect("bars"), &none);
        assert_eq!(history.depth(), (1, 0));
        apply(&mut settings, history.step(EditHistory::Undo).expect("a step"));
        assert_eq!(settings.classic_bars, was.classic_bars);
    }

    #[test]
    fn a_preference_changed_since_the_load_stays() {
        let skin = Layout::default();
        let mut settings = Settings::default();
        Preset::Light.apply(&mut settings, &skin);
        let was = settings.clone();
        Preset::Legacy.apply(&mut settings, &skin);
        let load = Load::new(&was, &settings);
        // Preferences › Aurora: the Aurora chat chosen again by hand
        settings.chat_style = was.chat_style;
        settings.classic_search = !settings.classic_search;
        let chosen = settings.classic_search;
        load.take_back(&mut settings);
        assert_eq!(settings.classic_search, chosen);
        assert_eq!(settings.classic_bars, was.classic_bars);
    }

    #[test]
    fn a_load_takes_back_every_preference_it_wrote() {
        let skin = Layout::default();
        for (from, to) in [(Preset::Light, Preset::Legacy), (Preset::Legacy, Preset::Light)] {
            let mut settings = Settings::default();
            from.apply(&mut settings, &skin);
            let was = settings.clone();
            to.apply(&mut settings, &skin);
            Load::new(&was, &settings).take_back(&mut settings);
            settings.toolbar = was.toolbar.clone();
            assert_eq!(json(&settings), json(&was), "{to:?} over {from:?}");
        }
        for (from, to) in [
            (crate::ui::options::Look::Aurora, crate::ui::options::Look::Legacy),
            (crate::ui::options::Look::Legacy, crate::ui::options::Look::Aurora),
        ] {
            let mut settings = Settings::default();
            crate::ui::options::set_look(&mut settings, from);
            let was = settings.clone();
            crate::ui::options::set_look(&mut settings, to);
            Load::new(&was, &settings).take_back(&mut settings);
            assert_eq!(json(&settings), json(&was), "{to:?} over {from:?}");
        }
    }
}
