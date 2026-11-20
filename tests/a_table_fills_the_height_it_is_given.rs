//! **A table fills the height it is given** (#111).
//!
//! `tree()` is `table_bundle`, and `Role::Table` virtualizes -- it needs a bounded height to lay rows
//! into. The root node stated `flex_direction: Column` and no height, so inside a `fill_region` a table
//! laid out at its content's height and left the rest of its panel empty. `bevy_dark_factory` #524 is
//! what that looked like: a file tree 245 px tall in a 356 px region.
//!
//! The test measures the table against the region that holds it, at two region heights. A table that
//! fills is the region's height at both and grows between them; one at its content's height does neither.

use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::ui::ComputedNode;
use bevy::window::WindowResolution;
use bevy::winit::WinitPlugin;
use bevy_gooey::*;

/// The host column's height at each measurement.
const SHORT: f32 = 320.0;
const TALL: f32 = 560.0;
const WIDTH: f32 = 360.0;

/// Enough rows that the table's content is shorter than the tall region, so a table that fills is
/// unambiguously taller than its own content.
const ROWS: usize = 3;

fn app() -> App {
    let mut app = App::new();
    app.insert_resource(MotionSettings::off());
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WIDTH as u32, TALL as u32),
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

/// A `host_px` tall fill region holding a table of `ROWS` rows; returns the table's height and the
/// region's, in logical pixels.
fn measure(host_px: f32) -> (f32, f32) {
    let mut app = app();
    let world = app.world_mut();
    let region = world
        .spawn((
            Node {
                width: Val::Px(WIDTH),
                height: Val::Px(host_px),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                ..default()
            },
            Role::FillRegion,
        ))
        .id();
    let table = world
        .spawn((
            tree((0..ROWS).map(|i| TreeNode::new(i as u64, format!("row {i}")))),
            ChildOf(region),
        ))
        .id();
    for _ in 0..12 {
        app.update();
    }
    let table_h = app
        .world()
        .get::<ComputedNode>(table)
        .map(|n| n.size().y)
        .unwrap_or(-1.0);
    let region_h = app
        .world()
        .get::<ComputedNode>(region)
        .map(|n| n.size().y)
        .unwrap_or(-1.0);
    (table_h, region_h)
}

/// **The table's height is the region's height**, at both region heights.
#[test]
fn a_table_in_a_fill_region_fills_it() {
    let (short_table, short_region) = measure(SHORT);
    assert!(
        short_region > 0.0,
        "the region has a height: {short_region}"
    );
    assert!(
        (short_table - short_region).abs() <= 1.0,
        "**the table is {short_table} px in a region {short_region} px tall** (#111): a virtualized table \
         with no stated height falls back to its content's height instead of filling the region."
    );

    let (tall_table, tall_region) = measure(TALL);
    assert!(
        tall_region > short_region,
        "a taller region gives more height: {short_region} -> {tall_region}"
    );
    assert!(
        (tall_table - tall_region).abs() <= 1.0,
        "**the table is {tall_table} px in a region {tall_region} px tall** (#111): a table that filled \
         would have grown with the region."
    );
}
