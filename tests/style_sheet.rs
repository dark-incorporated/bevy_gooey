//! The style sheet: classes on nodes, rules read from YAML, and the cascade writing a classed node's
//! box and colours, laid out for real in a headless window. Guide: `docs/guide/style-sheets.md`.
#![cfg(feature = "serde")]

use bevy::camera::NormalizedRenderTarget;
use bevy::picking::pointer::{Location, PointerAction, PointerId, PointerInput};
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::ui::{IsDefaultUiCamera, UiGlobalTransform};
use bevy::window::{PrimaryWindow, WindowRef, WindowResolution};
use bevy::winit::WinitPlugin;
use bevy_gooey::*;

fn headless() -> App {
    let mut app = App::new();
    app.insert_resource(MotionSettings::off());
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
    ));
    app.world_mut().spawn((Camera2d, IsDefaultUiCamera));
    app
}

fn settle(app: &mut App) {
    for _ in 0..4 {
        app.update();
    }
}

/// A sheet from YAML rules, as a host reads its file.
fn sheet(yaml: &str, revision: u64) -> StyleSheet {
    StyleSheet {
        rules: serde_yaml_ng::from_str(yaml).expect("the test's rules read"),
        revision,
    }
}

fn padding(app: &App, entity: Entity) -> UiRect {
    app.world().get::<Node>(entity).expect("a node").padding
}

fn point_at(app: &mut App, entity: Entity) {
    let node = app.world().get::<ComputedNode>(entity).expect("laid out");
    let at = app
        .world()
        .get::<UiGlobalTransform>(entity)
        .expect("placed")
        .translation
        * node.inverse_scale_factor();
    let window = app
        .world_mut()
        .query_filtered::<Entity, With<PrimaryWindow>>()
        .single(app.world())
        .expect("a window");
    let location = Location {
        target: NormalizedRenderTarget::Window(
            WindowRef::Entity(window)
                .normalize(None)
                .expect("a window ref"),
        ),
        position: at,
    };
    app.world_mut().write_message(PointerInput::new(
        PointerId::Mouse,
        location,
        PointerAction::Move { delta: Vec2::X },
    ));
    for _ in 0..3 {
        app.update();
    }
}

fn fill(app: &App, entity: Entity) -> Option<Color> {
    app.world()
        .get::<InkOverride>(entity)
        .and_then(|ink| ink.fill)
}

const RED: Color = Color::srgb(1.0, 0.0, 0.0);
const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);

