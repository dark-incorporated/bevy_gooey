//! `led`: a glowing dot that says whether something is live (`docs/research/led.md`).
//!
//! The row's [`Led`] is the state. Each state has its own silhouette and its word beside the dot,
//! so colour is never the only signal (WCAG 2.2 SC 1.4.1), and the pulse of [`LedState::Busy`]
//! stops under reduced or no motion (SC 2.3.3). The dot's fill, frame, silhouette, glow and the
//! word are data written by [`update_leds`], each only on a difference.

use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy::ui::UiTransform;

use crate::elevation::ShadowLayer;
use crate::motion::MotionSettings;
use crate::skin::{GooeySkin, LED_TEXT_SIZES, Role, SkinScope, led_text_px, skin_of};

/// What an [`led()`](crate::led) says.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, Default, PartialEq, Hash, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LedState {
    /// Nothing is there: a hollow ring, no glow.
    #[default]
    Off,
    /// Live and well: a steady disc in [`GooeySkin::live`] with a glow.
    Live,
    /// Working on it: a rounded square in [`GooeySkin::warning`] whose glow pulses.
    Busy,
    /// It failed: a diamond in [`GooeySkin::danger`] with a steady glow.
    Failed,
}

impl LedState {
    /// The word an [`led()`](crate::led) shows beside its dot.
    pub fn word(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Live => "live",
            Self::Busy => "busy",
            Self::Failed => "failed",
        }
    }
}

/// The state of an [`led()`](crate::led), on its row: write it and the dot and the word follow in the next frame.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, Reflect)]
#[reflect(Component, Debug, Default, PartialEq, Clone)]
pub struct Led(pub LedState);

/// How big an [`led()`](crate::led) is drawn: a step of [`GooeySkin::type_scale`] (the unit of
/// [`IconSize`](crate::IconSize)), put on the LED's root. The label and the state word are drawn at
/// that step's text size and the dot is as tall as 1.1 of those (the same share a body-sized LED
/// has), so a quiet readout in a status strip is `LedSize(1)` and the whole LED follows the player's
/// text size. A step past the end of the scale is its last. Without the component the LED is the
/// body's size, as it always was; taking it off gives that back.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_gooey::*;
/// # fn ui(mut commands: Commands) {
/// commands.spawn((led("Agent", LedState::Live), LedSize(1)));
/// # }
/// ```
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LedSize(pub usize);

/// Marks the text that shows an [`led()`](crate::led)'s state word.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, Reflect)]
#[reflect(Component, Debug, Default, PartialEq, Clone)]
pub struct LedWord;

/// The glow's blur, in dot sizes (CSS's blur radius: how far the halo fades out past the dot's
/// edge). The whole halo, spread included, stays within one dot of the dot (#91).
const GLOW_BLUR_DOTS: f32 = 0.6;
/// How far the glow grows past the dot before it blurs, in dot sizes.
const GLOW_SPREAD_DOTS: f32 = 0.0;
/// The glow's alpha for a steady LED: a halo, not a lamp, so the word and the chips beside it keep
/// their contrast (#91).
const GLOW_ALPHA: f32 = 0.4;
/// The glow's alpha range while [`LedState::Busy`] pulses.
const PULSE_ALPHA: (f32, f32) = (0.12, 0.45);
/// The pulse's period, seconds: slower than one a second, far under WCAG's flash threshold.
const PULSE_SECONDS: f64 = 1.2;
/// A diamond is a square turned this far.
const DIAMOND_DEGREES: f32 = 45.0;
/// A diamond is drawn at this scale so its diagonal is as wide as the dot.
const DIAMOND_SCALE: f32 = 0.78;
/// Corner radius of each silhouette, as a share of the dot.
const RADIUS_DISC: f32 = 50.0;
const RADIUS_SQUARE: f32 = 30.0;
const RADIUS_DIAMOND: f32 = 12.0;

/// Writes every [`Led`]'s dot and word from its state, each only on a difference. Runs in
/// [`crate::GooeySystems::Style`].
#[allow(clippy::too_many_arguments)]
pub fn update_leds(
    time: Option<Res<Time<Real>>>,
    motion: Option<Res<MotionSettings>>,
    skin: Res<GooeySkin>,
    scopes: Query<&SkinScope>,
    parents: Query<&ChildOf>,
    leds: Query<(Entity, &Led, Option<&LedSize>, &Children)>,
    roles: Query<&Role>,
    mut dots: Query<(
        &mut Node,
        &mut BackgroundColor,
        &mut BorderColor,
        &mut BoxShadow,
        &mut UiTransform,
    )>,
    mut words: Query<&mut Text, With<LedWord>>,
) {
    let steady = motion.is_some_and(|m| m.lands_travel()) || time.is_none();
    let phase = time.map_or(0.0, |t| {
        t.elapsed_secs_f64() % PULSE_SECONDS / PULSE_SECONDS
    }) as f32;
    for (entity, led, size, children) in &leds {
        let skin = skin_of(&skin, entity, &scopes, &parents);
        let side = led_text_px(skin, size.map(|size| size.0)) * LED_TEXT_SIZES;
        let (fill, frame, radius, glow, turn) = match led.0 {
            LedState::Off => (Color::NONE, skin.outline, RADIUS_DISC, None, false),
            LedState::Live => {
                let c = skin.live();
                (c, c, RADIUS_DISC, Some((c, GLOW_ALPHA)), false)
            }
            LedState::Busy => {
                let c = skin.warning();
                let alpha = if steady {
                    (PULSE_ALPHA.0 + PULSE_ALPHA.1) * 0.5
                } else {
                    let wave = 0.5 - 0.5 * (TAU * phase).cos();
                    PULSE_ALPHA.0 + (PULSE_ALPHA.1 - PULSE_ALPHA.0) * wave
                };
                (c, c, RADIUS_SQUARE, Some((c, alpha)), false)
            }
            LedState::Failed => {
                let c = skin.danger;
                (c, c, RADIUS_DIAMOND, Some((c, GLOW_ALPHA)), true)
            }
        };
        let shadow = match glow {
            Some((colour, alpha)) => BoxShadow(vec![
                ShadowLayer {
                    blur: side * GLOW_BLUR_DOTS,
                    spread: side * GLOW_SPREAD_DOTS,
                    color: colour.with_alpha(alpha),
                    ..default()
                }
                .to_bevy(1.0),
            ]),
            None => BoxShadow(Vec::new()),
        };
        let transform = if turn {
            UiTransform {
                rotation: Rot2::degrees(DIAMOND_DEGREES),
                scale: Vec2::splat(DIAMOND_SCALE),
                ..UiTransform::IDENTITY
            }
        } else {
            UiTransform::IDENTITY
        };
        for child in children.iter() {
            match roles.get(child) {
                Ok(Role::LedDot) => {
                    if let Ok((mut node, mut bg, mut border, mut glow_box, mut at)) =
                        dots.get_mut(child)
                    {
                        let corners = BorderRadius::all(Val::Percent(radius));
                        if node.border_radius != corners {
                            node.border_radius = corners;
                        }
                        bg.set_if_neq(BackgroundColor(fill));
                        border.set_if_neq(BorderColor::all(frame));
                        glow_box.set_if_neq(shadow.clone());
                        at.set_if_neq(transform);
                    }
                }
                _ => {
                    if let Ok(mut text) = words.get_mut(child)
                        && text.0 != led.0.word()
                    {
                        text.0 = led.0.word().into();
                    }
                }
            }
        }
    }
}
