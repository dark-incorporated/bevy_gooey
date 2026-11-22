# Iteration 102: a quieter LED glow (#91), a menu that follows the pointer (#92), a fold mark that is a control (#107)

Three bugs the editor filed against shipped widgets. None needs a new model; each needs the shipped
behaviour brought to what a desktop menu, a status lamp and a code gutter already do.

## Sources

- W3C, *Understanding SC 1.4.3 Contrast (Minimum)*, https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html
  (read 2026-11-22): "this success criterion applies to text in the page, including placeholder text
  and text that is shown when a pointer is hovering over an object or when an object has keyboard
  focus." Figure 2's note: a button using a text character such as ">" "to act as a right-pointing
  arrow" counts as non-text, "their contrast ratio of just above 3:1 passes SC 1.4.11".
- W3C APG, *Navigation Menubar Example*, https://www.w3.org/WAI/ARIA/apg/patterns/menubar/examples/menubar-navigation/
  (read 2026-11-22): in a submenu "Down Arrow: Moves focus to the next item in the submenu", and
  "Space Enter: ... activates the menu item"; roving focus means one item is the one the keys act on.
  Its accessibility note 4: an expanded/collapsed arrow is a drawn polygon in `currentcolor`, so it
  follows the text colour and stays visible in a high-contrast mode.
- Rune's neutral drops (the open reference this crate's `Elevation` follows; `docs/research/elevation.md`):
  a drop shadow is a quiet 1/2 px layer plus a 0.5 px ring; a larger, lighter halo is reserved for
  floating cards, never for a 12 px lamp.
- The owner's captures, filed as #91, #92 and #107 (bevy_dark_factory #493): "too big and too bright",
  "the highlighted row doesn't follow the pointer", "the v should collapse the text".

## Craft principle

A signal lamp is read at a glance and must not compete with the words beside it: the halo is a hint of
the lamp's hue at the lamp's edge, not a second, larger lamp (1.4.3 holds for the word that sits on the
halo). A menu is one cursor: the pointer and the keys move the same highlight, and whichever moved last
decides where the next key starts. A control's drawn shape is its hit target: if the eye is told "press
here", pressing there must work.

## How shipping games do it

- Status lamps in game HUDs and editors (the Unity/Unreal editor's play-state dot, the Steam
  connection light) are a flat disc with at most a faint rim of the same hue; the halo never leaves the
  row it is in.
- Every desktop menu (Windows, macOS, GTK, Qt) moves the highlight to the row under a moving pointer,
  keeps it when the pointer leaves the panel, and starts the arrow keys from it. A disabled row is
  highlighted too and chooses nothing.
- Code editors (VS Code, Zed, Rider) draw a fold mark as a triangle (down open, right folded), and a
  press on the mark itself folds.

## Model adopted

- LED: the glow is a `ShadowLayer` of blur 0.6 dots, spread 0, alpha 0.4 steady (0.12 to 0.45 pulsing),
  so its visible edge (spread plus CSS blur) stays within one dot of the dot, at every text scale.
- Menu: an observer on each row, `PointerMove`, puts `InputFocus` on the row (`FocusCause::Navigated`).
  Focus is the menu's one highlight already (the ring, the lit fill, `menu_by_input`, Enter), so
  focus-follows-hover is one path, not a second lit state.
- Fold: the mark is U+25BC (open) and U+25BA (closed), in the editor's mono face; a press anywhere in
  the mark's own box folds, resolved in `update_code_editors` with the gutter band, which is the one
  pointer path the editor already has (children are `Pickable::IGNORE`, so the root is the target).

## Parameters

| Name | Value | Why |
|---|---|---|
| `GLOW_BLUR_DOTS` | 0.6 | with spread 0, the halo ends 0.6 dots past the dot (bound: 1.0) |
| `GLOW_SPREAD_DOTS` | 0.0 | the halo is a fade, not a bigger disc |
| `GLOW_ALPHA` | 0.4 | the lamp reads; a word over the halo keeps 4.5:1 (7:1 high contrast) |
| `PULSE_ALPHA` | 0.12 to 0.45 | a Busy pulse is a breath, not a flash |
| `CHEVRON_OPEN`, `CHEVRON_CLOSED` | U+25BC, U+25BA | present in FiraMono and FiraMonoNerdFontMono (measured by the filer) |

## Validation targets

- LED halo edge (spread plus blur) at most 1.0 dot at text scale 0.5, 1.0, 1.45 and 3.0, in Live, Busy
  and Failed, steady and pulsing; alpha at most 0.5.
- `text` and `text_muted` over the densest halo (alpha 0.5 of live, warning, danger over
  `solid_surface()`) at 4.5:1 in dark, parchment and tool, 7:1 in high contrast.
- Menu: hovering the fourth row, Down goes to the fifth, Enter chooses it; a disabled row is hovered and
  chooses nothing; the pointer leaving keeps the row.
- Fold: a press at the mark's centre and at its corner toggles the fold, once each; the marks are the
  two triangles.

## Deviations

- The LED's halo bound and alpha ceiling are this crate's own (the owner asked for "smaller and less
  bright" without numbers); recorded as an assumed decision on #91.
- Focus-follows-hover (rather than a separate "hot" state) is assumed; recorded on #92.

## Pending index

home-still was not consulted for this note: all three items are behaviour already specified by the
issues and the W3C pages above, which were read directly.

## What great looks like

A Live lamp is a clean disc with a soft rim of its own hue that never touches the word beside it; the
Busy lamp breathes. Moving the mouse down a menu walks the highlight with it; pressing Down then goes
from where the mouse stopped. In the editor, a triangle in the gutter folds the block when pressed.
