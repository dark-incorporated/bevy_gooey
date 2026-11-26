//! One text, its own face (#118): a page of prose in a reading face beside a panel of widgets in
//! the skin's own, on one screen and in one skin.
//!
//! [`apply_skin`] is the one writer of a text's look, so a `TextFont` a host sets is written over at
//! the next restyle (a state change, a move, a skin change). `TextFace` pins the face on a text: the
//! role still gives it its ink and its size, and the host's weight and slant are kept.
//!
//! Run `cargo run --release --example text_face`.
//!
//! Capture mode: `GOOEY_FRAMES=<n>` saves a screenshot at frame `n` to `GOOEY_SHOT` (default
//! `text_face.png`), prints `gooey: frames <n> mean_ms <x>` and exits 10 frames later.
//! `GOOEY_SKIN=parchment`, `GOOEY_TEXT_SCALE=<f>` and `GOOEY_SIZE=<w>x<h>` set the start.
//!
//! The face beside the skin's is a 9.7 KB subset of Liberation Serif (SIL OFL 1.1,
//! `assets/fonts/OFL.txt`): any face a host loads works the same way.

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::WindowResolution;
use bevy_gooey::*;

/// The reading face the prose is drawn in.
const READING_FACE: &str = "fonts/liberation-serif-subset.ttf";

/// Capture mode's state, present only when `GOOEY_FRAMES` is set.
#[derive(Resource)]
struct Capture {
    at: u32,
    shot: String,
    frame: u32,
    total_ms: f64,
    counted: u32,
}

/// The first frame counted into the mean: start-up frames are not a HUD's cost.
const WARM_FRAMES: u32 = 20;

fn env<T: std::str::FromStr>(name: &str) -> Option<T> {
    std::env::var(name).ok().and_then(|s| s.parse().ok())
}

fn main() {
    let mut skin = match std::env::var("GOOEY_SKIN").as_deref() {
        Ok("parchment") => GooeySkin::parchment(),
        Ok("high_contrast") => GooeySkin::high_contrast(),
        Ok("tool") => GooeySkin::tool(),
        _ => GooeySkin::dark(),
    };
    if let Some(scale) = env("GOOEY_TEXT_SCALE") {
        skin.text_scale = scale;
    }
    let mut window = Window {
        title: "bevy_gooey text face".into(),
        ..default()
    };
    if let Some((w, h)) = std::env::var("GOOEY_SIZE")
        .ok()
        .and_then(|s| s.split_once('x').map(|(w, h)| (w.parse(), h.parse())))
        .and_then(|(w, h)| Some((w.ok()?, h.ok()?)))
    {
        window.resolution = WindowResolution::new(w, h).with_scale_factor_override(1.0);
    }
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }),
        GooeyPlugins,
    ))
    .insert_resource(ClearColor(skin.background))
    .insert_resource(skin)
    .add_systems(Startup, setup)
    .add_systems(Update, (follow_background, capture));
    if let Some(at) = env("GOOEY_FRAMES") {
        app.insert_resource(Capture {
            at,
            shot: std::env::var("GOOEY_SHOT").unwrap_or_else(|_| "text_face.png".into()),
            frame: 0,
            total_ms: 0.0,
            counted: 0,
        });
    }
    app.run();
}

/// A paragraph of the page, in the reading face. The weight is the skin's default: only the face is
/// pinned, so this text is as legible as any other at the player's text size.
fn prose(text: &str, face: &FontSource) -> impl Bundle {
    (label(text), TextFace::new(face.clone()))
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let face = assets.load(READING_FACE);
    // A heading keeps the skin's heading face: a face is per text, and a heading is not prose.
    let reader = commands
        .spawn((
            panel(),
            Sizing {
                width: Some(Extent::Percent(58.0)),
                height: Some(Extent::Percent(100.0)),
                ..default()
            },
            children![
                heading("Notes from the north orchard"),
                prose(
                    "Frost took the early blossom on the north rows, but the late varieties came \
                     through untouched. The hives lost three colonies of forty, which is a quarter \
                     of last year's winter.",
                    &face,
                ),
                prose(
                    "Prune the Bramley before the sap rises. Order grafting wax while the supplier \
                     still has last season's price, and set the new rows a metre further from the \
                     wall so the shadow of it never reaches them.",
                    &face,
                ),
                (
                    row(1.0),
                    children![
                        (label("Wrapped and set down at"), TextFace::new(face.clone())),
                        (label("C:\\orchard\\notes.md"), TextFace::new(face.clone())),
                    ],
                ),
                meter(60.0, 100.0),
                (
                    row(1.0),
                    children![button("Prune the north rows"), button("Leave them")],
                ),
            ],
        ))
        .id();
    // The other half: the skin's own face, so the two are read side by side on one screen.
    let settings = commands
        .spawn((
            panel(),
            Sizing {
                width: Some(Extent::Percent(42.0)),
                height: Some(Extent::Percent(100.0)),
                ..default()
            },
            children![
                heading("Orchard settings"),
                label("Everything here is the skin's body face."),
                toggle("Subtitles", true),
                choice("Frost warning", ["Off", "Gentle", "Hard"], 1),
                slider("Music volume", 0.6, 0.0..=1.0),
                binding("Prune", Binding::Key(KeyCode::Space)),
                field_problem_note(),
            ],
        ))
        .id();
    commands
        .spawn((
            screen(),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(24.0)),
                column_gap: Val::Px(24.0),
                ..default()
            },
        ))
        .add_children(&[reader, settings]);
}

/// A line saying what the screen is showing, in the note ink.
fn field_problem_note() -> impl Bundle {
    (
        label("Prose on the left is drawn in a face the host loaded; nothing else moved."),
        TextFlow {
            grow: Some(1.0),
            ..default()
        },
    )
}

fn follow_background(skin: Res<GooeySkin>, mut clear: ResMut<ClearColor>) {
    if skin.is_changed() {
        clear.0 = skin.background;
    }
}

fn capture(
    capture: Option<ResMut<Capture>>,
    time: Res<Time<Real>>,
    mut exit: MessageWriter<AppExit>,
    mut commands: Commands,
) {
    let Some(mut c) = capture else {
        return;
    };
    c.frame += 1;
    if c.frame > WARM_FRAMES && c.frame <= c.at {
        c.total_ms += time.delta_secs_f64() * 1000.0;
        c.counted += 1;
    }
    if c.frame == c.at {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(c.shot.clone()));
        let mean = if c.counted > 0 {
            c.total_ms / f64::from(c.counted)
        } else {
            0.0
        };
        println!("gooey: frames {} mean_ms {mean:.3}", c.at);
    }
    if c.frame == c.at + 10 {
        exit.write(AppExit::Success);
    }
}
