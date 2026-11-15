//! Scroll regions: `scroll_region` and `scroll_viewport`, two columns of thirty buttons each in a host 320 px tall.
//!
//! - The left panel is a `scroll_region(0.5, Some(Space::Px(12.0)), ..)`: a column that fills its host and scrolls, with gooey's rail down its right edge while the content does not fit. The wheel, a drag on the rail and keyboard focus (Tab, the arrows) scroll it, and so do PageUp and PageDown (focus moves a viewport) and Home and End (focus goes to the first or last button).
//! - The right panel is a `scroll_viewport(0.5, Some(Space::Px(12.0)))`: the same column as one entity, with a marker of this example's on it (`TagView`) and its rows spawned as its children. The line under it reads `ScrollPosition` off that entity. The first row is a **Volume** slider, which keeps Home and End for itself: with focus on it they move the slider, not the column.
//!
//! Capture mode is the gallery's: `GOOEY_FRAMES`, `GOOEY_SHOT`, `GOOEY_SHOT_DIR`, `GOOEY_SHOT_FROM`, `GOOEY_SHOT_EVERY`, `GOOEY_SIZE`, `GOOEY_SKIN` and `GOOEY_TEXT_SCALE`. `GOOEY_KEYS` takes `;`-separated steps, one every `GOOEY_KEY_EVERY` frames: `tab`, `up`, `down`, `left`, `right`, `enter`, `space`, `pageup`, `pagedown`, `home` and `end`.

use bevy::ecs::relationship::RelatedSpawner;
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_gooey::*;

/// Capture mode's state, present only when `GOOEY_FRAMES` is set.
#[derive(Resource)]
struct Capture {
    at: u32,
    shot: String,
    dir: Option<String>,
    from: u32,
    every: u32,
    kept: u32,
    frame: u32,
    total_ms: f64,
    counted: u32,
    step_every: u32,
    steps: Vec<Step>,
}

/// A staged step of `GOOEY_KEYS`.
#[derive(Clone, Debug)]
enum Step {
    Key(KeyCode, Key),
}

/// The frame `GOOEY_KEYS` starts at, after the layout has settled.
const KEY_FROM: u32 = 30;
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
        title: "bevy_gooey scroll".into(),
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
    .insert_resource(skin)
    .add_systems(Startup, setup)
    .add_systems(Update, (follow_background, capture, show_offset))
    // Staged keys are written before `PreUpdate`, as a real keyboard's are.
    .add_systems(First, stage);
    if let Some(at) = env("GOOEY_FRAMES") {
        app.insert_resource(Capture {
            at,
            shot: std::env::var("GOOEY_SHOT").unwrap_or_else(|_| "scroll.png".into()),
            dir: std::env::var("GOOEY_SHOT_DIR").ok(),
            from: env("GOOEY_SHOT_FROM").unwrap_or(KEY_FROM),
            every: env("GOOEY_SHOT_EVERY").unwrap_or(2),
            kept: 0,
            frame: 0,
            total_ms: 0.0,
            counted: 0,
            step_every: env("GOOEY_KEY_EVERY").unwrap_or(24),
            steps: steps_of(&std::env::var("GOOEY_KEYS").unwrap_or_default()),
        });
    }
    app.run();
}

fn steps_of(text: &str) -> Vec<Step> {
    text.split(';')
        .filter_map(|word| {
            Some(match word.trim() {
                "tab" => Step::Key(KeyCode::Tab, Key::Tab),
                "up" => Step::Key(KeyCode::ArrowUp, Key::ArrowUp),
                "down" => Step::Key(KeyCode::ArrowDown, Key::ArrowDown),
                "left" => Step::Key(KeyCode::ArrowLeft, Key::ArrowLeft),
                "right" => Step::Key(KeyCode::ArrowRight, Key::ArrowRight),
                "enter" => Step::Key(KeyCode::Enter, Key::Enter),
                "space" => Step::Key(KeyCode::Space, Key::Space),
                "pageup" => Step::Key(KeyCode::PageUp, Key::PageUp),
                "pagedown" => Step::Key(KeyCode::PageDown, Key::PageDown),
                "home" => Step::Key(KeyCode::Home, Key::Home),
                "end" => Step::Key(KeyCode::End, Key::End),
                _ => return None,
            })
        })
        .collect()
}

