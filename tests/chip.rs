//! The chip (iteration 105, `docs/research/chip.md`): one small clickable word for a filter or a
//! tag. Each test lays chips out for real in a headless window and measures them on screen; the skin
//! owns every size and colour. The layout tests are the contract a host's tag block had been
//! hand-rolling: a chip's words break at a word so it narrows before its row wraps (a `NoWrap`
//! label made it as wide as its whole phrase and left a wrapping row nothing to wrap), and a chip
//! is never wider than its host.
#![cfg(feature = "testing")]

use bevy::camera::NormalizedRenderTarget;
use bevy::input_focus::{FocusCause, InputFocus, InputFocusVisible};
use bevy::picking::hover::Hovered;
use bevy::picking::pointer::{Location, PointerAction, PointerId, PointerInput};
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::ui::{InteractionDisabled, IsDefaultUiCamera, Pressed, Selected, UiGlobalTransform};
use bevy::window::{PrimaryWindow, WindowRef, WindowResolution};
use bevy::winit::WinitPlugin;
use bevy_gooey::testing::{click, press};
use bevy_gooey::*;

/// How many times a chip's `Activate` was heard, and the `ValueChange<bool>`s a toggle sent.
#[derive(Resource, Default)]
struct Heard {
    activates: usize,
    changes: Vec<bool>,
}

fn app_with(skin: GooeySkin) -> App {
    let mut app = App::new();
    app.insert_resource(MotionSettings::off());
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
    .insert_resource(skin)
    .init_resource::<Heard>();
    app.world_mut().spawn((Camera2d, IsDefaultUiCamera));
    app
}

fn settle(app: &mut App) {
    for _ in 0..4 {
        app.update();
    }
}

fn size(app: &App, entity: Entity) -> Vec2 {
    let node = app.world().get::<ComputedNode>(entity).expect("laid out");
    node.size() * node.inverse_scale_factor()
}

/// A node's rectangle in logical px: `(left, top, right, bottom)`.
fn rect(app: &App, entity: Entity) -> (f32, f32, f32, f32) {
    let node = app.world().get::<ComputedNode>(entity).expect("laid out");
    let k = node.inverse_scale_factor();
    let at = app
        .world()
        .get::<UiGlobalTransform>(entity)
        .expect("placed")
        .translation
        * k;
    let half = node.size() * k / 2.0;
    (at.x - half.x, at.y - half.y, at.x + half.x, at.y + half.y)
}

fn fill(app: &App, entity: Entity) -> Color {
    app.world()
        .get::<BackgroundColor>(entity)
        .map(|b| b.0)
        .expect("a fill")
}

fn frame(app: &App, entity: Entity) -> Color {
    app.world()
        .get::<BorderColor>(entity)
        .map(|b| b.top)
        .expect("a frame")
}

fn ink(app: &App, chip: Entity) -> Color {
    let words = app
        .world()
        .get::<Children>(chip)
        .expect("a child")
        .iter()
        .find(|kid| app.world().get::<Role>(*kid) == Some(&Role::ChipText))
        .expect("a ChipText child");
    app.world().get::<TextColor>(words).expect("ink").0
}

/// A chip in a host `width` px wide (a column that does not shrink), on a screen.
fn chip_in(app: &mut App, label: &str, width: f32) -> (Entity, Entity) {
    let host = app
        .world_mut()
        .spawn(Node {
            width: Val::Px(width),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            ..default()
        })
        .id();
    let chip = app.world_mut().spawn(chip(label)).id();
    app.world_mut().entity_mut(host).add_child(chip);
    app.world_mut().spawn(screen()).add_child(host);
    settle(app);
    (host, chip)
}

