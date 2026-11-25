# A drawn part is born with its geometry, not laid out a frame later

Researched 2026-11-25 for issue #117 (a `bug`, reported by `dark-incorporated/bevy_dark_factory`
as its #540). The minimap of a `code_editor` shows its ground and its view band and **no line bars
at all** on the frame a file is opened for reading; one typed character later the same lane shows 89
bars at 89 distinct `top`s with real widths. The editor's own colour census over the 124 px lane at
y=240..800: ground 56,168 px, band 11,036 px, **bar pixels 0**.

## Sources

1. **Craft**: Shneiderman, *The Eyes Have It: A Task by Data Type Taxonomy for Information
   Visualizations* (T.R. 96-66, ISR; `10.1109/vl.1996.545307`) — the Visual Information-Seeking
   Mantra, and `overview` as one of the seven tasks.
2. **Shipping game**: Lightbown, *Designing the User Experience of Game Development Tools* (2015),
   ch. 2 — frequent iteration is what makes a tool's interface trustworthy; a tool that misreports
   its own state costs the iteration it was supposed to save.
3. **Technical**: Bevy 0.20.1 `bevy_ecs-0.20.1/src/system/commands/mod.rs:1250` — "Most `Commands`
   (and thereby `EntityCommands`) are deferred … the command is added to a 'command queue'. The
   command queue is applied later when the `ApplyDeferred` system runs." Plus this crate's own
   `docs/research/first-frame-skin.md` (#94), which found the same shape one component up: a node
   spawned in `Update` is styled and queried for layout a frame before it has a real target.

## Craft principle

> A useful starting point for designing advanced graphical user interfaces is the Visual
> Information-Seeking Mantra: **Overview first**, zoom and filter, then details-on-demand.

The overview is a *first* task, not a later one. A file's overview that is empty until the player
types is not an overview: the whole value of the minimap is knowing where you are before you move.

Lightbown, on the tool case:

> More frequent iterations allow developers to adapt the user experience faster, and with more
> confidence.

A tool whose own panel lies about its contents for one frame injects a false signal into exactly the
loop that quote is about: the person reading the panel cannot tell "this file is short" from "this
panel has not drawn yet".

## How shipping games do it

A minimap in a shipped game (a level's overview, a map screen) is drawn from data on the frame the
screen opens, because it is the screen's *first* job; nothing in it waits for the player's first
input. Editors follow the same rule for an outline or overview pane, and the reason is visible in
Bevy's own deferred-command rule above: an engine that applies spawns at the end of the schedule
makes "spawn now, fill in next frame" look correct in a test that updates twice and wrong on the one
frame a person sees.

## The defect, measured in this crate

`src/code_overlay.rs::draw_navigation` spawned the minimap's root and its band and then `return`ed;
the one frame that had a lane to lay out was the one frame that wrote nothing into it. And the bar
layout sat behind a redraw key holding `buffer.revision()`, so an unedited file was never asked
again. Both halves are the same mistake: `Commands` defers, so a node spawned this frame cannot be
found by the query this frame writes through.

## Model adopted

**Every part is born with the geometry it is due.** The spawn arm no longer returns: the band and
each bar are spawned with their own `top`, `left`, `width` and `display` already worked out from the
buffer, so the frame that creates them draws them. The pool's own write pass then skips the slots it
just created (they are not in the query yet) and rewrites only the parts it held from before.

**The lane's measured width joins the redraw key.** On the spawning frame the lane has no measured
box, so the bars are laid out at the skin's estimate; the frame the layout arrives rewrites them at
the real width. Without this the estimate would be kept for ever, which is why the issue's second
measurement (bars 6, 8, 9, 11 px wide) only appeared after an edit.

**A part the query cannot see leaves the key unwritten.** If a pooled entity is gone — despawned by
a host, or by the editor's own rebuild — the pass does not record the key, so the next frame restores
the part instead of believing it drawn. This is the same guard `first-frame-skin.md` needed in a
different place, and it is one bool, not a second path.

The minimap stays the overlay it is (`Role::CodeMinimap` absolute at the area's right edge): making
it a sibling that takes width out of the editor's row is a layout change the issue does not ask for,
and it would move every editor's text box.

## Parameters

- `MINIMAP_BAR_PX` 1.5, pitch `2 ×` it (unchanged).
- `MINIMAP_COLS` 100, the widest line the lane can show (unchanged).
- The bar's `left` is 2 px and its column is `(width − 4) / MINIMAP_COLS`, floored at 0.5 (unchanged).

## Validation targets

- A 400-line buffer in a `code_editor` with `CodeMinimap`, spawned and settled with **no input and
  no focus**: more than 20 bars at distinct `top`s, at least one wider than 1 px, and the band's top
  at the lane's top with a height covering the visible rows (`tests/code_navigation.rs`).
- Scrolling the same editor with no edit moves the band down its lane.
- `tests/code_navigation.rs`'s two existing minimap tests (a press and a drag scroll) stay green.

## Deviations

- None from the issue. The issue's acceptance asks for the fix in `draw_navigation` and no host-side
  call; this is that, and the test is gooey's own fixture app.

## Pending index

- No new document. The craft source was already in the corpus (`10.1109/vl.1996.545307`); the game
  tool source (`lightbown_designing_ux_of_game_dev_tools_2015`) was already indexed.

## What great looks like

A file opened for reading shows, on its first frame and with no pointer near it: a ladder of bars
whose lengths read the file's shape, the part in view as a band sitting exactly where the view is,
and diagnostic marks on the lines that carry them. Nothing on that panel changes between the first
frame and the tenth, because there was nothing left to fill in.
