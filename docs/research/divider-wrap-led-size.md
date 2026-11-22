# Divider thickness, wrapped table cells, LED label size (#108, #109, #110)

Three requests from the editor, one shape: a **widget's own part carries a number the host needs to
choose**, and the host cannot write it from outside because `apply_skin` rewrites the part on every
restyle (two writers for one value). Each is answered by a component the host puts on the widget's
root; the crate carries it to the part and `apply_skin` draws it.

## Sources

- **Technical, WCAG 2.2 SC 2.5.8 Target Size (Minimum)**,
  <https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html>: "Ensure targets meet a
  minimum size or have sufficient spacing around them." The exceptions are *Spacing*, *Equivalent*
  ("another control that can achieve the underlying function that does meet the requirements"),
  *Inline*, *User Agent Control* and *Essential*. A split divider is also a drag handle whose
  function is available from the keyboard (arrows, Home, End, Enter on the focused divider), which
  is the *Equivalent* exception.
- **Shipping app, VS Code `workbench.sash.size`**, release notes 1.46,
  <https://code.visualstudio.com/updates/v1_46>: "You can now use the `workbench.sash.size` setting
  to configure the feedback area size in pixels of the dragging area in between views/editors. Set it
  to a larger value if you feel it's hard to resize views using the mouse." The default is 4 px, the
  range 4 to 20: a desktop editor's seams are thin by default and the thickness is one number the
  user (here the host) owns.
- **Technical, CSS Text, `overflow-wrap`**,
  <https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/overflow-wrap>: "The
  `overflow-wrap` CSS property applies to text, setting whether the browser should insert line breaks
  within an otherwise unbreakable string to prevent text from overflowing its line box." and "In
  contrast to `word-break`, `overflow-wrap` will only create a break if an entire word cannot be
  placed on its own line without overflowing." A cell that reflows breaks at words and, failing that,
  inside one; a table of ids stays on one line by asking for it (`white-space: nowrap`).
- **Craft, WCAG 2.2 SC 1.4.10 Reflow**, <https://www.w3.org/WAI/WCAG22/Understanding/reflow.html>:
  content must not need scrolling in two dimensions at a narrow width; a cell that draws past its
  pane is the failure this request names (the editor's pane at its narrowest, 29 px of text past the
  edge, then 418 px at 240% scale).
- **Shipping app, VS Code status bar and activity readouts**: status text sits a step under the menu
  and title text so readouts do not read as a second menu. [INFERENCE from use, no quote fetched;
  see Pending index.]
- **This crate**: `GooeySkin::type_scale` (five steps), `IconSize(step)` (the one existing
  component that picks a step for a part), `Sizing` (a size written by `apply_skin` on any node with
  a `Role`), `docs/research/led.md` (the LED's size is `LED_TEXT_SIZES` body sizes).

## Craft principle

A host that can name a number should not have to fight the widget for it. The widget keeps one
writer (`apply_skin`); the host's number is **data the writer reads**, on the widget's root, and it
survives a hover, a press and a focus move because it is read in the same place as the colours. The
default is the widget's own number, so a host that sets nothing sees nothing change.

## How shipping games do it

- **Sash (divider) thickness** is a setting, not a constant: VS Code ships 4 px and lets the user
  go to 20; the hit area and the drawn seam are the same box and "feedback area" is the name for
  it. Dock UIs in game editors (Unity, Unreal's tab well) draw a 1 to 4 px seam with a wider hit
  zone.
- **Table cells** wrap in every spreadsheet-like tool that is not a code grid (Notion, Linear,
  GitHub's tables) and clip with an ellipsis in dense lists (Explorer, file trees). Which one is a
  per-column decision; a row grows with its tallest cell when it wraps.
- **Status readouts** are drawn a step under the labels around them.

## Model adopted

- `SplitThickness(f32)` (logical px) on a `split()` root. `update_splits` writes the divider's
  least size along the split's axis from it through the divider's `Sizing`, which `apply_skin`
  already applies after the role's own arm, so no per-frame host system and no second writer. The
  grip stays centred because the divider centres its child. Absent, the divider is
  `MIN_TARGET_PX` as before. The drag maths read the divider's laid-out size, so they follow.
- `Column::wrap()` on a table column: that column's cell text takes `LineBreak::WordOrCharacter`
  (it reflows to the column, breaking inside a word only when the word alone is wider than the cell)
  and a row whose cells wrap is **at least** one pitch tall and grows with its tallest cell. A
  table with a wrapping column binds **every** row (no pool): variable heights cannot be windowed
  from `row * pitch`, and the scroll's length and the cursor row's place are read from the laid-out
  rows instead. A column that does not ask stays single-line, so an id column never becomes a
  column of orphan letters.
- `LedSize(usize)` on an `led()` root: a `type_scale` step (`IconSize`'s unit). The dot's side is
  `type_size(step) * LED_TEXT_SIZES`, so the dot and the label stay in proportion, and the label
  and the word are drawn at `type_scale[step]`. `update_leds` carries it to the three parts, as
  the glow's size needs the same number.

## Parameters

| Name | Unit | Default | Range |
|---|---|---|---|
| `SplitThickness` | logical px | none (`MIN_TARGET_PX` = 24) | finite, floored at 1 |
| `Column::wrap` | flag | false | |
| `LedSize` | `type_scale` step | none (body `text_size`) | step past the end is the last |

## Validation targets

- A divider's laid-out thickness equals the setting at UI scale 1.0 and 2.4, survives hover, press
  and a focus move, and a 100 px drag still moves the fraction by 100 over the room.
- A wrapping column's cell text stays inside its cell at a pane narrower than the words, the row is
  taller than one pitch, and the single-line column beside it is unchanged; the last row is
  reachable by scrolling and the cursor row is revealed at its laid-out place.
- An LED's label, word and dot follow the step through a state change and a skin change; an LED with
  nothing set is byte-for-byte what it was.

## Deviations

- **A divider thinner than 24 px is under WCAG 2.5.8's minimum target.** Assumed: the host asked
  for it, the default is unchanged, the divider has a keyboard equivalent (arrows, Home, End, Enter,
  the D-pad), which is the *Equivalent* exception. `problems()` does not check a host's
  component.
- **A wrapping table is not windowed.** Assumed: the editor's case is a history list of tens to
  hundreds of lines. A table of ten thousand rows keeps single-line columns and its pool.

## Pending index

- home-still was not queried this pass; the sources above were read from the web. A shipping
  game's LED/readout size and an HIG statement of status-text size are not fetched.
- A windowed table with measured variable row heights (a prefix-sum of laid-out heights) would let a
  wrapping table of ten thousand rows exist; not built.

## What great looks like

A seam you can drag that is only as thick as the owner wants to see; a history line that wraps
under its words instead of running into the divider; a status chip that reads as a quiet
readout beside the menu titles, with its dot still in proportion to its words. Nothing else
about any of the three changed.
