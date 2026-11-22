//! A style sheet: class names on nodes ([`StyleClass`]), rules that pick nodes by those names and
//! their state ([`StyleRule`], [`Selector`]), and the cascade ([`apply_style_sheet`]) that writes a
//! classed node's box and colours from the rules that match it. Guide: `docs/guide/style-sheets.md`.
//!
//! The sheet is plain data the host owns: it reads its files (any serde format), builds a
//! [`StyleSheet`] and replaces the resource whole; the cascade restyles every classed node that
//! frame. It writes the components a host would otherwise spawn by hand ([`Spacing`], [`Sizing`],
//! [`Shape`], [`GapPx`], [`InkOverride`] and Bevy's `GlobalZIndex`) and nothing else, so
//! [`crate::apply_skin`] draws a styled node exactly as it draws one spawned with the same values.
//!
//! What a node was spawn with is its **base** ([`CascadeBase`], taken when its class is first
//! inserted). The written value is always the base with the matching declarations laid over it, so
//! deleting a rule gives the node back what its spawn code set.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::str::FromStr;

use bevy::ecs::entity::EntityHashSet;
use bevy::ecs::system::SystemParam;
use bevy::input_focus::InputFocus;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui::{Checked, InteractionDisabled, Pressed, Selected};

use crate::box_model::{
    Extent, GapPx, Ink, InkOverride, Radius, Shape, Sides, Sizing, Space, Spacing,
};
use crate::skin::{GooeySkin, Role};

/// The class names of a node, for a [`StyleSheet`]'s rules to pick it by: lower-case kebab names
/// (`chrome-chip`, `row-quiet`), matching `^[a-z][a-z0-9-]*$`.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_gooey::*;
/// # fn ui(mut commands: Commands) {
/// commands.spawn((panel(), StyleClass::of("card card-quiet")));
/// # }
/// assert_eq!(StyleClass::of("card  card-quiet").0, ["card", "card-quiet"]);
/// ```
#[derive(Component, Clone, Debug, Default, PartialEq, Eq, Reflect)]
#[reflect(Component, Debug, Default, PartialEq, Clone)]
pub struct StyleClass(pub Vec<String>);

impl StyleClass {
    /// The classes named in `classes`, split on whitespace. A name that is not lower-case kebab is
    /// dropped with a warning (no selector could name it); a repeated name is kept once.
    pub fn of(classes: &str) -> Self {
        let mut names: Vec<String> = Vec::new();
        for name in classes.split_whitespace() {
            if !is_class_name(name) {
                warn!(
                    "bevy_gooey: style class `{name}` dropped: a class is lower-case letters, digits and `-`, starting with a letter"
                );
                continue;
            }
            if !names.iter().any(|kept| kept == name) {
                names.push(name.to_owned());
            }
        }
        Self(names)
    }

    /// Whether the node has the class `name`.
    pub fn has(&self, name: &str) -> bool {
        self.0.iter().any(|class| class == name)
    }
}

/// Whether `name` is a class name a selector can name: `^[a-z][a-z0-9-]*$`.
pub fn is_class_name(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('a'..='z'))
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// One of the skin's own colours, for a [`Paint::Skin`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SkinColour {
    /// [`GooeySkin::background`].
    Background,
    /// [`GooeySkin::surface`].
    Surface,
    /// [`GooeySkin::surface_raised`].
    SurfaceRaised,
    /// [`GooeySkin::text`].
    Text,
    /// [`GooeySkin::text_muted`].
    TextMuted,
    /// [`GooeySkin::accent`].
    Accent,
    /// [`GooeySkin::accent_text`].
    AccentText,
    /// [`GooeySkin::danger`].
    Danger,
    /// [`GooeySkin::focus_ring`].
    FocusRing,
    /// [`GooeySkin::outline`].
    Outline,
    /// [`GooeySkin::scrim`].
    Scrim,
}

impl SkinColour {
    /// The colour under `skin`.
    pub fn of(self, skin: &GooeySkin) -> Color {
        match self {
            SkinColour::Background => skin.background,
            SkinColour::Surface => skin.surface,
            SkinColour::SurfaceRaised => skin.surface_raised,
            SkinColour::Text => skin.text,
            SkinColour::TextMuted => skin.text_muted,
            SkinColour::Accent => skin.accent,
            SkinColour::AccentText => skin.accent_text,
            SkinColour::Danger => skin.danger,
            SkinColour::FocusRing => skin.focus_ring,
            SkinColour::Outline => skin.outline,
            SkinColour::Scrim => skin.scrim,
        }
    }
}

/// A colour a rule paints with, resolved against the skin and the host's [`HostPaints`] every time
/// either changes, so a theme step repaints every styled node. In YAML: `!Skin Text`, `!Surface 3`,
/// `!Tone warning`, `!Host panel_bg`, `!Color "#ff8000"` (sRGB hex, `#rgb`, `#rgba`, `#rrggbb` or
/// `#rrggbbaa`, as CSS writes it).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Paint {
    /// One of the skin's own colours.
    Skin(SkinColour),
    /// A step of the skin's surface ladder ([`GooeySkin::surfaces`]); a step past the end is a
    /// problem, not the last step.
    Surface(u8),
    /// One of the skin's named tones ([`GooeySkin::tones`]).
    Tone(String),
    /// A colour the host named in [`HostPaints`].
    Host(String),
    /// A literal colour, the same under every skin; written as sRGB hex.
    Color(#[cfg_attr(feature = "serde", serde(with = "hex_colour"))] Color),
}

/// A literal [`Paint::Color`] as CSS writes one, `"#ff8000"`: a format's own nested enum (Bevy's
/// `Color` is one) is not something every serde format reads inside a tag, and a hex string is
/// what a person types.
#[cfg(feature = "serde")]
mod hex_colour {
    use bevy::color::{Color, Srgba};

