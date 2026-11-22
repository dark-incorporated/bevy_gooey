# Syntax themes (#116)

A host (an editor) wants to offer "a syntax highlighting theme" without shipping a whole other `GooeySkin`, which restyles every
control. `GooeySkin::syntax(role)` derived every code colour from fixed hues (`code_syntax.rs::hue`).

## Sources

- Craft: Hannebauer, Hesenius, Gruhn, "Does syntax highlighting help programming novices?", Empirical Software Engineering 23
  (2018) 2795-2828, https://link.springer.com/article/10.1007/s10664-017-9579-0 (found by web search; not in the corpus).
- Shipping: Ethan Schoonover, Solarized, https://ethanschoonover.com/solarized/ (found by web search; not in the corpus).
- Technical: Lalitha A R, "Perceptually-Minimal Color Optimization for Web Accessibility: A Multi-Phase Constrained Approach",
  arXiv 10.48550/arXiv.2512.05067 (home-still stem `10.48550_arXiv.2512.05067`); WCAG 2.2 SC 1.4.3 and 1.4.1.

## Craft principle

Hannebauer et al.: "The data provided no evidence that syntax highlighting improves novices' ability to comprehend source code."
Colour in code is a taste and a habit, not a measured aid, so a host can neither hard-code one palette nor leave a reader
with a palette they cannot read: the choice is data, and the legibility floor stays the crate's.

## How shipping games do it

Solarized is the reference for a code theme that is a *set of rules*, not a list of hex values: "I designed this colorscheme
with both precise CIELAB lightness relationships and a refined set of hues based on fixed color wheel relationships", and it
"reduces brightness contrast but, unlike many low contrast colorschemes, retains contrasting hues". Editors that ship themes
(VS Code, JetBrains, Sublime) all let the player switch between a handful and store the pick in settings; none of them
re-checks a switched palette against the ground, which is how a pretty theme ends up with a 2:1 comment.

## Model adopted

`SyntaxTheme` is a field of `GooeySkin` (`GooeySkin::syntax_theme`), a list of `SyntaxInk { role, hue, chroma, shift }`. A role
the theme does not list keeps the skin's own hue, so `SyntaxTheme::skin()` (empty) is exactly today's behaviour. `GooeySkin::syntax`
reads the theme and still runs the lightness search against the editor's ground, so **every theme is held to 4.5:1 (7:1 in the
high-contrast preset) on every skin**: a theme chooses hue, chroma and where the search starts, never whether the bar is met.
Presets: `skin()`, `warm()`, `cool()`, `mono()` (one hue, roles told apart by lightness) and `separated()` (widest hue spacing,
full chroma).

## Parameters

- `hue` degrees of Oklch (Lalitha: Oklch is perceptually uniform, so one chroma reads as one colourfulness at every hue).
- `chroma: Option<f32>`: `None` is the accent's, clamped to the usual 0.08 to 0.16; `Some` is clamped to 0 to 0.2.
- `shift: Option<f32>`: where the search starts, as a share of the gap from the text's lightness to the ground's (default
  `SYNTAX_L_SHIFT` 0.18); the search only moves toward the text, so a theme cannot get under the bar with it.
- Comment, operator and error are not themable: they are the skin's muted text, text and danger.

## Deviations

- The issue proposes a `SyntaxTheme` **resource** and `syntax_in(&SyntaxTheme, role)`. A resource cannot be read by
  `GooeySkin::problems()` (which has no world) or by `apply_skin` (at Bevy's 16-parameter limit) and cannot follow a
  `SkinScope`. A skin field has one implementation, is checked by `problems()`, follows a scope and recolours open editors in
  the skin's own change detection. The host sets `skin.syntax_theme = SyntaxTheme::warm()` instead of inserting a resource.
  Assumed decision (owner may override); recorded on the issue. A skin change restyles every widget, which is the cost of any
  settings switch and is not per frame.
- `with_colours_of` (the theme step) leaves the syntax theme alone, as it leaves `tones`: it is the host's.
- Design default: no pairwise-separation guarantee for `mono` (it has one hue; the lightness spread is limited by the bar).

## Validation targets

- every preset theme on every `SkinPreset` has `problems()` empty and `high_contrast_problems()` empty on high contrast;
- two themes give different `Role::CodeSyntax` colours for one role in one `code_editor`, in the frame of the change;
- the default theme is byte-equal to the old table; `warm` and `cool` roles pairwise at least 6 CIE L\*a\*b\* apart where the
  default holds them, `separated` at least as far as the default.

## What great looks like

A code pane in `warm` reads like an amber terminal, in `cool` like a night sky, in `mono` like a printed page: different
character, the same comfortable contrast, no role that vanishes into the ground or into its neighbour.

## Pending index

Hannebauer 2018 and Solarized are web sources not yet in home-still.
