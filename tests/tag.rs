//! A tag: a short word on a plate whose colour names a kind of thing. Read-only, it takes no input,
//! its ink is chosen from its fill so a host's tone is never unreadable, a label too long for its
//! host is cut and not folded, and one with nothing to say takes no room. Laid out for real in a
//! headless window, with motion off so a colour is the one `apply_skin` wrote.

use bevy::a11y::{AccessibilityNode, AccessibilityRequested};
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::window::WindowResolution;
use bevy::winit::WinitPlugin;
use bevy_gooey::*;

const ELITE: Color = Color::srgb(0.85, 0.65, 0.1);
const FROST: Color = Color::srgb(0.55, 0.78, 0.95);
const WCAG_TEXT: f32 = 4.5;

/// How many `Node`s changed.
#[derive(Resource, Default)]
struct NodeWrites(u32);

fn skin() -> GooeySkin {
    let mut skin = GooeySkin::dark();
    skin.tones.insert("elite".into(), ELITE);
    skin.tones.insert("frost".into(), FROST);
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

/// A row in a panel holding `widgets`; laid out. The row is returned.
fn row_of(app: &mut App, widgets: impl Bundle) -> Entity {
    let row = app.world_mut().spawn((row(1.0), widgets)).id();
    let panel = app.world_mut().spawn(panel()).add_child(row).id();
    app.world_mut().spawn(screen()).add_child(panel);
    frames(app, 3);
    row
}

fn words_of(app: &App, tag: Entity) -> Entity {
    app.world().get::<Children>(tag).expect("children")[0]
}

fn fill(app: &App, entity: Entity) -> Color {
    app.world()
        .get::<BackgroundColor>(entity)
        .expect("a fill")
        .0
}

fn ink(app: &App, tag: Entity) -> Color {
    app.world()
        .get::<TextColor>(words_of(app, tag))
        .expect("an ink")
        .0
}

fn size(app: &App, entity: Entity) -> Vec2 {
    app.world()
        .get::<ComputedNode>(entity)
        .expect("laid out")
        .size()
}

/// WCAG 2.2's contrast ratio, derived here from linear light and not from the crate's maths.
fn contrast(a: Color, b: Color) -> f32 {
    fn luminance(c: Color) -> f32 {
        let l = c.to_linear();
        0.2126 * l.red + 0.7152 * l.green + 0.0722 * l.blue
    }
    let (hi, lo) = {
        let (x, y) = (luminance(a), luminance(b));
        (x.max(y), x.min(y))
    };
    (hi + 0.05) / (lo + 0.05)
}

#[test]
fn a_tag_shows_its_words_and_takes_no_input() {
    let mut app = app();
    let row = row_of(&mut app, children![tag("ELITE")]);
    let tag = app.world().get::<Children>(row).expect("children")[0];
    let text = app
        .world()
        .get::<Text>(words_of(&app, tag))
        .expect("words")
        .0
        .clone();
    assert_eq!(text, "ELITE");
    let world = app.world();
    assert_eq!(world.get::<Pickable>(tag), Some(&Pickable::IGNORE));
    assert!(world.get::<TabIndex>(tag).is_none(), "a tag is no tab stop");
    assert!(world.get::<bevy::ui_widgets::Button>(tag).is_none());
    assert!(world.get::<Pickable>(words_of(&app, tag)) == Some(&Pickable::IGNORE));
}

#[test]
fn an_untoned_tag_is_the_quiet_plate_and_a_toned_one_is_its_tone_in_its_own_ink() {
    let mut app = app();
    let row = row_of(
        &mut app,
        children![
            tag("NEW"),
            (tag("ELITE"), TagTone("elite")),
            (tag("FROST"), TagTone("frost"))
        ],
    );
    let kids = app.world().get::<Children>(row).expect("children").to_vec();
    let skin = skin();
    assert_eq!(fill(&app, kids[0]), skin.surface_raised);
    assert_eq!(ink(&app, kids[0]), skin.text);
    assert_eq!(fill(&app, kids[1]), ELITE);
    assert_eq!(fill(&app, kids[2]), FROST);
    for tag in &kids[1..] {
        let ratio = contrast(fill(&app, *tag), ink(&app, *tag));
        assert!(ratio >= WCAG_TEXT, "a tag's words read at {ratio}");
    }
    // The two tones are light and the ink goes dark: one rule picks the ink from the plate.
    assert_ne!(ink(&app, kids[1]), skin.text);
}

#[test]
fn a_toned_tags_frame_is_its_own_colour_so_the_plate_is_one_shape() {
    let mut app = app();
    let row = row_of(&mut app, children![(tag("ELITE"), TagTone("elite"))]);
    let tag = app.world().get::<Children>(row).expect("children")[0];
    let border = app.world().get::<BorderColor>(tag).expect("a frame");
    assert_eq!(border.top, ELITE);
    assert_eq!(border.left, ELITE);
}

#[test]
fn the_words_read_at_4_5_to_1_on_every_colour_a_skin_names_in_every_preset() {
    for (name, preset) in [
        ("dark", GooeySkin::dark()),
        ("parchment", GooeySkin::parchment()),
        ("high contrast", GooeySkin::high_contrast()),
        ("tool", GooeySkin::tool()),
    ] {
        for tone in [
            "accent",
            "danger",
            "warning",
            "live",
            "text_muted",
            "outline",
            "surface",
            "text",
        ] {
            let fill = preset.tag_fill(Some(tone));
            let ink = preset.tag_ink(Some(tone));
            let ratio = contrast(fill, ink);
            assert!(
                ratio >= WCAG_TEXT,
                "{name}: a `{tone}` tag's words read at {ratio}"
            );
        }
        let quiet = contrast(preset.tag_fill(None), preset.tag_ink(None));
        assert!(quiet >= WCAG_TEXT, "{name}: the quiet tag reads at {quiet}");
    }
}

#[test]
fn a_tone_the_skin_does_not_know_is_the_quiet_plate_and_a_skin_change_repaints() {
    let mut app = app();
    let row = row_of(
        &mut app,
        children![
            (tag("?"), TagTone("nothing")),
            (tag("ELITE"), TagTone("elite"))
        ],
    );
    let kids = app.world().get::<Children>(row).expect("children").to_vec();
    assert_eq!(fill(&app, kids[0]), skin().surface_raised);
    let mut changed = skin();
    changed.tones.insert("elite".into(), FROST);
    app.insert_resource(changed);
    frames(&mut app, 2);
    assert_eq!(fill(&app, kids[1]), FROST);
}

#[test]
fn a_tone_given_or_taken_after_spawn_repaints_at_once() {
    let mut app = app();
    let row = row_of(&mut app, children![tag("ELITE")]);
    let tag = app.world().get::<Children>(row).expect("children")[0];
    app.world_mut().entity_mut(tag).insert(TagTone("elite"));
    frames(&mut app, 2);
    assert_eq!(fill(&app, tag), ELITE);
    let words = ink(&app, tag);
    assert_ne!(words, skin().text);
    app.world_mut().entity_mut(tag).remove::<TagTone>();
    frames(&mut app, 2);
    assert_eq!(fill(&app, tag), skin().surface_raised);
    assert_eq!(ink(&app, tag), skin().text);
}

#[test]
fn a_label_too_long_for_its_host_is_cut_on_one_line_and_never_leaves_the_plate() {
    let mut app = app();
    let host = app
        .world_mut()
        .spawn((
            Node {
                width: Val::Px(120.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![
                tag("A NAME FAR TOO LONG FOR THE ROOM IT IS GIVEN"),
                tag("ELITE")
            ],
        ))
        .id();
    let panel = app.world_mut().spawn(panel()).add_child(host).id();
    app.world_mut().spawn(screen()).add_child(panel);
    frames(&mut app, 3);
    let kids = app
        .world()
        .get::<Children>(host)
        .expect("children")
        .to_vec();
    let (long, short) = (size(&app, kids[0]), size(&app, kids[1]));
    assert!(
        long.x <= 120.5,
        "the plate is {} wide in a host of 120",
        long.x
    );
    assert!(
        (long.y - short.y).abs() < 1.5,
        "a cut tag is {} high, a short one {}",
        long.y,
        short.y
    );
}

#[test]
fn a_tag_is_no_taller_than_the_line_of_text_beside_it_at_1x_and_2x() {
    for scale in [1.0, 2.0] {
        let mut app = app();
        let mut skin = skin();
        skin.text_scale = scale;
        app.insert_resource(skin);
        let row = row_of(&mut app, children![label("Wolf"), tag("ELITE")]);
        let kids = app.world().get::<Children>(row).expect("children").to_vec();
        let line = size(&app, kids[0]).y;
        let plate = size(&app, kids[1]).y;
        assert!(
            plate <= line + 1.0,
            "at {scale}x a tag is {plate} high beside a line of {line}"
        );
    }
}

#[test]
fn a_tag_with_no_words_takes_no_room_and_gets_it_back_with_them() {
    let mut app = app();
    let row = row_of(&mut app, children![tag("2 STALE"), label("after")]);
    let kids = app.world().get::<Children>(row).expect("children").to_vec();
    let with = app
        .world()
        .get::<UiGlobalTransform>(kids[1])
        .expect("placed")
        .translation
        .x;
    let words = words_of(&app, kids[0]);
    app.world_mut()
        .get_mut::<Text>(words)
        .expect("words")
        .0
        .clear();
    frames(&mut app, 3);
    let without = app
        .world()
        .get::<UiGlobalTransform>(kids[1])
        .expect("placed")
        .translation
        .x;
    assert!(
        with - without > 20.0,
        "the label after moved {} px when the tag emptied",
        with - without
    );
    assert_eq!(
        app.world().get::<Node>(kids[0]).expect("node").display,
        Display::None
    );
    let words = words_of(&app, kids[0]);
    app.world_mut().get_mut::<Text>(words).expect("words").0 = "1 STALE".into();
    frames(&mut app, 3);
    assert_eq!(
        app.world().get::<Node>(kids[0]).expect("node").display,
        Display::Flex
    );
    let back = app
        .world()
        .get::<UiGlobalTransform>(kids[1])
        .expect("placed")
        .translation
        .x;
    assert!(
        (back - with).abs() < 12.0,
        "the label is back near {with}: {back}"
    );
}

#[test]
fn a_settled_tag_writes_no_node() {
    let mut app = app();
    row_of(
        &mut app,
        children![tag("NEW"), (tag("ELITE"), TagTone("elite"))],
    );
    frames(&mut app, 5);
    app.world_mut().resource_mut::<NodeWrites>().0 = 0;
    frames(&mut app, 30);
    assert_eq!(app.world().resource::<NodeWrites>().0, 0);
}

#[test]
fn a_tag_is_a_label_read_by_its_words() {
    let mut app = app();
    app.world().resource::<AccessibilityRequested>().set(true);
    let row = row_of(&mut app, children![(tag("ELITE"), TagTone("elite"))]);
    frames(&mut app, 3);
    let tag = app.world().get::<Children>(row).expect("children")[0];
    let node = &app
        .world()
        .get::<AccessibilityNode>(tag)
        .expect("a tag has a node")
        .0;
    assert_eq!(node.role(), accesskit::Role::Label);
    assert_eq!(node.value(), Some("ELITE"));
    assert!(
        app.world()
            .get::<AccessibilityNode>(words_of(&app, tag))
            .is_none_or(|node| node.0.is_hidden()),
        "the words are read once, on the plate"
    );
}