    pub fn serialize<S: serde::Serializer>(
        colour: &Color,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&colour.to_srgba().to_hex())
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Color, D::Error> {
        use serde::de::Error;
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        Srgba::hex(&text).map(Color::Srgba).map_err(|refused| {
            D::Error::custom(format_args!(
                "`{text}` is not an sRGB hex colour (#rgb, #rgba, #rrggbb or #rrggbbaa): {refused}"
            ))
        })
    }
}

impl Paint {
    /// The colour under `skin` and the host's paints, or why there is none.
    pub fn resolve(&self, skin: &GooeySkin, host: Option<&HostPaints>) -> Result<Color, String> {
        match self {
            Paint::Skin(colour) => Ok(colour.of(skin)),
            Paint::Surface(level) => {
                skin.surfaces
                    .get(usize::from(*level))
                    .copied()
                    .ok_or_else(|| {
                        format!(
                            "surface {level} is past the ladder ({} steps)",
                            skin.surfaces.len()
                        )
                    })
            }
            Paint::Tone(name) => skin.tone(name).ok_or_else(|| format!("no tone `{name}`")),
            Paint::Host(name) => host
                .and_then(|paints| paints.0.get(name))
                .copied()
                .ok_or_else(|| format!("no host paint `{name}`")),
            Paint::Color(colour) => Ok(*colour),
        }
    }
}

/// Colours the host names for its sheet to paint with (`!Host panel_bg`): a host whose chrome has
/// a palette of its own derived from the skin writes it here whenever the skin changes, before
/// [`crate::GooeySystems::Cascade`], and the cascade repaints in the same frame.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct HostPaints(pub BTreeMap<String, Color>);

/// What one rule sets. Every field is an `Option`: a field left `None` is not this rule's, so a
/// rule with only a `gap` changes only the gap. A side of `padding`, `margin` or `border` is its own
/// property, as in CSS: `padding: {top: Zero}` in a later rule leaves the other three sides of an
/// earlier one.
///
/// Text size is not here: it stays a role of the host's type scale.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(default, deny_unknown_fields)
)]
pub struct StyleDecl {
    /// Room inside the box, side by side ([`Spacing::padding`]).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub padding: Option<Sides<Space>>,
    /// Room outside the box, side by side ([`Spacing::margin`]).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub margin: Option<Sides<Space>>,
    /// The gap between the box's children ([`GapPx`], resolved under the skin).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub gap: Option<Space>,
    /// [`Sizing::width`].
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub width: Option<Extent>,
    /// [`Sizing::height`].
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub height: Option<Extent>,
    /// [`Sizing::min_width`].
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub min_width: Option<Extent>,
    /// [`Sizing::min_height`].
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub min_height: Option<Extent>,
    /// [`Sizing::max_width`].
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub max_width: Option<Extent>,
    /// [`Sizing::max_height`].
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub max_height: Option<Extent>,
    /// [`Sizing::clip`].
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub clip: Option<bool>,
    /// The border's weight per side, in the skin's line weights ([`Shape::border`]).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub border: Option<Sides<f32>>,
    /// Which skin colour the border is drawn in ([`Shape::ink`]).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub border_ink: Option<Ink>,
    /// The corner radius ([`Shape::radius`]).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub radius: Option<Radius>,
    /// The fill, a level of the skin's surface ladder ([`Shape::fill`]).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fill: Option<u8>,
    /// The background colour ([`InkOverride::fill`]).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub background: Option<Paint>,
    /// The text colour of a text node ([`InkOverride::ink`]).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub color: Option<Paint>,
    /// The border colour ([`InkOverride::edge`]).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub border_color: Option<Paint>,
    /// The node's global stacking order (Bevy's `GlobalZIndex`).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub z: Option<i32>,
}

impl StyleDecl {
    /// This declaration with every property `over` sets laid on top (a box side by side).
    pub fn overlaid(self, over: &StyleDecl) -> StyleDecl {
        fn sides<T: Copy>(under: Option<Sides<T>>, over: Option<Sides<T>>) -> Option<Sides<T>> {
            match (under, over) {
                (Some(under), Some(over)) => Some(under.overlaid(&over)),
                (under, over) => over.or(under),
            }
        }
        StyleDecl {
            padding: sides(self.padding, over.padding),
            margin: sides(self.margin, over.margin),
            gap: over.gap.or(self.gap),
            width: over.width.or(self.width),
            height: over.height.or(self.height),
            min_width: over.min_width.or(self.min_width),
            min_height: over.min_height.or(self.min_height),
            max_width: over.max_width.or(self.max_width),
            max_height: over.max_height.or(self.max_height),
            clip: over.clip.or(self.clip),
            border: sides(self.border, over.border),
            border_ink: over.border_ink.or(self.border_ink),
            radius: over.radius.or(self.radius),
            fill: over.fill.or(self.fill),
            background: over.background.clone().or(self.background),
            color: over.color.clone().or(self.color),
            border_color: over.border_color.clone().or(self.border_color),
            z: over.z.or(self.z),
        }
    }

    /// Each colour this declaration paints with, by its field's name.
    fn paints(&self) -> [(&'static str, Option<&Paint>); 3] {
        [
            ("background", self.background.as_ref()),
            ("color", self.color.as_ref()),
            ("border_color", self.border_color.as_ref()),
        ]
    }
}

/// A state a [`Selector`] can ask of a node (`:hover`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StyleState {
    /// `:hover`: Bevy's `Hovered` reads true (only nodes that carry `Hovered`, the interactive ones).
    Hover,
    /// `:pressed`: the node has Bevy's `Pressed`.
    Pressed,
    /// `:selected`: the node has Bevy's `Selected`.
    Selected,
    /// `:checked`: the node has Bevy's `Checked`.
    Checked,
    /// `:disabled`: the node has Bevy's `InteractionDisabled`.
    Disabled,
    /// `:focus`: the node is `InputFocus`'s entity.
    Focus,
}

