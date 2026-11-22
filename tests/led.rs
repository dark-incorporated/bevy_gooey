//! The LED (`led`): four states with four silhouettes and their words, a state written by the game
//! showing in the next frame, a busy glow that pulses under default motion and holds steady under
//! reduced motion, a settled LED that writes nothing, and the colours it draws in held to 3:1 on the
//! panel in every preset. Laid out for real in a headless window on a 100 ms clock.

use std::time::Duration;

use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::time::TimeUpdateStrategy;
use bevy::ui::{ComputedNode, UiTransform};
use bevy::window::WindowResolution;
use bevy::winit::WinitPlugin;
use bevy_gooey::*;

/// One frame of the test clock, seconds.
const FRAME: f32 = 0.1;

/// How many frames the dot's fill, glow or transform changed in.
#[derive(Resource, Default)]
struct Writes(u32);

fn app(settings: MotionSettings) -> App {
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
    .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
        FRAME,
    )))
    .insert_resource(settings)
    .init_resource::<Writes>()
    .add_systems(
        Last,
        |changed: Query<(), DotWritten>, mut n: ResMut<Writes>| {
            n.0 += changed.iter().count() as u32;
        },
    );
    app.world_mut().spawn(Camera2d);
    app.update();
    app
}

/// A dot whose fill, glow or transform changed.
type DotWritten = (
    With<LedDotMarker>,
    Or<(
        Changed<BackgroundColor>,
        Changed<BoxShadow>,
        Changed<UiTransform>,
    )>,
);

/// Tags the dot so the write counter reads it alone.
#[derive(Component)]
struct LedDotMarker;

fn frames(app: &mut App, n: usize) {
    for _ in 0..n {
        app.update();
    }
}

/// An LED on a screen; returns its row and its dot.
fn lit(app: &mut App, state: LedState) -> (Entity, Entity) {
    let row = app.world_mut().spawn(led("Agent", state)).id();
    app.world_mut().spawn(screen()).add_child(row);
    frames(app, 4);
    let dot = dot_of(app, row);
    app.world_mut().entity_mut(dot).insert(LedDotMarker);
    (row, dot)
}

fn dot_of(app: &App, row: Entity) -> Entity {
    app.world()
        .get::<Children>(row)
        .expect("children")
        .iter()
        .find(|c| app.world().get::<Role>(*c) == Some(&Role::LedDot))
        .expect("a dot")
}

fn word(app: &mut App, row: Entity) -> String {
    let world = app.world_mut();
    let kids: Vec<Entity> = world
        .get::<Children>(row)
        .expect("children")
        .iter()
        .collect();
    kids.into_iter()
        .find_map(|k| world.get::<LedWord>(k).and_then(|_| world.get::<Text>(k)))
        .expect("a word")
        .0
        .clone()
}

/// What tells a state's silhouette apart: its corner radius, its turn and whether it is hollow.
fn silhouette(app: &App, dot: Entity) -> (String, f32, f32, bool) {
    let node = app.world().get::<Node>(dot).expect("node");
    let at = app.world().get::<UiTransform>(dot).expect("transform");
    (
        format!("{:?}", node.border_radius.top_left),
        at.rotation.as_degrees(),
        at.scale.x,
        app.world().get::<BackgroundColor>(dot).expect("fill").0 == Color::NONE,
    )
}

fn glow_alpha(app: &App, dot: Entity) -> Option<f32> {
    app.world()
        .get::<BoxShadow>(dot)
        .and_then(|s| s.0.first())
        .map(|s| s.color.to_srgba().alpha)
}

fn set(app: &mut App, row: Entity, state: LedState) {
    app.world_mut().entity_mut(row).insert(Led(state));
    frames(app, 2);
}

