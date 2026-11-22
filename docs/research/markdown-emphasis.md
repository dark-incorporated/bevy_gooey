# Emphasis and strong in two inks (`CodeMarkdown`)

Written 2026-11-22 for issue #115, asked by bevy_dark_factory's Files editor: `*emphasis*` and `**strong**` were both drawn
as `SyntaxRole::Type`, so a person could not tell them apart, and the editor had to drop its `emphasis != strong` assertion to
adopt `CodeMarkdown`.

## Sources

- **Craft**: WCAG 2.2, Understanding SC 1.4.1 Use of Color (https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html):
  "Color is not used as the only visual means of conveying information, indicating an action, prompting a response, or
  distinguishing a visual element." The `*` and `**` marks stay in the text (the editor never hides them), so the two inks are
  a second cue over a first one that already exists; the ink difference is for scanning.
- **Shipping editors**: the TextMate scope naming guide (https://macromates.com/manual/en/language_grammars, "Naming
  Conventions"): `markup.bold` "bold text (text which is strong and similar should preferably be derived from this name)" and
  `markup.italic`, two scopes, which every theme of VS Code, Sublime Text and TextMate styles apart. A Markdown file in any of
  them draws the two in two looks.
- **Technical**: CommonMark spec 0.31.2, "Emphasis and strong emphasis" (https://spec.commonmark.org/0.31.2/): "Text wrapped
  with one * or _ will be wrapped with an HTML <em> tag; double *'s or _'s will be wrapped with an HTML <strong> tag." The
  delimiter run's length is the whole distinction, so the scanner already knows which one it found.
- Corpus (home-still): not queried, the three pages above settle a one-variant change.

## Craft principle

Two meanings that the document keeps apart are kept apart on screen, by something other than the marks alone.

## How shipping games do it

Editors do not give emphasis and strong one look: TextMate's `markup.bold` and `markup.italic` are separate scopes, so a theme
inks (or weights) them separately.

## Model adopted

Two new `SyntaxRole`s, `Emphasis` and `Strong`, each with its own hue in `GooeySkin::syntax` (derived from the seeds, held at
the same contrast bar as every other role, in `problems()`). `markdown_highlights` gives a run of one `*` or `_` `Emphasis` and a
run of two or more `Strong`. A host's own highlighter may use them for any language with the same two ideas.
Alternative considered: documenting `Function` for emphasis and keeping `Type` for strong. It needs no new variant, but a
function colour on italic text names the wrong thing to a host that reads `Highlights::spans`, and the issue asks for roles that
mean what they say.

## Parameters

Hues (Oklch degrees): `Emphasis` 290 (a violet) and `Strong` 350 (a rose), at the accent's chroma like the rest. Existing:
Key 250, String 145, Number 55, Keyword 320, Reference 205, Function 95, Type 175, Error the danger hue (about 25).
Chosen by sweeping every 10 degrees through the skin's own generator in all four presets and taking the hues farthest (CIE
L\*a\*b\* distance) from every existing role: 290 and 350 are the two widest gaps on the wheel (250-320 and 320-25). A first
pick of 25 for strong measured 2.1 from `Error` (the same colour); 345 for emphasis measured 0.5 from `Keyword` in parchment.

## Validation targets

`*em*` and `**strong**` in one line resolve to two different `SyntaxRole`s and two different drawn colours in every preset;
neither is within 12 CIE L\*a\*b\* units of any other role's colour (the ten older roles keep 13.0 in parchment to 25.2 in tool
between their closest two); `problems()` empty in every preset (each clears 4.5:1, 7:1 in high contrast).

## Deviations

`SyntaxRole` gains two variants and `SyntaxRole::ALL` grows from 10 to 12: a breaking change for a host that matches
exhaustively, so a minor bump. A design choice, not a cited default.

## Pending index

None.

## What great looks like

A Markdown pane where the italic words and the bold words each read at a glance as their own colour, neither mistakable for a
heading (Keyword), a code span (String) or a link (Reference).
