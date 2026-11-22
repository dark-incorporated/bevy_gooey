//! Three host knobs on one screen: a split whose divider is a thin 6 px line (`SplitThickness`), a
//! table whose Notes column wraps its text (`Column::wrap`) and a status column of LEDs whose
//! labels are drawn one type-scale step down (`LedSize`).
//!
//! Run `cargo run --release --example host_knobs`.
//!
//! Capture mode: `GOOEY_FRAMES=<n>` saves a screenshot at frame `n` to `GOOEY_SHOT` (default
//! `host_knobs.png`), prints `gooey: frames <n> mean_ms <x>` and exits 10 frames later.
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
/// The divider's thickness: a hairline between two panes, in logical px.
const DIVIDER_PX: f32 = 6.0;
/// The status LEDs' type-scale step: one below the body's.
const LED_STEP: usize = 1;

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
        title: "bevy_gooey host knobs".into(),
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
            shot: std::env::var("GOOEY_SHOT").unwrap_or_else(|_| "host_knobs.png".into()),
            frame: 0,
            total_ms: 0.0,
            counted: 0,
        });
    }
    app.run();
}

/// The table's rows: a name, a count and a note that is sometimes much longer than its cell.
fn rows() -> Vec<Vec<String>> {
    [
        (
            "Meadow",
            "12",
            "Open ground east of the mill; the stream runs along its southern edge.",
        ),
        ("Harbour", "9", "Quiet."),
        (
            "Mill",
            "3",
            "Wheel is jammed. The miller says the pond froze overnight and cracked the axle.",
        ),
        ("Cellar", "1", "Locked from the inside."),
        (
            "Orchard",
            "27",
            "Frost took the early blossom; the late varieties are untouched.",
        ),
    ]
    .into_iter()
    .map(|(a, b, c)| vec![a.to_string(), b.to_string(), c.to_string()])
    .collect()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        screen(),
        children![(
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            children![(
                split(
                    SplitAxis::Horizontal,
                    (
                        panel(),
                        children![
                            heading("Status"),
                            (led("Agent", LedState::Live), LedSize(LED_STEP)),
                            (led("Build", LedState::Busy), LedSize(LED_STEP)),
                            (led("Deploy", LedState::Failed), LedSize(LED_STEP)),
                            (led("Mirror", LedState::Off), LedSize(LED_STEP)),
                            label("Labels at one step down"),
                        ],
                    ),
                    (
                        panel(),
                        children![
                            heading("Places"),
                            table(
                                [
                                    Column::new("Name").width(7.0),
                                    Column::new("Qty").width(4.0),
                                    Column::new("Notes").width(15.0).unsortable().wrap(),
                                ],
                                rows(),
                            ),
                        ],
                    ),
                ),
                SplitThickness(DIVIDER_PX),
            )],
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