impl StyleState {
    /// Every state, in the order an error lists them.
    pub const ALL: [StyleState; 6] = [
        StyleState::Hover,
        StyleState::Pressed,
        StyleState::Selected,
        StyleState::Checked,
        StyleState::Disabled,
        StyleState::Focus,
    ];

    /// The name a selector writes after `:`.
    pub fn name(self) -> &'static str {
        match self {
            StyleState::Hover => "hover",
            StyleState::Pressed => "pressed",
            StyleState::Selected => "selected",
            StyleState::Checked => "checked",
            StyleState::Disabled => "disabled",
            StyleState::Focus => "focus",
        }
    }
}

/// How two compounds of a selector are related.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Combinator {
    /// `a b`: any ancestor.
    Descendant,
    /// `a > b`: the parent.
    Child,
}

/// `.a.b:hover`: classes and states one node must all have.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Compound {
    classes: Vec<String>,
    states: Vec<StyleState>,
}

/// One selector of a comma list: compounds joined by combinators, the last one the subject.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Complex {
    compounds: Vec<Compound>,
    /// `combinators[i]` joins `compounds[i]` and `compounds[i + 1]`.
    combinators: Vec<Combinator>,
    specificity: u32,
}

/// Why a selector does not read: where, counted in characters from 1, and what was wrong there.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectorError {
    /// The column of the offending character, from 1 (one past the end for a selector cut short).
    pub column: usize,
    /// What is wrong there.
    pub reason: String,
}

impl fmt::Display for SelectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "at column {}: {}", self.column, self.reason)
    }
}

impl std::error::Error for SelectorError {}

/// Which nodes a [`StyleRule`] styles: `.card`, `.list .row:hover`, `.bar > .chip, .chip:focus`.
///
/// The grammar is `list := complex ("," complex)*`, `complex := compound ((" " | ">") compound)*`,
/// `compound := ("." class | ":" state)+`. The last compound of each complex selector names at least
/// one class. States are `hover`, `pressed`, `selected`, `checked`, `disabled` and `focus`
/// ([`StyleState`]). A complex selector's specificity is its classes plus its states; a list matches
/// with the highest of the alternatives that match.
///
/// ```
/// # use bevy_gooey::*;
/// let selector: Selector = ".list .row:hover".parse()?;
/// assert_eq!(selector.specificity(), 3);
/// let refused = ".row:wobble".parse::<Selector>().unwrap_err();
/// assert_eq!(refused.column, 6);
/// # Ok::<(), SelectorError>(())
/// ```
#[derive(Clone, Debug)]
pub struct Selector {
    text: String,
    complexes: Vec<Complex>,
}

impl PartialEq for Selector {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}

impl FromStr for Selector {
    type Err = SelectorError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Selector::parse(text)
    }
}

impl Selector {
    /// Reads `text`, or says where and why it does not read.
    pub fn parse(text: &str) -> Result<Self, SelectorError> {
        let complexes = SelectorParser {
            chars: text.chars().collect(),
            at: 0,
        }
        .list()?;
        Ok(Self {
            text: text.trim().to_owned(),
            complexes,
        })
    }

    /// The selector as written (trimmed).
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The highest specificity of its alternatives.
    pub fn specificity(&self) -> u32 {
        self.complexes
            .iter()
            .map(|complex| complex.specificity)
            .max()
            .unwrap_or(0)
    }

    /// Whether any alternative looks past its subject (a descendant or child combinator).
    fn looks_up(&self) -> bool {
        self.complexes
            .iter()
            .any(|complex| complex.compounds.len() > 1)
    }

    /// Whether a compound before the subject asks a state, so an ancestor's state change restyles
    /// its descendants.
    fn asks_an_ancestor_state(&self) -> bool {
        self.complexes.iter().any(|complex| {
            complex.compounds[..complex.compounds.len() - 1]
                .iter()
                .any(|compound| !compound.states.is_empty())
        })
    }

    /// The specificity this selector matches `entity` with, or `None` when it does not.
    fn matches(&self, tree: &impl CascadeTree, entity: Entity) -> Option<u32> {
        self.complexes
            .iter()
            .filter(|complex| complex_matches(complex, complex.compounds.len() - 1, tree, entity))
            .map(|complex| complex.specificity)
            .max()
    }
}

/// Whether `complex`'s compounds up to `index` match `entity` and its ancestors.
fn complex_matches(
    complex: &Complex,
    index: usize,
    tree: &impl CascadeTree,
    entity: Entity,
) -> bool {
    if !compound_matches(&complex.compounds[index], tree, entity) {
        return false;
    }
    if index == 0 {
        return true;
    }
    match complex.combinators[index - 1] {
        Combinator::Child => tree
            .parent(entity)
            .is_some_and(|parent| complex_matches(complex, index - 1, tree, parent)),
        Combinator::Descendant => {
            let mut ancestor = tree.parent(entity);
            while let Some(up) = ancestor {
                if complex_matches(complex, index - 1, tree, up) {
                    return true;
                }
                ancestor = tree.parent(up);
            }
            false
        }
    }
}

fn compound_matches(compound: &Compound, tree: &impl CascadeTree, entity: Entity) -> bool {
    let classes_hold = compound.classes.is_empty()
        || tree
            .classes(entity)
            .is_some_and(|classes| compound.classes.iter().all(|name| classes.has(name)));
    classes_hold
        && compound
            .states
            .iter()
            .all(|state| tree.in_state(entity, *state))
}

