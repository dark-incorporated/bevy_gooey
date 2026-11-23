# A tag: `tag`, a read-only label whose colour names a category

Researched 2026-11-22 (iteration 108). No open issue asked for it; the owner's projects hand-roll it. Sanctuary
(`src/ui/widgets.rs`) builds `name_tag(theme, text, colour, node)` ("a key prompt, a speaker's name, a badge") and `badge(..)`;
the Cauldron editor (`bevy_dark_factory`) draws console level pills, a "N STALE" badge on a tab (`anim_watch.rs`
`paint_stale_badge`, which writes an *empty string* when there is nothing to say) and tag blocks. This crate has `chip`
(clickable) and `CornerBadge` (private to a side-tab strip), but no plain, quiet, coloured label a HUD or a tool can put
beside a name: "ELITE", "NEW", "WARN", "Rare", "2 STALE".

## Sources

1. IBM, *Carbon Design System: Tag, usage* (read 2026-11-22, https://carbondesignsystem.com/components/tag/usage/; craft):
   "Read-only tags: Tags that have no interactive functionality and are commonly used for categorizing and labeling."
   "Do use colors to help distinguish between different tag labels or categories for easier scanning." and the don't beside it:
   "Do not use the same color for every tag if they are supposed to be used as labels or categories." On a title too long:
   "Do not wrap overflow tag title text to multiple lines."; "Tag titles should describe the tag in a few words or be under 20
   characters when possible."
2. W3C, *Understanding SC 1.4.1 Use of Color* (WCAG 2.2, read 2026-11-22,
   https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html; technical): "Color is not used as the only visual means of
   conveying information, indicating an action, prompting a response, or distinguishing a visual element." A tag's colour
   is the category, its words are the name, so the colour is never the only cue; and *WCAG 2.2 SC 1.4.3 Contrast (Minimum)*,
   4.5:1, for the words on whatever fill a host gives.
3. Item rarity in *Diablo IV* (shipping game; the tiers Normal white, Magic blue, Rare yellow, Legendary orange, Unique gold,
   as listed by PCGamesN, https://www.pcgamesn.com/diablo-4/rarity-levels; the page was read through a search summary, so the
   colours are quoted from it and nothing more is claimed): a coloured name that is also a word ("Rare", "Legendary") on the
   tooltip, so the colour is the glance and the word is the fact.
4. This crate (`src/button_variant.rs`, `docs/research/meter-tone-segments.md`): `GooeySkin::tones`, `colour_named` and
   `ink_on` (the skin's own colours first, 7:1 on the fill, else the best of black and white), the tone model of `MeterTone`.
5. bevy_dark_factory `console.rs`, `anim_watch.rs`, Sanctuary `widgets.rs` (the owner's tools; shipping reference): what a
   tag has to do in a real screen: hide when it has nothing to say, never wrap, take a colour by category.

The home-still corpus has no paper on label pills (searches for "badge", "status label colour categorical" and "tag chips
categorical colour legibility" returned nothing on topic); the sources above were read directly.

## Craft principle

A tag is **a short word on a plate whose colour says which kind of thing it names**, and it is quiet: it takes no input, no
focus, and no width a sentence beside it needs.

- Colour distinguishes categories (source 1), and the word says it too (sources 2, 3), so a player who cannot tell two hues
  apart still reads "ELITE" and "NEW".
- A tag never wraps (source 1): a long name is cut at the plate, not folded into a second line that makes the row taller.
- Words on a plate are read at the contrast floor whatever the plate's colour (source 2): the ink is chosen from the fill, not
  by the host.

## How shipping games do it

A rarity colour behind a rarity word (source 3) is the whole idea, and a tool shows it as a level pill or a stale count
(source 5). A game's own categories ("Fire", "Boss", "Quest") are its own palette: they are data in the skin
(`GooeySkin::tones`), so a player's palette swap is one skin change.

## Model adopted

- **M1. `tag(label)`**: `Role::Tag` (a plain `Node`, `Pickable::IGNORE`, no `Button`, no `TabIndex`, no `Hovered`) holding one
  `Role::TagText` child (`Text`, `LineBreak::NoWrap`, `Pickable::IGNORE`). Its look is written only in `apply_skin`.
- **M2. `TagTone(&'static str)`**, a component beside `tag`: the plate takes the skin's tone of that name
  (`GooeySkin::tones`), else one of the skin's own colours (`danger`, `accent`, ...): `MeterTone`'s resolution
  (`meter_fill`). Unknown or none is the quiet plate (`surface_raised`, `text` words, `outline` frame), which is also the
  untoned tag.
- **M3. The ink is the skin's, chosen from the fill** (`ink_on`: panel, ground, text, black, white, first to reach 7:1,
  else the best), so a host's tone is never unreadable, and the frame is the plate's own colour so a toned tag is one solid
  shape (the line is laid out either way).
- **M4. A size no state changes**: padding half a `pad()` across and a quarter up and down, the skin's radius, the small step
  of the type scale (`type_scale[1]`), `flex_shrink` 0 and `overflow` clipped, so it never decides a row's height (a tag in a
  row of body text is no taller than the text) and a too-long label is cut, not folded.
- **M5. A tag with nothing to say is not drawn.** The host writes the label's `Text` and the one rule is a system that gives
  the plate `Display::None` (and gives it back) when its words are empty, so the stale-count case needs no `Shown` bookkeeping.
- **M6. Accessibility**: a tag is `Spec::Node` (a static label) whose name is its words; the child text is hidden from the tree
  by `names`, as a chip's.
- No new skin field, no preset change, no resource. `TagTone` is reflected.

## Parameters

| Name | Value | Why |
|---|---|---|
| padding across | `pad() * 0.5` | a word on a plate, not a command (a chip's is `pad()`) |
| padding up and down | a quarter of `pad()` | the plate hugs the line |
| text | `type_scale[1]` | one step under the body, so a tag beside body text is not taller than it |
| radius | `GooeySkin::radius` | the skin's corner |
| frame | `GooeySkin::border` snapped | laid out always; the plate's colour when toned |

## Validation targets

`tests/tag.rs`, laid out for real in a headless window, each test written to fail first:

1. A tag shows its words; it takes no input: it is `Pickable::IGNORE`, has no `TabIndex` and focus never lands on it.
2. An untoned tag is `surface_raised` with `text` words at 4.5:1; a toned one is the tone's colour with the ink `ink_on` gives.
3. The words are at least 4.5:1 on the plate for every colour a skin names, in all four presets (a check that can fail).
4. A tone the skin does not know falls back to the quiet plate; a skin change (a theme step) repaints it.
5. A label wider than its host is cut: the plate stays inside and the text is one line.
6. The tag's height is at most a body line's plus a line weight at 1x and 2x text.
7. An empty label takes no room and the neighbour moves; writing words brings it back, and nothing is written for 30 idle frames.
8. Accessibility: a tag is named by its words.

## Deviations

The Carbon guide truncates with an ellipsis and a tooltip with the full title; this crate has no ellipsis in Bevy 0.20's text
and a clipped plate is the available form (the host puts a `Tooltip` on the plate for a long name; a tooltip needs a focus
stop or a pointer, and a tag takes neither, so it is the host's call). Not built: sizes, icons and a dismiss button (a
dismissible tag is a `chip` with a glyph).

## Pending index

Nothing on label pills was found in the corpus; sources 1 and 2 are worth indexing.

## What great looks like

A name plate in a Diablo or Hades tooltip: "LEGENDARY" in one solid colour with its own ink, one line, hugging the word; three
of them in a row are three colours that never need a legend. Beside body text it is no taller than the text. When a counter
is zero, nothing is there.
