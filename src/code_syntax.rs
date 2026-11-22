//! The skin's colours for code: one for each [`SyntaxRole`], and one for each [`CodeSeverity`]. They
//! are **derived from the skin's own seeds**, like [`GooeySkin::warning`] and [`GooeySkin::live`],
//! so a preset, a theme step and a host's own skin all get a set that belongs to it and nothing is
//! added to the skin's data.
//!
//! A role's colour is its hue at the accent's chroma and a lightness a little toward the ground
//! from the text's, moved back toward the text's until it holds the contrast bar on the editor's
//! own ground ([`GooeySkin::solid_surface`]): 7:1 when the skin's text already holds it (the high
//! contrast preset, and any preset that clears it), else 4.5:1. [`GooeySkin::problems`] checks
//! every one, so a skin that cannot give a set is reported.
//!
//! A host picks the hues with a [`SyntaxTheme`] ([`GooeySkin::syntax_theme`], #116): the search for
//! the contrast bar still runs, so a theme chooses hue, chroma and where the search starts, never
//! whether the bar is met.

use bevy::color::{Oklcha, Srgba};
use bevy::prelude::*;

use crate::code_decor::{CodeSeverity, SyntaxRole};
use crate::model::{ENHANCED_CONTRAST, TEXT_CONTRAST, contrast_ratio};
use crate::skin::GooeySkin;

/// How far toward the ground a role's lightness starts from the text's, of the gap.
const SYNTAX_L_SHIFT: f32 = 0.18;
/// How much of the way back to the text's lightness each step takes when a colour is too close to the ground.
const SYNTAX_L_BACK: f32 = 0.25;
/// The least and most chroma a role's colour takes.
const SYNTAX_CHROMA: (f32, f32) = (0.08, 0.16);
/// How much chroma is kept at each step of pulling a colour into sRGB.
const SYNTAX_GAMUT_PULL: f32 = 0.92;

/// How many times a colour is moved back toward the text's lightness looking for the contrast bar.
const SYNTAX_LIGHTNESS_STEPS: usize = 24;
/// How many times a colour's chroma is pulled in looking for sRGB.
const SYNTAX_GAMUT_STEPS: usize = 24;

/// The most chroma a theme may ask for: Oklch's sRGB gamut tops out a little above it for most hues.
const SYNTAX_CHROMA_MAX: f32 = 0.2;

/// What a [`SyntaxTheme`] says about one [`SyntaxRole`]: the colour is the hue at the chroma, a
/// lightness found from `shift`, then moved toward the text's until it holds the contrast bar.
///
/// [`SyntaxRole::Comment`], [`SyntaxRole::Operator`] and [`SyntaxRole::Error`] take the skin's muted
/// text, text and danger colours and are not themable ([`SkinProblem::BadSize`](crate::SkinProblem::BadSize)
/// with `syntax_theme` says a theme named one).
#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
#[reflect(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(default)
)]
pub struct SyntaxInk {
    /// The role this ink colours.
    pub role: SyntaxRole,
    /// Degrees of Oklch hue.
    pub hue: f32,
    /// Oklch chroma, 0 to 0.2; `None` takes the skin's accent's, held between 0.08 and 0.16.
    pub chroma: Option<f32>,
    /// Where the search for the contrast bar starts, as a share (0 to 1) of the gap from the text's
    /// lightness to the ground's; `None` is 0.18. A larger share starts darker (on a dark skin),
    /// and the search only moves toward the text, so it can never get a colour under the bar.
    pub shift: Option<f32>,
}

impl Default for SyntaxInk {
    fn default() -> Self {
        Self {
            role: SyntaxRole::Key,
            hue: 0.0,
            chroma: None,
            shift: None,
        }
    }
}

impl SyntaxInk {
    /// An ink of `role` at `hue` degrees, the skin's chroma and the usual start.
    pub fn new(role: SyntaxRole, hue: f32) -> Self {
        Self {
            role,
            hue,
            chroma: None,
            shift: None,
        }
    }

    /// The same ink at `chroma`.
    pub fn chroma(mut self, chroma: f32) -> Self {
        self.chroma = Some(chroma);
        self
    }

