# A tree's labels wrap: `TreeWrap` (#124)

Researched before any code, iteration 118. A `tree` builds its one column itself
(`widgets::tree` → `table_bundle([Column::new("").unsortable()], Tree::new(nodes), true)`), so a host
has no handle to call `.wrap()` on: the Files tree and the history list of `bevy_dark_factory` draw a
label longer than their narrow left pane on one line, cut at the pane.

## Sources

- **Technical, WCAG 2.2 SC 1.4.10 Reflow**, <https://www.w3.org/WAI/WCAG22/Understanding/reflow.html>,
  read 2026-11-27: "Content can be presented without loss of information or functionality, and
  without requiring scrolling in two dimensions... Except for parts of the content which require
  two-dimensional layout for usage or meaning." Its note on truncation is the one that names this
  case: "A web page presents long strings of text which are truncated to save space... The content
  is presented as truncated, but a link is provided to a web page where the content is fully visible
  without truncation, or a mechanism is provided on the web page to reveal the truncated content." A
  tree row cut at the pane, with no tooltip and no horizontal scroll, is truncation with neither
  mechanism, so the words are simply gone.
- **Technical, the same page's section "Nested lists or code formatting"** (technique G224): "Lists
  and blocks of code often rely on indentation of text to visually convey hierarchy and meaning. If
  the indentation were to be removed, then meaning or functionality could suffer... maintaining
  indentations... can be important." A tree's indent and chevron column are the hierarchy. Wrapping
  must therefore put a continuation line **under the label**, at the label's own left edge, and never
  at the tree's left edge: a wrapped row that starts its second line at column 0 reads as a sibling
  of its own parent.
- **Technical, CSS `overflow-wrap`**, <https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/overflow-wrap>:
  "applies to text, setting whether the browser should insert line breaks within an otherwise
  unbreakable string to prevent text from overflowing its line box." Already this crate's rule for a
  wrapping table cell (`WRAPPED` = `LineBreak::WordOrCharacter`, #109): the words break, and a
  single word longer than the cell breaks inside itself. A tree row is a cell and takes the same
  rule, not a second one.
- **Craft, WCAG 2.2 SC 2.5.8 Target Size**, <https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html>:
  a control's target stays a minimum size. A tree's chevron box is the row's full height today
  (`align_self: Stretch`); a row that grows to three lines makes that box taller, never smaller, so
  the wrap may not shrink it back to one line.
- **This crate, #109/#123**, `docs/research/divider-wrap-led-size.md` and `tests/table.rs`: a
  wrapping column already windows on each row's **measured** height (`rows_in_view`, `row_tops`), so
  ten thousand wrapped rows cost a viewport of entities. A tree is the same `Table` with the same
  rows column, so the wrap costs nothing new here; the only gap is that the column is crate-owned.

## Craft principle

**A node's name is its identity in an outliner, so the outline must be able to show it.** Cutting it
at the pane leaves the player reading filenames as "a_dwarf_warrior_game_re…" on every row of a list
whose entire job is telling them apart. The host that wants density keeps the one-line row (the
default); the host with hundreds of long names asks once for wrap and gets it, at a height the table
already knows how to window.

## What the crate gets

`TreeWrap`, a unit marker on a tree's root (beside `TreeView`, the same shape as `CodeSoftWrap` on a
code editor): `update_tables` reads it as "every column of this table wraps", so

- `wraps` (the measured-height window, the row padding, `rows_in_view`) is on;
- `cache.wrapped` records the **effective** per-column flags, so adding or removing the marker
  despawns and respawns the pool exactly as a `Column::wrap` change does (one path, not two);
- each cell's text is spawned `WRAPPED` instead of `ONE_LINE`;
- each row gets `row_sizing(pitch)` (`height: Auto`, `min_height` one pitch), so a one-line node is
  still exactly a row and a three-line node is three.

The twisty keeps `align_self: Stretch` inside the taller cell, so it is a row high and its click
target grows with the row; the label is its sibling inside the cell, so continuation lines start at
the label's left edge under the indent (G224's rule, and free: the indent is a margin on the twisty).

Assumed decision: the marker, not `tree_with(nodes, TreeOptions)`. The issue offers both; a
component is the crate's existing shape for a knob on a widget whose parts a function builds
(`CodeSoftWrap`, `CodeMarkdown`, `TreeWrap`'s neighbour `TableRows`), it needs no second constructor
for `tree`, and it keeps one path for "the tree's column" — the `Table`'s column — rather than a
second place that decides a column's wrap.
