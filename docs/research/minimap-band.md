# The band is as tall as the text in view, wherever the view stands

Researched 2026-11-27 for issue #123 (a `bug` filed by `dark-incorporated/bevy_dark_factory` as its
#540, `consumer:bevy_grimoire`), at bevy_gooey v0.61.0. The minimap's view band is correct at the top
of a file and **grows as the view scrolls**: in a 20-line view of the editor's Files workspace, after
`scroll_to_line(30)` the band covered **44 lines**.

The cause is one arithmetic slip in `draw_navigation`:

```rust
let drawn = (view.first_row + view.needed).min(view.rows).max(1);
let bottom_line = lines
    .row_of(fold.line_at((view.first_row + drawn - 1).min(view.rows - 1)))
```

`drawn` is already an **absolute** row index — one past the last row drawn — and the next line adds
`first_row` to it a second time, so the band's bottom is `2 · first_row + needed - 1`. At
`first_row = 0` the two expressions agree, which is why a file opened at the top looks right.

## Sources

1. **Craft**: Nielsen, J., *10 Usability Heuristics for User Interface Design* (NN/g, 1994, reviewed
   2024), heuristic 1, *Visibility of system status* —
   https://www.nngroup.com/articles/ten-usability-heuristics/. A viewport indicator is status: it
   says how much of the file the player is holding. A band that lengthens as the player scrolls says
   the opposite of what happened.
2. **Shipping game**: Lightbown, *Designing the User Experience of Game Development Tools* (2015),
   ch. 5, on feedback — "Feedback is all about how the tool communicates with the user. Examples of
   feedback include **what the tool is doing now**, what just happened, and how much time is left in
   a particular process" (home-still `lightbown_designing_ux_of_game_dev_tools_2015`, p. 92). Where
   the view is and how much it holds is the most-read feedback a code pane gives.
3. **Technical / shipping editor**: Visual Studio Code, `src/vs/editor/browser/viewParts/minimap/
   minimap.ts` (read at `main`, 2026-11-27), which computes the slider from the viewport and the
   content and states the rule in its own comment: "In this case, **the height of the slider directly
   reflects the visible line count**" — `sliderHeight = Math.floor(viewportLineCount *
   minimapLineHeight / pixelRatio)`, and in its editor-height mode
   `Math.max(1, Math.floor(viewportHeight * viewportHeight / logicalScrollHeight))`, which is the
   visible share of the content and does not move with the scroll.

## Craft principle

> The design should always keep users informed about what is going on, through appropriate feedback
> within a reasonable amount of time. (Nielsen, heuristic 1)

and, on the same subject, Lightbown:

> Feedback is all about how the tool communicates with the user. Examples of feedback include what
> the tool is doing now, what just happened, and how much time is left in a particular process.

The band answers two questions at once — where am I, and how much am I seeing — and its **height is
the second answer**. A height that depends on the scroll position makes the second answer a function
of the first: at line 30 of a 400-line file a 20-line view claims to hold 44 lines, so the player
reads the file as shorter than it is and the band as a scrollbar that cannot be trusted.

## How shipping games do it

VS Code's minimap is the shipped form of this widget and its slider is computed from the viewport's
line count and the content's length (source 3): the slider's height is a **share of the content**, so
scrolling moves it and never stretches it. The one case it does size by the visible line count — a
viewport containing whitespace gaps — still sizes by the *lines in view*, never by their position.
An outline pane in a shipped tool (a level's overview, a file tree's scroll rail) follows the same
rule: position moves, extent does not.

## Model adopted

- The band's **bottom is the last row drawn** — `(first_row + rows_in_view - 1).min(rows - 1)`, both
  absolute — mapped to its line, exactly as the top is the line of `first_row`. The double add goes.
- The rows in view are the viewport's own height in rows (`ceil(area.y / line_px)`, at least one),
  **not the pool's count**: the pool carries `POOL_SPARE_ROWS` = 2 rows past the viewport so a scroll
  never shows a gap, and a band that counted them would claim two more lines than are on screen at
  the top as well.
- A line is still one slot however many display rows it takes (the model of
  `docs/research/minimap-wrap.md`): `FoldMap::row_of` maps the bottom row's line, so a wrapped view
  bands the lines it shows and not the rows.

## Parameters

- `View::rows_in_view`: `(size.y / line_px).ceil().max(1)`, written by `update_code_editors` beside
  `needed`, which is this plus `POOL_SPARE_ROWS`. One quantity, one place: the pool and the band
  cannot disagree about the viewport.
- The band's height: `(in_view as f32 * pitch).min(count as f32 * pitch).max(pitch)` — unchanged, in
  lane slots of `MINIMAP_BAR_PX * 2` = 3 px.

## Validation targets

- An unwrapped 400-line file in a 480 px editor: the band's height in 3 px slots equals the rows the
  view holds at `first_row = 0` **and** after `scroll_to_line(30)` and `scroll_to_line(200)`, within
  one slot (`tests/code_navigation.rs::the_minimap_band_is_as_tall_as_the_lines_in_view_wherever_the_view_stands`).
- The issue's own measurement, as a failing-first case: a 20-line view scrolled to line 30 bands
  under 0.6 of what the doubled expression produced (44 lines for a 20-line view).
- The wrapped band still counts **lines**, not rows, and stays under 0.6 of the unwrapped band of the
  same file (`under_soft_wrap_the_minimap_band_is_as_tall_as_the_lines_in_view`, existing).
- The band still starts at the lane's top on an unscrolled editor and travels down it
  (`an_opened_editor_lays_its_minimap_out_with_no_input_at_all`, existing).
- Nothing written while the band does not move: `set_px` writes only on a difference.

## Deviations

- No public API change: `View` is `pub(crate)`, and the band's height was never documented as a
  number.
- A partially visible row at the top counts as a line in view (its line is where the band starts), and
  the band may then be one slot shorter than `CodeEditor::visible_lines()`'s `ceil`; the validation
  target allows one slot for exactly this reason.

## Pending index

- home-still has no text-editor or minimap literature (`docs/research/minimap-wrap.md` recorded the
  same gap, and `hs distill search "scrollbar thumb proportional…"` returned unrelated software books
  and UI-design benchmarks this pass). The three sources above were read directly: NN/g's page,
  home-still's Lightbown stem, and VS Code's `minimap.ts` at `main`.
- Nothing new to file under the corpus: `corpus.md`'s menus, HUD and accessibility stems are the
  nearest, and none of them is about overview panes.

## What great looks like

Scroll a long file and the band travels down the lane **without changing height**: a 20-line view is
the same 20 slots' worth at line 0, at line 30, and at the end. At the end of the file the band's foot
sits on the lane's last bar. A wrapped file's band counts lines, not the rows they wrapped into, so
narrowing the window lengthens neither the lane nor the band.