    /// The same ink starting its lightness search at `shift` of the gap.
    pub fn shift(mut self, shift: f32) -> Self {
        self.shift = Some(shift);
        self
    }

    /// Whether this ink can be used: a themable role, a finite hue, a chroma in 0 to 0.2 and a
    /// shift in 0 to 1.
    fn usable(&self) -> bool {
        themable(self.role)
            && self.hue.is_finite()
            && self
                .chroma
                .is_none_or(|c| (0.0..=SYNTAX_CHROMA_MAX).contains(&c))
            && self.shift.is_none_or(|s| (0.0..=1.0).contains(&s))
    }
}

/// Whether a role takes a hue from a theme.
fn themable(role: SyntaxRole) -> bool {
    !matches!(
        role,
        SyntaxRole::Comment | SyntaxRole::Operator | SyntaxRole::Error
    )
}

/// The hues the code editor's colours come from: a list of [`SyntaxInk`]s, the later one winning
/// for a role named twice. A role the theme does not name keeps the skin's own hue, so the empty
/// theme ([`SyntaxTheme::skin`], the default) draws exactly what a skin without a theme always drew.
///
/// It is a field of the skin ([`GooeySkin::syntax_theme`]): a host's settings screen assigns one,
/// every open code editor recolours in that frame, [`GooeySkin::problems`] holds it to the contrast
/// bar on the skin it is on, and a [`SkinScope`](crate::SkinScope) can give one pane its own.
///
/// ```
/// use bevy_gooey::{GooeySkin, SyntaxRole, SyntaxTheme};
/// let mut skin = GooeySkin::dark();
/// let before = skin.syntax(SyntaxRole::Key);
/// skin.syntax_theme = SyntaxTheme::warm();
/// assert_ne!(skin.syntax(SyntaxRole::Key), before);
/// assert!(skin.problems().is_empty());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Reflect)]
#[reflect(Default, Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(default)
)]
pub struct SyntaxTheme {
    /// The inks, in any order.
    pub inks: Vec<SyntaxInk>,
}

/// A theme of the nine themable roles, in the order of [`THEMED`].
fn theme_of(inks: [(f32, Option<f32>, Option<f32>); 9]) -> SyntaxTheme {
    SyntaxTheme {
        inks: THEMED
            .into_iter()
            .zip(inks)
            .map(|(role, (hue, chroma, shift))| SyntaxInk {
                role,
                hue,
                chroma,
                shift,
            })
            .collect(),
    }
}

/// The roles a theme colours.
const THEMED: [SyntaxRole; 9] = [
    SyntaxRole::Key,
    SyntaxRole::String,
    SyntaxRole::Number,
    SyntaxRole::Keyword,
    SyntaxRole::Reference,
    SyntaxRole::Function,
    SyntaxRole::Type,
    SyntaxRole::Emphasis,
    SyntaxRole::Strong,
];

impl SyntaxTheme {
    /// The skin's own hues: nothing listed.
    pub fn skin() -> Self {
        Self::default()
    }

    /// Amber, orange, sand and olive: the hues of a warm terminal.
    pub fn warm() -> Self {
        theme_of([
            (70.0, None, Some(0.0)),
            (130.0, None, None),
            (42.0, None, Some(0.10)),
            (15.0, None, Some(0.26)),
            (60.0, None, Some(0.26)),
            (102.0, None, Some(0.34)),
            (158.0, None, Some(0.26)),
            (300.0, None, Some(0.30)),
            (345.0, None, Some(0.05)),
        ])
    }

    /// Blues, teals and violets: the hues of a night sky.
    pub fn cool() -> Self {
        theme_of([
            (250.0, None, None),
            (175.0, None, Some(0.10)),
            (215.0, None, Some(0.26)),
            (290.0, None, None),
            (235.0, None, Some(0.26)),
            (195.0, None, Some(0.10)),
            (155.0, None, Some(0.26)),
            (310.0, None, Some(0.10)),
            (270.0, None, Some(0.26)),
        ])
    }

