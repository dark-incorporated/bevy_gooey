//! Notification center rows with actions, ids, an unlogged toast, an in-place count and the Note
//! kind (G-92, #103): a row is a log entry with its own id, buttons and count; rows that did not change
//! are the same entities; a press on a row's words sends nothing. Laid out for real in a headless
//! window, driven by real pointer and key input.

use bevy::camera::NormalizedRenderTarget;
use bevy::picking::pointer::{Location, PointerAction, PointerButton, PointerId, PointerInput};
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::time::TimeUpdateStrategy;
use bevy::ui::UiGlobalTransform;
use bevy::window::{PrimaryWindow, WindowRef, WindowResolution};
use bevy::winit::WinitPlugin;
use bevy_gooey::*;
use std::time::Duration;

/// Every `NotificationAction` a host would see: the entry and the id.
#[derive(Resource, Default)]
struct Sent(Vec<(u64, String)>);

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1280, 720).with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: None,
                    ..default()
                }
                .into(),
                ..default()
            })
            .disable::<WinitPlugin>(),
        GooeyPlugins,
    ))
    .init_resource::<Sent>()
    .add_observer(|action: On<NotificationAction>, mut sent: ResMut<Sent>| {
        sent.0.push((action.entry, action.id.clone()))
    });
    app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
        0.1,
    )));
    app.world_mut().spawn(Camera2d);
    app
}

fn rect(app: &App, entity: Entity) -> Rect {
    let size = app
        .world()
        .get::<ComputedNode>(entity)
        .expect("laid out")
        .size();
    let at = app
        .world()
        .get::<UiGlobalTransform>(entity)
        .expect("placed")
        .translation;
    Rect::from_center_size(at, size)
}

fn window(app: &mut App) -> Entity {
    app.world_mut()
        .query_filtered::<Entity, With<PrimaryWindow>>()
        .single(app.world())
        .expect("one primary window")
}

fn pointer(app: &mut App, at: Vec2, action: PointerAction) {
    let window = window(app);
    let location = Location {
        target: NormalizedRenderTarget::Window(
            WindowRef::Entity(window)
                .normalize(None)
                .expect("a window ref"),
        ),
        position: at,
    };
    app.world_mut()
        .write_message(PointerInput::new(PointerId::Mouse, location, action));
    app.update();
}

fn children_of(world: &World, entity: Entity) -> Vec<Entity> {
    world
        .get::<Children>(entity)
        .map(|c| c.iter().collect())
        .unwrap_or_default()
}

fn descendants(world: &World, root: Entity) -> Vec<Entity> {
    let mut found = Vec::new();
    let mut stack = vec![root];
    while let Some(entity) = stack.pop() {
        for child in children_of(world, entity) {
            found.push(child);
            stack.push(child);
        }
    }
    found
}

fn frames(app: &mut App, n: usize) {
    for _ in 0..n {
        app.update();
    }
}

fn sent(app: &mut App) -> Vec<(u64, String)> {
    std::mem::take(&mut app.world_mut().resource_mut::<Sent>().0)
}

fn push(app: &mut App, kind: ToastKind, text: &str) -> u64 {
    app.world_mut()
        .resource_mut::<ToastLog>()
        .push(ToastLogEntry {
            seconds: 3.0,
            kind,
            text: text.to_string(),
        })
}

fn push_with_action(app: &mut App, text: &str, label: &str, id: &str) -> u64 {
    let entry = push(app, ToastKind::Info, text);
    app.world_mut()
        .resource_mut::<ToastLog>()
        .get_mut(entry)
        .expect("the entry")
        .add_action(label, id);
    entry
}

/// A center in a screen, drawn.
fn center(app: &mut App) -> Entity {
    let center = app.world_mut().spawn(notification_center()).id();
    app.world_mut().spawn(screen()).add_child(center);
    frames(app, 4);
    center
}

/// Every row of the center: its entry (none for the "No notifications yet" line) and its entity.
fn row_entities(app: &App, center: Entity) -> Vec<(Option<u64>, Entity)> {
    let mut found: Vec<(f32, Option<u64>, Entity)> = descendants(app.world(), center)
        .into_iter()
        .filter_map(|e| {
            let row = app.world().get::<NotificationRow>(e)?;
            Some((rect(app, e).min.y, row.entry(), e))
        })
        .collect();
    found.sort_by(|a, b| a.0.total_cmp(&b.0));
    found.into_iter().map(|(_, id, e)| (id, e)).collect()
}