#[test]
fn each_state_has_its_own_silhouette_and_its_word() {
    let mut app = app(MotionSettings::default());
    let (row, dot) = lit(&mut app, LedState::Off);
    let mut seen = Vec::new();
    for state in [
        LedState::Off,
        LedState::Live,
        LedState::Busy,
        LedState::Failed,
    ] {
        set(&mut app, row, state);
        assert_eq!(word(&mut app, row), state.word());
        let shape = silhouette(&app, dot);
        assert!(
            !seen.contains(&shape),
            "{state:?} repeats a silhouette {shape:?}"
        );
        seen.push(shape);
    }
    set(&mut app, row, LedState::Off);
    assert_eq!(
        app.world().get::<BackgroundColor>(dot).map(|b| b.0),
        Some(Color::NONE)
    );
    assert_eq!(glow_alpha(&app, dot), None, "an unlit lens has no glow");
    set(&mut app, row, LedState::Live);
    assert_eq!(
        app.world().get::<BackgroundColor>(dot).map(|b| b.0),
        Some(app.world().resource::<GooeySkin>().live())
    );
    assert!(
        glow_alpha(&app, dot).is_some_and(|a| a > 0.2),
        "a lit lens glows"
    );
}

#[test]
fn the_dot_is_a_square_of_its_text_sizes() {
    let mut app = app(MotionSettings::default());
    let (_, dot) = lit(&mut app, LedState::Live);
    let size = app
        .world()
        .get::<ComputedNode>(dot)
        .expect("laid out")
        .size();
    let skin = app.world().resource::<GooeySkin>();
    let expected = skin.text_size * skin.text_scale * 1.1;
    assert!(
        (size.x - size.y).abs() < 1.0 && (size.x - expected).abs() < 1.5,
        "{size:?} vs {expected}"
    );
}

#[test]
fn a_busy_glow_pulses_and_a_reduced_one_holds_still() {
    let mut pulsing = app(MotionSettings::default());
    let (_, dot) = lit(&mut pulsing, LedState::Busy);
    let mut alphas = Vec::new();
    for _ in 0..12 {
        frames(&mut pulsing, 1);
        alphas.push(glow_alpha(&pulsing, dot).expect("a glow"));
    }
    let (lo, hi) = alphas
        .iter()
        .fold((f32::MAX, f32::MIN), |(l, h), a| (l.min(*a), h.max(*a)));
    assert!(hi - lo > 0.2, "it swings over a period: {lo}..{hi}");
    assert!(lo >= 0.11 && hi <= 0.46, "inside its range: {lo}..{hi}");
    for settings in [MotionSettings::reduced(), MotionSettings::off()] {
        let mut still = app(settings);
        let (_, dot) = lit(&mut still, LedState::Busy);
        let first = glow_alpha(&still, dot);
        frames(&mut still, 12);
        assert_eq!(glow_alpha(&still, dot), first, "steady");
        assert!(first.is_some());
    }
}

#[test]
fn a_settled_led_writes_nothing() {
    for state in [LedState::Off, LedState::Live, LedState::Failed] {
        let mut app = app(MotionSettings::default());
        lit(&mut app, state);
        frames(&mut app, 6);
        let writes = app.world().resource::<Writes>().0;
        frames(&mut app, 30);
        assert_eq!(app.world().resource::<Writes>().0, writes, "{state:?}");
    }
}

#[test]
fn the_colours_it_draws_in_hold_three_to_one_on_the_panel_in_every_preset() {
    for (name, skin) in [
        ("dark", GooeySkin::dark()),
        ("parchment", GooeySkin::parchment()),
        ("high_contrast", GooeySkin::high_contrast()),
        ("tool", GooeySkin::tool()),
    ] {
        let ground = Srgba::from(skin.solid_surface());
        for (what, colour) in [
            ("live", skin.live()),
            ("warning", skin.warning()),
            ("danger", skin.danger),
            ("outline", skin.outline),
        ] {
            let c = Srgba::from(colour);
            let ratio = bevy_gooey::model::contrast_ratio(
                [c.red, c.green, c.blue],
                [ground.red, ground.green, ground.blue],
            );
            assert!(ratio >= 3.0, "{name}: {what} is {ratio:.2}:1 on the panel");
        }
    }
}

/// The most a glow may reach past the dot's edge, in dot sizes (#91): a light on the dot, not on the bar.
const GLOW_MAX_EXTENT_DOTS: f32 = 1.0;
/// The brightest a glow may be, as an alpha, at its densest.
const GLOW_MAX_ALPHA: f32 = 0.5;