    /// One hue, told apart by lightness and a little chroma: quiet reading, and for readers who
    /// cannot tell hues apart. The roles are no further apart than the contrast bar leaves room for,
    /// which is less than the hued themes give: the one theme with no separation promise.
    pub fn mono() -> Self {
        let ink = |role, shift| SyntaxInk::new(role, 250.0).chroma(0.11).shift(shift);
        SyntaxTheme {
            inks: vec![
                ink(SyntaxRole::Key, 0.0),
                ink(SyntaxRole::String, 0.22),
                ink(SyntaxRole::Number, 0.38),
                ink(SyntaxRole::Keyword, 0.05),
                ink(SyntaxRole::Reference, 0.30),
                ink(SyntaxRole::Function, 0.14),
                ink(SyntaxRole::Type, 0.46),
                ink(SyntaxRole::Emphasis, 0.34),
                ink(SyntaxRole::Strong, 0.0),
            ],
        }
    }

    /// The widest hue spacing the nine roles allow, at full chroma, with neighbours also split by
    /// lightness: for a coloured pane meant to be read at a glance.
    pub fn separated() -> Self {
        theme_of([
            (255.0, Some(0.16), Some(0.14)),
            (140.0, Some(0.16), Some(0.28)),
            (50.0, Some(0.16), Some(0.14)),
            (330.0, Some(0.16), Some(0.28)),
            (195.0, Some(0.16), Some(0.14)),
            (100.0, Some(0.16), Some(0.28)),
            (285.0, Some(0.16), Some(0.28)),
            (165.0, Some(0.16), Some(0.05)),
            (355.0, Some(0.16), Some(0.05)),
        ])
    }

    /// The ink this theme gives `role`, if it names one that can be used.
    fn ink(&self, role: SyntaxRole) -> Option<&SyntaxInk> {
        self.inks
            .iter()
            .rev()
            .find(|ink| ink.role == role)
            .filter(|ink| ink.usable())
    }

    /// Whether every ink is usable; [`GooeySkin::problems`] reports a theme that is not.
    pub(crate) fn is_usable(&self) -> bool {
        self.inks.iter().all(SyntaxInk::usable)
    }
}

/// The themes the crate ships, in the order a settings row lists them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash, Clone)]
pub enum SyntaxPreset {
    /// [`SyntaxTheme::skin`]: the skin's own hues.
    Skin,
    /// [`SyntaxTheme::warm`].
    Warm,
    /// [`SyntaxTheme::cool`].
    Cool,
    /// [`SyntaxTheme::mono`].
    Mono,
    /// [`SyntaxTheme::separated`].
    Separated,
}

impl SyntaxPreset {
    /// Every preset, in the order a settings row lists them.
    pub const ALL: [SyntaxPreset; 5] = [
        Self::Skin,
        Self::Warm,
        Self::Cool,
        Self::Mono,
        Self::Separated,
    ];

    /// The name a settings row shows.
    pub fn name(self) -> &'static str {
        match self {
            Self::Skin => "Skin",
            Self::Warm => "Warm",
            Self::Cool => "Cool",
            Self::Mono => "Mono",
            Self::Separated => "Separated",
        }
    }

    /// The theme this preset names.
    pub fn theme(self) -> SyntaxTheme {
        match self {
            Self::Skin => SyntaxTheme::skin(),
            Self::Warm => SyntaxTheme::warm(),
            Self::Cool => SyntaxTheme::cool(),
            Self::Mono => SyntaxTheme::mono(),
            Self::Separated => SyntaxTheme::separated(),
        }
    }
}

/// A role's hue, degrees of Oklch.
fn hue(role: SyntaxRole) -> f32 {
    match role {
        SyntaxRole::Key => 250.0,
        SyntaxRole::String => 145.0,
        SyntaxRole::Number => 55.0,
        SyntaxRole::Keyword => 320.0,
        SyntaxRole::Reference => 205.0,
        SyntaxRole::Function => 95.0,
        SyntaxRole::Type => 175.0,
        SyntaxRole::Emphasis => 290.0,
        SyntaxRole::Strong => 350.0,
        // Roles that take a colour the skin already has, or the danger hue, never ask.
        SyntaxRole::Comment | SyntaxRole::Operator | SyntaxRole::Error => 0.0,
    }
}

fn rgb(color: Color) -> [f32; 3] {
    let c = Srgba::from(color);
    [c.red, c.green, c.blue]
}

