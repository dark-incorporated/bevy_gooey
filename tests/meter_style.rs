//! A meter a game can tell apart and count: `MeterTone` paints the fill (and a lighter trail) in a
//! tone of the skin, `MeterSegments` cuts it into cells with notches that stand at the cells'
//! edges. Laid out for real in a headless window, with motion off so a colour is the one
//! `apply_skin` wrote.

use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::ui::UiGlobalTransform;
use bevy::window::WindowResolution;
use bevy::winit::WinitPlugin;
use bevy_gooey::*;

const HEALTH: Color = Color::srgb(0.78, 0.18, 0.22);
const MANA: Color = Color::srgb(0.25, 0.45, 0.9);

/// How many `Node`s changed.
#[derive(Resource, Default)]
struct NodeWrites(u32);

fn skin() -> GooeySkin {
    let mut skin = GooeySkin::dark();
    skin.tones.insert("health".into(), HEALTH);
    skin.tones.insert("mana".into(), MANA);
    skin
}

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
    .insert_resource(skin())
    .insert_resource(MotionSettings::off());
    app.init_resource::<NodeWrites>().add_systems(
        Update,
        |changed: Query<(), Changed<Node>>, mut writes: ResMut<NodeWrites>| {
            writes.0 += changed.iter().count() as u32
        },
    );
    app.world_mut().spawn(Camera2d);
    app
}

fn frames(app: &mut App, n: usize) {
    for _ in 0..n {
        app.update();
    }
}

/// A meter of `value` in a panel, with `extra` beside its components; laid out.
fn bar(app: &mut App, value: f32, extra: impl Bundle) -> Entity {
    let track = app.world_mut().spawn((meter(value, 100.0), extra)).id();
    let panel = app.world_mut().spawn(panel()).add_child(track).id();
    app.world_mut().spawn(screen()).add_child(panel);
    frames(app, 3);
    track
}

fn fill_of(app: &App, track: Entity) -> Entity {
    app.world().get::<Children>(track).expect("children")[0]
}

fn trail_of(app: &App, track: Entity) -> Entity {
    app.world().get::<Children>(track).expect("children")[1]
}

fn colour(app: &App, entity: Entity) -> Color {
    app.world().get::<BackgroundColor>(entity).expect("a fill").0
}

fn notches(app: &App, track: Entity) -> Vec<Entity> {
    let world = app.world();
    let Some(children) = world.get::<Children>(track) else {
        return Vec::new();
    };
    children
        .iter()
        .filter(|&c| world.get::<Role>(c) == Some(&Role::MeterNotchLane))
        .flat_map(|lane| {
            world
                .get::<Children>(lane)
                .map(|c| c.iter().collect::<Vec<_>>())
                .unwrap_or_default()
        })
        .collect()
}

fn lanes(app: &App, track: Entity) -> usize {
    let world = app.world();
    world.get::<Children>(track).map_or(0, |children| {
        children
            .iter()
            .filter(|&c| world.get::<Role>(c) == Some(&Role::MeterNotchLane))
            .count()
    })
}

fn rect(app: &App, entity: Entity) -> Rect {
    let size = app.world().get::<ComputedNode>(entity).expect("laid out").size();
    let at = app.world().get::<UiGlobalTransform>(entity).expect("placed").translation;
    Rect::from_center_size(at, size)
}

#[test]
fn a_toned_fill_is_the_skins_tone_and_an_untoned_one_the_accent() {
    let mut app = app();
    let health = bar(&mut app, 60.0, MeterTone("health"));
    let mana = bar(&mut app, 60.0, MeterTone("mana"));
    let plain = bar(&mut app, 60.0, ());
    assert_eq!(colour(&app, fill_of(&app, health)), HEALTH);
    assert_eq!(colour(&app, fill_of(&app, mana)), MANA);
    assert_eq!(colour(&app, fill_of(&app, plain)), skin().accent);
    // A skin's own colour is a name too, and one the skin does not know is the accent.
    let danger = bar(&mut app, 60.0, MeterTone("danger"));
    let unknown = bar(&mut app, 60.0, MeterTone("no such tone"));
    assert_eq!(colour(&app, fill_of(&app, danger)), skin().danger);
    assert_eq!(colour(&app, fill_of(&app, unknown)), skin().accent);
}

#[test]
fn a_toned_trail_is_its_fill_lifted_and_an_untoned_one_stays_danger() {
    let mut app = app();
    let toned = bar(&mut app, 100.0, MeterTone("health"));
    let plain = bar(&mut app, 100.0, ());
    assert_eq!(colour(&app, trail_of(&app, plain)), skin().danger);
    let trail = colour(&app, trail_of(&app, toned));
    assert_eq!(trail, skin().meter_trail(Some("health")));
    assert_ne!(trail, skin().danger, "a red bar with a danger trail is one longer bar");
    let l = |c: Color| {
        let c = c.to_srgba();
        cie_lightness([c.red, c.green, c.blue])
    };
    assert!(
        l(trail) - l(HEALTH) >= STATE_STEP * 2.0,
        "the lost length must stand off the fill: {} against {}",
        l(trail),
        l(HEALTH)
    );
}

