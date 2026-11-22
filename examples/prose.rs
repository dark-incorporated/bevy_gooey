//! The same page of notes in two code editors side by side (#114): on the left a notes pane that
//! wraps at its edge and draws Markdown in the skin's inks (`CodeSoftWrap`, `CodeMarkdown`), on
//! the right the editor as it was, plain text on one scrolling line per paragraph.
//!
//! Run `cargo run --release --example prose`.
//!
//! Capture mode: `GOOEY_FRAMES=<n>` saves a screenshot at frame `n` to `GOOEY_SHOT` (default
//! `prose.png`), prints `gooey: frames <n> mean_ms <x>` and exits 10 frames later.
//! `GOOEY_SKIN=parchment`, `GOOEY_TEXT_SCALE=<f>` and `GOOEY_SIZE=<w>x<h>` set the start.

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::WindowResolution;
use bevy_gooey::*;

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
        title: "bevy_gooey prose".into(),
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
            shot: std::env::var("GOOEY_SHOT").unwrap_or_else(|_| "prose.png".into()),
            frame: 0,
            total_ms: 0.0,
            counted: 0,
        });
    }
    app.run();
}

/// The page both editors show: a heading, a long paragraph, a list, a quote, a fence and links.
const PAGE: &str = "# Orchard notes\n\nFrost took the early blossom on the **north rows**, but the late varieties are untouched, and the beekeeper says the hives came through the cold snap with `three` colonies lost out of forty, which is better than last year.\n\n- Prune the *Bramley* before the sap rises\n- Order grafting wax from [the supplier](https://example.com/wax)\n- 2. Re-hang the gate\n\n> Wrap what you read, not what you write.\n\n```toml\nrows = 12\n```\n\nSee [the plan](plan.md) for the rest of the season, which is long and has a good deal in it.";

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let pane = |title: &'static str, editor: impl Bundle| {
        (
            Node {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![(panel(), children![heading(title), editor])],
        )
    };
    commands.spawn((
        screen(),
        children![(
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(24.0)),
                column_gap: Val::Px(24.0),
                ..default()
            },
            children![
                pane(
                    "Wrapped, as Markdown",
                    (code_editor("Notes", PAGE), CodeSoftWrap, CodeMarkdown)
                ),
                pane("Plain", code_editor("Notes", PAGE)),
            ],
        )],
    ));
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
