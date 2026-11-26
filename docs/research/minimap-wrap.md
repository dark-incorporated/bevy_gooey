# A minimap summarizes the file, not the scroll

Researched 2026-11-26 for the pass after #117. The minimap of a `code_editor` was keyed on **display
rows**, the unit the scroll and the pooled rows use. With soft wrap on (#114) one line is several
rows, so a 120-line prose file in an 800 px editor drew **158 bars of 84 different widths** down a
lane that holds 160: the first forty lines, four segments each, and the view band covered four times
the text it should. Measured on the tree that opened this pass, by
`tests/code_navigation.rs::under_soft_wrap_the_minimap_draws_one_bar_per_line_not_per_row`, which
failed with `one bar per line, each its own line's width: 158 bars, 84 widths`.

## Sources

1. **Craft**: Shneiderman, *The Eyes Have It: A Task by Data Type Taxonomy for Information
   Visualizations* (T.R. 96-66, ISR; `10.1109/vl.1996.545307`) — the Visual Information-Seeking
   Mantra. The overview is an outline of the *document*; a summary that redraws the viewport's own
   fragmentation is a picture of the scroll, not of the file.
2. **Shipping editor**: Visual Studio Code, release 1.43 (2020), "Minimap size" —
   "When set to `proportional` (the default), **each line** gets a certain amount of vertical space
   in the minimap, depending on `editor.minimap.scale`. For example … with a configured scale of `1`
   …, **each line gets 4 px** in the minimap." One amount of space per *line*, so a wrapped line
   takes one slot. The same release's `fit` and `fill` modes sample *lines*. VS Code's own
   `logicalLine` argument (Basic editing, "Multiple selections") names the same distinction: a line
   is the logical line, a row is what wrapping makes of it.
3. **Technical**: this crate's `docs/research/code-soft-wrap.md` (#114) — `RowMap` derefs to
   `FoldMap` and shadows three of its lookups (`row_of`, `line_at`, `rows`) with display-row
   meanings, "with wrapping, the row methods count visual rows". The unwrapped semantics survive on
   the `FoldMap` half, where a unit is a line that shows. The minimap needs that half; the scroll,
   the caret and the pooled rows need the `RowMap` half. Two units, one map — the bug was reading
   the wrong one.
4. **Shipping game**: Lightbown, *Designing the User Experience of Game Development Tools* (2015),
   ch. 2 — a tool that misreports its own state costs the iteration it was meant to save. A band
   four times too tall tells the person reading it that they are looking at four times the text
   they are.

## Craft principle

The overview answers "where am I in the file, and how much of it am I seeing". Both halves of that
question are **line** questions. Wrapping is a property of the viewport's width, and a player who
narrows a window must not see their file's outline grow longer: the file did not change. A lane that
lengthens when the window narrows is a lane that lies about the file.

## Model adopted

- The lane's unit is a **line that shows**: `FoldMap::rows(lines)` for its length,
  `FoldMap::row_of` for a line's slot, `FoldMap::line_at` for the line in a slot. `RowMap` shadows
  those three, so `draw_navigation` reads the fold half explicitly and says why in a comment.
- A bar's width is its line's own display width, clamped to the file's longest line — the same
  measure as before, now that a slot is a whole line rather than a segment of one.
- The band's top is the slot of the line the view starts on, and its height is the lines the view
  holds (`ceil(area.y / line_px)`): both in line units, so the band and the bars agree.
- A press or a drag on the lane maps a slot back to its line and then to that line's **first display
  row**; the scroll's own unit stays a display row, because the scroll really is in rows.
- Folding is unchanged: a closed fold hides lines, so the lane shortens, as it always did.

## Validation targets

- 40 wrapped lines of 60 to 177 columns (all under the 200-column cap, three columns apart): at
  least 40 drawn bars, one place each, one width each, the longest 2.5 times the shortest
  (`tests/code_navigation.rs::under_soft_wrap_the_minimap_draws_one_bar_per_line_not_per_row`; keyed
  on rows it drew 41 bars of 18 widths).
- The band says **lines**, not rows: the same 30 lines of 4000 columns banded wrapped against
  unwrapped cover under 0.6 of the slots (`under_soft_wrap_the_minimap_band_is_as_tall_as_the_lines_in_view`;
  keyed on rows the two bands were identical).
- The unwrapped lane is unchanged: one bar per line, one place each
  (`without_wrap_the_minimap_still_draws_one_bar_per_line`).
- The band starts at the lane's top on an unscrolled editor and moves down it with the scroll, and a
  press or a drag still scrolls (`the_minimap_band_follows_the_scroll`, existing).
- The lane still takes room beside the code and its bars stay inside it
  (`the_minimap_lane_takes_room_beside_the_code…`, #117, existing).

## Deviations

- Nothing in the public API moves. The lane's unit is an implementation detail; a host sees the same
  `Role::CodeMinimap` and `Role::CodeMinimapBar`.
- The indent guides needed no change: they are keyed on rows, take the row's line's level, and a
  wrapped line's rows all carry that line's level, so a guide is continuous over a wrapped line and
  a run of it. Read to confirm, not edited.
- No incremental wrap rebuild (the standing #2 backlog item) and no variable-height windowing.

## Pending index

- home-still returned nothing on point for "code editor soft wrap visual row line number gutter" or
  "text editor minimap overview scrollbar readability navigation" (an unrelated boundary-labelling
  paper, software books). The three sources above are the citation of record.
- UAX #14 and Knuth-Plass remain unread; the monospace measure does not need them.

## What great looks like

Narrow the window and the prose reflows; the lane down the right edge keeps its length, its bars
keep their places, and a line that is now four rows tall is still one bar as wide as the line. The
band over the part in view is exactly as tall as the lines on screen, and a press anywhere on the
lane puts that line's first row where the eye expects it.
