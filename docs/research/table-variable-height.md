# A windowed table of rows that differ in height (#123)

Item: the backlog's second item, "windowed variable-height table". A table with a `.wrap()`ed column
binds **every row** it has (iteration 109's shortcut, `pool := len`), because every row can be a
different height and the table's window was arithmetic on one pitch. Ten thousand wrapped rows mean
ten thousand row entities, ten thousand cells and ten thousand `Text` layouts — a table that cannot
be used for what a table is for. This note is about keeping the window the crate already has when
rows differ.

## Sources

- **Craft (game UX), David Lightbown, *Designing the User Experience of Game Development Tools*, 2nd
  ed. (CRC Press, 2026), ch. 7 "Design", on Nielsen's three response-time limits**:
  "1 second 'is about the limit for the user's flow of thought to stay uninterrupted.' The user will
  notice the delay and will 'lose the feeling of operating directly on the data,' which can make the
  tool feel sluggish." A table binds its rows in the frame the scroll moves; a frame that walks ten
  thousand rows is a tool that has lost direct manipulation, not a tool that is 200 ms slow.
- **Craft (standards), WCAG 2.2 SC 1.4.10 Reflow**: "content can be presented without loss of
  information or functionality, and without requiring scrolling in two dimensions". A cell that
  wraps is the crate's answer to a narrow pane; wrapping is only usable if the table still holds
  thousands of rows.
- **Shipping game, Unity UI Toolkit `ListView` / `MultiColumnListView` `VirtualizationMethod`**,
  <https://docs.unity3d.com/6000.1/Documentation/Manual/UIE-uxml-element-ListView.html>:
  "`VirtualizationMethod.FixedHeight` sets all items to the same height… `VirtualizationMethod.DynamicHeight`
  waits for the actual height of every item when geometry changes. More flexible but less
  performant." Unity ships **both**, and its documentation names the trade: variable heights are a
  measurement problem, not a reason to drop the window. Its `fixedItemHeight` in `DynamicHeight` mode
  "serves as the default height to help calculate the number of items necessary and the scrollable
  area before items are laid out" — the estimate a variable-height window starts from.
- **Technical, virtualized lists on the web (TanStack Virtual / `react-virtual`)**: a virtualizer
  keeps a **measurement cache**, `getTotalSize()` for the sizer's height, and `measureElement`
  updating the cache when a row's height changes; items are placed from the cache's prefix sums. The
  same shape as this note's `heights` + `tops`.
- **Technical, this crate**: `model::table_window` (the one-pitch window), `scroll_to_reveal` (the
  least scroll that keeps a row in view), `table_anchor` (the row at the top kept at the top when
  rows come and go), and Bevy 0.20's `ComputedNode::size` in physical pixels against
  `inverse_scale_factor`.

## Craft principle

**A row's place is known only after it has been laid out, so a variable-height table must remember
what it measured and never guess twice.** The window is drawn from remembered heights; a row never
measured is one pitch (what it is until its words are laid out), and the frame after the pool is
laid out replaces the estimates with the real numbers. A scroll is then "the first row whose box
holds this offset", found from the remembered heights — and a row half above the top is still the
first row drawn, because a scroll is not a row boundary and a table that snapped to one would jump.

The second half is **stability**: a measured row that moves must move the rows after it and no
others, and the table must not re-sum all of them to find out. A table that rebuilds its whole height
list per frame on ten thousand wrapped rows has traded one unbounded cost for another.

## How shipping games do it

- **Unity** ships `FixedHeight` and `DynamicHeight` as one enum on the collection view, with the
  fixed height still required in dynamic mode as the estimate. It does not offer "draw everything".
- **Web virtualizers** (TanStack Virtual, `react-virtualized`, `react-window`'s variable mode) all
  keep a size cache keyed by item, a prefix sum of sizes for placement, and a total size for the
  scrollbar; `measureElement` is called by a resize observer after layout and only the changed item
  and the ones after it are re-summed.
- **Slack, Discord, iMessage Web** virtualize message lists whose rows are one line or many; the
  visible window plus an overscan item is the whole list that exists in the DOM.

So: a measurement cache, a prefix sum maintained incrementally, a total size for the scroll, a
window from the scroll, an overscan row. The crate already has the window and the total-size idea
(the rows column's padding); what it lacks is the cache.

## Model adopted

`src/model.rs`, engine-free:

- `rows_in_view(scroll, view, tops) -> (first, end)`: `tops` holds each row's top and one entry past
  the last row (the content's end). `first` is the last row whose top is at or above `scroll` (the
  row whose box holds it), `end` the first row whose top is at or past the view's bottom; `end` is
  at least `first + 1` and at most the row count, so the cut row that says "there is more" is drawn.
  A scroll or view that is not usable and a table with no rows answer `(0, 0)`.
- `row_tops(tops, heights, from, pitch)`: the prefix sum, from row `from` on (0 means from the top),
  with an unmeasured row (`0.0`, negative or not finite) taking the pitch. `tops` is `heights.len() + 1`
  entries. `from` past the end is no work, so an idle table writes nothing.

`src/table.rs`:

- `TableCache::heights` (a display row's measured height, `0.0` unknown) and `tops` (the prefix sum),
  with `tops_dirty: Option<usize>` the earliest row whose height moved. Each frame the pool's rows
  are read from `ComputedNode` **at the display row the cache drew them at** and a reading more than
  half a pixel off what was stored marks that row dirty; the sum is then walked from the earliest
  dirty row only. Half a pixel because layout rounds to device pixels: a row a hundredth off is the
  same row, and re-summing ten thousand rows for it is the cost this pass removes.
- A rebuilt order (a sort, an insertion, a removal) makes every height unknown and starts the walk
  from the top, because row *i* is no longer the row that was measured at *i*.
- The window is `rows_in_view`; the pool is the rows in view plus **one** (the cut row). The rows
  column's top and bottom padding are the measured room above the first bound row and below the
  last, so the scroll's length is the measured content and the cut row still says there is more.
- The anchor (the row at the top kept at the top when rows come and go) and the focus follow read
  the cursor row's own top from `tops` instead of summing the pool's laid-out heights.
- One-pitch tables are **unchanged**: same arithmetic, same pool, same anchor, byte for byte.

## Parameters

| Parameter | Value | Why |
| --- | --- | --- |
| An unmeasured row's height | the pitch (`GooeySkin::table_pitch`) | what the row is before its words are laid out (Unity's `fixedItemHeight`) |
| A height's change that re-sums | more than 0.5 logical px | layout rounds to device pixels; a hundredth of a pixel is the same row |
| Pool | rows in view + 1 | the cut row says there is more |
| Re-sum on a rebuild | all rows, from row 0 | a sort puts different rows at the same indexes |
| Re-sum on a measurement | from the earliest row that moved | a row's height moves only the rows after it |

## Validation targets

1. Ten thousand rows with a wrapping column spawn at most a viewport's worth of row entities (the
   same count as a one-pitch table of the same height), at 1x and 200% text, after a scroll to the
   end, and after a resize.
2. A tall row and a short row: the row under the scroll's top is the one drawn first, whole rows
   stack with no gap and no overlap, and every row of the data is reachable (End shows the last).
3. Scrolled to row 5,000, 100 rows inserted above: the row at the top stays at the top, at its own
   offset, not at `first × pitch`.
4. The cursor's reveal (Down on a tall row) puts the whole row in view, using its measured height.
5. A sort of a wrapping table starts from the top and re-measures from row 0.
6. `rows_in_view` and `row_tops`: the unit props above, in `model`.
7. Nothing written while idle: thirty frames at rest change no component.
8. Frame time: no regression over 0.5 ms on the gallery and the table example.

## Deviations

- **The window's overscan is one row, not the two a one-pitch table keeps** (`ceil(rows) + 1` vs the
  pool's `+2`). A variable-height table cannot know how many rows a viewport holds until its rows are
  measured, so the pool is *the rows the remembered heights put in view* plus the cut one; the first
  frame before anything is measured is the pitch window, which is the same size a one-pitch table
  draws. A larger overscan would draw rows nobody asked for and, in a table whose rows are three
  lines tall, cost more than it saves.
- **The height cache is by display row, not by key** (the web's advice is by id). A sort permutes the
  rows, so a cache by row index is wrong exactly when the order changes — which this note answers by
  throwing the whole cache away on a rebuild. Keying by `u64` would keep measurements across a sort
  at the cost of a `HashMap` lookup per row per frame; the rebuild is rare (a sort, an insertion, a
  removal) and the throw-away is the simpler of one path.
- **No estimate from the text's own line count.** A row's height could be computed from its longest
  cell's width and the face's advance before layout, but the face is the host's and the advance is
  the layout's; a guess that is wrong by a line per row drifts the whole scroll. The pitch is the
  one estimate that is already true before layout.

## Pending index

- The corpus has no source on virtualized lists or measurement caches (`hs distill search` on
  virtualization, list recycling, and scroll anchoring: unrelated hits). The technical sources above
  were read directly: Unity's ListView manual pages and the TanStack Virtual `measureElement` docs.
- A table whose rows change height *while scrolled to them* (a cell relabelled with longer words)
  re-measures and shifts the rows below; the anchor keeps the top row but not the exact offset. Not
  built: CSS's `overflow-anchor` equivalent (anchoring the row that is *being read*, not the top
  one). It is the next depth on this feature, and the note above records why the top row was chosen
  first: it is the one the player scrolled to.

## What great looks like

- A table of ten thousand wrapped rows that scrolls at the frame rate of the rest of the interface:
  the wheel moves the rows the way it moves a list, and the pool count is the same as a plain
  table's.
- Tall and short rows interleaved with no gap, no overlap, and no one-pixel drift as they are
  measured: the seam between two rows is one line, the same as a row's own frame.
- Scrolling to the middle and back leaves every row at the pixel it was at: no accumulating drift
  from repeated measuring.
- A sort of a wrapping table shows the new top immediately and the rows below it whole; the scroll
  does not stay at an offset that no longer means anything.