fn point_at(app: &mut App, entity: Entity) {
    let node = app.world().get::<ComputedNode>(entity).expect("laid out");
    let at = app
        .world()
        .get::<UiGlobalTransform>(entity)
        .expect("placed")
        .translation
        * node.inverse_scale_factor();
    let window = app
        .world_mut()
        .query_filtered::<Entity, With<PrimaryWindow>>()
        .single(app.world())
        .expect("a window");
    let location = Location {
        target: NormalizedRenderTarget::Window(
            WindowRef::Entity(window)
                .normalize(None)
                .expect("a window ref"),
        ),
        position: at,
    };
    app.world_mut().write_message(PointerInput::new(
        PointerId::Mouse,
        location,
        PointerAction::Move { delta: Vec2::X },
    ));
    for _ in 0..3 {
        app.update();
    }
}

fn hear(_: On<Activate>, mut heard: ResMut<Heard>) {
    heard.activates += 1;
}

fn hear_change(change: On<ValueChange<bool>>, mut heard: ResMut<Heard>) {
    heard.changes.push(change.value);
}

const PHRASE: &str = "NOT IMPORTED YET BECAUSE OF A FAILURE (4)";

// ---- layout -----------------------------------------------------------------------------------

#[test]
fn a_chip_narrows_and_wraps_its_words_inside_a_host_narrower_than_its_phrase() {
    let mut app = app_with(GooeySkin::dark());
    let (_, whole) = chip_in(&mut app, PHRASE, 2000.0);
    let phrase = size(&app, whole);
    let mut app = app_with(GooeySkin::dark());
    let (host, narrow) = chip_in(&mut app, PHRASE, 180.0);
    let held = size(&app, narrow);
    assert!(
        phrase.x > 300.0,
        "the test needs a phrase wider than its host: {phrase}"
    );
    assert!(
        held.x <= 180.5,
        "a chip in a 180 px host is {} px wide",
        held.x
    );
    assert!(
        held.y > phrase.y * 1.5,
        "the words did not wrap: {held} against {phrase}"
    );
    let (hl, _, hr, _) = rect(&app, host);
    let (cl, _, cr, _) = rect(&app, narrow);
    assert!(cl >= hl - 0.5 && cr <= hr + 0.5, "the chip left its host");
}

#[test]
fn a_name_with_no_space_to_break_at_breaks_at_a_letter_inside_its_host() {
    let mut app = app_with(GooeySkin::dark());
    let path = "kenney_prototype_kit_models_environment_props_barrels_and_crates_dark_01";
    let (host, chip) = chip_in(&mut app, path, 200.0);
    let (hl, _, hr, _) = rect(&app, host);
    let (cl, _, cr, _) = rect(&app, chip);
    assert!(
        cl >= hl - 0.5 && cr <= hr + 0.5,
        "an unbroken name left its 200 px host: chip {cl}..{cr}, host {hl}..{hr}"
    );
}

#[test]
fn a_wrapping_row_of_chips_makes_a_second_line_and_keeps_every_chip_inside() {
    let mut app = app_with(GooeySkin::dark());
    let host = app
        .world_mut()
        .spawn((row_with(
            1.0,
            RowLayout {
                wrap: Some(FlexWrap::Wrap),
                align: Some(AlignItems::FlexStart),
                ..default()
            },
        ),))
        .id();
    app.world_mut()
        .entity_mut(host)
        .entry::<Node>()
        .and_modify(|mut node| {
            node.width = Val::Px(300.0);
        });
    let chips: Vec<Entity> = [
        "Weapons (12)",
        "Armour (9)",
        "Materials (40)",
        "Potions (3)",
        "Quest items (2)",
        "Not imported (4)",
    ]
    .iter()
    .map(|name| app.world_mut().spawn(chip(*name)).id())
    .collect();
    for chip in &chips {
        app.world_mut().entity_mut(host).add_child(*chip);
    }
    app.world_mut().spawn(screen()).add_child(host);
    settle(&mut app);
    let (hl, _, hr, _) = rect(&app, host);
    let mut tops: Vec<i32> = Vec::new();
    for chip in &chips {
        let (l, t, r, _) = rect(&app, *chip);
        assert!(
            l >= hl - 0.5 && r <= hr + 0.5,
            "a chip is outside its row: {l}..{r} in {hl}..{hr}"
        );
        tops.push(t.round() as i32);
    }
    // A chip that fits a line goes to the next line whole: none wraps its own words.
    let one_line = size(&app, chips[1]).y;
    for chip in &chips {
        assert!(
            size(&app, *chip).y <= one_line + 0.5,
            "a chip wrapped its words in a row that had a line for it"
        );
    }
    tops.sort_unstable();
    tops.dedup();
    assert!(
        tops.len() >= 2,
        "six chips wider than 300 px in all stayed on one line: {tops:?}"
    );
}