/// A hand-written reader of the selector grammar: no dependency, and an error names its column.
struct SelectorParser {
    chars: Vec<char>,
    at: usize,
}

impl SelectorParser {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.at).copied()
    }

    fn fail<T>(&self, at: usize, reason: impl Into<String>) -> Result<T, SelectorError> {
        Err(SelectorError {
            column: at + 1,
            reason: reason.into(),
        })
    }

    /// Skips spaces and tabs; whether any were skipped.
    fn blank(&mut self) -> bool {
        let start = self.at;
        while self.peek().is_some_and(char::is_whitespace) {
            self.at += 1;
        }
        self.at > start
    }

    fn list(&mut self) -> Result<Vec<Complex>, SelectorError> {
        let mut complexes = Vec::new();
        loop {
            self.blank();
            complexes.push(self.complex()?);
            match self.peek() {
                None => return Ok(complexes),
                Some(',') => self.at += 1,
                Some(other) => return self.fail(self.at, format!("unexpected `{other}`")),
            }
        }
    }

    fn complex(&mut self) -> Result<Complex, SelectorError> {
        let mut compounds = vec![self.compound()?];
        let mut combinators = Vec::new();
        loop {
            let spaced = self.blank();
            match self.peek() {
                None | Some(',') => break,
                Some('>') => {
                    self.at += 1;
                    self.blank();
                    combinators.push(Combinator::Child);
                }
                Some('.' | ':') if spaced => combinators.push(Combinator::Descendant),
                Some(other) => return self.fail(self.at, format!("unexpected `{other}`")),
            }
            compounds.push(self.compound()?);
        }
        let subject = compounds.last().map(|compound| compound.classes.is_empty());
        if subject == Some(true) {
            return self.fail(
                self.at,
                "the last part of a selector names a class (`.row:hover`, not `:hover`)",
            );
        }
        let specificity = compounds
            .iter()
            .map(|compound| compound.classes.len() + compound.states.len())
            .sum::<usize>();
        Ok(Complex {
            compounds,
            combinators,
            specificity: u32::try_from(specificity).unwrap_or(u32::MAX),
        })
    }

    fn compound(&mut self) -> Result<Compound, SelectorError> {
        let mut compound = Compound {
            classes: Vec::new(),
            states: Vec::new(),
        };
        loop {
            match self.peek() {
                Some('.') => {
                    self.at += 1;
                    let start = self.at;
                    match self.peek() {
                        Some('a'..='z') => {}
                        Some(other) => {
                            return self.fail(
                                start,
                                format!("a class starts with a lower-case letter, not `{other}`"),
                            );
                        }
                        None => return self.fail(start, "a class name is missing after `.`"),
                    }
                    while self
                        .peek()
                        .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
                    {
                        self.at += 1;
                    }
                    if let Some(bad) = self.peek().filter(|c| c.is_alphanumeric() || *c == '_') {
                        return self.fail(
                            self.at,
                            format!("a class is lower-case letters, digits and `-`, not `{bad}`"),
                        );
                    }
                    compound
                        .classes
                        .push(self.chars[start..self.at].iter().collect());
                }
                Some(':') => {
                    self.at += 1;
                    let start = self.at;
                    while self
                        .peek()
                        .is_some_and(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
                    {
                        self.at += 1;
                    }
                    let name: String = self.chars[start..self.at].iter().collect();
                    let Some(state) = StyleState::ALL
                        .into_iter()
                        .find(|state| state.name() == name)
                    else {
                        let known = StyleState::ALL.map(StyleState::name).join(", ");
                        return self.fail(
                            start,
                            format!("unknown state `{name}` (the states are {known})"),
                        );
                    };
                    compound.states.push(state);
                }
                _ => break,
            }
        }
        if compound.classes.is_empty() && compound.states.is_empty() {
            return match self.peek() {
                Some(other) => self.fail(
                    self.at,
                    format!("expected `.class` or `:state`, not `{other}`"),
                ),
                None => self.fail(self.at, "expected `.class` or `:state`, the selector ends"),
            };
        }
        Ok(compound)
    }
}

/// One rule: which nodes, what they get, and where the rule came from.
#[derive(Clone, Debug, PartialEq)]
pub struct StyleRule {
    /// The nodes it styles.
    pub selector: Selector,
    /// Where it was read from, for an inspector (the host sets it after reading; empty from serde).
    pub source: String,
    /// What it sets.
    pub decl: StyleDecl,
}

/// A sheet's rules **in document order**, which breaks a tie of specificity: the later wins. With
/// serde it is a map of selector to declaration, read in the order written; a selector that does
/// not read is refused with its column.
///
/// ```
/// # #[cfg(feature = "serde")] {
/// # use bevy_gooey::*;
/// let rules: StyleRules =
///     ron::from_str(r#"{".card": (gap: Some(Step(2))), ".card:hover": (z: Some(5))}"#).unwrap();
/// assert_eq!(rules.0[1].selector.text(), ".card:hover");
/// # }
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyleRules(pub Vec<StyleRule>);

#[cfg(feature = "serde")]
impl serde::Serialize for StyleRules {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for rule in &self.0 {
            map.serialize_entry(rule.selector.text(), &rule.decl)?;
        }
        map.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for StyleRules {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RulesVisitor;

        impl<'de> serde::de::Visitor<'de> for RulesVisitor {
            type Value = StyleRules;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map of selector to declaration")
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                use serde::de::Error;
                let mut rules = Vec::new();
                while let Some(text) = map.next_key::<String>()? {
                    let selector = Selector::parse(&text).map_err(|refused| {
                        A::Error::custom(format_args!("selector `{text}`: {refused}"))
                    })?;
                    let decl = map.next_value::<StyleDecl>()?;
                    rules.push(StyleRule {
                        selector,
                        source: String::new(),
                        decl,
                    });
                }
                Ok(StyleRules(rules))
            }
        }

