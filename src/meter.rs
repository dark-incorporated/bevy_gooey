//! HUD meters: a value against a maximum, drawn as a fill, with a trail that shows what was lost.

use bevy::prelude::*;
use bevy::ecs::spawn::SpawnIter;

use crate::model::{MeterShown, meter_fraction, meter_motion};
use crate::motion::{MotionSettings, MotionToken};
use crate::skin::{GooeySkin, Role, SkinScope, skin_of};

/// A meter's reading. Change it and [`update_meters`] moves the fill: a loss drops it at once and
/// leaves a [`Role::MeterTrail`] of the lost length that holds a moment and drains; a gain takes
/// it up on the skin's snappy spring (see [`meter_motion`]; [`MotionSettings::off`] makes both instant). Any value is safe: the
/// fill is clamped to empty and full, and a non-finite or non-positive maximum reads as empty.
#[derive(Component, Clone, Copy, Debug, PartialEq, Reflect)]
#[reflect(Component, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[require(MeterMotion)]
pub struct Meter {
    /// The current value, e.g. health left.
    pub value: f32,
    /// The value that fills the meter.
    pub max: f32,
}

/// The tone a [`crate::meter`]'s fill is drawn in: the name of a [`GooeySkin::tones`] entry, else
/// of one of the skin's own colours (`danger`, `accent`, ...), the names
/// [`GooeySkin::tone_problems`] resolves. With none the fill is the accent. Put it on the
/// [`crate::meter`]'s root, `(meter(60.0, 100.0), MeterTone("health"))`: three bars in a corner are
/// told apart by colour at a glance, and the colour is the skin's data, so a player's own palette is
/// one skin change. A toned meter's trail is its own colour lifted
/// ([`GooeySkin::meter_trail`]); colour is never the only cue, the length and the label are.
///
/// A name the skin does not know draws the accent. Check a tone you add with
/// [`GooeySkin::tone_problems`]: the fill on the track ([`GooeySkin::surface_raised`]) at 3:1.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component, Debug, PartialEq, Clone)]
pub struct MeterTone(pub &'static str);

/// Cuts a [`crate::meter`] into this many equal cells: `n - 1` notches, each a line weight wide in
/// the track's colour, standing at `i / n` of the fill's own length, so a unit half full shows half
/// full. Put it on the meter's root. Below 2 there is nothing to cut; above
/// [`METER_MAX_SEGMENTS`] the notches would crowd the fill and are refused with a warning.
/// [`update_meter_notches`] builds them when it changes and takes them away when it is removed.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component, Debug, PartialEq, Clone)]
pub struct MeterSegments(pub u32);

/// The most cells a [`MeterSegments`] meter takes: past it a cell of a 160 px bar is under three
/// line weights wide.
pub const METER_MAX_SEGMENTS: u32 = 64;

/// Builds the notches of each [`MeterSegments`] meter: one [`Role::MeterNotchLane`] child of the
/// meter holding the [`Role::MeterNotch`]es, replaced when the count changes and despawned when the
/// component goes. A settled meter costs nothing. Runs in [`crate::GooeySystems::Style`], before
/// [`crate::apply_skin`], which writes the lane's and the notches' look.
pub fn update_meter_notches(
    mut commands: Commands,
    segmented: Query<(Entity, &MeterSegments), Changed<MeterSegments>>,
    mut gone: RemovedComponents<MeterSegments>,
    children: Query<&Children>,
    roles: Query<&Role>,
) {
    let lanes = |meter: Entity| -> Vec<Entity> {
        children
            .get(meter)
            .into_iter()
            .flat_map(|c| c.iter())
            .filter(|&child| roles.get(child) == Ok(&Role::MeterNotchLane))
            .collect()
    };
    for meter in gone.read() {
        for lane in lanes(meter) {
            commands.entity(lane).despawn();
        }
    }
    for (meter, segments) in &segmented {
        for lane in lanes(meter) {
            commands.entity(lane).despawn();
        }
        let cells = segments.0;
        if cells > METER_MAX_SEGMENTS {
            warn!(
                "bevy_gooey: a meter's MeterSegments({cells}) is past the most, {METER_MAX_SEGMENTS}: no notches"
            );
            continue;
        }
        if cells < 2 {
            continue;
        }
        let notches = (1..cells).map(move |i| {
            (
                Node {
                    left: Val::Percent(100.0 * i as f32 / cells as f32),
                    ..default()
                },
                Role::MeterNotch,
                Pickable::IGNORE,
            )
        });
        commands.entity(meter).with_child((
            Node::default(),
            Role::MeterNotchLane,
            // Over the fill (`ZIndex(1)`), so a cell half full is cut where the cell ends.
            ZIndex(2),
            Pickable::IGNORE,
            Children::spawn(SpawnIter(notches)),
        ));
    }
}

/// What a meter draws now: its fill and trail, and whether it has been drawn once (the first frame
/// is the value, never a rise from empty). Internal; a game reads [`Meter`].
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct MeterMotion {
    shown: MeterShown,
    primed: bool,
}

/// Moves each [`Meter`]'s fill and trail toward its value: the fill's width, the trail's width and
/// its negative start. A settled meter whose value did not change is skipped before any node is
/// touched. Runs in [`crate::GooeySystems::Style`], on real time (a paused game's HUD still settles).
pub fn update_meters(
    settings: Option<Res<MotionSettings>>,
    skin: Res<GooeySkin>,
    scopes: Query<&SkinScope>,
    parents: Query<&ChildOf>,
    time: Option<Res<Time<Real>>>,
    mut meters: Query<(Entity, Ref<Meter>, &mut MeterMotion, &Children)>,
    mut parts: Query<(&Role, &mut Node)>,
) {
    // With no clock the old jump: a zero scale is the target at once.
    let scale = match (&settings, &time) {
        // Reduced motion lands the fill: it is travel.
        (Some(settings), Some(_)) if settings.lands_travel() => 0.0,
        (Some(settings), Some(_)) => settings.duration_scale,
        (None, Some(_)) => 1.0,
        _ => 0.0,
    };
    let dt = time.map_or(0.0, |time| time.delta_secs());
    for (entity, meter, mut motion, children) in &mut meters {
        let target = meter_fraction(meter.value, meter.max);
        let shown = motion.shown;
        if motion.primed && !meter.is_changed() && shown.fill == target && shown.trail == target {
            continue;
        }
        let gain = skin_of(&skin, entity, &scopes, &parents)
            .motion
            .spring(MotionToken::Snappy);
        let next = if motion.primed {
            meter_motion(shown, target, dt, gain, scale)
        } else {
            meter_motion(shown, target, dt, gain, 0.0)
        };
        if motion.primed && next == shown {
            continue;
        }
        motion.shown = next;
        motion.primed = true;
        let fill = Val::Percent(next.fill * 100.0);
        // Nothing at all while there is no trail: a node under the fill would still fringe its
        // rounded corners in the trail's colour.
        let (trail, back) = if next.trail > next.fill {
            (
                Val::Percent(next.trail * 100.0),
                Val::Percent(-next.fill * 100.0),
            )
        } else {
            (Val::ZERO, Val::ZERO)
        };
        for child in children.iter() {
            let Ok((role, mut node)) = parts.get_mut(child) else {
                continue;
            };
            match role {
                Role::MeterFill if node.width != fill => node.width = fill,
                // A flex item after the fill, pulled back over it by the fill's own length (a
                // percentage margin is of the container's width, as the width is): it starts at
                // the bar's left edge and the row is as long as the trail.
                Role::MeterTrail if node.width != trail || node.margin.left != back => {
                    node.width = trail;
                    node.margin.left = back;
                }
                _ => {}
            }
        }
    }
}
