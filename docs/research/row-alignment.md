# A row aligns its children, and an override it ignores says so

Researched 2026-11-27 for iteration 118 (issue **#117** filed by this pass, `docs/loop-log.md`).
A row of mixed-height children is a HUD's commonest layout: a name wrapping to two lines beside a
count that fits one, a label beside a 24 px icon, a wrapped `fill_text` beside a count in a `band`.
This note records what the crate does today, what three independent sources say it should do, and
the two ways of saying so in code. The implementation is small; the note is here because the
decision it settles (a silent override, or a warning) is one the owner may want to revisit.

## The bug, in one line

`Role::Row`'s arm of `apply_skin` writes `align_items` only when the node carries a `RowLayout`
with `align: Some(..)` (`src/skin.rs`, the `takes_a_row_layout` list). Without one, the node keeps
Bevy's `Node::default()`, whose `align_items` is `AlignItems::DEFAULT` — flexbox's initial value.

**flexbox's initial value is `stretch`.** CSS Flexible Box Layout Module Level 1, §8.4 `align-items`:
the property's initial value is `stretch`, and "This value [stretch] … makes the cross size of the
item's margin box as close to the same size as the line as possible." So a plain `row(0.0)` draws a
one-line count stretched to the height of the wrapped two-line name beside it, with its words at the
top of the taller box. That is the picture a game developer sees on the first screen they build, and
it reads as "the crate has no idea how a row should look".

The measured case is in `examples/kit.rs`: a `band` holding a `fill_text` long enough to wrap and a
`count_text` "3". Today the count's box is as tall as the wrapped name (48 px against the count's own
24 px row) with the digit on the top line. `tests/rows.rs` pins the wrong picture before the change
and the right one after (see the acceptance below).

## Source 1 — craft: the game-UI design literature

David Lightbown, *Designing the User Experience of Game Development Tools* (CRC Press, 2015). The
book's design chapter is this crate's design reference (`docs/research/gooey.md` lists it), and its
thesis is that a tool's controls must read as **one system**: a row's parts share one baseline and
one rhythm, so a glance finds the value without re-reading the label. A count floating at the top of
a box sized by its neighbour is exactly the "each panel was built by a different person" look the
book warns against — the parts are not sharing the alignment a row exists to give them.

The corpus's Gestalt source says the same thing more formally: Wagemans et al., "A century of
Gestalt psychology in visual perception" (*Psychological Bulletin*, 2012, doi:10.1037/a0029333),
§2 — **good continuation** is "the tendency for elements to be grouped to form smooth contours". Two
short items in a row whose middles sit on one horizontal line are one line; two items whose tops
sit on one line and whose middles do not are two. A row that aligns its children's middles reads as a
row, which is what the widget is for.

## Source 2 — shipping game: Rune

Rune (`https://github.com/anomalyco/rune` at `f73f72cdc563534e4f7de9f5a79bed007ac42384`, MIT,
© Anomaly), this crate's open look reference, gives every row it builds an explicit cross-axis
alignment rather than leaving it to the layout engine: its list rows, its toolbars and its dialog
footers all centre their items (a label beside an icon, a count beside a name). The one place it
does not is a body of text that must top-align with a fixed header — and it says so at that site,
per row, instead of relying on the engine's default. That is the shape this crate should copy: a
sensible default per role, an override that wins, and no third behaviour hidden in a default value.

*Confidence: reported.* Rune's source was read at the pinned commit for its row builders; the claim
is that no row it builds is left at the engine's `stretch`. It is a look reference, not a
specification.

## Source 3 — technical: CSS Flexbox and Bevy's `Node`

- CSS Flexible Box Layout §8.4: `align-items`' initial value is `stretch`; `align-items: center`
  "the flex item's margin box is centred within the line in the cross axis". `align-self` on a child
  overrides its parent's `align-items` — one child can opt out of the row's line.
- `bevy_ui` 0.20's `Node::default()` takes `AlignItems::DEFAULT`, which is taffy's flexbox default,
  `Stretch`. `apply_skin` writes a `Role`'s fields after the node is spawned, so a role arm that does
  not write `align_items` leaves the node at `Stretch` — the engine's initial value, not the crate's
  intent.
- `Role::Toolbar`, `Role::StatusBar`, `Role::StatusGroup` and `Role::ToggleGroup` **already** write
  `align_items: AlignItems::Center` (measured in `src/skin.rs`), and `tests/toolbar.rs` and
  `tests/status_bar.rs` hold the result. So the crate has one alignment for its bars and the engine's
  opposite for its plain rows: the second inconsistency the bug is about.

## What the fix is, and the one decision it settles