fn row_of(app: &App, center: Entity, entry: u64) -> Entity {
    row_entities(app, center)
        .into_iter()
        .find(|(id, _)| *id == Some(entry))
        .map(|(_, e)| e)
        .expect("a row for the entry")
}

fn row_words(app: &App, row: Entity) -> String {
    children_of(app.world(), row)
        .into_iter()
        .find_map(|e| app.world().get::<Text>(e).map(|t| t.0.clone()))
        .expect("a row has words")
}

/// The buttons inside a row, left to right.
fn row_buttons(app: &App, row: Entity) -> Vec<Entity> {
    let mut found: Vec<Entity> = descendants(app.world(), row)
        .into_iter()
        .filter(|&e| app.world().get::<Role>(e) == Some(&Role::Button))
        .collect();
    found.sort_by(|a, b| rect(app, *a).min.x.total_cmp(&rect(app, *b).min.x));
    found
}

fn label_of(app: &App, button: Entity) -> String {
    row_words(app, button)
}

/// A click as the pointer makes it, at a point.
fn click_at(app: &mut App, at: Vec2) {
    pointer(app, at, PointerAction::Move { delta: Vec2::ZERO });
    app.update();
    pointer(app, at, PointerAction::Press(PointerButton::Primary));
    pointer(app, at, PointerAction::Release(PointerButton::Primary));
    frames(app, 2);
}

// ---- 1. an action is a button; the row's words are not ---------------------------------------

#[test]
fn an_entry_with_an_action_is_a_row_with_a_button_that_sends_its_entry_and_id() {
    let mut app = app();
    let first = push(&mut app, ToastKind::Info, "Opened");
    let entry = push_with_action(&mut app, "Moved the bed", "Undo", "undo");
    let center = center(&mut app);
    let row = row_of(&app, center, entry);
    let buttons = row_buttons(&app, row);
    assert_eq!(buttons.len(), 1, "one button for the one action");
    assert_eq!(label_of(&app, buttons[0]), "Undo");
    assert!(
        row_words(&app, row).ends_with("Moved the bed"),
        "{}",
        row_words(&app, row)
    );
    assert!(
        row_buttons(&app, row_of(&app, center, first)).is_empty(),
        "an entry with none has none"
    );
    // The button's own click: one action, with the entry's id and the action's.
    {
        let at = rect(&app, buttons[0]).center();
        click_at(&mut app, at);
    }
    assert_eq!(sent(&mut app), vec![(entry, "undo".to_string())]);
    // The row's words: nothing is sent, and neither is it for the row's blank end.
    let words = descendants(app.world(), row)
        .into_iter()
        .find(|&e| {
            app.world()
                .get::<Text>(e)
                .is_some_and(|t| t.0.ends_with("Moved the bed"))
        })
        .expect("the words");
    let at = Vec2::new(rect(&app, words).min.x + 6.0, rect(&app, words).center().y);
    click_at(&mut app, at);
    assert!(
        sent(&mut app).is_empty(),
        "a click on a line of history undoes nothing"
    );
}

#[test]
fn an_action_button_is_reached_and_pressed_with_the_keys() {
    let mut app = app();
    let entry = push_with_action(&mut app, "Moved the bed", "Undo", "undo");
    center(&mut app);
    let mut seen = Vec::new();
    for _ in 0..4 {
        bevy_gooey::testing::press(&mut app, KeyCode::Tab);
        seen.push(bevy_gooey::testing::focused_label(&app));
        if seen.last() == Some(&Some("Undo".to_string())) {
            break;
        }
    }
    assert_eq!(
        seen.last(),
        Some(&Some("Undo".to_string())),
        "Tab reaches it: {seen:?}"
    );
    bevy_gooey::testing::press(&mut app, KeyCode::Enter);
    assert_eq!(sent(&mut app), vec![(entry, "undo".to_string())]);
}

