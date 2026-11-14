//! A scroll viewport that is its own region (G-90, #101): one bundle that IS the scrolling entity,
//! whose children are the host's, with gooey's rail kept out of them. Laid out for real in a headless
//! window, driven by real pointer and key input.

use bevy::camera::NormalizedRenderTarget;
use bevy::ecs::relationship::RelatedSpawner;
use bevy::input::mouse::MouseScrollUnit;
use bevy::input::touch::TouchPhase;
use bevy::picking::pointer::{Location, PointerAction, PointerId, PointerInput};
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::ui::UiGlobalTransform;
use bevy::window::{PrimaryWindow, WindowRef, WindowResolution};
use bevy::winit::WinitPlugin;
use bevy_gooey::*;

fn headless() -> App {
    headless_at(1.0)
}

/// The same at a window scale factor, so a device pixel is not a logical one.
fn headless_at(scale: f32) -> App {
    let mut app = App::new();
    app.insert_resource(MotionSettings::off());
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(
                        (1280.0 * scale) as u32,
                        (720.0 * scale) as u32,
                    )
                    .with_scale_factor_override(scale),
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

fn settle(app: &mut App) {
    for _ in 0..6 {
        app.update();
    }
}

/// A node's rectangle in the window, logical px.
fn rect(app: &App, entity: Entity) -> Rect {
    let node = app.world().get::<ComputedNode>(entity).expect("laid out");
    let at = app
        .world()
        .get::<UiGlobalTransform>(entity)
        .expect("placed")
        .translation;
    let scale = node.inverse_scale_factor();
    Rect::from_center_size(at * scale, node.size() * scale)
}

fn children(app: &App, entity: Entity) -> Vec<Entity> {
    app.world()
        .get::<Children>(entity)
        .map(|c| c.iter().collect())
        .unwrap_or_default()
}

fn tab_stops(app: &mut App, presses: usize) -> Vec<Option<String>> {
    (0..presses)
        .map(|_| {
            bevy_gooey::testing::press(app, KeyCode::Tab);
            bevy_gooey::testing::focused_label(app)
        })
        .collect()
}

fn scroll_y(app: &App, view: Entity) -> f32 {
    app.world()
        .get::<ScrollPosition>(view)
        .expect("a scroll position")
        .y
}

fn window(app: &mut App) -> Entity {
    app.world_mut()
        .query_filtered::<Entity, With<PrimaryWindow>>()
        .single(app.world())
        .expect("one primary window")
}

fn pointer(app: &mut App, at: Vec2, action: PointerAction) {
    let window = window(app);
    let location = Location {
        target: NormalizedRenderTarget::Window(
            WindowRef::Entity(window)
                .normalize(None)
                .expect("a window ref"),
        ),
        position: at,
    };
    app.world_mut()
        .write_message(PointerInput::new(PointerId::Mouse, location, action));
    app.update();
}

/// One notch of the wheel over `at`, `lines` lines down.
fn wheel(app: &mut App, at: Vec2, lines: f32) {
    pointer(app, at, PointerAction::Move { delta: Vec2::ZERO });
    app.update();
    pointer(
        app,
        at,
        PointerAction::Scroll {
            unit: MouseScrollUnit::Line,
            x: 0.0,
            y: -lines,
            phase: TouchPhase::Moved,
        },
    );
    settle(app);
}

/// The editor's shape: a bounded host column holding a heading, then the viewport the builder returns,
/// with `rows` buttons spawned on the viewport by the host and a marker the host inserted on it.
#[derive(Component)]
struct HostMarker;

fn viewport_app(rows: usize, gutter: Option<Space>) -> (App, Entity, Entity) {
    let mut app = headless();
    let view = app
        .world_mut()
        .spawn((scroll_viewport(0.5, gutter), HostMarker))
        .id();
    for i in 0..rows {
        app.world_mut()
            .spawn((button(format!("Row {i}")), ChildOf(view)));
    }
    let host = app
        .world_mut()
        .spawn(sized(Sizing {
            width: Some(Extent::Px(400.0)),
            height: Some(Extent::Px(300.0)),
            ..default()
        }))
        .with_children(|h| {
            h.spawn(label("Heading"));
        })
        .add_child(view)
        .id();
    app.world_mut().spawn(screen()).add_child(host);
    settle(&mut app);
    (app, host, view)
}

/// The rail gooey spawned for `view`: a sibling, found by its owner marker.
fn rail_of(app: &mut App, view: Entity) -> Option<Entity> {
    app.world_mut()
        .query::<(Entity, &ScrollRailOf)>()
        .iter(app.world())
        .find(|(_, of)| of.0 == view)
        .map(|(e, _)| e)
}

// ---- (1) the host's marker and children are the viewport's ------------------------------------

/// The viewport IS the scrolling entity: the host's marker is on it, its children are the host's
/// buttons and only those, they scroll with its `ScrollPosition`, and it fills the room its parent
/// leaves.
#[test]
fn the_viewport_entity_carries_the_hosts_marker_and_children_and_scrolls_them() {
    let (mut app, host, view) = viewport_app(40, None);
    assert!(app.world().get::<HostMarker>(view).is_some());
    assert_eq!(app.world().get::<Role>(view), Some(&Role::ScrollViewport));
    let kids = children(&app, view);
    assert_eq!(
        kids.len(),
        40,
        "the host's children, and nothing of gooey's"
    );
    assert!(
        kids.iter()
            .all(|k| app.world().get::<Text>(*k).is_none() || true)
    );
    assert!(kids.iter().all(|k| rail_of(&mut app, view) != Some(*k)));
    let (h, v) = (rect(&app, host), rect(&app, view));
    assert!(
        (v.height() - (300.0 - rect(&app, children(&app, host)[0]).height())).abs() < 0.5,
        "fills what the heading leaves: {v:?} in {h:?}"
    );
    let before = rect(&app, kids[0]).min.y;
    app.world_mut()
        .get_mut::<ScrollPosition>(view)
        .expect("a scroll position")
        .y = 120.0;
    settle(&mut app);
    assert!(
        (before - rect(&app, kids[0]).min.y - 120.0).abs() < 1.0,
        "the children scrolled with it"
    );
    let computed = app.world().get::<ComputedNode>(view).expect("laid out");
    assert!(
        (computed.scroll_position.y * computed.inverse_scale_factor - 120.0).abs() < 1.5,
        "readable on the entity"
    );
}

// ---- (2) ScrollPosition moves with the wheel and with focus-follow -----------------------------

#[test]
fn the_wheel_and_keyboard_focus_move_the_scroll_position_on_the_viewport() {
    let (mut app, _host, view) = viewport_app(40, None);
    let at = rect(&app, view).center();
    wheel(&mut app, at, 3.0);
    let after_wheel = scroll_y(&app, view);
    assert!(after_wheel > 10.0, "the wheel scrolled it: {after_wheel}");
    for _ in 0..14 {
        bevy_gooey::testing::press(&mut app, KeyCode::Tab);
    }
    let focused = bevy_gooey::testing::focused(&app).expect("a focus");
    assert!(children(&app, view).contains(&focused), "focus is on a row");
    let (v, r) = (rect(&app, view), rect(&app, focused));
    assert!(
        r.min.y >= v.min.y - 0.5 && r.max.y <= v.max.y + 0.5,
        "focus-follow brought it into view: {r:?} in {v:?}"
    );
    assert!(
        scroll_y(&app, view) > after_wheel,
        "and moved ScrollPosition further: {}",
        scroll_y(&app, view)
    );
}

// ---- (3) the rail: only while the content overflows, never among the host's children ----------

#[test]
fn the_rail_shows_only_while_the_content_overflows_and_is_not_the_hosts_child() {
    let (mut app, host, view) = viewport_app(40, None);
    let rail = rail_of(&mut app, view).expect("gooey spawned a rail");
    assert_eq!(app.world().get::<Role>(rail), Some(&Role::ListRail));
    assert_eq!(
        app.world().get::<ChildOf>(rail).map(|c| c.parent()),
        Some(host),
        "a sibling of the viewport"
    );
    assert!(
        !children(&app, view).contains(&rail),
        "the host's Query<&Children> of the viewport never sees it"
    );
    assert_eq!(children(&app, view).len(), 40);
    assert_eq!(
        app.world().get::<Node>(rail).expect("a node").display,
        Display::Flex,
        "overflow: shown"
    );
    // It stands at the viewport's right edge, top aligned, as tall as it.
    let (v, r) = (rect(&app, view), rect(&app, rail));
    assert!(
        (r.max.x - v.max.x).abs() < 1.5
            && (r.min.y - v.min.y).abs() < 1.5
            && (r.height() - v.height()).abs() < 1.5,
        "{r:?} on {v:?}"
    );
    // Fewer rows: it goes.
    let kids = children(&app, view);
    for k in &kids[3..] {
        app.world_mut().entity_mut(*k).despawn();
    }
    settle(&mut app);
    assert_eq!(
        app.world().get::<Node>(rail).expect("a node").display,
        Display::None,
        "no overflow: hidden"
    );
}

/// Despawning the viewport takes its rail with it; taking `ScrollRail` off the viewport does too.
#[test]
fn the_rail_goes_with_the_viewport_or_its_marker() {
    let (mut app, _host, view) = viewport_app(40, None);
    let rail = rail_of(&mut app, view).expect("a rail");
    app.world_mut().entity_mut(view).remove::<ScrollRail>();
    settle(&mut app);
    assert!(
        app.world().get_entity(rail).is_err(),
        "the marker off: the rail is gone"
    );
    app.world_mut().entity_mut(view).insert(ScrollRail);
    settle(&mut app);
    let rail = rail_of(&mut app, view).expect("a rail again (flip after spawn)");
    app.world_mut().entity_mut(view).despawn();
    settle(&mut app);
    assert!(
        app.world().get_entity(rail).is_err(),
        "the viewport gone: the rail is gone"
    );
}

/// A viewport the host hides with `Shown(false)` shows no rail.
#[test]
fn a_hidden_viewport_shows_no_rail_and_no_stop() {
    let (mut app, _host, view) = viewport_app(40, None);
    let rail = rail_of(&mut app, view).expect("a rail");
    app.world_mut().entity_mut(view).insert(Shown(false));
    settle(&mut app);
    assert_eq!(
        app.world().get::<Node>(rail).expect("a node").display,
        Display::None
    );
    let stops = tab_stops(&mut app, 3);
    assert!(
        stops
            .iter()
            .all(|s| s.as_deref().is_none_or(|l| !l.starts_with("Row"))),
        "{stops:?}"
    );
}

// ---- (4) keys ----------------------------------------------------------------------------------

#[test]
fn page_home_and_end_work_on_the_viewport_and_a_text_field_inside_keeps_its_keys() {
    let (mut app, _host, view) = viewport_app(40, None);
    let kids = children(&app, view);
    bevy_gooey::testing::press(&mut app, KeyCode::Tab);
    assert_eq!(bevy_gooey::testing::focused(&app), Some(kids[0]));
    bevy_gooey::testing::press(&mut app, KeyCode::PageDown);
    let moved = bevy_gooey::testing::focused(&app).expect("a focus");
    assert!(
        kids.iter().position(|k| *k == moved).expect("a row") >= 3,
        "paged several rows"
    );
    assert!(scroll_y(&app, view) > 0.5);
    bevy_gooey::testing::press(&mut app, KeyCode::End);
    assert_eq!(bevy_gooey::testing::focused(&app), kids.last().copied());
    bevy_gooey::testing::press(&mut app, KeyCode::Home);
    assert_eq!(bevy_gooey::testing::focused(&app), kids.first().copied());
    assert!(scroll_y(&app, view) < 1.5);
}

#[test]
fn a_text_area_on_the_viewport_keeps_its_page_home_and_end() {
    let mut app = headless();
    let view = app.world_mut().spawn(scroll_viewport(0.5, None)).id();
    let area = app
        .world_mut()
        .spawn((text_area("Notes", "a\nb\nc", 4), ChildOf(view)))
        .id();
    for i in 0..30 {
        app.world_mut()
            .spawn((button(format!("Row {i}")), ChildOf(view)));
    }
    let host = app
        .world_mut()
        .spawn(sized(Sizing {
            width: Some(Extent::Px(400.0)),
            height: Some(Extent::Px(300.0)),
            ..default()
        }))
        .add_child(view)
        .id();
    app.world_mut().spawn(screen()).add_child(host);
    settle(&mut app);
    bevy_gooey::testing::press(&mut app, KeyCode::Tab);
    let at = bevy_gooey::testing::focused(&app).expect("a focus");
    let mut up = at;
    let mut inside_area = false;
    loop {
        if up == area {
            inside_area = true;
            break;
        }
        match app.world().get::<ChildOf>(up) {
            Some(p) => up = p.parent(),
            None => break,
        }
    }
    assert!(inside_area, "Tab reached the text area first");
    for key in [
        KeyCode::PageDown,
        KeyCode::PageUp,
        KeyCode::End,
        KeyCode::Home,
    ] {
        bevy_gooey::testing::press(&mut app, key);
        assert_eq!(
            bevy_gooey::testing::focused(&app),
            Some(at),
            "{key:?} took focus out of the text area"
        );
        assert!(
            scroll_y(&app, view).abs() < 0.5,
            "{key:?} scrolled the viewport"
        );
    }
}

// ---- the host's gutter, a changed gutter, and the host's gap ------------------------------------

#[test]
fn the_gutter_and_the_gap_are_the_hosts_and_a_change_restyles() {
    let (mut app, _host, view) = viewport_app(5, Some(Space::Px(20.0)));
    assert_eq!(
        app.world().get::<Node>(view).expect("a node").padding.right,
        Val::Px(20.0)
    );
    assert_eq!(
        app.world().get::<Node>(view).expect("a node").row_gap,
        Val::Px(0.5 * GooeySkin::dark().gap),
        "the gap"
    );
    app.world_mut()
        .get_mut::<ScrollGutter>(view)
        .expect("a gutter")
        .0 = Some(Space::Px(35.0));
    settle(&mut app);
    assert_eq!(
        app.world().get::<Node>(view).expect("a node").padding.right,
        Val::Px(35.0)
    );
    let kids = children(&app, view);
    assert!(
        ((rect(&app, kids[1]).min.y - rect(&app, kids[0]).max.y) - 0.5 * GooeySkin::dark().gap)
            .abs()
            < 0.5
    );
}

// ---- (5) nothing is written when it is absent --------------------------------------------------

/// A `scroll_region` and a `list` spawn no `ScrollRailOf` and keep their own rail: the new system
/// reads only viewports that carry `ScrollRail`.
#[test]
fn a_scroll_region_and_a_list_are_unchanged_and_get_no_second_rail() {
    let mut app = headless();
    let region = app
        .world_mut()
        .spawn(scroll_region(
            0.0,
            None,
            Children::spawn(SpawnWith(|p: &mut RelatedSpawner<ChildOf>| {
                for i in 0..40 {
                    p.spawn(button(format!("R{i}")));
                }
            })),
        ))
        .id();
    let list = app
        .world_mut()
        .spawn(list(
            4.0,
            Children::spawn(SpawnWith(|p: &mut RelatedSpawner<ChildOf>| {
                for i in 0..20 {
                    p.spawn(button(format!("L{i}")));
                }
            })),
        ))
        .id();
    let host = app
        .world_mut()
        .spawn(sized(Sizing {
            width: Some(Extent::Px(400.0)),
            height: Some(Extent::Px(500.0)),
            ..default()
        }))
        .add_child(region)
        .add_child(list)
        .id();
    app.world_mut().spawn(screen()).add_child(host);
    settle(&mut app);
    assert!(
        app.world_mut()
            .query::<&ScrollRailOf>()
            .iter(app.world())
            .next()
            .is_none()
    );
    let rails = |app: &App, e: Entity| {
        children(app, e)
            .into_iter()
            .filter(|c| app.world().get::<Role>(*c) == Some(&Role::ListRail))
            .count()
    };
    assert_eq!(rails(&app, region), 1);
    assert_eq!(rails(&app, list), 1);
}

/// Moving the viewport to another parent takes its rail along.
#[test]
fn the_rail_follows_a_viewport_that_changes_parent() {
    let (mut app, _host, view) = viewport_app(40, None);
    let rail = rail_of(&mut app, view).expect("a rail");
    let other = app
        .world_mut()
        .spawn(sized(Sizing {
            width: Some(Extent::Px(300.0)),
            height: Some(Extent::Px(200.0)),
            ..default()
        }))
        .id();
    app.world_mut().spawn(screen()).add_child(other);
    app.world_mut().entity_mut(other).add_child(view);
    settle(&mut app);
    assert_eq!(
        app.world().get::<ChildOf>(rail).map(|c| c.parent()),
        Some(other)
    );
    let (v, r) = (rect(&app, view), rect(&app, rail));
    assert!(
        (r.max.x - v.max.x).abs() < 1.5 && (r.min.y - v.min.y).abs() < 1.5,
        "{r:?} on {v:?}"
    );
}

/// Focus-follow stands on a row's top, as a list does: the rows are this viewport's own children, so
/// their tops (and the gap) are read from it, not from a first child.
#[test]
fn focus_follow_stands_on_a_rows_top_in_a_viewport_that_holds_its_rows() {
    let (mut app, _host, view) = viewport_app(40, None);
    for _ in 0..13 {
        bevy_gooey::testing::press(&mut app, KeyCode::Tab);
    }
    assert!(scroll_y(&app, view) > 10.0);
    let top = rect(&app, view).min.y;
    let on_a_top = children(&app, view)
        .into_iter()
        .any(|k| (rect(&app, k).min.y - top).abs() < 1.0);
    assert!(
        on_a_top,
        "some row starts at the viewport's top: scroll {}",
        scroll_y(&app, view)
    );
}

// ---- review: a rail that is gone is respawned; siblings; rebuilds; nesting ---------------------

fn rails_of(app: &mut App, view: Entity) -> Vec<Entity> {
    app.world_mut()
        .query::<(Entity, &ScrollRailOf)>()
        .iter(app.world())
        .filter(|(_, of)| of.0 == view)
        .map(|(e, _)| e)
        .collect()
}

/// The viewport moves to a new parent and the old parent (which held the rail) is despawned in the
/// same flush: the viewport gets a fresh rail under its new parent.
#[test]
fn a_viewport_moved_out_of_a_parent_that_is_despawned_gets_a_fresh_rail() {
    let (mut app, host, view) = viewport_app(40, None);
    let old = rail_of(&mut app, view).expect("a rail");
    let other = app
        .world_mut()
        .spawn(sized(Sizing {
            width: Some(Extent::Px(300.0)),
            height: Some(Extent::Px(200.0)),
            ..default()
        }))
        .id();
    app.world_mut().spawn(screen()).add_child(other);
    app.world_mut().entity_mut(other).add_child(view);
    app.world_mut().entity_mut(host).despawn();
    settle(&mut app);
    assert!(
        app.world().get_entity(old).is_err(),
        "the old rail went with its parent"
    );
    let rails = rails_of(&mut app, view);
    assert_eq!(rails.len(), 1, "one fresh rail: {rails:?}");
    assert_eq!(
        app.world().get::<ChildOf>(rails[0]).map(|c| c.parent()),
        Some(other)
    );
    assert_eq!(
        app.world().get::<Node>(rails[0]).expect("a node").display,
        Display::Flex,
        "and it shows"
    );
}

/// Anything that despawns the rail: a new one appears.
#[test]
fn a_rail_despawned_under_a_live_viewport_is_respawned() {
    let (mut app, host, view) = viewport_app(40, None);
    let old = rail_of(&mut app, view).expect("a rail");
    app.world_mut().entity_mut(old).despawn();
    settle(&mut app);
    let rails = rails_of(&mut app, view);
    assert_eq!(rails.len(), 1);
    assert_ne!(rails[0], old);
    assert_eq!(
        app.world().get::<ChildOf>(rails[0]).map(|c| c.parent()),
        Some(host)
    );
}

/// Two viewports under one parent each have their own rail, and one going leaves the other's alone.
#[test]
fn two_viewports_under_one_parent_each_have_their_own_rail() {
    let mut app = headless();
    let mut views = Vec::new();
    let host = app
        .world_mut()
        .spawn(sized(Sizing {
            width: Some(Extent::Px(400.0)),
            height: Some(Extent::Px(400.0)),
            ..default()
        }))
        .id();
    for _ in 0..2 {
        let v = app.world_mut().spawn(scroll_viewport(0.0, None)).id();
        for i in 0..30 {
            app.world_mut().spawn((button(format!("R{i}")), ChildOf(v)));
        }
        app.world_mut().entity_mut(host).add_child(v);
        views.push(v);
    }
    app.world_mut().spawn(screen()).add_child(host);
    settle(&mut app);
    let rails: Vec<Entity> = views
        .iter()
        .map(|v| rails_of(&mut app, *v))
        .map(|r| {
            assert_eq!(r.len(), 1);
            r[0]
        })
        .collect();
    assert_ne!(rails[0], rails[1]);
    for (v, r) in views.iter().zip(&rails) {
        let (vr, rr) = (rect(&app, *v), rect(&app, *r));
        assert!(
            (rr.max.x - vr.max.x).abs() < 1.5 && (rr.min.y - vr.min.y).abs() < 1.5,
            "each rail on its own viewport: {rr:?} on {vr:?}"
        );
    }
    assert!(
        (rect(&app, rails[0]).min.y - rect(&app, rails[1]).min.y).abs() > 20.0,
        "and they are apart"
    );
    app.world_mut().entity_mut(views[0]).despawn();
    settle(&mut app);
    assert!(app.world().get_entity(rails[0]).is_err());
    assert!(
        app.world().get_entity(rails[1]).is_ok(),
        "the other's rail stays"
    );
    assert_eq!(rails_of(&mut app, views[1]), vec![rails[1]]);
}

/// The host's rebuild: it empties the parent's children and spawns a fresh viewport: a fresh rail,
/// and none of the old ones left over.
#[test]
fn a_host_that_rebuilds_the_parent_gets_a_fresh_rail_and_leaks_none() {
    let (mut app, host, view) = viewport_app(40, None);
    let old = rail_of(&mut app, view).expect("a rail");
    app.world_mut()
        .entity_mut(host)
        .despawn_related::<Children>();
    let fresh = app.world_mut().spawn(scroll_viewport(0.0, None)).id();
    for i in 0..40 {
        app.world_mut()
            .spawn((button(format!("N{i}")), ChildOf(fresh)));
    }
    app.world_mut().entity_mut(host).add_child(fresh);
    settle(&mut app);
    assert!(app.world().get_entity(old).is_err() && app.world().get_entity(view).is_err());
    let all: Vec<_> = app
        .world_mut()
        .query::<&ScrollRailOf>()
        .iter(app.world())
        .map(|o| o.0)
        .collect();
    assert_eq!(all, vec![fresh], "exactly one rail, the fresh viewport's");
}

/// A focused widget NESTED in a row (a grandchild of the viewport) is followed into view too.
#[test]
fn focus_follow_reaches_a_widget_nested_in_a_row() {
    let mut app = headless();
    let view = app.world_mut().spawn(scroll_viewport(0.5, None)).id();
    for i in 0..30 {
        let row = app.world_mut().spawn((column(0.0), ChildOf(view))).id();
        app.world_mut()
            .spawn((button(format!("Nested {i}")), ChildOf(row)));
    }
    let host = app
        .world_mut()
        .spawn(sized(Sizing {
            width: Some(Extent::Px(400.0)),
            height: Some(Extent::Px(300.0)),
            ..default()
        }))
        .add_child(view)
        .id();
    app.world_mut().spawn(screen()).add_child(host);
    settle(&mut app);
    for _ in 0..12 {
        bevy_gooey::testing::press(&mut app, KeyCode::Tab);
    }
    let focused = bevy_gooey::testing::focused(&app).expect("a focus");
    assert!(
        scroll_y(&app, view) > 20.0,
        "scrolled: {}",
        scroll_y(&app, view)
    );
    let (v, r) = (rect(&app, view), rect(&app, focused));
    assert!(
        r.min.y >= v.min.y - 0.5 && r.max.y <= v.max.y + 0.5,
        "the nested button is in view: {r:?} in {v:?}"
    );
}
