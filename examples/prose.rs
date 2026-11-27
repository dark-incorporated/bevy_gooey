//! The same page of notes in two code editors side by side (#114): on the left a notes pane that
//! wraps at its edge and draws Markdown in the skin's inks (`CodeSoftWrap`, `CodeMarkdown`), on
//! the right the editor as it was, plain text on one scrolling line per paragraph.
//!
//! Run `cargo run --release --example prose`.
//!
//! Capture mode: `GOOEY_FRAMES=<n>` saves a screenshot at frame `n` to `GOOEY_SHOT` (default
//! `prose.png`), prints `gooey: frames <n> mean_ms <x>` and exits 10 frames later.
//! `GOOEY_SHOT_DIR=<dir>` also keeps every `GOOEY_SHOT_EVERY`th frame from `GOOEY_SHOT_FROM`
//! (default 30) there as `f_%04d.png`, which is a GIF's.
//! `GOOEY_SKIN=parchment`, `GOOEY_TEXT_SCALE=<f>` and `GOOEY_SIZE=<w>x<h>` set the start.
//!
//! The ink theme (#116): F3 steps through the `SyntaxPreset`s (skin, warm, cool, mono, separated),
//! `GOOEY_SYNTAX=warm` (or any other `SyntaxPreset` name, lower case) starts on one. The skin does
//! the rest: both editors recolour in the frame it changes.
//!
//! `GOOEY_MINIMAP=1` puts a long script in the right pane with `CodeMinimap` on it: a file opened
//! for reading, whose bars and band are drawn on its first frame with no input at all (#117).
//!
//! `GOOEY_WRAPPED_MINIMAP=1` turns on both `CodeSoftWrap` and `CodeMinimap` in the left pane, over
//! the long script, so the lane can be seen drawing **one bar per line** where the pane draws
//! thousands of rows: the file's outline does not lengthen when the window narrows.
//!
//! `GOOEY_SCROLL=<line>` puts the minimap pane's view at that line before the capture, so a still can
//! show the band where a bug was: it is as tall at line 200 as at line 0 (#123).
//!
//! `GOOEY_FIND=<query>` opens the find bar on the notes pane with that query and presses Enter
//! `GOOEY_FIND_STEPS` times (default 2) before the shot: the bar's count walks its matches and the
//! text is untouched (#128).

use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input_focus::{FocusCause, InputFocus};
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::WindowResolution;
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
    if let Ok(name) = std::env::var("GOOEY_SYNTAX") {
        match SyntaxPreset::ALL
            .into_iter()
            .find(|p| p.name().eq_ignore_ascii_case(&name))
        {
            Some(preset) => skin.syntax_theme = preset.theme(),
            None => eprintln!("gooey: GOOEY_SYNTAX {name:?} is not a syntax preset"),
        }
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
    .add_systems(
        Update,
        (
            follow_background,
            step_syntax_theme,
            scroll_lane,
            find_stage,
            capture,
        ),
    );
    if let Ok(query) = std::env::var("GOOEY_FIND") {
        app.insert_resource(FindStage {
            query,
            steps: env("GOOEY_FIND_STEPS").unwrap_or(2),
            sent: 0,
            frame: 0,
            opened: false,
            focused: false,
        });
    }
    if let Some(line) = env("GOOEY_SCROLL") {
        app.insert_resource(LaneScroll { line });
    }
    if let Some(at) = env("GOOEY_FRAMES") {
        app.insert_resource(Capture {
            at,
            shot: std::env::var("GOOEY_SHOT").unwrap_or_else(|_| "prose.png".into()),
            dir: std::env::var("GOOEY_SHOT_DIR").ok(),
            from: env("GOOEY_SHOT_FROM").unwrap_or(30),
            every: env("GOOEY_SHOT_EVERY").unwrap_or(1),
            kept: 0,
            frame: 0,
            total_ms: 0.0,
            counted: 0,
        });
    }
    app.run();
}