/// Where the glow's visible edge stands past the dot, in logical px: the layer's spread on each side
/// (Bevy's `spread_radius` grows the whole width, so half is a side's) plus the blur's fade
/// (CSS's blur radius is twice the Gaussian's standard deviation Bevy takes).
fn glow_extent_px(app: &App, dot: Entity) -> f32 {
    let layer = &app.world().get::<BoxShadow>(dot).expect("shadow").0[0];
    let px = |v: Val| match v {
        Val::Px(p) => p,
        other => panic!("a glow length is logical px, not {other:?}"),
    };
    px(layer.spread_radius) / 2.0 + 2.0 * px(layer.blur_radius)
}

#[test]
fn a_glow_stays_within_a_dot_of_the_dot_at_every_text_scale_and_state() {
    for scale in [0.5_f32, 1.0, 1.45, 3.0] {
        for state in [LedState::Live, LedState::Busy, LedState::Failed] {
            for settings in [MotionSettings::default(), MotionSettings::reduced()] {
                let mut app = app(settings);
                app.world_mut().resource_mut::<GooeySkin>().text_scale = scale;
                let (_, dot) = lit(&mut app, state);
                let skin = app.world().resource::<GooeySkin>();
                let side = skin.text_size * skin.text_scale * 1.1;
                for _ in 0..12 {
                    frames(&mut app, 1);
                    let extent = glow_extent_px(&app, dot) / side;
                    assert!(
                        extent <= GLOW_MAX_EXTENT_DOTS + 1e-3 && extent > 0.0,
                        "{state:?} at text scale {scale}: the glow reaches {extent:.2} dots"
                    );
                    let alpha = glow_alpha(&app, dot).expect("a glow");
                    assert!(alpha <= GLOW_MAX_ALPHA, "{state:?}: alpha {alpha}");
                }
            }
        }
    }
}

/// The Gaussian's upper tail, 1 - Phi(z): Abramowitz and Stegun 7.1.26's erf, error under 1.5e-7.
fn upper_tail(z: f32) -> f32 {
    let x = z.abs() / std::f32::consts::SQRT_2;
    let t = 1.0 / (1.0 + 0.327_591_1 * x);
    let poly = t
        * (0.254_829_6
            + t * (-0.284_496_74 + t * (1.421_413_8 + t * (-1.453_152_1 + t * 1.061_405_4))));
    let erf = 1.0 - poly * (-x * x).exp();
    let tail = 0.5 * (1.0 - erf);
    if z >= 0.0 { tail } else { 1.0 - tail }
}

/// The word beside the dot starts one half gap from its edge, where a halo of density `a` at the dot's
/// edge has faded to `a` times the Gaussian's upper tail at that distance (a box shadow is half its
/// strength on the box's edge): the densest it ever is under a glyph. The ink must hold its contrast
/// on that blend, in every preset, state and text scale.
#[test]
fn the_word_beside_an_led_keeps_its_contrast_under_its_glow_in_every_preset() {
    for (name, skin, needed) in [
        ("dark", GooeySkin::dark(), 4.5),
        ("parchment", GooeySkin::parchment(), 4.5),
        ("high_contrast", GooeySkin::high_contrast(), 7.0),
        ("tool", GooeySkin::tool(), 4.5),
    ] {
        for scale in [0.5_f32, 1.0, 1.45, 3.0] {
            for state in [LedState::Live, LedState::Busy, LedState::Failed] {
                let mut app = app(MotionSettings::off());
                *app.world_mut().resource_mut::<GooeySkin>() = GooeySkin {
                    text_scale: scale,
                    ..skin.clone()
                };
                let (_, dot) = lit(&mut app, state);
                let skin = app.world().resource::<GooeySkin>().clone();
                let layer = &app.world().get::<BoxShadow>(dot).expect("shadow").0[0];
                let Val::Px(sigma) = layer.blur_radius else {
                    panic!("a glow length is logical px")
                };
                let word_at = skin.gap * 0.5;
                let density = GLOW_MAX_ALPHA * upper_tail(word_at / sigma);
                let ground = Srgba::from(skin.solid_surface());
                let glow = layer.color.to_srgba();
                let under = [
                    glow.red * density + ground.red * (1.0 - density),
                    glow.green * density + ground.green * (1.0 - density),
                    glow.blue * density + ground.blue * (1.0 - density),
                ];
                // The word is a `Role::Label`, drawn in `text`.
                let ink = Srgba::from(skin.text);
                let ratio =
                    bevy_gooey::model::contrast_ratio([ink.red, ink.green, ink.blue], under);
                assert!(
                    ratio >= needed,
                    "{name} at {scale}: the word under a {state:?} glow is {ratio:.2}:1, needs {needed}"
                );
            }
        }
    }
}

