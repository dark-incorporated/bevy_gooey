# A keystroke in a wrapped pane costs what the pane draws, not the file (#122)

Written 2026-11-27. `code_wrap.md` (#114) shipped soft wrap with one cache, keyed on
`(revision, cols, tab_width)` and **rebuilt whole** on any change, and recorded that as its open
gap: "an incremental rebuild from `take_line_edits` is the open gap". Measured on this pass, in the
debug build the gate runs, typing one character at the end of a 20,000-line, 100-column file costs
**66.0 ms** a frame against **16.9 ms** for the same gesture in a 20-line file: **49 ms of work per
keystroke** that grows with the file, in a widget whose whole promise is that a file's size is not
the frame's cost.

## Sources

- **Craft**: Jakob Nielsen, *Response Times: The 3 Important Limits* (NN/g, 1993, revised 2014),
  https://www.nngroup.com/articles/response-times-3-important-limits/ — the perceptual budget a
  text pane is judged against.
- **Shipping (shipped engine editor)**: Roblox Studio, *Script Editor*
  (https://create.roblox.com/docs/studio/script-editor) — the shipped game-engine editor whose
  wrapping this crate's `CodeSoftWrap` follows, and whose panes are edited at this scale.
- **Technical**: Raph Levien, *Rope science, part 5 — incremental word wrapping* (xi-editor, 2016),
  https://xi-editor.io/docs/rope_science_05.html — the algorithm and, more usefully, the shape of
  its correctness argument.
- **Technical (cost of the whole-document scan)**: Peng Lyu, *Text Buffer Reimplementation* (VS
  Code, 2018), https://code.visualstudio.com/blogs/2018/03/23/text-buffer-reimplementation — on
  which operations are actually hot and why a per-edit whole-document pass is the thing shipped
  editors refuse.

## Craft principle

Nielsen: "**0.1 second** is about the limit for having the user feel that the system is **reacting
instantaneously**, meaning that no special feedback is necessary except to display the result", and
"Normally, no special feedback is necessary during delays of more than 0.1 but less than 1.0 second,
but the user does lose the feeling of operating directly on the data." Typing is the one gesture in
a UI that is *always* held to the first limit: at 10 keys a second a 50 ms rebuild is half the
key's own budget, and the caret's own spring is set to half a button's press because "the caret
must keep up with typing" (`CARET_SPRING_FRACTION`). A cost that grows with the file breaks the
promise the editor already makes in its own docs: "20,000 lines cost what 20 do".

## How shipping games do it

Roblox Studio ships text wrapping as a per-user switch, off by default, and a Studio pane of tens
of thousands of lines is edited without a pause; the engine editor is the reference this crate's
wrap follows. Levien states the rule the shipped editors share: "Recomputing the whole thing on
every edit would take too long to meet my performance targets … The only viable approach is some
form of *incremental* computation."

## Model adopted

The unit is the **line**, not the document, because `code_wrap.md` already made the break a pure
function of one line: `break_columns(line, cols, tab)`. A line's row count is
`1 + breaks(line).len()` (0 when a fold hides it), so the map is a **prefix sum of row counts** plus
one boxed slice of break columns per line. A splice therefore costs:

1. **the lines it replaced** — recompute `break_columns` for the `inserted` lines only;
2. **the tail** — add the row-count delta to every later `first_row` entry. That is an integer add
   per line, not a character walk; measured at a few µs for 20,000 lines.

The deltas are the ones `CodeBuffer` already records for the fold map (`LineEdit::Splice { first,
removed, inserted }`, `LineEdit::Reset`), so no new change tracking is invented. `update_code_folds`
drains that log once per frame (it must, to follow the text); it now also hands the edits to the
wrap map through the editor, so one log has one reader and both maps follow the same delta.

**One function, two costs.** The map's value after an incremental step is identical to a fresh
build's — that is the property that makes this one path and not a second one. It is held by a test
that applies 400 random edits of every kind (a character, a newline, a delete, a replaced range, an
undo) to a wrapped buffer and compares every row lookup against a freshly built map. Where the
delta cannot be trusted — a `Reset` (a whole-text load, a replace-all, an edit from elsewhere), a
width or tab change, a fold opening or closing, or any splice whose arithmetic does not land on the
buffer's line count — the map is rebuilt from scratch. That is not a fallback mode: the answer is
the same either way, and the rebuild is the check that says so.

## Parameters

- A line shorter than `cols` bytes cannot break: skipped without a character walk (unchanged).
- The tail shift is one `usize` add per line after the edit; the rebuild is one character walk per
  line of the document. The two differ by the width of a line, about 100×.
- `LINE_EDIT_LOG` (4,096) still bounds the log; past it the buffer records `Reset` and the map
  rebuilds, so a pathological burst of edits degrades to the old cost, never to a wrong map.

## Validation targets

- Typing at the end of a 20,000-line, 100-column wrapped file costs the same frame as typing in a
  20-line one, within `PERF_BUDGET` (2×) — the crate's own rule for the editor
  (`typing_in_a_wrapped_20000_line_file_costs_what_typing_in_a_20_line_one_does`).
- Every row lookup after any sequence of random edits equals a fresh build's
  (`an_incremental_rebuild_answers_exactly_what_a_fresh_one_does`).
- The existing `code_editor`, `code_decor` and `code_navigation` suites pass unchanged: a wrap, a
  caret, a selection band, the minimap lane and the markdown ink are all read through the same map.
- With the component absent nothing changes: no wrap map is built at all.

## Deviations

- xi stops its forward scan when a computed break agrees with the old one; this map needs no such
  early stop, because it recomputes only the lines the splice replaced and never rescans the tail
  (the tail's break columns cannot change — a line's breaks depend on that line alone). The
  agreement walk is therefore not built.
- The tail is shifted rather than stored in a rope. A `Vec<usize>` shift is measurable at 20,000
  lines but is a small fraction of a frame; a balanced tree over 20,000 entries would add an
  allocation per edit to save microseconds. Design default, revisit if a 200,000-line pane is asked
  for.

## Pending index

- home-still holds no text-editor or line-breaking sources (`code-soft-wrap.md` recorded the same);
  the three technical sources above are the shipped editors' own write-ups, read directly.
- UAX #14 is still not read in full; the monospace rule does not need it.

## What great looks like

Typing into a 20,000-line note feels like typing into a 20-line one: the character appears on the
frame the key is pressed, the paragraph below reflows as the window narrows, and nothing about the
file's size can be felt in the hand. Resizing the window while a long file is open re-wraps once and
then costs nothing; scrolling a settled file costs nothing; and a file loaded, replaced or edited
from elsewhere is exactly what a fresh parse would have drawn.