#[test]
fn a_hidden_center_has_no_stops_and_a_shown_one_has_them() {
    let mut app = app();
    push_with_action(&mut app, "Moved the bed", "Undo", "undo");
    let center = center(&mut app);
    app.world_mut().entity_mut(center).insert(Shown(false));
    frames(&mut app, 3);
    let inside = descendants(app.world(), center);
    for _ in 0..6 {
        bevy_gooey::testing::press(&mut app, KeyCode::Tab);
        let at = bevy_gooey::testing::focused(&app);
        assert!(
            at.is_none_or(|e| !inside.contains(&e)),
            "focus went into a hidden center"
        );
    }
    // A row added while it is hidden is no stop either.
    push_with_action(&mut app, "Moved the lamp", "Undo", "undo");
    frames(&mut app, 3);
    let inside = descendants(app.world(), center);
    for _ in 0..6 {
        bevy_gooey::testing::press(&mut app, KeyCode::Tab);
        let at = bevy_gooey::testing::focused(&app);
        assert!(
            at.is_none_or(|e| !inside.contains(&e)),
            "focus went into a row added while hidden"
        );
    }
    app.world_mut().entity_mut(center).insert(Shown(true));
    frames(&mut app, 3);
    let mut reached = false;
    for _ in 0..8 {
        bevy_gooey::testing::press(&mut app, KeyCode::Tab);
        reached |= bevy_gooey::testing::focused_label(&app).as_deref() == Some("Undo");
    }
    assert!(reached, "shown again, the buttons are reachable");
}

#[test]
fn the_buttons_stand_at_the_rows_end() {
    let mut app = app();
    let entry = push_with_action(&mut app, "Moved the bed", "Undo", "undo");
    let center = center(&mut app);
    let row = row_of(&app, center, entry);
    let button = row_buttons(&app, row)[0];
    let (r, b) = (rect(&app, row), rect(&app, button));
    assert!(
        r.max.x - b.max.x < 40.0,
        "at the end: row {r:?} button {b:?}"
    );
    assert!(
        b.min.x > r.center().x,
        "past the words: row {r:?} button {b:?}"
    );
}

// ---- 2. the count, in place -------------------------------------------------------------------

#[test]
fn an_entry_updated_in_place_reads_its_count_and_the_row_count_does_not_change() {
    let mut app = app();
    let a = push(&mut app, ToastKind::Alert, "Cannot move the bed");
    let b = push(&mut app, ToastKind::Info, "Opened");
    let center = center(&mut app);
    let before = row_entities(&app, center);
    assert_eq!(before.len(), 2);
    assert!(!row_words(&app, row_of(&app, center, a)).contains("(x"));
    app.world_mut()
        .resource_mut::<ToastLog>()
        .get_mut(a)
        .expect("entry")
        .set_count(3);
    frames(&mut app, 3);
    let after = row_entities(&app, center);
    assert_eq!(after.len(), 2, "no row added or lost");
    assert_eq!(
        after.iter().map(|r| r.0).collect::<Vec<_>>(),
        before.iter().map(|r| r.0).collect::<Vec<_>>(),
        "same order"
    );
    assert!(
        row_words(&app, row_of(&app, center, a)).ends_with("! Cannot move the bed  (x3)"),
        "{}",
        row_words(&app, row_of(&app, center, a))
    );
    assert!(!row_words(&app, row_of(&app, center, b)).contains("(x"));
    // Back to one: the count goes.
    app.world_mut()
        .resource_mut::<ToastLog>()
        .get_mut(a)
        .expect("entry")
        .set_count(1);
    frames(&mut app, 3);
    assert!(row_words(&app, row_of(&app, center, a)).ends_with("! Cannot move the bed"));
}

// ---- 3. unlogged and logged_as ----------------------------------------------------------------

#[test]
fn an_unlogged_toast_adds_no_row_and_a_logged_one_does_with_its_entry() {
    let mut app = app();
    let center = center(&mut app);
    app.world_mut().spawn(toast("Quiet").unlogged());
    frames(&mut app, 4);
    assert_eq!(
        app.world().resource::<ToastLog>().len(),
        0,
        "nothing was logged"
    );
    assert_eq!(
        row_entities(&app, center),
        vec![(None, row_entities(&app, center)[0].1)],
        "only the empty line"
    );
    let shown = app
        .world_mut()
        .query::<&Toast>()
        .iter(app.world())
        .next()
        .expect("it was shown")
        .entry();
    assert_eq!(shown, None, "an unlogged toast has no entry");
    let plain = app.world_mut().spawn(toast("Loud")).id();
    frames(&mut app, 4);
    let log = app.world().resource::<ToastLog>();
    assert_eq!(log.len(), 1);
    let id = log.entries().next().expect("one").id();
    assert_eq!(
        app.world().get::<Toast>(plain).expect("toast").entry(),
        Some(id)
    );
    assert_eq!(row_entities(&app, center).len(), 1);
}