/// The page both editors show: a heading, a paragraph, a list, a quote, a fence and links.
const PAGE: &str = "# Orchard notes\n\nFrost took the early blossom on the **north rows**, but the late varieties are untouched, and the hives came through the cold snap with `three` colonies lost out of forty.\n\n- Prune the *Bramley* before the sap rises\n- Order grafting wax from [the supplier](https://example.com/wax)\n\n> Wrap what you read, not what you write.\n\n```toml\nrows = 12\n```\nSee [the plan](plan.md) for the rest.";

/// One half of the screen: a titled panel around an editor.
/// A pane's outer column; its contents are spawned by the caller, so an editor's own markers can
/// go on the editor and not on the column around it.
fn pane() -> impl Bundle {
    (Node {
        width: Val::Percent(50.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default()
    },)
}

/// A file long enough to need the minimap: 300 lines of a level script.
fn script() -> String {
    let mut text = String::from("// Orchard level script\n");
    for row in 0..300 {
        match row % 4 {
            0 => text.push_str(&format!("let row{row} = place(\"apple\", {row}, 0);\n")),
            1 => text.push_str(&format!("if row{row}.ripe {{ orchard.pick(row{row}); }}\n")),
            2 => text.push('\n'),
            _ => text.push_str(&format!("// the {row}th row is left for the frost\n")),
        }
    }
    text
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    // A wrapped pane can carry the lane too: it is an outline of the file, so its length does not
    // follow the wrap (one bar per line, `docs/research/minimap-wrap.md`).
    let wrapped_lane = std::env::var("GOOEY_WRAPPED_MINIMAP").is_ok();
    let wrapped_title = if wrapped_lane {
        "Wrapped script, with a minimap"
    } else {
        "Wrapped, as Markdown"
    };
    let wrapped = commands.spawn(pane()).id();
    commands.entity(wrapped).with_children(|pane| {
        pane.spawn((
            panel(),
            Sizing {
                height: Some(Extent::Percent(100.0)),
                ..default()
            },
        ))
        .with_children(|body| {
            body.spawn(heading(wrapped_title));
            if wrapped_lane {
                body.spawn((code_editor("Script", script()), CodeSoftWrap, CodeMinimap));
            } else {
                body.spawn((
                    code_editor("Notes", PAGE),
                    CodeSoftWrap,
                    CodeMarkdown,
                    Notes,
                ));
            }
        });
    });
    // The right pane is a file opened for reading in the capture mode: no focus, no keystroke, so
    // its minimap has to be whole on the frame it is first drawn (#117).
    let right_lane = std::env::var("GOOEY_MINIMAP").is_ok();
    let right_title: &str = if right_lane {
        "Script, with a minimap"
    } else {
        "Plain"
    };
    let right = commands.spawn(pane()).id();
    commands.entity(right).with_children(|pane| {
        pane.spawn((
            panel(),
            Sizing {
                height: Some(Extent::Percent(100.0)),
                ..default()
            },
        ))
        .with_children(|body| {
            body.spawn(heading(right_title));
            if right_lane {
                body.spawn((code_editor("Script", script()), CodeMinimap, Lane));
            } else {
                body.spawn(code_editor("Notes", PAGE));
            }
        });
    });
    commands
        .spawn(screen())
        .insert(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(24.0)),
            column_gap: Val::Px(24.0),
            ..default()
        })
        .add_children(&[wrapped, right]);
}

/// The wrapped notes pane `GOOEY_FIND` opens its bar on.
#[derive(Component)]
struct Notes;

/// The pane `GOOEY_SCROLL` scrolls: the editor with the lane on it.
#[derive(Component)]
struct Lane;