`Role::Row`, `Role::Column`, `Role::ButtonRow`, `Role::FillColumn`, `Role::FillRegion` and any node
that takes a `RowLayout` get `align_items: AlignItems::Center` from their arm when the host has not
set one, so a plain row centres its children and a wrapping row's second line stands down from its
first. A `RowLayout { align: Some(..) }` still wins; `GapPx` and the other overrides are untouched.

The decision this note records is what happens when a host puts a `RowLayout` on a role that the
arm does not read — a `Dock`, a `Split`, a table, a list or a text cell. Three options:

1. **Ignore it, silently.** What the crate does today. A `RowLayout` on a `Split` compiles, runs,
   and changes nothing; the host reads its own code, sees `align: Some(Center)`, and gets `stretch`.
   A silent drop of a component the host wrote on purpose is the worst failure mode because it costs
   a debugging session and produces no message.
2. **Panic on a misplaced `RowLayout`.** Refuses at spawn. Too harsh for a component a host may
   legitimately attach ahead of its own system, and panicking in a UI kit is not a thing this crate
   does (`AGENTS.md`: "a skin change that fails `problems()` warns and never panics").
3. **Warn, once per role.** `warn!` the first time a `RowLayout` sits on a role the arms do not read,
   naming both the role and the fields it asked for. The host reads the log once, moves the component
   to the node that needs it, and the row that was meant to align starts aligning.

**Chosen: (3).** It is the only option where the fix for a silent drop is a message the developer
sees without reading the crate's source. It matches the crate's existing "warn, never guess" rule for
a skipped component (`Tooltip` on a node with no room, `Shown` on a role with no display, `Space`
with a non-finite value). It is not a fallback: the role's own `align_items` is still what draws; the
warning only says the host's override was not heard. It costs nothing at runtime after the first
frame (the warning is latched per role in a `Local`), so a HUD pays no frame time for it.

The warning text is exact so a search turns it up:

```
row layout on Role::Split, which the skin does not read; put RowLayout on a row
```

and the roles it may sit on are exactly `takes_a_row_layout`'s list. Everything else — including
`Role::Dock`, `Role::DockBody`, `Role::Split`, `Role::SplitPane`, `Role::Table`, `Role::TableView`,
`Role::List`, `Role::ListView`, `Role::TextCell`, `Role::TextField`, and the `Role::Scroll*` family —
warns. `Role::Placed` is on the list (a placed cluster can be a row with `RowLayout::direction`), so
it does not warn; a host that wants a row from a placed node keeps it.

## The second half: `align_items` today

A row's arm writes `align_items` only through a `RowLayout`. The fix writes `Center` as the arm's
own value for the five row roles, and leaves a `RowLayout::align` to win over it exactly as it wins
over the role's gaps and direction today (`takes_a_row_layout`). `Role::Column` aligns its children
on the horizontal axis — a column of full-width rows is unaffected (they stretch), a column of a
label and a narrow choice now centres the choice, which is the column's own reading of the same rule.
`Role::FillRegion` and `Role::FillColumn` grow, and centring their children is the same choice: a
fill region's one child is a viewport that fills it, so nothing moves there.

## Acceptance (written first, in `tests/rows.rs`)

1. A plain `row(0.0)` of a 24 px square and a 48 px square: the two squares' **middles** are within
   1 px of each other, and neither is stretched to its neighbour's height (`size.y` is its own).
2. The same row with `RowLayout { align: Some(AlignItems::FlexStart) }`: the two tops are level — the
   override still wins.
3. A `band` holding a wrapping `fill_text` and a `count_text` "3": the count's box is one row high
   (not the wrapped name's height) and its digit is on the band's middle line. The measured pair is
   the picture in `examples/kit.rs`.
4. A `RowLayout` on a `Role::Split` logs the warning once, and a second frame logs nothing more.
5. A settled row writes nothing on its second frame (no `Changed<Role>` churn) — the arm's
   `set_if_neq`-style write must not tick every frame.

## Why not `align-content` or a per-child `align_self`

`align-content` is about the lines of a wrapping flow, not the items on a line; a wrapped row's
second line stands at the top by design (`role::Row`'s `align_content` is left alone). A per-child
`align_self` would need every widget's arm to write its own, and would give each widget a second way
to say the same thing — one path, the row's arm, is the rule.

## Pending index

The Lightbown book is in the corpus, but no page of it that states the row-alignment rule in words
was retrievable this pass (`distill_search` over its chunks returned the front matter only); the
claim above is attributed to the book's design chapter as this crate already uses it, not to a
quoted page. Record it under Pending index until a page is cited.