        deserializer.deserialize_map(RulesVisitor)
    }
}

/// The sheet the cascade styles from. The host builds it from its files and **replaces it whole**,
/// bumping `revision`; any change restyles every classed node that frame. Not initialised by the
/// plugin: with no sheet the cascade does nothing.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct StyleSheet {
    /// The rules, in cascade order.
    pub rules: StyleRules,
    /// Moves with every replacement; the host's to keep.
    pub revision: u64,
}

impl StyleSheet {
    fn looks_up(&self) -> bool {
        self.rules.0.iter().any(|rule| rule.selector.looks_up())
    }

    fn asks_an_ancestor_state(&self) -> bool {
        self.rules
            .0
            .iter()
            .any(|rule| rule.selector.asks_an_ancestor_state())
    }

    /// Why a rule's colours do not resolve under `skin` and the host's paints, one line each,
    /// `.row:hover background: no host paint `row_hoverr``, deduplicated.
    pub fn problems(&self, skin: &GooeySkin, host: Option<&HostPaints>) -> Vec<String> {
        let mut found = BTreeSet::new();
        for rule in &self.rules.0 {
            for (field, paint) in rule.decl.paints() {
                if let Some(Err(reason)) = paint.map(|paint| paint.resolve(skin, host)) {
                    found.insert(format!("{} {field}: {reason}", rule.selector));
                }
            }
        }
        found.into_iter().collect()
    }
}

/// What the sheet asks that cannot be drawn (a tone, a host paint, a surface step that is not
/// there), rewritten whenever the sheet, the skin or the host's paints change. The property is
/// left as the node's base, never guessed. Inserted by the cascade the first time it runs.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct StyleSheetProblems(pub Vec<String>);

/// The six components the cascade owns on a classed node, as values (`None`: not on the node).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Owned {
    spacing: Option<Spacing>,
    sizing: Option<Sizing>,
    shape: Option<Shape>,
    gap: Option<GapPx>,
    ink: Option<InkOverride>,
    z: Option<GlobalZIndex>,
}

/// The components of [`Owned`], by name and by their bit in [`CascadeBase`]'s warnings.
const OWNED_NAMES: [&str; 6] = [
    "Spacing",
    "Sizing",
    "Shape",
    "GapPx",
    "InkOverride",
    "GlobalZIndex",
];

/// The warning bit for a classed node with no `Role`.
const NO_ROLE_BIT: u8 = 1 << 6;

/// What a classed node had before the cascade touched it: the six components its spawn code set
/// (or not), and its `Node`, so a property the sheet stops setting goes back to the spawn's value.
/// Put on by an observer when [`StyleClass`] is first inserted; the cascade's own bookkeeping.
#[derive(Component, Clone, Debug)]
pub struct CascadeBase {
    base: Owned,
    node: Option<Node>,
    written: Option<Owned>,
    warned: u8,
}

/// Takes a node's base the first time it gets a class. A class inserted again (a new set of names)
/// keeps the base it has: the components on it now are the cascade's.
pub(crate) fn take_cascade_base(
    inserted: On<Insert<StyleClass>>,
    bases: Query<(), With<CascadeBase>>,
    owned: Query<OwnedQuery>,
    nodes: Query<&Node>,
    mut commands: Commands,
) {
    let entity = inserted.entity;
    if bases.contains(entity) {
        return;
    }
    let base = owned.get(entity).map(owned_of).unwrap_or_default();
    commands.entity(entity).try_insert(CascadeBase {
        base,
        node: nodes.get(entity).ok().cloned(),
        written: None,
        warned: 0,
    });
}

/// Gives a node whose class is taken off what its spawn code set, and forgets its base.
pub(crate) fn drop_cascade_base(
    removed: On<Remove<StyleClass>>,
    bases: Query<&CascadeBase>,
    mut commands: Commands,
) {
    let entity = removed.entity;
    let Ok(base) = bases.get(entity) else {
        return;
    };
    let base = base.base;
    commands
        .entity(entity)
        .queue_silenced(move |mut node: EntityWorldMut| {
            put(&mut node, base.spacing);
            put(&mut node, base.sizing);
            put(&mut node, base.shape);
            put(&mut node, base.gap);
            put(&mut node, base.ink);
            put(&mut node, base.z);
            node.remove::<CascadeBase>();
            if let Some(mut role) = node.get_mut::<Role>() {
                role.set_changed();
            }
        });
}

/// Inserts `value` on `node`, or takes the component off for `None`.
fn put<C: Component<Mutability = bevy::ecs::component::Mutable>>(
    node: &mut EntityWorldMut,
    value: Option<C>,
) {
    match value {
        Some(value) => {
            node.insert(value);
        }
        None => {
            node.remove::<C>();
        }
    }
}

/// The six owned components of one node, as references.
type OwnedRefs<'a> = (
    Option<&'a Spacing>,
    Option<&'a Sizing>,
    Option<&'a Shape>,
    Option<&'a GapPx>,
    Option<&'a InkOverride>,
    Option<&'a GlobalZIndex>,
);

type OwnedQuery = OwnedRefs<'static>;

fn owned_of((spacing, sizing, shape, gap, ink, z): OwnedRefs) -> Owned {
    Owned {
        spacing: spacing.copied(),
        sizing: sizing.copied(),
        shape: shape.copied(),
        gap: gap.copied(),
        ink: ink.copied(),
        z: z.copied(),
    }
}

