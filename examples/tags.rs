//! `tag`: a short word on a plate whose colour names a kind of thing. Name plates ("ELITE", "BOSS"),
//! item rarity, a console's level pills and a "2 STALE" count that takes no room when it is cleared
//! (the button writes an empty string; nothing else changes).
//!
//! Run `cargo run --release --example tags`.
//!
//! Capture mode, for a still: `GOOEY_FRAMES=<n>` saves a screenshot at frame `n` to `GOOEY_SHOT`
//! (default `tags.png`) and exits 10 frames later. `GOOEY_SKIN=parchment`, `high_contrast`, `tool`
//! or `desk`, `GOOEY_TEXT_SCALE=<f>` and `GOOEY_SIZE=<w>x<h>` set the start; `GOOEY_CLEAR=1` clears
//! the stale count before the shot.

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::ui_widgets::Activate;
use bevy::window::WindowResolution;
use bevy_gooey::*;

fn env<T: std::str::FromStr>(name: &str) -> Option<T> {
    std::env::var(name).ok().and_then(|s| s.parse().ok())
}

/// The host's own categories are data in the skin: a player's palette swap is one skin change.
const RARE: Color = Color::srgb(0.95, 0.78, 0.25);
const LEGENDARY: Color = Color::srgb(0.95, 0.5, 0.15);
const FROST: Color = Color::srgb(0.55, 0.78, 0.95);

fn main() {
    let size = std::env::var("GOOEY_SIZE").unwrap_or_else(|_| "560x400".into());
    let (w, h) = size.split_once('x').unwrap_or(("560", "400"));
    let mut skin = match std::env::var("GOOEY_SKIN").as_deref() {
        Ok("parchment") => GooeySkin::parchment(),
        Ok("high_contrast") => GooeySkin::high_contrast(),
        Ok("tool") => GooeySkin::tool(),
        Ok("desk") => GooeySkin::desk(),
        _ => GooeySkin::dark(),
    };
    skin.tones.insert("rare".into(), RARE);
    skin.tones.insert("legendary".into(), LEGENDARY);
    skin.tones.insert("frost".into(), FROST);
    if let Some(scale) = env::<f32>("GOOEY_TEXT_SCALE") {
        skin.text_scale = scale;
    }
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "tags".into(),
                    resolution: WindowResolution::new(
                        w.parse().unwrap_or(560),
                        h.parse().unwrap_or(400),
                    )
                    .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
            GooeyPlugins,
        ))
        .insert_resource(skin)
        .add_systems(Startup, setup)
        .add_systems(Update, (follow_background, capture))
        .run();
}

/// The tag a button clears.
#[derive(Component)]
struct Stale;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        screen(),
        children![(
            Node {
                width: Val::Px(440.0),
                ..default()
            },
            children![(
                panel(),
                children![
                    heading("Tags"),
                    plates("Wolf", (tag("ELITE"), TagTone("danger")), tag("NEW")),
                    plates(
                        "Frost Giant",
                        (tag("BOSS"), TagTone("accent")),
                        (tag("FROST"), TagTone("frost")),
                    ),
                    plates(
                        "Ashen Blade",
                        (tag("Legendary"), TagTone("legendary")),
                        (tag("Rare"), TagTone("rare")),
                    ),
                    plates(
                        "Console",
                        (tag("WARN"), TagTone("warning")),
                        (tag("LIVE"), TagTone("live")),
                    ),
                    (
                        row(0.5),
                        children![
                            label("Stale files"),
                            (tag("2 STALE"), TagTone("warning"), Stale),
                            label("after"),
                        ],
                    ),
                    (
                        button("Clear stale"),
                        observe(
                            |_: On<Activate>,
                             mut tags: Query<&Children, With<Stale>>,
                             mut texts: Query<&mut Text>| {
                                for kids in &mut tags {
                                    for kid in kids {
                                        if let Ok(mut text) = texts.get_mut(*kid) {
                                            text.0.clear();
                                        }
                                    }
                                }
                            }
                        ),
                    ),
                ],
            )],
        )],
    ));
}

/// A name and two tags beside it.
fn plates(name: &'static str, first: impl Bundle, second: impl Bundle) -> impl Bundle {
    (row(0.5), children![label(name), first, second])
}

fn follow_background(skin: Res<GooeySkin>, mut clear: ResMut<ClearColor>) {
    if skin.is_changed() {
        clear.0 = skin.background;
    }
}

fn capture(
    mut frame: Local<u32>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
    mut stale: Query<&Children, With<Stale>>,
    mut texts: Query<&mut Text>,
) {
    *frame += 1;
    let Some(at) = env::<u32>("GOOEY_FRAMES") else {
        return;
    };
    if *frame == at - 5 && env::<u32>("GOOEY_CLEAR") == Some(1) {
        for kids in &mut stale {
            for kid in kids {
                if let Ok(mut text) = texts.get_mut(*kid) {
                    text.0.clear();
                }
            }
        }
    }
    if *frame == at {
        let path = std::env::var("GOOEY_SHOT").unwrap_or_else(|_| "tags.png".into());
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
    if *frame > at + 10 {
        exit.write(AppExit::Success);
    }
}