#[test]
fn a_toast_logged_as_an_existing_entry_adds_no_row_and_names_it() {
    let mut app = app();
    let entry = push(&mut app, ToastKind::Alert, "Cannot move the bed");
    let center = center(&mut app);
    let before = row_entities(&app, center);
    let toast = app
        .world_mut()
        .spawn(toast_alert("Cannot move the bed").logged_as(entry))
        .id();
    frames(&mut app, 4);
    assert_eq!(
        app.world().resource::<ToastLog>().len(),
        1,
        "no second entry"
    );
    assert_eq!(
        row_entities(&app, center),
        before,
        "no second row, the first untouched"
    );
    assert_eq!(
        app.world().get::<Toast>(toast).expect("toast").entry(),
        Some(entry)
    );
}

// ---- 4. ids -----------------------------------------------------------------------------------

#[test]
fn a_capped_log_keeps_its_ids_and_an_action_on_a_drawn_row_names_its_entry() {
    let mut app = app();
    let mut ids = Vec::new();
    for n in 0..ToastLog::CAPACITY + 5 {
        ids.push(push(&mut app, ToastKind::Info, &format!("Entry {n}")));
    }
    assert!(ids.windows(2).all(|w| w[0] < w[1]), "ids only go up");
    let log = app.world().resource::<ToastLog>();
    assert_eq!(log.len(), ToastLog::CAPACITY);
    assert!(
        log.get(ids[0]).is_none() && log.get(ids[4]).is_none(),
        "the oldest five are gone"
    );
    assert_eq!(
        log.get(ids[5]).map(|r| r.text.clone()),
        Some("Entry 5".to_string()),
        "the rest keep their ids"
    );
    let newest = *ids.last().expect("ids");
    app.world_mut()
        .resource_mut::<ToastLog>()
        .get_mut(newest)
        .expect("entry")
        .add_action("Undo", "undo");
    let center = center(&mut app);
    let row = row_of(&app, center, newest);
    {
        let at = rect(&app, row_buttons(&app, row)[0]).center();
        click_at(&mut app, at);
    }
    assert_eq!(sent(&mut app), vec![(newest, "undo".to_string())]);
}

#[test]
fn ids_are_never_reused_after_a_remove_or_a_clear() {
    let mut app = app();
    let mut log = ToastLog::default();
    let entry = |text: &str| ToastLogEntry {
        seconds: 1.0,
        kind: ToastKind::Info,
        text: text.into(),
    };
    let a = log.push(entry("a"));
    let b = log.push(entry("b"));
    assert_ne!(a, b);
    assert_eq!(log.remove(b).map(|r| r.id()), Some(b));
    assert!(log.get(b).is_none() && log.remove(b).is_none(), "gone");
    let c = log.push(entry("c"));
    assert!(c > b, "{c} after {b}");
    log.clear();
    let d = log.push(entry("d"));
    assert!(d > c, "{d} after {c}");
    app.insert_resource(log);
}

// ---- 5. rows that did not change are not respawned --------------------------------------------

#[test]
fn rows_that_did_not_change_are_the_same_entities_after_the_log_changes() {
    let mut app = app();
    let a = push(&mut app, ToastKind::Info, "One");
    let b = push_with_action(&mut app, "Two", "Undo", "undo");
    let c = push(&mut app, ToastKind::Info, "Three");
    let center = center(&mut app);
    let (ra, rb, rc) = (
        row_of(&app, center, a),
        row_of(&app, center, b),
        row_of(&app, center, c),
    );
    // A new newest: the old rows stay, the new one is first.
    let d = push(&mut app, ToastKind::Info, "Four");
    frames(&mut app, 3);
    let rows = row_entities(&app, center);
    assert_eq!(
        rows.iter().map(|r| r.0).collect::<Vec<_>>(),
        vec![Some(d), Some(c), Some(b), Some(a)]
    );
    assert_eq!(
        (rows[1].1, rows[2].1, rows[3].1),
        (rc, rb, ra),
        "the same entities"
    );
    // One count changes: only that row is rebuilt.
    app.world_mut()
        .resource_mut::<ToastLog>()
        .get_mut(b)
        .expect("entry")
        .set_count(2);
    frames(&mut app, 3);
    let rows = row_entities(&app, center);
    assert_eq!((rows[1].1, rows[3].1), (rc, ra), "the others are the same");
    assert_ne!(rows[2].1, rb, "the changed one is rebuilt");
    assert!(app.world().get_entity(rb).is_err());
    // One is removed: only its row goes.
    app.world_mut().resource_mut::<ToastLog>().remove(c);
    frames(&mut app, 3);
    let rows = row_entities(&app, center);
    assert_eq!(
        rows.iter().map(|r| r.0).collect::<Vec<_>>(),
        vec![Some(d), Some(b), Some(a)]
    );
    assert_eq!(rows[2].1, ra);
    assert!(app.world().get_entity(rc).is_err());
}