#[test]
fn a_tone_follows_the_component_and_the_skin() {
    let mut app = app();
    let track = bar(&mut app, 60.0, MeterTone("health"));
    let fill = fill_of(&app, track);
    app.world_mut().entity_mut(track).insert(MeterTone("mana"));
    frames(&mut app, 2);
    assert_eq!(colour(&app, fill), MANA, "a changed tone repaints the fill");
    // A player's own palette is one skin change.
    let mut mine = skin();
    mine.tones.insert("mana".into(), Color::srgb(0.9, 0.7, 0.1));
    app.insert_resource(mine);
    frames(&mut app, 2);
    assert_eq!(colour(&app, fill), Color::srgb(0.9, 0.7, 0.1));
    app.world_mut().entity_mut(track).remove::<MeterTone>();
    frames(&mut app, 2);
    assert_eq!(colour(&app, fill), skin().accent, "no tone, the accent");
}

#[test]
fn segments_put_a_notch_at_each_cell_edge_a_line_wide_over_the_fill() {
    let mut app = app();
    let track = bar(&mut app, 60.0, MeterSegments(10));
    let cuts = notches(&app, track);
    assert_eq!(cuts.len(), 9);
    let fill = rect(&app, fill_of(&app, track));
    let outer = rect(&app, track);
    // The room the fill's percentage is of: the track inside its frame and gap.
    let inset = 2.0 * skin().border;
    let room = outer.width() - 2.0 * inset;
    assert!((fill.width() - 0.6 * room).abs() < 1.0, "{fill:?} {outer:?}");
    let mut xs: Vec<f32> = cuts.iter().map(|&c| rect(&app, c).min.x).collect();
    xs.sort_by(f32::total_cmp);
    for (i, x) in xs.iter().enumerate() {
        let want = outer.min.x + inset + room * (i as f32 + 1.0) / 10.0;
        assert!((x - want).abs() < 1.0, "notch {i} at {x}, wanted {want}");
    }
    for &cut in &cuts {
        let r = rect(&app, cut);
        assert!(
            (r.width() - skin().border).abs() < 0.5,
            "a notch is a line weight wide: {r:?}"
        );
        assert!(
            (r.height() - fill.height()).abs() < 1.0,
            "it cuts the fill's whole height: {r:?} {fill:?}"
        );
        assert_eq!(colour(&app, cut), skin().surface_raised, "in the track's colour");
        assert_eq!(
            app.world().get::<Pickable>(cut).copied(),
            Some(Pickable::IGNORE)
        );
    }
    assert_eq!(app.world().get::<Role>(fill_of(&app, track)), Some(&Role::MeterFill));
}

#[test]
fn a_changed_count_rebuilds_and_nothing_to_cut_builds_nothing() {
    let mut app = app();
    let track = bar(&mut app, 60.0, MeterSegments(10));
    assert_eq!(notches(&app, track).len(), 9);
    app.world_mut().entity_mut(track).insert(MeterSegments(4));
    frames(&mut app, 3);
    assert_eq!(notches(&app, track).len(), 3);
    assert_eq!(lanes(&app, track), 1, "the old lane is gone");
    for cells in [0, 1, METER_MAX_SEGMENTS + 1] {
        app.world_mut().entity_mut(track).insert(MeterSegments(cells));
        frames(&mut app, 3);
        assert_eq!(lanes(&app, track), 0, "{cells} cells draw no notches");
    }
    app.world_mut().entity_mut(track).insert(MeterSegments(METER_MAX_SEGMENTS));
    frames(&mut app, 3);
    assert_eq!(notches(&app, track).len() as u32, METER_MAX_SEGMENTS - 1);
    app.world_mut().entity_mut(track).remove::<MeterSegments>();
    frames(&mut app, 3);
    assert_eq!(lanes(&app, track), 0, "removing the component removes the notches");
}

#[test]
fn a_settled_styled_meter_writes_nothing() {
    let mut app = app();
    bar(&mut app, 60.0, (MeterTone("health"), MeterSegments(8)));
    frames(&mut app, 5);
    app.world_mut().resource_mut::<NodeWrites>().0 = 0;
    frames(&mut app, 30);
    assert_eq!(app.world().resource::<NodeWrites>().0, 0);
}

#[test]
fn a_toned_meter_still_reads_its_value_and_stays_inside_its_frame() {
    let mut app = app();
    let track = bar(&mut app, 25.0, (MeterTone("mana"), MeterSegments(5)));
    let (outer, fill) = (rect(&app, track), rect(&app, fill_of(&app, track)));
    let inset = 2.0 * skin().border;
    let room = outer.width() - 2.0 * inset;
    assert!((fill.width() - 0.25 * room).abs() < 1.0);
    assert!((fill.min.x - outer.min.x - inset).abs() < 0.5);
}