/// The label and the word of an LED row: their `TextFont` sizes in logical px.
fn text_sizes(app: &App, row: Entity) -> Vec<f32> {
    app.world()
        .get::<Children>(row)
        .expect("children")
        .iter()
        .filter_map(|c| app.world().get::<TextFont>(c))
        .map(|f| match f.font_size {
            FontSize::Px(px) => px,
            other => panic!("not a pixel size: {other:?}"),
        })
        .collect()
}

fn dot_side(app: &App, dot: Entity) -> f32 {
    app.world()
        .get::<ComputedNode>(dot)
        .expect("laid out")
        .size()
        .x
}

#[test]
fn an_led_size_draws_the_label_the_word_and_the_dot_at_that_step() {
    let mut app = app(MotionSettings::off());
    let (row, dot) = lit(&mut app, LedState::Live);
    let plain = text_sizes(&app, row);
    let plain_dot = dot_side(&app, dot);
    assert_eq!(plain.len(), 2, "a label and a word");

    app.world_mut().entity_mut(row).insert(LedSize(1));
    frames(&mut app, 3);
    let skin = app.world().resource::<GooeySkin>().clone();
    let small = skin.type_size(1);
    assert!(
        small < skin.text_size * skin.text_scale,
        "step 1 is quieter than the body"
    );
    for px in text_sizes(&app, row) {
        assert!((px - small).abs() <= 1.0, "{px} vs {small}");
    }
    let side = dot_side(&app, dot);
    assert!(
        (side - small * 1.1).abs() < 1.5 && side < plain_dot,
        "dot {side} vs {}",
        small * 1.1
    );

    // It holds through a state change and a skin change (the player's text size).
    set(&mut app, row, LedState::Failed);
    app.world_mut().resource_mut::<GooeySkin>().text_scale = 1.5;
    frames(&mut app, 3);
    let skin = app.world().resource::<GooeySkin>().clone();
    for px in text_sizes(&app, row) {
        assert!(
            (px - skin.type_size(1)).abs() <= 1.0,
            "{px} vs {}",
            skin.type_size(1)
        );
    }
    let side = dot_side(&app, dot);
    assert!((side - skin.type_size(1) * 1.1).abs() < 1.5, "dot {side}");

    // Taking it off is the body's size again.
    app.world_mut().entity_mut(row).remove::<LedSize>();
    frames(&mut app, 3);
    let skin = app.world().resource::<GooeySkin>().clone();
    for px in text_sizes(&app, row) {
        assert!((px - skin.text_size * skin.text_scale).abs() <= 1.0, "{px}");
    }
    assert!((dot_side(&app, dot) - skin.text_size * skin.text_scale * 1.1).abs() < 1.5);
}

#[test]
fn an_led_without_a_size_is_unchanged() {
    let mut app = app(MotionSettings::off());
    let (row, dot) = lit(&mut app, LedState::Live);
    let skin = app.world().resource::<GooeySkin>().clone();
    for px in text_sizes(&app, row) {
        assert!((px - skin.text_size * skin.text_scale).abs() <= 1.0, "{px}");
    }
    assert!((dot_side(&app, dot) - skin.text_size * skin.text_scale * 1.1).abs() < 1.5);
    // A step past the end of the scale is its last.
    app.world_mut().entity_mut(row).insert(LedSize(99));
    frames(&mut app, 3);
    let last = skin.type_size(skin.type_scale.len() - 1);
    for px in text_sizes(&app, row) {
        assert!((px - last).abs() <= 1.0, "{px} vs {last}");
    }
}