#[test]
fn an_action_added_or_taken_off_after_the_row_was_drawn_shows() {
    let mut app = app();
    let entry = push(&mut app, ToastKind::Info, "Moved the bed");
    let center = center(&mut app);
    assert!(row_buttons(&app, row_of(&app, center, entry)).is_empty());
    app.world_mut()
        .resource_mut::<ToastLog>()
        .get_mut(entry)
        .expect("entry")
        .add_action("Undo", "undo");
    frames(&mut app, 3);
    assert_eq!(row_buttons(&app, row_of(&app, center, entry)).len(), 1);
    app.world_mut()
        .resource_mut::<ToastLog>()
        .get_mut(entry)
        .expect("entry")
        .clear_actions();
    frames(&mut app, 3);
    assert!(row_buttons(&app, row_of(&app, center, entry)).is_empty());
}

#[test]
fn a_focused_action_button_whose_row_goes_does_not_leave_focus_on_nothing() {
    let mut app = app();
    let entry = push_with_action(&mut app, "Moved the bed", "Undo", "undo");
    center(&mut app);
    for _ in 0..4 {
        bevy_gooey::testing::press(&mut app, KeyCode::Tab);
        if bevy_gooey::testing::focused_label(&app).as_deref() == Some("Undo") {
            break;
        }
    }
    assert_eq!(
        bevy_gooey::testing::focused_label(&app).as_deref(),
        Some("Undo")
    );
    app.world_mut().resource_mut::<ToastLog>().remove(entry);
    frames(&mut app, 3);
    assert_eq!(bevy_gooey::testing::focused(&app), None);
}

// ---- 6. inputs that mean nothing are ignored ---------------------------------------------------

#[test]
fn empty_and_non_finite_inputs_are_ignored() {
    let mut log = ToastLog::default();
    let id = log.push(ToastLogEntry {
        seconds: f32::NAN,
        kind: ToastKind::Info,
        text: "x".into(),
    });
    assert_eq!(
        log.get(id).expect("entry").seconds,
        0.0,
        "a time that is not a number is 0"
    );
    let record = log.get_mut(id).expect("entry");
    record.set_count(0);
    assert_eq!(record.count(), 1, "a count of none is no count");
    record.add_action("", "undo");
    record.add_action("Undo", "");
    assert!(record.actions().is_empty());
    record.add_action("Undo", "undo");
    record.add_action("Undo", "undo");
    assert_eq!(record.actions().len(), 1, "the same action twice is one");
    assert!(
        log.get_mut(id + 99).is_none()
            && log.get(id + 99).is_none()
            && log.remove(id + 99).is_none()
    );
}

// ---- 7. the Note kind -------------------------------------------------------------------------

#[test]
fn a_note_is_a_third_kind_with_its_own_mark_and_no_alert_bang() {
    let mut app = app();
    app.insert_resource(ToastSettings {
        merge_seconds: 0.0,
        ..ToastSettings::default()
    });
    let center = center(&mut app);
    let note = app.world_mut().spawn(toast_note("Saved")).id();
    let info = app.world_mut().spawn(toast("Opened")).id();
    let alert = app.world_mut().spawn(toast_alert("Failed")).id();
    frames(&mut app, 10);
    assert_eq!(
        app.world().get::<Toast>(note).expect("toast").kind(),
        ToastKind::Note
    );
    let mark = |app: &App, toast: Entity| {
        descendants(app.world(), toast)
            .into_iter()
            .find(|&e| app.world().get::<Role>(e) == Some(&Role::ToastMark))
            .and_then(|e| app.world().get::<BackgroundColor>(e).map(|c| c.0))
            .expect("a mark")
    };
    let (n, i, a) = (mark(&app, note), mark(&app, info), mark(&app, alert));
    assert!(n != i && n != a && i != a, "three marks: {n:?} {i:?} {a:?}");
    let rows = row_entities(&app, center);
    let words: Vec<String> = rows.iter().map(|r| row_words(&app, r.1)).collect();
    assert!(
        words[2].ends_with("Saved") && !words[2].contains('!'),
        "{words:?}"
    );
    assert!(words[0].contains("! Failed"), "{words:?}");
}
