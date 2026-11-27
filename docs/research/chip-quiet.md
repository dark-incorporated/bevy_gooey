# A chip with no tab stop: `quiet_chip` (#121)

Researched 2026-11-27 (iteration 113). Asked for by `dark-incorporated/bevy_dark_factory` (its #551,
filed here as #121): the Cauldron editor keeps focus traversal off on purpose, so every control it
spawns must be reachable **without** a `TabIndex`, and its screen-wide `Activate` observers must not
hear a click it has already routed itself.

## Sources

1. W3C, *WCAG 2.2 SC 2.1.1 Keyboard* (craft: accessibility), read 2026-11-27: "All functionality of
   the content is operable through a keyboard interface". Its own note: "This success criterion does
   not require that every visible control that can be activated using a mouse or touchscreen must
   also be focusable and actionable using the keyboard." A pointer-only control is conformant when
   the same function has another route; F54 fails only *using only pointing-device-specific event
   handlers for a function* with no equivalent.
2. W3C, *WAI-ARIA Authoring Practices, Developing a Keyboard Interface*, "Focusability of disabled
   controls" and "The Tab Sequence", read 2026-11-27: the tab sequence is the interface's route
   between components, and "all interactive UI components need to be reachable via the keyboard …
   either by including them in the tab sequence or by making them accessible from a component that
   is in the tab sequence". A control a host deliberately keeps out of the sequence is a host
   decision, not a crate one, but it must not be a *second* implementation of the control.
3. This crate: `plain_button` (`src/widgets.rs`, iteration 96), `quiet_list_row` (iteration 96),
   `status_item_quiet` (`src/widgets.rs`, iteration 97). The established shape of a quiet variant:
   the same `Role` (so the same look, from the one `apply_skin` arm), `Hovered` kept (so the pointer
   still answers), and `bevy::ui_widgets::Button`, `TabIndex` and `AutoDirectionalNavigation` left
   off — no `Activate`, no focus, no pressed state, no `FocusRegion` neighbours.
4. bevy_dark_factory `crates/bevy_dark_factory/tests/the_sweep_is_finished.rs`
   `focus_traversal_stays_off_until_somebody_reopens_it` (the consumer's own gate, read 2026-11-27):
   a `TabIndex` in a panel is a build failure unless the line says
   `// FOCUS-DECISION-REOPENED: <why>`, because `bevy_picking` writes `Hovered` from the window's
   cursor while the editor senses the pointer itself, so click-to-focus would focus whatever the
   physical pointer rests on. That is the concrete reason `chip()` cannot be used there.
5. bevy_dark_factory `chrome::chip` and #494/#526 (the same note `chip.md` cites): the editor's own
   16-call-site `chip`, which is the second implementation this removes.
6. W3C, *WCAG 2.2 SC 2.5.8 Target Size (Minimum)* and the chip note's sources 1-7: the look, size
   and break rules of a chip do not depend on what hears its click, so nothing but the input
   components changes.

The home-still corpus has no paper on pointer-only controls; searches for "pointer only controls
keyboard alternative" and "focus traversal off game UI" were not run against it this pass (the
sources above are the standards pages themselves, read directly).

## Craft principle

**A control's look is the skin's; what may hear it is the host's.** A chip that must stay off the tab
sequence and off the `Activate` bus is still a chip: same frame, same radius, same wrapping words,
same hover and same lit state — only the three input components go. Anything else (a second bundle,
a different fill, a hand-built `Node`) makes the quiet control drift from the loud one the day either
is polished.

## Model adopted

- `quiet_chip(label)` (`src/widgets.rs`, beside `chip`): `Node::default()`, `Role::Chip`,
  `Hovered::default()`, and one `Role::ChipText` child with the chip's own break rule (`CHIP_TEXT`).
  No `Button`, no `TabIndex`, no `AutoDirectionalNavigation`.
- The look is **the same arm**: nothing in `apply_skin` changes. The fill, frame, ink, hover, lit
  `Selected` accent, disabled look and the whole-chip hit floor are the `Role::Chip` arm's.
- `Selected` lights it exactly as it lights a `chip`, and nothing sends anything when it is inserted
  or removed (there is no `Activate` observer on a quiet chip; `toggle_chip_on_activate` is on
  `toggle_chip` only). A host writes `Selected` from its own `On<PointerClick>`.
- No pressed state: `Pressed` is Bevy's `Button` state, and there is no `Button`. The hover fill is
  the pressed look's neighbour in the same skin table, so a host that wants one puts `Pressed` on
  the node itself if it insists (nothing in the crate reads it off a quiet chip).
- Accessibility: the node is created by `update_accessibility` (a `Role::Chip` is a `Spec::Node`),
  and its **name is its words**, `ChipText` joined into `names`, exactly as a `chip`'s. Its role stays
  the container the crate gives a role-less widget, as `plain_button` and `quiet_list_row` do: the
  crate declares no `Click`, because a `Click` request would have to send an `Activate` the host
  never hears. Documented on the function: **for anything a keyboard or a screen reader must
  activate, use `chip`** (source 1: the pointer-only route is conformant only when the same function
  has another one; F54 fails the case where it does not).
- `feedback.rs` is unchanged: a quiet chip never takes focus, so `feedback_on_focus` never fires for
  it, and it sends no `Activate` for the activate cue.

## Parameters

None new. The chip's — `GooeySkin::border` snapped, `radius`, `pad()` across, a button's air up and
down, half a gap between children, `min_height` = the control height (never under `target()`) — are
the `Role::Chip` arm's, and a quiet chip takes them because it carries the same role.

## Validation targets

`tests/chip.rs`, laid out for real in a headless window (the file's existing rig):

1. **The acceptance**: a `quiet_chip` carries `Role::Chip` and `Hovered` and none of `Button`,
   `TabIndex`, `AutoDirectionalNavigation`. The first run is a compile error (the function does not
   exist); with the function in place and a component left on, the test names it.
2. **The same look on every preset**: rest fill `surface_raised`, frame not `NONE`, ink `text` — for
   dark, parchment, tool and high contrast.
3. **The pointer answers but sends nothing**: a move onto it hovers it (the fill is `hovered()`), a
   click sends no `Activate`, and `Selected` inserted by the host lights it (fill `accent`, ink
   `accent_text`), removing it puts it back, and no `ValueChange` was ever sent.
4. **It is no tab stop**: with a `chip` beside it, one Tab lands on the `chip` and a second leaves
   the box (never the quiet one), and `find_by_label` names it by its words.
5. **The words are its accessibility name** and it declares no `Click` (a reader cannot be promised
   an action the host owns).

## Deviations

- **No `Pressed` look.** A quiet control of this crate has no pressed state (`plain_button`,
  `quiet_list_row`), and inventing one for a chip would need a press observer the crate does not put
  on a node it does not own. Recorded on the issue as an assumed decision.
- **The reader's role is left as the container the crate gives a role-less widget**, the
  `plain_button` precedent, rather than declaring `Button` with no `Click`. Assumed decision: the
  label is what the acceptance asks for, and a role that promises an unsent action is worse than a
  named container.
- **`toggle_chip` gets no quiet twin.** A quiet chip cannot hear `Activate`, so a quiet toggle would
  be a host-written `Selected` flip plus a host-sent `ValueChange`; the kit's rule is one path per
  feature, and the host's flip is that path. If the Cauldron asks for it, it is a second function
  with its own issue.

## Pending index

- Nothing indexed for this note: the sources are the WCAG and APG pages above and the consumer's own
  test, all read directly.

## What great looks like

A filter row in an editor whose focus is the host's: nine chips, the lit one lit, and a tenth that
clears the row — every one the same height, frame and ink, the pointer lighting each under it, no
one of them a tab stop and none of their clicks reaching a screen's `Activate` handler; a screen
reader hears "Clear filters" as words among the row, and the row's keyboard route is the editor's
own.