fn in_gamut(color: Color) -> bool {
    let c = Srgba::from(color);
    [c.red, c.green, c.blue]
        .iter()
        .all(|v| (0.0..=1.0).contains(v))
}

impl GooeySkin {
    /// The colour a span of `role` is drawn in, on the code editor's own ground. Derived from the
    /// skin's seeds (see the module docs); [`GooeySkin::problems`] holds every one at the text
    /// contrast bar.
    ///
    /// ```
    /// use bevy_gooey::{GooeySkin, SyntaxRole};
    /// let skin = GooeySkin::dark();
    /// assert_ne!(skin.syntax(SyntaxRole::Key), skin.syntax(SyntaxRole::String));
    /// assert_eq!(skin.syntax(SyntaxRole::Comment), skin.text_muted);
    /// assert!(skin.problems().is_empty());
    /// ```
    pub fn syntax(&self, role: SyntaxRole) -> Color {
        let ink = self.syntax_theme.ink(role);
        let (hue, base_chroma) = match role {
            SyntaxRole::Comment => return self.text_muted,
            SyntaxRole::Operator => return self.text,
            // Wrong is the danger colour's own hue, held to the same bar as the rest.
            SyntaxRole::Error => {
                let danger = Oklcha::from(self.danger);
                (danger.hue, danger.chroma)
            }
            other => (
                ink.map_or_else(|| hue(other), |ink| ink.hue),
                Oklcha::from(self.accent).chroma,
            ),
        };
        let ground = self.solid_surface();
        let ground_rgb = rgb(ground);
        let needed = if contrast_ratio(rgb(self.text), ground_rgb) >= ENHANCED_CONTRAST {
            ENHANCED_CONTRAST
        } else {
            TEXT_CONTRAST
        };
        let text = Oklcha::from(self.text);
        let ground_l = Oklcha::from(ground).lightness;
        let chroma = match ink.and_then(|ink| ink.chroma) {
            Some(asked) => asked,
            None => base_chroma.clamp(SYNTAX_CHROMA.0, SYNTAX_CHROMA.1),
        };
        let shift = ink.and_then(|ink| ink.shift).unwrap_or(SYNTAX_L_SHIFT);
        let mut lightness = text.lightness + (ground_l - text.lightness) * shift;
        for _ in 0..SYNTAX_LIGHTNESS_STEPS {
            let mut lch = Oklcha::new(lightness, chroma, hue, 1.0);
            for _ in 0..SYNTAX_GAMUT_STEPS {
                if in_gamut(Color::from(lch)) {
                    break;
                }
                lch.chroma *= SYNTAX_GAMUT_PULL;
            }
            let color = Color::from(lch);
            if contrast_ratio(rgb(color), ground_rgb) >= needed {
                return color;
            }
            lightness += (text.lightness - lightness) * SYNTAX_L_BACK;
        }
        // No hue at this chroma clears the bar before it reaches the text's own lightness: the
        // text's colour does, because the skin's own check says so.
        self.text
    }

    /// The colour of a [`CodeSeverity`]: the danger colour, the caution colour, the accent, and the
    /// muted text.
    pub fn severity(&self, severity: CodeSeverity) -> Color {
        match severity {
            CodeSeverity::Error => self.danger,
            CodeSeverity::Warning => self.warning(),
            CodeSeverity::Info => self.accent,
            CodeSeverity::Hint => self.text_muted,
        }
    }

    /// The pairs [`GooeySkin::problems`] checks for the code editor: every syntax colour on the
    /// editor's ground.
    pub(crate) fn syntax_pairs(&self) -> Vec<(&'static str, [f32; 3], [f32; 3])> {
        const LABELS: [&str; 12] = [
            "syntax key on surface",
            "syntax string on surface",
            "syntax number on surface",
            "syntax keyword on surface",
            "syntax comment on surface",
            "syntax reference on surface",
            "syntax function on surface",
            "syntax type on surface",
            "syntax operator on surface",
            "syntax emphasis on surface",
            "syntax strong on surface",
            "syntax error on surface",
        ];
        let ground = rgb(self.solid_surface());
        SyntaxRole::ALL
            .into_iter()
            .zip(LABELS)
            .map(|(role, label)| (label, rgb(self.syntax(role)), ground))
            .collect()
    }
}