#[test]
fn a_rule_pads_a_classed_panel_by_a_step_of_the_skin() {
    let mut app = headless();
    app.insert_resource(sheet(r#"".card": {padding: !Step 2}"#, 1));
    let card = app
        .world_mut()
        .spawn((panel(), StyleClass::of("card")))
        .id();
    app.world_mut().spawn(screen()).add_child(card);
    settle(&mut app);
    let step = Val::Px(app.world().resource::<GooeySkin>().space(2));
    let drawn = padding(&app, card);
    assert_eq!(
        [drawn.left, drawn.right, drawn.top, drawn.bottom],
        [step; 4],
        "every side is step 2 of the spacing scale"
    );
}

#[test]
fn an_empty_sheet_gives_the_panel_back_its_spawn_padding() {
    let mut app = headless();
    app.insert_resource(sheet(r#"".card": {padding: {all: !Px 30, top: Zero}}"#, 1));
    let card = app
        .world_mut()
        .spawn((panel(), StyleClass::of("card")))
        .id();
    let plain = app.world_mut().spawn(panel()).id();
    app.world_mut().spawn(screen()).add_children(&[card, plain]);
    settle(&mut app);
    let styled = padding(&app, card);
    assert_eq!(styled.top, Val::Px(0.0), "a side wins over `all`");
    assert_eq!(styled.left, Val::Px(30.0));
    assert_ne!(styled, padding(&app, plain), "the rule moved the padding");

    app.insert_resource(StyleSheet {
        rules: StyleRules::default(),
        revision: 2,
    });
    settle(&mut app);
    assert_eq!(
        padding(&app, card),
        padding(&app, plain),
        "with the rule gone, the panel is padded as one spawned without a class"
    );
    assert!(
        app.world().get::<Spacing>(card).is_none(),
        "the spawn set no Spacing, so none is left"
    );
}

#[test]
fn a_descendant_hover_rule_lights_only_the_hovered_row_in_the_list_and_beats_an_earlier_class() {
    let mut app = headless();
    app.insert_resource(sheet(
        r##"
".list .row:hover": {background: !Color "#0000ff"}
".row": {background: !Color "#ff0000"}
"##,
        1,
    ));
    let first = app
        .world_mut()
        .spawn((button("First"), StyleClass::of("row")))
        .id();
    let second = app
        .world_mut()
        .spawn((button("Second"), StyleClass::of("row")))
        .id();
    let list = app
        .world_mut()
        .spawn((column(1.0), StyleClass::of("list")))
        .add_children(&[first, second])
        .id();
    let outside = app
        .world_mut()
        .spawn((button("Outside"), StyleClass::of("row")))
        .id();
    app.world_mut()
        .spawn(screen())
        .add_children(&[list, outside]);
    settle(&mut app);
    for row in [first, second, outside] {
        assert_eq!(fill(&app, row), Some(RED), "no row is hovered yet");
    }

    point_at(&mut app, first);
    assert_eq!(
        fill(&app, first),
        Some(BLUE),
        "the hovered row in the list: specificity 3 beats the later `.row`"
    );
    assert_eq!(
        fill(&app, second),
        Some(RED),
        "its neighbour is not hovered"
    );

    point_at(&mut app, outside);
    assert_eq!(
        fill(&app, first),
        Some(RED),
        "the pointer left the first row"
    );
    assert_eq!(
        fill(&app, outside),
        Some(RED),
        "a hovered row outside any `.list` is only a `.row`"
    );
}

#[test]
fn of_two_rules_equally_specific_the_later_wins() {
    let mut app = headless();
    app.insert_resource(sheet(
        r#"
".a": {z: 1}
".b": {z: 2}
"#,
        1,
    ));
    let node = app.world_mut().spawn((panel(), StyleClass::of("a b"))).id();
    app.world_mut().spawn(screen()).add_child(node);
    settle(&mut app);
    assert_eq!(
        app.world().get::<GlobalZIndex>(node),
        Some(&GlobalZIndex(2))
    );

    app.insert_resource(sheet(
        r#"
".b": {z: 2}
".a": {z: 1}
"#,
        2,
    ));
    settle(&mut app);
    assert_eq!(
        app.world().get::<GlobalZIndex>(node),
        Some(&GlobalZIndex(1))
    );
}

#[test]
fn code_keeps_what_the_sheet_does_not_decide_and_a_host_can_ask_which_it_does() {
    let mut app = headless();
    app.insert_resource(sheet(
        r##"".card": {padding: !Step 2, background: !Color "#ff0000"}"##,
        1,
    ));
    let card = app
        .world_mut()
        .spawn((panel(), StyleClass::of("card")))
        .id();
    app.world_mut().spawn(screen()).add_child(card);
    settle(&mut app);
    let base = app
        .world()
        .get::<CascadeBase>(card)
        .expect("a classed node has a cascade base");
    assert!(
        base.decides::<Spacing>() && base.decides::<InkOverride>(),
        "the rule decides the padding and the fill"
    );
    assert!(
        !base.decides::<Sizing>() && !base.decides::<GapPx>() && !base.decides::<Node>(),
        "and nothing else"
    );
    assert_eq!(fill(&app, card), Some(RED));

    // A width code writes after the sheet ran is the code's: the cascade leaves it where it is.
    let width = Extent::Px(123.0);
    app.world_mut().entity_mut(card).insert(Sizing {
        width: Some(width),
        ..default()
    });
    settle(&mut app);
    assert_eq!(
        app.world()
            .get::<Sizing>(card)
            .and_then(|sizing| sizing.width),
        Some(width),
        "the sheet does not decide the width, so it does not take it back"
    );

    app.insert_resource(sheet(r#"".card": {padding: !Step 2}"#, 2));
    settle(&mut app);
    assert!(
        !app.world()
            .get::<CascadeBase>(card)
            .expect("still classed")
            .decides::<InkOverride>(),
        "a sheet that stops painting the card no longer decides its ink"
    );
}

#[test]
fn a_host_paint_that_is_not_there_is_one_problem_and_leaves_the_colour_alone() {
    let mut app = headless();
    app.insert_resource(sheet(
        r#"
".row": {background: !Host missing, gap: !Px 7}
".row:hover": {background: !Host missing}
".quiet": {background: !Host missing}
"#,
        1,
    ));
    let row = app
        .world_mut()
        .spawn((column(1.0), StyleClass::of("row")))
        .id();
    app.world_mut().spawn(screen()).add_child(row);
    settle(&mut app);
    let problems = &app.world().resource::<StyleSheetProblems>().0;
    assert_eq!(
        problems,
        &[
            ".quiet background: no host paint `missing`",
            ".row background: no host paint `missing`",
            ".row:hover background: no host paint `missing`",
        ],
        "one line per rule and field, the rule that matches nothing included"
    );
    assert!(
        app.world().get::<InkOverride>(row).is_none(),
        "no colour is guessed"
    );
    assert_eq!(
        app.world().get::<GapPx>(row),
        Some(&GapPx(7.0)),
        "the rule's other properties are still written"
    );

    let mut paints = HostPaints::default();
    paints.0.insert("missing".into(), RED);
    app.insert_resource(paints);
    settle(&mut app);
    assert!(app.world().resource::<StyleSheetProblems>().0.is_empty());
    assert_eq!(
        fill(&app, row),
        Some(RED),
        "the host's paint, once it is there"
    );
}

#[test]
fn a_selector_that_does_not_read_is_refused_with_its_column() {
    let upper = serde_yaml_ng::from_str::<StyleRules>(r#"".Row": {gap: Zero}"#)
        .expect_err("an upper-case class is refused");
    assert!(
        upper.to_string().contains("selector `.Row`: at column 2"),
        "{upper}"
    );
    let wobble = ron::from_str::<StyleRules>(r#"{".row:wobble": (gap: Zero)}"#)
        .expect_err("an unknown state is refused");
    assert!(
        wobble
            .to_string()
            .contains("selector `.row:wobble`: at column 6: unknown state `wobble`"),
        "{wobble}"
    );
    let subject = ".row :hover"
        .parse::<Selector>()
        .expect_err("no class last");
    assert!(subject.reason.contains("names a class"), "{subject}");
    let field = serde_yaml_ng::from_str::<StyleRules>(r#"".row": {paddin: Zero}"#)
        .expect_err("an unknown property is refused");
    assert!(field.to_string().contains("paddin"), "{field}");
}

#[test]
fn a_skin_field_the_skin_does_not_have_is_refused() {
    let refused = ron::from_str::<GooeySkin>("(gap: 4.0, gapp: 1.0)").expect_err("refused");
    assert!(refused.to_string().contains("gapp"), "{refused}");
    assert!(ron::from_str::<GooeySkin>("(gap: 4.0)").is_ok());
}

#[test]
fn explain_lists_the_matching_rules_in_cascade_order_with_their_source() {
    let mut app = headless();
    let mut rules: StyleRules = serde_yaml_ng::from_str(
        r#"
".chip.chip-quiet": {gap: !Px 2}
".chip": {gap: !Px 1, z: 3}
".other": {z: 9}
"#,
    )
    .expect("reads");
    for rule in &mut rules.0 {
        rule.source = "editor.yaml".into();
    }
    app.insert_resource(StyleSheet { rules, revision: 1 });
    let chip = app
        .world_mut()
        .spawn((panel(), StyleClass::of("chip chip-quiet")))
        .id();
    app.world_mut().spawn(screen()).add_child(chip);
    settle(&mut app);
    let explained = explain(app.world(), chip).expect("a classed node");
    assert_eq!(explained.classes, ["chip", "chip-quiet"]);
    let order: Vec<(&str, u32)> = explained
        .matched
        .iter()
        .map(|rule| (rule.selector.as_str(), rule.specificity))
        .collect();
    assert_eq!(order, [(".chip", 1), (".chip.chip-quiet", 2)]);
    assert!(
        explained
            .matched
            .iter()
            .all(|rule| rule.source == "editor.yaml")
    );
    assert_eq!(explained.resolved.gap, Some(Space::Px(2.0)));
    assert_eq!(explained.resolved.z, Some(3));
    assert_eq!(app.world().get::<GapPx>(chip), Some(&GapPx(2.0)));
}