/// What the cascade reads to match a selector: the classes, the parent and the state of a node.
trait CascadeTree {
    fn classes(&self, entity: Entity) -> Option<&StyleClass>;
    fn parent(&self, entity: Entity) -> Option<Entity>;
    fn in_state(&self, entity: Entity, state: StyleState) -> bool;
}

/// The states a selector asks of a node, as the cascade reads them.
type StateRefs = (
    Option<&'static Hovered>,
    Has<Pressed>,
    Has<Selected>,
    Has<Checked>,
    Has<InteractionDisabled>,
);

/// The cascade's reading of the world inside a system.
#[derive(SystemParam)]
pub struct CascadeReader<'w, 's> {
    classes: Query<'w, 's, &'static StyleClass>,
    parents: Query<'w, 's, &'static ChildOf>,
    states: Query<'w, 's, StateRefs>,
    focus: Option<Res<'w, InputFocus>>,
}

impl CascadeTree for CascadeReader<'_, '_> {
    fn classes(&self, entity: Entity) -> Option<&StyleClass> {
        self.classes.get(entity).ok()
    }

    fn parent(&self, entity: Entity) -> Option<Entity> {
        self.parents.get(entity).ok().map(ChildOf::parent)
    }

    fn in_state(&self, entity: Entity, state: StyleState) -> bool {
        if state == StyleState::Focus {
            return self.focus.as_ref().and_then(|focus| focus.get()) == Some(entity);
        }
        let Ok((hovered, pressed, selected, checked, disabled)) = self.states.get(entity) else {
            return false;
        };
        match state {
            StyleState::Hover => hovered.is_some_and(Hovered::get),
            StyleState::Pressed => pressed,
            StyleState::Selected => selected,
            StyleState::Checked => checked,
            StyleState::Disabled => disabled,
            StyleState::Focus => false,
        }
    }
}

impl CascadeTree for World {
    fn classes(&self, entity: Entity) -> Option<&StyleClass> {
        self.get::<StyleClass>(entity)
    }

    fn parent(&self, entity: Entity) -> Option<Entity> {
        self.get::<ChildOf>(entity).map(ChildOf::parent)
    }

    fn in_state(&self, entity: Entity, state: StyleState) -> bool {
        let has = |check: fn(&EntityRef) -> bool| self.get_entity(entity).is_ok_and(|e| check(&e));
        match state {
            StyleState::Hover => self.get::<Hovered>(entity).is_some_and(Hovered::get),
            StyleState::Pressed => has(|e| e.contains::<Pressed>()),
            StyleState::Selected => has(|e| e.contains::<Selected>()),
            StyleState::Checked => has(|e| e.contains::<Checked>()),
            StyleState::Disabled => has(|e| e.contains::<InteractionDisabled>()),
            StyleState::Focus => {
                self.get_resource::<InputFocus>().and_then(InputFocus::get) == Some(entity)
            }
        }
    }
}

/// The rules matching `entity`, as (rule index, specificity), in cascade order: lowest specificity
/// first, a tie in sheet order, so laying each over the last leaves the winner on top.
fn matching(sheet: &StyleSheet, tree: &impl CascadeTree, entity: Entity) -> Vec<(usize, u32)> {
    if tree.classes(entity).is_none() {
        return Vec::new();
    }
    let mut found: Vec<(usize, u32)> = sheet
        .rules
        .0
        .iter()
        .enumerate()
        .filter_map(|(index, rule)| {
            rule.selector
                .matches(tree, entity)
                .map(|specificity| (index, specificity))
        })
        .collect();
    found.sort_by_key(|(index, specificity)| (*specificity, *index));
    found
}

/// Every declaration matching `entity`, laid over each other in cascade order.
fn cascaded(sheet: &StyleSheet, tree: &impl CascadeTree, entity: Entity) -> StyleDecl {
    matching(sheet, tree, entity)
        .into_iter()
        .fold(StyleDecl::default(), |under, (index, _)| {
            under.overlaid(&sheet.rules.0[index].decl)
        })
}

/// The six components for a node whose base is `base` under the declaration `decl`. A colour that
/// does not resolve is left as the base's.
fn resolved(base: &Owned, decl: &StyleDecl, skin: &GooeySkin, host: Option<&HostPaints>) -> Owned {
    let spacing = if decl.padding.is_some() || decl.margin.is_some() {
        let under = base.spacing.unwrap_or_default();
        let lay = |under: Sides<Space>, over: Option<Sides<Space>>| {
            over.map_or(under, |over| under.overlaid(&over))
        };
        Some(Spacing {
            padding: lay(under.padding, decl.padding),
            margin: lay(under.margin, decl.margin),
        })
    } else {
        base.spacing
    };
    let sizes = [
        decl.width,
        decl.height,
        decl.min_width,
        decl.min_height,
        decl.max_width,
        decl.max_height,
    ];
    let sizing = if sizes.iter().any(Option::is_some) || decl.clip.is_some() {
        let under = base.sizing.unwrap_or_default();
        Some(Sizing {
            width: decl.width.or(under.width),
            height: decl.height.or(under.height),
            min_width: decl.min_width.or(under.min_width),
            min_height: decl.min_height.or(under.min_height),
            max_width: decl.max_width.or(under.max_width),
            max_height: decl.max_height.or(under.max_height),
            clip: decl.clip.or(under.clip),
        })
    } else {
        base.sizing
    };
    let shape = if decl.border.is_some()
        || decl.border_ink.is_some()
        || decl.radius.is_some()
        || decl.fill.is_some()
    {
        let under = base.shape.unwrap_or_default();
        Some(Shape {
            border: decl
                .border
                .map_or(under.border, |over| under.border.overlaid(&over)),
            ink: decl.border_ink.or(under.ink),
            radius: decl.radius.or(under.radius),
            fill: decl.fill.or(under.fill),
        })
    } else {
        base.shape
    };
    let gap = decl.gap.map(|space| GapPx(space.px(skin))).or(base.gap);
    let paint = |paint: &Option<Paint>| {
        paint
            .as_ref()
            .and_then(|paint| paint.resolve(skin, host).ok())
    };
    let (fill, ink, edge) = (
        paint(&decl.background),
        paint(&decl.color),
        paint(&decl.border_color),
    );
    let ink = if fill.is_some() || ink.is_some() || edge.is_some() {
        let under = base.ink.unwrap_or_default();
        Some(InkOverride {
            fill: fill.or(under.fill),
            ink: ink.or(under.ink),
            edge: edge.or(under.edge),
        })
    } else {
        base.ink
    };
    Owned {
        spacing,
        sizing,
        shape,
        gap,
        ink,
        z: decl.z.map(GlobalZIndex).or(base.z),
    }
}

