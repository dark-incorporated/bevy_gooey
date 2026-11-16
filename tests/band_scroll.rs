//! A row that is a button keeps its own height in a scroll viewport (#105). `Role::Band` and
//! `Role::ListRow` set an explicit `min_height` (the skin's target), which switches off a flex item's
//! automatic content minimum, while a `scroll_viewport` is a flex column whose children keep Bevy's
//! default `flex_shrink: 1`. So whenever the rows together were taller than the viewport every row
//! was squeezed down to its `min_height` and a wrapping `fill_text` was drawn over the rows before and
//! after it. Measured on the finished layout at a scale factor of 2, at UI scales 1.0 and 2.4,
//! physical px.

use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::ui::{ComputedNode, UiGlobalTransform};
use bevy::window::WindowResolution;
use bevy::winit::WinitPlugin;
use bevy_gooey::*;

/// The names the issue measured: one line, two lines and four lines in a narrow panel.
const NAMES: [&str; 3] = [
    "characters",
    "dimensional_crab",
    "kenney_prototype-kit/Models/GLB format",
];

/// The rows this test builds: the two roles that are a button holding text in a list.
#[derive(Clone, Copy)]
enum Row {
    Band,
    ListRow,
}

impl Row {
    fn spawn(self, world: &mut World) -> Entity {
        match self {
            Self::Band => world.spawn(band()).id(),
            Self::ListRow => world.spawn(list_row()).id(),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Band => "a band",
            Self::ListRow => "a list row",
        }
    }
}

/// The editor's window (2560x1440) at a scale factor of 2, with the interface's own scale given.
fn app(ui_scale: f32) -> App {
    let mut app = App::new();
    app.insert_resource(MotionSettings::off());
    app.insert_resource(UiScaleSettings {
        reference_short_side: 720.0,
        scale: ui_scale,
    });
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(2560, 1440).with_scale_factor_override(2.0),
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
    ));
    app.world_mut().spawn(Camera2d);
    app
}

/// What one layout measured: every way a row is wrong, in sentences; the rows' own heights summed;
/// and the height of the viewport's host.
struct Measured {
    defects: Vec<String>,
    rows_px: f32,
    host_px: f32,
}

/// A `width_px` wide, `host_px` tall column holding a scroll viewport of `row`s: the three named texts
/// and, with `extra`, enough more that the rows are taller than the viewport.
///
/// Every way a row is wrong is a sentence: a row shorter than its own wrapped text, and two rows
/// drawn over each other.
fn measure(row: Row, ui_scale: f32, extra: usize, host_px: f32, width_px: f32) -> Measured {
    let mut app = app(ui_scale);
    let world = app.world_mut();
    let viewport = world
        .spawn(scroll_viewport(0.5, Some(Space::Px(12.0))))
        .id();
    let mut names: Vec<String> = NAMES.iter().map(|name| name.to_string()).collect();
    names.extend((0..extra).map(|i| format!("pack_{i:03}")));
    let rows: Vec<(Entity, Entity)> = names
        .into_iter()
        .map(|name| {
            // The shape `band`'s own doc gives: a glyph, the filling name, a count.
            let band = row.spawn(world);
            world.spawn((Text::new(">"), Role::ButtonText, ChildOf(band)));
            let text = world.spawn((fill_text(name), ChildOf(band))).id();
            world.spawn((count_text("9"), ChildOf(band)));
            world.entity_mut(band).insert(ChildOf(viewport));
            (band, text)
        })
        .collect();
    let host = world
        .spawn(Node {
            width: Val::Px(width_px),
            height: Val::Px(host_px),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .id();
    world.entity_mut(host).add_child(viewport);
    for _ in 0..12 {
        app.update();
    }
    let world = app.world();
    let mut defects = Vec::new();
    let mut rects = Vec::new();
    let mut rows_px = 0.0;
    for (index, (band, text)) in rows.iter().enumerate() {
        let laid = world.get::<ComputedNode>(*band).expect("laid out").size();
        let words = world.get::<ComputedNode>(*text).expect("laid out").size();
        let at = world
            .get::<UiGlobalTransform>(*band)
            .expect("placed")
            .translation;
        rects.push(Rect::from_center_size(at, laid));
        rows_px += laid.y;
        if laid.y + 0.5 < words.y {
            defects.push(format!(
                "row {index} ({:?}) is {laid:?} tall and holds a text {words:?}",
                NAMES.get(index).map_or("a further row", |name| name)
            ));
        }
    }
    for (i, a) in rects.iter().enumerate() {
        for (j, b) in rects.iter().enumerate().skip(i + 1) {
            let over = a.intersect(*b).size();
            if over.x > 1.0 && over.y > 1.0 {
                defects.push(format!(
                    "rows {i} and {j} are drawn over each other by {over:?}"
                ));
            }
        }
    }
    if !defects.is_empty() {
        defects.insert(
            0,
            format!(
                "{} at a UI scale of {ui_scale}, {} rows in a {width_px} px by {host_px} px viewport:",
                row.name(),
                rects.len()
            ),
        );
    }
    Measured {
        defects,
        rows_px,
        host_px,
    }
}

/// The host is tall enough that 63 rows at 2.4 are not: about 2000 physical px.
const ROOMY: f32 = 4000.0;
/// A viewport the three named rows alone fill at either scale.
const SHORT: f32 = 200.0;
/// The widths the issue gives: the 190-logical-px panel its body names and the 144.5 its own
/// reproduction column measured, each the scroll viewport's content.
const WIDTHS: [f32; 2] = [144.5, 190.0];

#[test]
fn a_band_is_as_tall_as_its_wrapped_text_when_the_rows_fit() {
    for ui_scale in [1.0, 2.4] {
        for width_px in WIDTHS {
            let measured = measure(Row::Band, ui_scale, 0, ROOMY, width_px);
            assert!(
                measured.rows_px < measured.host_px,
                "the rows do not fit: {:?}",
                measured.defects
            );
            assert_eq!(measured.defects, Vec::<String>::new());
        }
    }
}

#[test]
fn a_band_is_as_tall_as_its_wrapped_text_when_the_rows_overflow_the_viewport() {
    for ui_scale in [1.0, 2.4] {
        for width_px in WIDTHS {
            let measured = measure(Row::Band, ui_scale, 60, SHORT, width_px);
            assert!(
                measured.rows_px > measured.host_px,
                "the rows fit, so nothing is squeezed: {:?}",
                measured.defects
            );
            assert_eq!(measured.defects, Vec::<String>::new());
        }
    }
}

#[test]
fn a_list_row_is_as_tall_as_its_wrapped_text_when_the_rows_overflow_the_viewport() {
    for ui_scale in [1.0, 2.4] {
        for width_px in WIDTHS {
            let measured = measure(Row::ListRow, ui_scale, 60, SHORT, width_px);
            assert!(
                measured.rows_px > measured.host_px,
                "the rows fit, so nothing is squeezed: {:?}",
                measured.defects
            );
            assert_eq!(measured.defects, Vec::<String>::new());
        }
    }
}