/// `GOOEY_SCROLL=<line>` puts the lane's view at that line, once, so a still can show the band where
/// a bug was (#123).
fn scroll_lane(
    scroll: Option<Res<LaneScroll>>,
    mut lane: Query<&mut CodeEditor, With<Lane>>,
    mut commands: Commands,
) {
    let Some(scroll) = scroll else {
        return;
    };
    for mut editor in &mut lane {
        // A line's height is known only once the editor has been laid out; before that
        // `scroll_to_line` would set the goal to 0 px and the resource would be spent.
        if editor.visible_lines().is_empty() {
            return;
        }
        editor.scroll_to_line(scroll.line);
    }
    commands.remove_resource::<LaneScroll>();
}

/// `GOOEY_SCROLL`'s line.
#[derive(Resource)]
struct LaneScroll {
    line: usize,
}

/// `GOOEY_FIND`: the query, how many Enters to press in the bar, and where the staging stands.
#[derive(Resource)]
struct FindStage {
    query: String,
    steps: u32,
    sent: u32,
    frame: u32,
    opened: bool,
    focused: bool,
}

/// `GOOEY_FIND`'s staging: open the bar on the notes pane with the query, then press Enter in it
/// once every [`FIND_BEAT`] frames, as the player would, until the count has stepped `steps` times.
fn find_stage(
    stage: Option<ResMut<FindStage>>,
    mut editors: Query<&mut CodeFind, With<CodeMarkdown>>,
    fields: Query<Entity, With<bevy::ui_widgets::TextInput>>,
    parents: Query<&ChildOf>,
    notes: Query<(), With<Notes>>,
    window: Query<Entity, With<bevy::window::PrimaryWindow>>,
    mut focus: ResMut<InputFocus>,
    mut keys: MessageWriter<KeyboardInput>,
) {
    let Some(mut stage) = stage else {
        return;
    };
    stage.frame += 1;
    if !stage.opened && stage.frame > 20 {
        for mut find in &mut editors {
            find.open = true;
            find.query.clone_from(&stage.query);
            stage.opened = true;
        }
        return;
    }
    // The bar's query takes the keys once it is built, which is a frame after it opens.
    if stage.opened && !stage.focused {
        for field in &fields {
            let mut at = field;
            while let Ok(parent) = parents.get(at) {
                if notes.contains(parent.parent()) {
                    focus.set(field, FocusCause::Navigated);
                    stage.focused = true;
                    return;
                }
                at = parent.parent();
            }
        }
        return;
    }
    if stage.sent >= stage.steps {
        return;
    }
    if stage.frame < 40 || (stage.frame - 40) % FIND_BEAT != 0 {
        return;
    }
    let Ok(window) = window.single() else {
        return;
    };
    for state in [ButtonState::Pressed, ButtonState::Released] {
        keys.write(KeyboardInput {
            key_code: KeyCode::Enter,
            logical_key: Key::Enter,
            state,
            text: None,
            repeat: false,
            window,
        });
    }
    stage.sent += 1;
}

/// Frames between two staged Enters.
const FIND_BEAT: u32 = 12;

/// F3 gives the skin the next ink theme after the one it has (the skin's own after a custom one).
fn step_syntax_theme(keys: Res<ButtonInput<KeyCode>>, mut skin: ResMut<GooeySkin>) {
    if !keys.just_pressed(KeyCode::F3) {
        return;
    }
    let at = SyntaxPreset::ALL
        .into_iter()
        .position(|p| p.theme() == skin.syntax_theme);
    let next = SyntaxPreset::ALL[at.map_or(0, |i| (i + 1) % SyntaxPreset::ALL.len())];
    info!("gooey: syntax theme {}", next.name());
    skin.syntax_theme = next.theme();
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
    if let Some(dir) = c.dir.clone()
        && c.frame >= c.from
        && (c.frame - c.from) % c.every.max(1) == 0
        && c.frame <= c.at
    {
        c.kept += 1;
        let path = format!("{dir}/f_{:04}.png", c.frame);
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
    if c.frame == c.at {
        if let Some(dir) = &c.dir {
            println!("gooey: kept {} frames in {dir}", c.kept);
        }
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
