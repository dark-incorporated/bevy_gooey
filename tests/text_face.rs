//! One text, its own face (#118): `apply_skin` is the one writer of a text's look, so a host's
//! `TextFont` is written over on the next restyle and a second face needed a system of the host's
//! re-applying it every frame. `TextFace` pins the face on one text; the role keeps the ink and the
//! size. Measured on the glyphs Bevy actually shaped, not on the field, so a face that is written
//! but not drawn fails.
//!
//! The face is a subset of Liberation Serif (SIL OFL 1.1, `assets/fonts/OFL.txt`), 9.7 KB, loaded
//! here from bytes so the test is deterministic and needs no asset server.

use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::text::{Font, FontSource, TextLayoutInfo};
use bevy::ui::InteractionDisabled;
use bevy::window::WindowResolution;
use bevy::winit::WinitPlugin;
use bevy_gooey::*;

/// How many frames' writes the app has seen to a text's face.
#[derive(Resource, Default)]
struct FaceWrites(u32);

const SERIF: &str = "assets/fonts/liberation-serif-subset.ttf";

fn serif_bytes() -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(SERIF);
    std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
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
    .insert_resource(MotionSettings::off())
    .init_resource::<FaceWrites>()
    .add_systems(
        Update,
        |changed: Query<(), Changed<TextFont>>, mut writes: ResMut<FaceWrites>| {
            writes.0 += changed.iter().count() as u32;
        },
    );
    app.world_mut().spawn(Camera2d);
    app
}

fn settle(app: &mut App) {
    for _ in 0..6 {
        app.update();
    }
}

/// The serif face as a `FontSource`, added to the app's assets once.
fn serif(app: &mut App) -> FontSource {
    let font = Font::from_bytes(serif_bytes());
    FontSource::Handle(app.world_mut().resource_mut::<Assets<Font>>().add(font))
}

/// The advance Bevy shaped for `entity`'s words, in logical px (the window's scale is 1).
fn shaped(app: &App, entity: Entity) -> Vec2 {
    app.world()
        .get::<TextLayoutInfo>(entity)
        .expect("a shaped run")
        .size
}

fn font_of(app: &App, entity: Entity) -> FontSource {
    app.world()
        .get::<TextFont>(entity)
        .expect("a text")
        .font
        .clone()
}

fn size_of(app: &App, entity: Entity) -> f32 {
    match app
        .world()
        .get::<TextFont>(entity)
        .expect("a text")
        .font_size
    {
        FontSize::Px(px) => px,
        other => panic!("a skin never sets {other:?}"),
    }
}

fn ink_of(app: &App, entity: Entity) -> Color {
    app.world().get::<TextColor>(entity).expect("ink").0
}

/// The part of `root` with `role`, the first in tree order.
fn part(app: &App, root: Entity, role: Role) -> Entity {
    let world = app.world();
    let mut stack = vec![root];
    while let Some(entity) = stack.pop() {
        if world.get::<Role>(entity) == Some(&role) {
            return entity;
        }
        if let Some(children) = world.get::<Children>(entity) {
            stack.extend(children.iter().rev());
        }
    }
    panic!("a {role:?} under the widget");
}

/// A panel with a plain label, a serif label and a serif bold label, and their entities.
struct Ui {
    plain: Entity,
    serif: Entity,
    bold: Entity,
    face: FontSource,
}

fn ui(app: &mut App) -> Ui {
    let face = serif(app);
    let world = app.world_mut();
    let plain = world.spawn(label("iiii iiii")).id();
    let serif = world
        .spawn((label("iiii iiii"), TextFace::new(face.clone())))
        .id();
    let bold = world
        .spawn((
            label("iiii iiii"),
            TextFace::new(face.clone()),
            TextFont {
                weight: FontWeight::BOLD,
                ..default()
            },
        ))
        .id();
    let panel = world.spawn((panel(), children![()])).id();
    let holder = world
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .id();
    world.entity_mut(panel).add_child(holder);
    world.entity_mut(holder).add_children(&[plain, serif, bold]);
    world.spawn(screen()).add_child(panel);
    settle(app);
    Ui {
        plain,
        serif,
        bold,
        face,
    }
}

#[test]
fn a_face_is_drawn_and_the_role_keeps_the_ink_and_the_size() {
    let mut app = app();
    let ui = ui(&mut app);
    let serif = ui.face.clone();
    let skin = GooeySkin::dark();

    // The face is the drawn one: a proportional serif shapes the same words at a different width
    // than the body face, which is what a face only written to the field would not do.
    assert_eq!(font_of(&app, ui.serif), serif);
    assert_eq!(font_of(&app, ui.plain), GooeyFonts::default().body);
    let plain = shaped(&app, ui.plain).x;
    let faced = shaped(&app, ui.serif).x;
    assert!(plain > 0.0, "the plain run was never shaped");
    assert!(
        (faced - plain).abs() > 2.0,
        "the serif face shaped the run at {faced} against the body face's {plain}"
    );
    // The role still decides the ink and the size; a face is a face.
    assert_eq!(ink_of(&app, ui.serif), skin.text);
    assert_eq!(size_of(&app, ui.serif), size_of(&app, ui.plain));
    assert_eq!(ink_of(&app, ui.bold), skin.text);
    assert_eq!(size_of(&app, ui.bold), size_of(&app, ui.plain));
    // And the host's weight survives: a bold run in a custom face stays bold.
    assert_eq!(
        app.world().get::<TextFont>(ui.bold).expect("bold").weight,
        FontWeight::BOLD
    );
}