type Writable = (
    &'static mut CascadeBase,
    Option<&'static mut Spacing>,
    Option<&'static mut Sizing>,
    Option<&'static mut Shape>,
    Option<&'static mut GapPx>,
    Option<&'static mut InkOverride>,
    Option<&'static mut GlobalZIndex>,
    Option<&'static mut Node>,
    Option<&'static mut Role>,
    &'static StyleClass,
);

/// The states whose change restyles a classed node.
type StateChanged = Or<(
    Changed<Hovered>,
    Added<Pressed>,
    Added<Selected>,
    Added<Checked>,
    Added<InteractionDisabled>,
)>;

/// The cascade: writes each classed node's [`Spacing`], [`Sizing`], [`Shape`], [`GapPx`],
/// [`InkOverride`] and `GlobalZIndex` from its base and the [`StyleSheet`]'s matching rules, highest
/// specificity winning and a tie going to the later rule. In [`crate::GooeySystems::Cascade`], before
/// the skin draws.
///
/// It restyles every classed node when the sheet, the skin or [`HostPaints`] change, and otherwise
/// only the nodes whose class, state (`Hovered`, `Pressed`, `Selected`, `Checked`,
/// `InteractionDisabled`, focus) or parent changed, with their classed descendants when a rule looks
/// up the tree. A component the host's code changed since the cascade last wrote it is adopted as the
/// node's new base, with one warning per node and component: a value set in code and a rule fight.
#[allow(clippy::too_many_arguments)]
pub fn apply_style_sheet(
    sheet: Res<StyleSheet>,
    skin: Res<GooeySkin>,
    host: Option<Res<HostPaints>>,
    problems: Option<ResMut<StyleSheetProblems>>,
    tree: CascadeReader,
    mut last_focus: Local<Option<Entity>>,
    everyone: Query<Entity, With<StyleClass>>,
    reclassed: Query<Entity, Changed<StyleClass>>,
    restated: Query<Entity, StateChanged>,
    moved: Query<Entity, Changed<ChildOf>>,
    mut unstated: (
        RemovedComponents<Pressed>,
        RemovedComponents<Selected>,
        RemovedComponents<Checked>,
        RemovedComponents<InteractionDisabled>,
    ),
    children: Query<&Children>,
    mut nodes: Query<Writable>,
    mut commands: Commands,
) {
    let full =
        sheet.is_changed() || skin.is_changed() || host.as_ref().is_some_and(|h| h.is_changed());
    let host = host.as_deref();
    if full {
        let found = StyleSheetProblems(sheet.problems(&skin, host));
        match problems {
            Some(mut problems) => {
                problems.set_if_neq(found);
            }
            None => commands.insert_resource(found),
        }
    }
    let focus = tree.focus.as_ref().and_then(|focus| focus.get());
    let mut dirty = EntityHashSet::default();
    if full {
        dirty.extend(everyone.iter());
    } else {
        let looks_up = sheet.looks_up();
        let ancestor_states = sheet.asks_an_ancestor_state();
        let with_descendants = |entity: Entity, deep: bool, dirty: &mut EntityHashSet| {
            dirty.insert(entity);
            if deep {
                dirty.extend(children.iter_descendants(entity));
            }
        };
        for entity in &reclassed {
            with_descendants(entity, looks_up, &mut dirty);
        }
        for entity in &moved {
            with_descendants(entity, looks_up, &mut dirty);
        }
        let (pressed, selected, checked, disabled) = &mut unstated;
        let state_changes = restated
            .iter()
            .chain(pressed.read())
            .chain(selected.read())
            .chain(checked.read())
            .chain(disabled.read())
            .chain((focus != *last_focus).then_some(focus).flatten())
            .chain((focus != *last_focus).then_some(*last_focus).flatten())
            .collect::<Vec<_>>();
        for entity in state_changes {
            with_descendants(entity, ancestor_states, &mut dirty);
        }
    }
    *last_focus = focus;
    for entity in dirty {
        let Ok((mut base, spacing, sizing, shape, gap, ink, z, node, role, classes)) =
            nodes.get_mut(entity)
        else {
            continue;
        };
        if role.is_none() && base.warned & NO_ROLE_BIT == 0 {
            base.warned |= NO_ROLE_BIT;
            warn!(
                "bevy_gooey: class `{}` on a node with no Role: gooey styles only Role nodes",
                classes.0.join(" ")
            );
        }
        let current = Owned {
            spacing: spacing.as_deref().copied(),
            sizing: sizing.as_deref().copied(),
            shape: shape.as_deref().copied(),
            gap: gap.as_deref().copied(),
            ink: ink.as_deref().copied(),
            z: z.as_deref().copied(),
        };
        if let Some(written) = base.written {
            let drift = [
                current.spacing != written.spacing,
                current.sizing != written.sizing,
                current.shape != written.shape,
                current.gap != written.gap,
                current.ink != written.ink,
                current.z != written.z,
            ];
            for (bit, moved) in drift.into_iter().enumerate() {
                if !moved {
                    continue;
                }
                if base.warned & (1 << bit) == 0 {
                    base.warned |= 1 << bit;
                    warn!(
                        "bevy_gooey: `{}` on a node of class `{}` was changed in code after the style sheet wrote it; the code's value is its base now (style it in the sheet or drop the class)",
                        OWNED_NAMES[bit],
                        classes.0.join(" ")
                    );
                }
                match bit {
                    0 => base.base.spacing = current.spacing,
                    1 => base.base.sizing = current.sizing,
                    2 => base.base.shape = current.shape,
                    3 => base.base.gap = current.gap,
                    4 => base.base.ink = current.ink,
                    _ => base.base.z = current.z,
                }
            }
        }
        let decl = cascaded(&sheet, &tree, entity);
        let want = resolved(&base.base, &decl, &skin, host);
        if base.written != Some(want) {
            base.written = Some(want);
        }
        let mut node = node;
        let spawned = base.node.clone();
        let mut gone = false;
        let mut restore = |restore_fields: fn(&mut Node, &Node)| {
            if let (Some(node), Some(spawned)) = (node.as_deref_mut(), spawned.as_ref()) {
                restore_fields(node, spawned);
            }
        };
        let mut entity_commands = commands.entity(entity);
        gone |= write(
            &mut entity_commands,
            spacing,
            want.spacing,
            &mut restore,
            |node, spawned| {
                node.padding = spawned.padding;
                node.margin = spawned.margin;
            },
        );
        gone |= write(
            &mut entity_commands,
            sizing,
            want.sizing,
            &mut restore,
            |node, spawned| {
                node.width = spawned.width;
                node.height = spawned.height;
                node.min_width = spawned.min_width;
                node.min_height = spawned.min_height;
                node.max_width = spawned.max_width;
                node.max_height = spawned.max_height;
                node.overflow = spawned.overflow;
            },
        );
        gone |= write(
            &mut entity_commands,
            shape,
            want.shape,
            &mut restore,
            |node, spawned| {
                node.border = spawned.border;
                node.border_radius = spawned.border_radius;
            },
        );
        gone |= write(
            &mut entity_commands,
            gap,
            want.gap,
            &mut restore,
            |node, spawned| {
                node.row_gap = spawned.row_gap;
                node.column_gap = spawned.column_gap;
            },
        );
        gone |= write(&mut entity_commands, ink, want.ink, &mut restore, |_, _| {});
        gone |= write(&mut entity_commands, z, want.z, &mut restore, |_, _| {});
        if gone && let Some(mut role) = role {
            // The skin draws the role afresh, so what the sheet stops setting is the role's again.
            role.set_changed();
        }
    }
}