/// The scrolling entity of the right-hand column: the example's own marker on the entity that
/// `scroll_viewport` returned, which is how a host finds it.
#[derive(Component)]
struct TagView;

/// The line under the right-hand panel that reads the viewport's `ScrollPosition`.
#[derive(Component)]
struct Offset;

/// What a region or a viewport fills: a box with a height, since a column of thirty rows has none.
fn host() -> impl Bundle {
    sized(Sizing {
        width: Some(Extent::Px(300.0)),
        height: Some(Extent::Px(320.0)),
        ..default()
    })
}

/// Two panels, each a column of thirty buttons: a `scroll_region` and a `scroll_viewport`.
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let region = commands
        .spawn(scroll_region(
            0.5,
            Some(Space::Px(12.0)),
            Children::spawn(SpawnWith(|rows: &mut RelatedSpawner<ChildOf>| {
                for i in 1..=30 {
                    rows.spawn(button(format!("Tag {i}")));
                }
            })),
        ))
        .id();
    let region_host = commands.spawn(host()).add_child(region).id();
    let left = commands
        .spawn((
            panel(),
            children![
                heading("scroll_region"),
                label("The wheel, the rail, Tab, Page and Home.")
            ],
        ))
        .add_child(region_host)
        .id();

    let view = commands
        .spawn((scroll_viewport(0.5, Some(Space::Px(12.0))), TagView))
        .id();
    commands.spawn((slider("Volume", 5.0, 0.0..=10.0), ChildOf(view)));
    for i in 1..=30 {
        commands.spawn((button(format!("Tag {i}")), ChildOf(view)));
    }
    let view_host = commands.spawn(host()).add_child(view).id();
    let right = commands
        .spawn((
            panel(),
            children![
                heading("scroll_viewport"),
                label("The same column as one entity.")
            ],
        ))
        .add_child(view_host)
        .id();
    commands.spawn((label(""), Offset, ChildOf(right)));

    commands
        .spawn(screen())
        .insert(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(24.0),
            padding: UiRect::top(Val::Px(12.0)),
            ..default()
        })
        .add_children(&[left, right]);
}

/// The right-hand column's scroll offset, read off the entity the host put its marker on.
fn show_offset(
    view: Query<&ScrollPosition, With<TagView>>,
    mut text: Query<&mut Text, With<Offset>>,
) {
    let (Ok(position), Ok(mut text)) = (view.single(), text.single_mut()) else {
        return;
    };
    let line = format!("ScrollPosition.y = {:.0}", position.y);
    if text.0 != line {
        text.0 = line;
    }
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
    if let Some(dir) = c.dir.clone() {
        let every = c.every.max(1);
        if c.frame >= c.from && (c.frame - c.from) % every == 0 {
            c.kept += 1;
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(format!("{dir}/f_{:04}.png", c.kept)));
        }
        if c.frame == c.at {
            println!("gooey: kept {} frames in {dir}", c.kept);
        }
    }
    if c.frame == c.at + 10 {
        exit.write(AppExit::Success);
    }
}

/// `GOOEY_KEYS`: real keyboard input, one step every `GOOEY_KEY_EVERY` frames.
fn stage(
    capture: Option<Res<Capture>>,
    window: Query<Entity, With<PrimaryWindow>>,
    mut keys: MessageWriter<KeyboardInput>,
) {
    let (Some(c), Ok(window)) = (capture, window.single()) else {
        return;
    };
    let Some(since) = c.frame.checked_sub(KEY_FROM) else {
        return;
    };
    let every = c.step_every.max(8);
    let Some(step) = c.steps.get((since / every) as usize) else {
        return;
    };
    let phase = since % every;
    let mut key = |key_code: KeyCode, logical_key: Key, text: Option<&str>, state: ButtonState| {
        keys.write(KeyboardInput {
            key_code,
            logical_key,
            state,
            text: text.map(Into::into),
            repeat: false,
            window,
        });
    };
    match step {
        Step::Key(key_code, logical) => match phase {
            0 => key(*key_code, logical.clone(), None, ButtonState::Pressed),
            n if n == every / 2 => key(*key_code, logical.clone(), None, ButtonState::Released),
            _ => {}
        },
    }
}