#[test]
fn a_chip_is_never_smaller_than_the_skins_target() {
    for text_px in [6.0, 14.0, 48.0] {
        let mut skin = GooeySkin::dark();
        skin.text_size = text_px;
        let floor = skin.target();
        let mut app = app_with(skin);
        let (_, chip) = chip_in(&mut app, "x", 600.0);
        let hit = size(&app, chip);
        assert!(
            hit.y >= floor - 0.01,
            "a chip of {text_px} px text is {} px high, under the floor {floor}",
            hit.y
        );
    }
}

// ---- state ------------------------------------------------------------------------------------

#[test]
fn a_chip_takes_its_fill_frame_and_ink_from_the_skin_by_state_and_never_changes_size() {
    for preset in SkinPreset::ALL {
        let skin = preset.skin();
        let mut app = app_with(skin.clone());
        let (_, chip) = chip_in(&mut app, "Weapons", 400.0);
        let rest_size = size(&app, chip);
        let name = preset.name();
        assert_eq!(fill(&app, chip), skin.surface_raised, "{name} rest");
        assert_eq!(frame(&app, chip), skin.outline, "{name} rest frame");
        assert_eq!(ink(&app, chip), skin.text, "{name} rest ink");

        app.world_mut().entity_mut(chip).insert(Selected);
        settle(&mut app);
        assert_eq!(fill(&app, chip), skin.accent, "{name} selected");
        assert_eq!(frame(&app, chip), skin.accent, "{name} selected frame");
        assert_eq!(ink(&app, chip), skin.accent_text, "{name} selected ink");
        assert_eq!(size(&app, chip), rest_size, "{name}: selecting moved size");

        app.world_mut().entity_mut(chip).insert(Pressed);
        settle(&mut app);
        assert_eq!(
            fill(&app, chip),
            skin.primary_pressed(),
            "{name} selected pressed"
        );
        app.world_mut()
            .entity_mut(chip)
            .remove::<(Pressed, Selected)>();
        settle(&mut app);
        app.world_mut().entity_mut(chip).insert(Pressed);
        settle(&mut app);
        assert_eq!(fill(&app, chip), skin.accent, "{name} pressed");
        assert_eq!(ink(&app, chip), skin.accent_text, "{name} pressed ink");
        app.world_mut().entity_mut(chip).remove::<Pressed>();
        settle(&mut app);

        point_at(&mut app, chip);
        assert_eq!(fill(&app, chip), skin.hovered(), "{name} hovered");
        assert_eq!(size(&app, chip), rest_size, "{name}: hover moved size");

        app.world_mut().entity_mut(chip).insert(InteractionDisabled);
        settle(&mut app);
        assert_eq!(fill(&app, chip), skin.surface, "{name} disabled");
        assert_eq!(ink(&app, chip), skin.text_muted, "{name} disabled ink");
        assert_eq!(size(&app, chip), rest_size, "{name}: disabling moved size");
    }
}

#[test]
fn a_chips_ink_holds_contrast_on_every_fill_it_stands_on_in_every_preset() {
    fn rgb(color: Color) -> [f32; 3] {
        let c = color.to_srgba();
        [c.red, c.green, c.blue]
    }
    for preset in SkinPreset::ALL {
        let skin = preset.skin();
        let mut app = app_with(skin.clone());
        let (_, chip) = chip_in(&mut app, "Weapons", 400.0);
        let check = |app: &App, what: &str| {
            let ratio = contrast_ratio(rgb(fill(app, chip)), rgb(ink(app, chip)));
            assert!(
                ratio >= 4.5,
                "{} {what}: {ratio:.2}:1 is under 4.5:1",
                preset.name()
            );
        };
        check(&app, "rest");
        app.world_mut().entity_mut(chip).insert(Selected);
        settle(&mut app);
        check(&app, "selected");
        point_at(&mut app, chip);
        check(&app, "selected hovered");
        app.world_mut().entity_mut(chip).insert(Pressed);
        settle(&mut app);
        check(&app, "selected pressed");
        app.world_mut()
            .entity_mut(chip)
            .remove::<(Pressed, Selected)>();
        settle(&mut app);
        point_at(&mut app, chip);
        check(&app, "hovered");
    }
}