/// Writes `want` into the node's component (in place, inserted, or taken off), touching nothing
/// that already holds it. Whether it took a component off: the node's `Node` fields it governed are
/// put back as spawned (`restore`) and the role redrawn.
fn write<C: Component<Mutability = bevy::ecs::component::Mutable> + PartialEq + Copy>(
    commands: &mut EntityCommands,
    current: Option<Mut<C>>,
    want: Option<C>,
    restore: &mut impl FnMut(fn(&mut Node, &Node)),
    restore_fields: fn(&mut Node, &Node),
) -> bool {
    match (current, want) {
        (Some(mut current), Some(want)) => {
            if *current != want {
                *current = want;
            }
            false
        }
        (None, Some(want)) => {
            commands.try_insert(want);
            false
        }
        (Some(_), None) => {
            commands.try_remove::<C>();
            restore(restore_fields);
            true
        }
        (None, None) => false,
    }
}

/// One rule that matched a node, for an inspector.
#[derive(Clone, Debug, PartialEq)]
pub struct MatchedRule {
    /// The rule's selector as written.
    pub selector: String,
    /// Where the rule was read from ([`StyleRule::source`]).
    pub source: String,
    /// The specificity it matched with.
    pub specificity: u32,
    /// What it sets.
    pub decl: StyleDecl,
}

/// Why a node looks the way the sheet makes it: its classes, every rule that matches it in cascade
/// order (the last wins a property), and what they add up to.
#[derive(Clone, Debug, PartialEq)]
pub struct StyleExplain {
    /// The node's classes.
    pub classes: Vec<String>,
    /// The matching rules, lowest specificity first, a tie in sheet order.
    pub matched: Vec<MatchedRule>,
    /// Every matching declaration laid over the last.
    pub resolved: StyleDecl,
}

/// What the [`StyleSheet`] does to `entity`, or `None` when it has no [`StyleClass`] or there is no
/// sheet. A pure read, for a host's style inspector.
pub fn explain(world: &World, entity: Entity) -> Option<StyleExplain> {
    let sheet = world.get_resource::<StyleSheet>()?;
    let classes = world.get::<StyleClass>(entity)?;
    let matched: Vec<MatchedRule> = matching(sheet, world, entity)
        .into_iter()
        .map(|(index, specificity)| {
            let rule = &sheet.rules.0[index];
            MatchedRule {
                selector: rule.selector.text().to_owned(),
                source: rule.source.clone(),
                specificity,
                decl: rule.decl.clone(),
            }
        })
        .collect();
    let resolved = matched.iter().fold(StyleDecl::default(), |under, rule| {
        under.overlaid(&rule.decl)
    });
    Some(StyleExplain {
        classes: classes.0.clone(),
        matched,
        resolved,
    })
}