#[test]
fn the_face_survives_a_skin_change_a_move_and_a_state_change() {
    let mut app = app();
    let ui = ui(&mut app);
    let serif = ui.face.clone();

    // A skin change restyles every widget: the size follows the skin, the face does not move.
    let before = size_of(&app, ui.serif);
    app.world_mut().resource_mut::<GooeySkin>().text_scale = 1.5;
    app.update();
    app.update();
    assert_eq!(font_of(&app, ui.serif), serif, "after a skin change");
    let large = size_of(&app, ui.serif);
    assert!(
        large > before,
        "the skin's text size did not reach the faced text: {large} against {before}"
    );
    assert_eq!(large, size_of(&app, ui.plain), "the size is the role's");

    // A move restyles the node (spawn or re-parent): the face is still the host's.
    let holder = app
        .world()
        .get::<ChildOf>(ui.serif)
        .expect("parent")
        .parent();
    let other = app
        .world_mut()
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .id();
    let panel = app.world().get::<ChildOf>(holder).expect("parent").parent();
    app.world_mut().entity_mut(panel).add_child(other);
    app.world_mut().entity_mut(other).add_child(holder);
    settle(&mut app);
    assert_eq!(font_of(&app, ui.serif), serif, "after a move");

    // A state change on an owning widget restyles its parts, never the label beside them.
    let button = app
        .world_mut()
        .spawn((button("Play"), TextFace::new(serif.clone())))
        .id();
    let words = part(&app, button, Role::ButtonText);
    // The face is per node, so the host writes it on the words as well as on the button: the words
    // are the drawn node.
    app.world_mut()
        .entity_mut(words)
        .insert(TextFace::new(serif.clone()));
    app.world_mut()
        .entity_mut(button)
        .insert(InteractionDisabled);
    settle(&mut app);
    assert_eq!(font_of(&app, words), serif, "after a state change");
    assert_eq!(font_of(&app, ui.serif), serif, "beside it");
}

#[test]
fn taking_the_face_off_gives_the_roles_face_back() {
    let mut app = app();
    let ui = ui(&mut app);
    assert_ne!(font_of(&app, ui.serif), GooeyFonts::default().body);

    app.world_mut().entity_mut(ui.serif).remove::<TextFace>();
    app.update();
    assert_eq!(
        font_of(&app, ui.serif),
        GooeyFonts::default().body,
        "the role's face after the component went"
    );
    assert_eq!(shaped(&app, ui.serif).x, shaped(&app, ui.plain).x);
}

#[test]
fn a_face_on_a_panel_does_not_reach_the_text_below_it() {
    // No cascade, deliberately: a panel's face does not leak into a Markdown page's spans or a code
    // editor's glyphs, which are exactly the texts that must not take it.
    let mut app = app();
    let serif = serif(&mut app);
    let world = app.world_mut();
    let words = world.spawn(label("iiii iiii")).id();
    let panel = world.spawn((panel(), TextFace::new(serif))).id();
    world.entity_mut(panel).add_child(words);
    world.spawn(screen()).add_child(panel);
    settle(&mut app);
    assert_eq!(font_of(&app, words), GooeyFonts::default().body);
}

#[test]
fn a_code_editor_follows_its_own_font_and_not_a_face_pinned_beside_it() {
    // The editor measures one column from `GooeyFonts::mono`, so its face is the role's and never a
    // neighbour's: a `TextFace` on the prose beside it moves neither glyph of the code.
    let mut app = app();
    let serif = serif(&mut app);
    app.insert_resource(GooeyFonts {
        mono: serif.clone(),
        ..Default::default()
    });
    let world = app.world_mut();
    let editor = world.spawn(code_editor("Script", "let x = 1;")).id();
    let prose = world
        .spawn((label("The lantern is out."), TextFace::new(serif.clone())))
        .id();
    let panel = world.spawn((panel(), children![()])).id();
    world.entity_mut(panel).add_children(&[editor, prose]);
    world.spawn(screen()).add_child(panel);
    settle(&mut app);
    let line = part(&app, editor, Role::CodeText);
    assert_eq!(
        font_of(&app, line),
        serif,
        "the editor draws in GooeyFonts::mono"
    );
    assert_eq!(font_of(&app, prose), serif);

    // Move the skin's mono away: the editor's glyphs follow it, the pinned prose does not.
    let body = GooeyFonts::default();
    app.insert_resource(body.clone());
    settle(&mut app);
    assert_eq!(
        font_of(&app, line),
        body.mono,
        "the editor follows GooeyFonts::mono"
    );
    assert_eq!(
        font_of(&app, prose),
        serif,
        "the prose keeps the face the host pinned"
    );
}

#[test]
fn a_settled_text_with_a_face_writes_nothing() {
    let mut app = app();
    let ui = ui(&mut app);
    for _ in 0..4 {
        app.update();
    }
    app.world_mut().resource_mut::<FaceWrites>().0 = 0;
    for _ in 0..20 {
        app.update();
    }
    assert_eq!(
        app.world().resource::<FaceWrites>().0,
        0,
        "a face is written once, not every frame"
    );
    assert_eq!(font_of(&app, ui.serif), font_of(&app, ui.serif).clone());
}