#[test]
fn the_focus_ring_is_the_chips_frame_while_it_holds_visible_focus() {
    let skin = GooeySkin::dark();
    let mut app = app_with(skin.clone());
    let (_, chip) = chip_in(&mut app, "Weapons", 400.0);
    app.world_mut()
        .resource_mut::<InputFocus>()
        .set(chip, FocusCause::Navigated);
    app.insert_resource(InputFocusVisible(true));
    settle(&mut app);
    assert_eq!(frame(&app, chip), skin.focus_ring);
    app.insert_resource(InputFocusVisible(false));
    settle(&mut app);
    assert_eq!(frame(&app, chip), skin.outline);
}

// ---- a chip is a button -----------------------------------------------------------------------

#[test]
fn a_click_enter_and_space_each_send_one_activate_and_a_disabled_chip_none() {
    let mut app = app_with(GooeySkin::dark());
    let (_, chip) = chip_in(&mut app, "Weapons", 400.0);
    app.world_mut().entity_mut(chip).observe(hear);
    click(&mut app, chip);
    assert_eq!(app.world().resource::<Heard>().activates, 1, "a click");
    press(&mut app, KeyCode::Enter);
    assert_eq!(app.world().resource::<Heard>().activates, 2, "Enter");
    press(&mut app, KeyCode::Space);
    assert_eq!(app.world().resource::<Heard>().activates, 3, "Space");
    app.world_mut().entity_mut(chip).insert(InteractionDisabled);
    settle(&mut app);
    click(&mut app, chip);
    press(&mut app, KeyCode::Enter);
    assert_eq!(app.world().resource::<Heard>().activates, 3, "disabled");
}

#[test]
fn a_plain_chip_never_flips_itself() {
    let mut app = app_with(GooeySkin::dark());
    let (_, chip) = chip_in(&mut app, "Weapons", 400.0);
    app.world_mut().entity_mut(chip).observe(hear);
    click(&mut app, chip);
    assert!(app.world().get::<Selected>(chip).is_none());
    app.world_mut().entity_mut(chip).insert(Selected);
    click(&mut app, chip);
    assert!(app.world().get::<Selected>(chip).is_some());
    assert_eq!(app.world().resource::<Heard>().activates, 2);
}

#[test]
fn a_toggle_chip_flips_and_reports_once_a_click() {
    let mut app = app_with(GooeySkin::dark());
    let chip = app
        .world_mut()
        .spawn((toggle_chip("Armour", false), observe(hear_change)))
        .id();
    app.world_mut().spawn(screen()).add_child(chip);
    settle(&mut app);
    assert!(app.world().get::<Selected>(chip).is_none());
    click(&mut app, chip);
    assert_eq!(app.world().resource::<Heard>().changes, vec![true]);
    assert!(app.world().get::<Selected>(chip).is_some());
    click(&mut app, chip);
    assert_eq!(app.world().resource::<Heard>().changes, vec![true, false]);
    assert!(app.world().get::<Selected>(chip).is_none());
    // The game writes the state: the chip moves and says nothing.
    app.world_mut().entity_mut(chip).insert(Selected);
    settle(&mut app);
    assert_eq!(app.world().resource::<Heard>().changes.len(), 2);
    // The pointer is still over it from the click, so the lit fill is the hovered one.
    assert_eq!(
        fill(&app, chip),
        GooeySkin::dark().primary_hovered(),
        "a chip the game lit is lit"
    );
}

