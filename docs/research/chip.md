# A chip: `chip` and `toggle_chip`

Researched 2026-11-22 (iteration 105). No open issue asked for it; the owner's own project did. bevy_dark_factory (the Cauldron editor) builds its tag block, its terminal bar and its import filters from a hand-rolled `chip` (`crates/bevy_dark_factory/src/chrome.rs`: Bevy's `Button` + `Hovered` + a `held_box` of `Spacing`, `BorderPx`, `InkFollow`, `Shape`, `RowLayout` + a label with its own `LineBreak`), and its comments record the two lessons that cost it a bug each: #494 (a `NoWrap` label made the chip as wide as its whole counted phrase, so a `FlexWrap` row had nothing to wrap and the chip was painted over the panel's border) and #526 (the owner's rule: *"shrink the buttons, then, when they can't shrink more due to the text, then make it a new row"*). Those are facts about a small clickable word that every tool and every filter bar of a game has, so the kit owns it and the editor's name is in no API.

## Sources

1. W3C, *CSS Flexible Box Layout Module Level 1*, 4.5 Automatic Minimum Size of Flex Items (technical): for a non-replaced item `min-width: auto` is "the larger of the content size suggestion and the transferred size suggestion, capped by the specified size suggestion". A text's content size suggestion is its min-content width: its longest unbreakable run. So the break rule of the label, not the chip's own style, decides how far a chip can narrow.
2. W3C, *WCAG 2.2 SC 2.5.8 Target Size (Minimum)* (craft: accessibility): "The size of the target for pointer inputs is at least 24 by 24 CSS pixels". A chip is a small target, so the floor is the skin's `target()` and not the height of its words. (Read from the SC 2.5.5 text on the same page: AAA asks 44 by 44, which a comfortable skin's `target()` is.)
3. W3C, *WAI-ARIA Authoring Practices, Button pattern* (technical): "Toggle button: A two-state button that can be either off (not pressed) or on (pressed). To tell assistive technologies that a button is a toggle button, specify a value for the attribute aria-pressed." And: "it is critical the label on a toggle does not change when its state changes." So a filter chip is a button with a pressed state, and its words stay when it is lit.
4. Nielsen Norman Group, Kate Moran, *Helpful Filter Categories and Values for Better UX*, 2018 (craft): "develop filter categories and filter values that are appropriate, predictable, free of jargon, and prioritized". A filter value is a short word the player can see is on or off at a glance and that never moves when it flips.
5. Valve, Steam store *Browse by tag* (shipping reference, read 2026-11-22): the tag page is a wall of clickable one-word pills "listed in order of how frequently they have been applied", the one you have chosen lit. A pill of a word and a count, wrapping onto a second line when the row is full, is how a store of thousands of games is browsed.
6. bevy_dark_factory `chrome.rs` `chip`, `held_box`, `small_box` and issues #494, #526 (the owner's tool; shipping reference): the chip's layout contract as the editor needed it, read from the source.
7. This crate: `band` and `edge_tab` (`src/widgets.rs`, `docs/research/kit-parts.md`): the state-by-colour rule (a part's border is always laid out and lit by colour so a state moves nothing); `Role::Tab`'s selected look (the accent, with `accent_text` words); `StartsOn` (a toggle's start state through an `on_add` hook).

The home-still corpus has no paper on chips or tag filters (searches for "filter chips", "tag cloud", "small targets touch selection error rate" and "line breaking text wrapping minimum width" returned nothing on topic); the craft sources are the W3C and NN/g pages above, read directly.

## Craft principle

A chip is **one small clickable word whose size never depends on its state and whose width gives way before its row does**.

- A frame is always laid out and only its colour changes, so lighting a chip (hover, selected, focus) moves no neighbour (sources 4 and 7).
- The words break at a word, or at a letter when one word is longer than the chip may be, so the chip's least width is its longest word (source 1) and a `FlexWrap` row has something to wrap. A chip in a lane narrower than its phrase narrows and wraps its own words first; only when it cannot narrow further does the row give it a second line. Its width is capped at its host's, so one long name never overflows the panel.
- The hit area is the skin's floor, not the glyph's height (source 2).
- A chip that is on says so by the fill, not by the words (source 3: the label does not change), and a screen reader hears a pressed button.

## How shipping games do it

Steam's tag browser (source 5) is a wall of one-word pills that wrap; choosing one lights it and nothing around it moves. The Cauldron's own terminal bar and import filters (source 6) are chips with a count in the label ("NOT IMPORTED (4)") in a wrapping row; its two bugs were the two halves of the rule above: a label that could not break kept the chip as wide as the whole phrase (#494), and a chip that would not shrink before the row wrapped made a second line where narrowing would do (#526). Inventory and crafting screens use the same part for their category filters (weapons, armour, materials): a row of words, the active one lit, wrapping on a narrow window.

## Model adopted

- `chip(label)` (`src/widgets.rs`): Bevy's `Button` (focus, `Activate`, Enter, Space, South, a click) + `Hovered` + `TabIndex` + `AutoDirectionalNavigation` + `Role::Chip`, holding one `Role::ChipText` child with `LineBreak::WordOrCharacter`. `Role::ChipText` is `ButtonText`'s ink under a role of its own, so a host counting `Role::ButtonText` to find a dialog's answers does not count chips (the `MenuTitleText` precedent).
- Its look is written only in `apply_skin`: a frame of one line weight on every side, always; the skin's radius; `pad()` across and a button's air up and down; `min_height` = the control's height (never under `target()`); `flex_shrink` 1, `flex_grow` 0, `min_width` auto (the longest word), `max_width` 100%; a half gap between children (an icon beside the words).
- Colours by state, all existing skin colours: rest `surface_raised` fill, `outline` frame, `text` words; hovered `hovered()`; pressed `accent` with `accent_text`; **selected** (Bevy's `Selected`) the `accent` fill (hovered `primary_hovered()`, pressed `primary_pressed()`) with `accent_text` words; focus the `focus_ring` frame; disabled `surface` fill, `text_muted` words. Every pair is one `problems()` already checks (text on `surface_raised`, `accent_text` on accent and on its two lifts, `outline` on the ground).
- `toggle_chip(label, on)`: `chip(label)` + the public marker `ToggleChip` + a start state through an `on_add` hook (as `toggle`'s `StartsOn`), and an observer that flips `Selected` on `Activate` and sends one `ValueChange<bool>` on the chip (`is_final: true`), so a filter bar is a row of these and the game observes the values. A game that sets `Selected` itself moves the chip with no event (the crate's rule for every widget's state). A plain `chip` never flips itself: a host that owns the selection (an exclusive filter, a tab-like strip) writes `Selected` on the one that is on.
- Accessibility (`access.rs`): a chip is a `Spec::Node`; its name is its words (`ChipText` joins `names`), a click is `Activate`; a `ToggleChip` reports `toggled` from `Selected` (a pressed button); a plain chip says nothing about selection.
- No new skin field, no preset change, no new resource. `Role::Chip` is in `owns_parts`, the focus-change list and `feedback.rs` (the `UiFeedback` activate and focus cues every button gives).

## Parameters

- Frame: `GooeySkin::border` snapped to device pixels (`snap_line_width`), as every role's.
- Radius: `GooeySkin::radius`.
- Padding: `pad()` across (not a button's `pad() * 1.5`: a chip is a word and a count, not a command) and `air(text * LIST_LINE_TEXT_SIZES, pad() * 0.6)` up and down.
- Height: `min_height` = `ControlSize` height (`target()` at least).
- Column gap: half the skin's gap.

## Validation targets

`tests/chip.rs`, laid out for real in a headless window, each test written to fail first (the API does not exist before this change, so the first run is a compile error; once the stub compiles the tests must fail for the right reasons: width, state, event):

1. **The label breaks at a word**: a chip of "NOT IMPORTED (4)" in a 150 px host is at most 150 px wide, its label wraps to two lines, the chip's least width (alone in a 20 px host) is its longest word's, never the phrase's.
2. **A wrapping row has something to wrap**: five chips in a `FlexWrap` row of 300 px lay out on at least two lines, every chip inside the row's box, none painted over its right edge.
3. **A word longer than the chip breaks at a letter** (an 80-character path with no spaces): the chip stays inside its host.
4. **State changes a colour and no size**: rest, hovered, pressed, selected, disabled and focused chips are the same `ComputedNode` size, and each state's fill, frame and ink is the skin's colour by the table above, in every preset.
5. **The hit area is the floor**: a chip is at least `target()` on both sides at skin text sizes 6, 14 and 48 px.
6. **A chip is a button**: a click, Enter and Space each send one `Activate`; focus shows the ring; a disabled chip sends nothing.
7. **A toggle chip flips itself**: one click is one `ValueChange<bool>` `true`, `Selected` present; a second is `false`, `Selected` gone; a game writing `Selected` moves the chip with no event; spawned `on` it starts lit and sends nothing.
8. **A plain chip never flips itself**: a click sends `Activate` and leaves `Selected` as the host had it.
9. **Accessibility**: a chip's name is its words, a toggle chip is `toggled` with its state, a plain chip carries no toggled state.
10. **Words are not `ButtonText`**: a world with chips and one `button` has exactly one `Role::ButtonText`.
11. **Idle writes nothing**: thirty frames of a settled row of chips changes no `Node`, `BackgroundColor` or `BorderColor`.
12. Contrast in every preset (`problems()` stays empty; the chip's ink on its fill at 4.5:1 for rest, hovered, pressed and selected).

## Deviations

- **A chip's radius is the skin's, not a pill.** A pill radius on a chip that wraps to two lines becomes a stadium that clips its second line's corners; the skin's radius is what every other widget draws, so a chip reads as the same family.
- **`toggle_chip` is built although the issue-less brief only asked for "a chip".** The Cauldron's filter bars need both a command chip and an on/off chip, and a toggle that has to be wired by the host in every call site is the repeated hand-rolling the kit exists to end. Assumed decision: ship both, one `Role`.
- **No close ("x") glyph, no leading icon slot, no count slot.** A chip's children are the host's after its label: an `icon(..)` or a `count_text(..)` goes beside it (a half gap apart), and a "remove" mark is a `glyph_button` placed by the host. Adding them as parts of the chip would be a second way to compose them.
- **The label is `ChipText`, not `ControlText`/`ButtonText`.** See Model adopted; the cost is one more `Role` in `names`.

## Pending index

- Nothing indexed for this note: home-still's corpus has no chip or tag-filter paper (see Sources). The W3C and NN/g pages are cited from their own URLs.

## What great looks like

A row of nine filter chips in a panel narrower than their words: the row wraps onto a second line with the same gap between lines as between chips, every chip whole inside the panel, the lit one an accent pill whose neighbours do not move by a pixel when the player clicks another; at 200% text a long chip narrows and wraps its own words before the row gives it a second line; a keyboard player Tabs along the row, Space flips each, the ring is one whole-pixel frame, and a screen reader says "Weapons, toggle button, pressed".