#[test]
fn a_toggle_chip_spawned_on_starts_lit_and_says_nothing() {
    let mut app = app_with(GooeySkin::dark());
    let chip = app
        .world_mut()
        .spawn((toggle_chip("Armour", true), observe(hear_change)))
        .id();
    app.world_mut().spawn(screen()).add_child(chip);
    settle(&mut app);
    assert!(app.world().get::<Selected>(chip).is_some());
    assert!(app.world().resource::<Heard>().changes.is_empty());
    assert_eq!(fill(&app, chip), GooeySkin::dark().accent);
}

#[test]
fn a_disabled_toggle_chip_does_not_flip() {
    let mut app = app_with(GooeySkin::dark());
    let chip = app
        .world_mut()
        .spawn((
            toggle_chip("Armour", false),
            InteractionDisabled,
            observe(hear_change),
        ))
        .id();
    app.world_mut().spawn(screen()).add_child(chip);
    settle(&mut app);
    click(&mut app, chip);
    assert!(app.world().get::<Selected>(chip).is_none());
    assert!(app.world().resource::<Heard>().changes.is_empty());
}

// ---- roles and the rest -----------------------------------------------------------------------

#[test]
fn a_chips_words_are_not_a_buttons() {
    let mut app = app_with(GooeySkin::dark());
    let (_, _) = chip_in(&mut app, "Weapons", 400.0);
    app.world_mut().spawn((screen(), children![button("OK")]));
    settle(&mut app);
    let buttons = app
        .world_mut()
        .query::<&Role>()
        .iter(app.world())
        .filter(|role| **role == Role::ButtonText)
        .count();
    assert_eq!(buttons, 1, "a chip's words counted as ButtonText");
}

#[test]
fn a_settled_chip_writes_nothing() {
    let mut app = app_with(GooeySkin::dark());
    let (_, chip) = chip_in(&mut app, PHRASE, 180.0);
    app.world_mut().entity_mut(chip).insert(Selected);
    settle(&mut app);
    let since = app.world().read_change_tick();
    for _ in 0..30 {
        app.update();
    }
    let now = app.world().read_change_tick();
    let entity = app.world().entity(chip);
    let changed = |ticks: Option<bevy::ecs::change_detection::ComponentTicks>| {
        ticks.is_some_and(|t| t.is_changed(since, now))
    };
    assert!(
        !changed(entity.get_change_ticks::<Node>()),
        "Node written while idle"
    );
    assert!(
        !changed(entity.get_change_ticks::<BackgroundColor>()),
        "BackgroundColor written while idle"
    );
    assert!(
        !changed(entity.get_change_ticks::<BorderColor>()),
        "BorderColor written while idle"
    );
}

/// The contract `dark-incorporated/bevy_dark_factory` asked for (#119): its `chrome::chip` can be
/// gooey's chip plus a host marker, because the node carries `Role::Chip`, the skin's chip arm
/// styles its fill and frame, and it is named by its words. Held against three skins, as their
/// test does, so a skin that stops styling a chip fails here and not in the editor.
#[test]
fn a_host_can_spawn_gooeys_chip_and_only_add_its_own_marker() {
    /// A host's own name for the thing, as `chrome::Chip` is bevy_dark_factory's.
    #[derive(Component)]
    struct HostChip;

    for skin in [GooeySkin::dark(), GooeySkin::parchment(), GooeySkin::tool()] {
        let mut app = app_with(skin.clone());
        app.world_mut().spawn(screen()).with_children(|screen| {
            screen.spawn((chip("armour"), HostChip));
        });
        settle(&mut app);
        app.world_mut()
            .resource::<bevy::a11y::AccessibilityRequested>()
            .set(true);
        settle(&mut app);
        let found = app
            .world_mut()
            .query_filtered::<Entity, With<HostChip>>()
            .single(app.world())
            .expect("the host's chip");
        assert_eq!(
            app.world().get::<Role>(found),
            Some(&Role::Chip),
            "the node a host spawns carries the role the skin styles"
        );
        let fill = app.world().get::<BackgroundColor>(found).expect("a fill").0;
        let edge = app.world().get::<BorderColor>(found).expect("a frame");
        assert_eq!(
            fill, skin.surface_raised,
            "{:?}: a chip off is the raised fill",
            skin.background
        );
        assert_ne!(edge.top, Color::NONE, "a chip is a frame all round");
        // And it is named by its words, not left to the host.
        let name = app
            .world()
            .get::<bevy::a11y::AccessibilityNode>(found)
            .expect("a node, once a reader asks")
            .0
            .label()
            .map(str::to_owned);
        assert_eq!(name.as_deref(), Some("armour"), "the words are its name");
    }
}

// ---- a chip with no tab stop (#121) -----------------------------------------------------------

/// A `quiet_chip` in a host `width` px wide, on a screen, with a `chip` beside it.
fn quiet_chip_in(app: &mut App, label: &str, width: f32) -> (Entity, Entity) {
    let host = app
        .world_mut()
        .spawn(Node {
            width: Val::Px(width),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            ..default()
        })
        .id();
    let quiet = app.world_mut().spawn(quiet_chip(label)).id();
    app.world_mut().entity_mut(host).add_child(quiet);
    app.world_mut().spawn(screen()).add_child(host);
    settle(app);
    (host, quiet)
}

/// Focus `entity` the way Tab would, and show the ring.
fn focus(app: &mut App, entity: Entity) {
    app.world_mut()
        .resource_mut::<InputFocus>()
        .set(entity, FocusCause::Navigated);
    app.insert_resource(InputFocusVisible(true));
    settle(app);
}

fn focused(app: &App) -> Option<Entity> {
    app.world().resource::<InputFocus>().get()
}

#[test]
fn a_quiet_chip_is_a_chip_with_no_button_no_tab_stop_and_no_directional_navigation() {
    let mut app = app_with(GooeySkin::dark());
    let (_, quiet) = quiet_chip_in(&mut app, "Armour", 400.0);
    let world = app.world();
    assert_eq!(
        world.get::<Role>(quiet),
        Some(&Role::Chip),
        "the quiet chip carries the role the skin draws"
    );
    assert!(
        world.get::<Hovered>(quiet).is_some(),
        "the pointer must still answer it"
    );
    assert!(
        world.get::<bevy::ui_widgets::Button>(quiet).is_none(),
        "no Button: no Activate, no pressed state"
    );
    assert!(
        world
            .get::<bevy::input_focus::tab_navigation::TabIndex>(quiet)
            .is_none(),
        "no TabIndex: not a tab stop"
    );
    assert!(
        world
            .get::<bevy::ui::auto_directional_navigation::AutoDirectionalNavigation>(quiet)
            .is_none(),
        "no directional navigation of its own"
    );
    // And its words are the chip's own part, with the chip's break rule.
    let words = world
        .get::<Children>(quiet)
        .expect("a child")
        .iter()
        .find(|kid| world.get::<Role>(*kid) == Some(&Role::ChipText))
        .expect("a ChipText child");
    assert_eq!(
        world.get::<Text>(words).map(|text| text.0.clone()),
        Some("Armour".to_string())
    );
}

#[test]
fn a_quiet_chip_looks_exactly_like_a_chip_in_every_preset() {
    for preset in SkinPreset::ALL {
        let skin = preset.skin();
        let name = preset.name();
        let mut loud = app_with(skin.clone());
        let (_, chip) = chip_in(&mut loud, "Weapons", 400.0);
        let mut quiet = app_with(skin.clone());
        let (_, quiet_chip) = quiet_chip_in(&mut quiet, "Weapons", 400.0);
        assert_eq!(
            fill(&quiet, quiet_chip),
            fill(&loud, chip),
            "{name}: rest fill"
        );
        assert_eq!(
            frame(&quiet, quiet_chip),
            frame(&loud, chip),
            "{name}: rest frame"
        );
        assert_eq!(
            ink(&quiet, quiet_chip),
            ink(&loud, chip),
            "{name}: rest ink"
        );
        assert_eq!(
            size(&quiet, quiet_chip),
            size(&loud, chip),
            "{name}: the same words and the same box"
        );
        assert_eq!(
            fill(&quiet, quiet_chip),
            skin.surface_raised,
            "{name}: the raised fill"
        );
        assert_eq!(ink(&quiet, quiet_chip), skin.text, "{name}: the text ink");
        assert_eq!(
            frame(&quiet, quiet_chip),
            skin.outline,
            "{name}: the outline frame"
        );
        // The host's `Selected` is the lit look, and nothing in the crate sends anything when it
        // arrives or leaves.
        quiet.world_mut().entity_mut(quiet_chip).observe(hear);
        quiet
            .world_mut()
            .entity_mut(quiet_chip)
            .observe(hear_change);
        quiet.world_mut().entity_mut(quiet_chip).insert(Selected);
        settle(&mut quiet);
        assert_eq!(
            fill(&quiet, quiet_chip),
            skin.accent,
            "{name}: selected fill"
        );
        assert_eq!(
            ink(&quiet, quiet_chip),
            skin.accent_text,
            "{name}: selected ink"
        );
        // The pointer lights it, exactly as it lights a chip: a lit chip under the pointer is the
        // lit look's hover.
        point_at(&mut quiet, quiet_chip);
        assert_eq!(
            fill(&quiet, quiet_chip),
            skin.primary_hovered(),
            "{name}: selected hovered"
        );
        quiet
            .world_mut()
            .entity_mut(quiet_chip)
            .remove::<Selected>();
        settle(&mut quiet);
        assert_eq!(
            fill(&quiet, quiet_chip),
            skin.hovered(),
            "{name}: unselected under the pointer"
        );
        assert_eq!(
            quiet.world().resource::<Heard>().activates,
            0,
            "{name}: a quiet chip is never activated"
        );
        assert!(
            quiet.world().resource::<Heard>().changes.is_empty(),
            "{name}: and it reports no toggle"
        );
    }
}

#[test]
fn a_click_on_a_quiet_chip_reaches_only_the_host_and_the_tab_walks_past_it() {
    let mut app = app_with(GooeySkin::dark());
    let host = app
        .world_mut()
        .spawn(Node {
            width: Val::Px(600.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .id();
    let loud = app.world_mut().spawn(chip("Weapons")).id();
    let quiet = app.world_mut().spawn(quiet_chip("Armour")).id();
    app.world_mut().entity_mut(host).add_child(loud);
    app.world_mut().entity_mut(host).add_child(quiet);
    app.world_mut().spawn(screen()).add_child(host);
    settle(&mut app);
    app.world_mut().entity_mut(quiet).observe(hear);
    click(&mut app, quiet);
    assert_eq!(
        app.world().resource::<Heard>().activates,
        0,
        "a quiet chip sends no Activate for a screen-wide handler to hear"
    );
    assert!(
        app.world().get::<Selected>(quiet).is_none(),
        "and lights nothing itself"
    );
    // One Tab from the loud chip leaves the box: the quiet one is not in the sequence.
    focus(&mut app, loud);
    press(&mut app, KeyCode::Tab);
    assert_ne!(focused(&app), Some(quiet), "Tab walked onto a quiet chip");
}

#[test]
fn a_quiet_chip_is_named_by_its_words_and_promises_no_click() {
    let mut app = app_with(GooeySkin::dark());
    let (_, quiet) = quiet_chip_in(&mut app, "Clear filters", 400.0);
    app.world_mut()
        .resource::<bevy::a11y::AccessibilityRequested>()
        .set(true);
    settle(&mut app);
    let node = &app
        .world()
        .get::<bevy::a11y::AccessibilityNode>(quiet)
        .expect("a node, once a reader asks")
        .0;
    assert_eq!(
        node.label(),
        Some("Clear filters"),
        "the words are its name"
    );
    assert!(
        !node.supports_action(accesskit::Action::Click),
        "a quiet chip declares no Click: the host owns the click, so the crate sends no Activate"
    );
}
