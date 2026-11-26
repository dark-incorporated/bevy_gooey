# One text, its own face

Researched 2026-11-25 for issue #118 (`grimoire-ui`, filed by `dark-incorporated/bevy_dark_factory`
as its #551 follow-up), the oldest open consumer issue at this revision.

## The defect

`apply_skin` is the one writer of every colour and size a widget draws, and it writes the face from
`GooeyFonts` at the same time (`src/skin.rs`, the `let text = match role` arm: `Role::Label`,
`Role::Heading`, the fallback and every other role answer `&look.fonts.body` or
`&look.fonts.heading`, and `restyle` writes `next.font = font.clone()`). A host that sets a
`TextFont` of its own on a text is written over the next time that text restyles — and a restyle is
not rare: a skin change, a scope change, a parent move, a state change on the owner, a target change
on the first frame all restyle. So a host cannot own a text's face, and the consumer runs a system
after `GooeySystems::Style` that re-applies the prose face to every prose text **every frame**. That
is a second writer of a component the crate claims, which is the same defect the crate's own guide
names for the divider thickness (`SplitThickness`) and the LED's glow: two writers, and the last one
wins.

`GooeyFonts` itself cannot be the answer: it is one resource holding `body`, `heading` and `mono`,
so setting `body` to the prose face turns every label, keycap and Inspector column proportional too.

## Sources

1. **Craft** — Carol Mertz, *Down to the Letter: The Importance of Typography in Video Games*
   (Game Developer, 2015-05-13). "Utilize as few fonts as is reasonable; one to three typefaces
   should be sufficient for standard needs, **each with its own consistent purpose**." And: "When
   setting larger blocks of text, or small but important words like user interface elements, focus
   on classic and easy-to-read type styles like sans-serif, serif, or monospace." A game interface
   is expected to carry more than one face, each attached to a *kind* of text (a display face for
   titles, a body face for prose, a monospace for code, keys and ids) — which is exactly the split
   the consumer needs, and it is per text, not per screen.
2. **Accessibility** — Xbox Accessibility Guideline 101, *Text display*, "Font style": "Include at
   least one sans serif type-face option" and "If stylistic fonts are used (for example, blood
   dripping off font), provide a non-stylized font option." A player must be able to ask for a
   plainer face than the game's display face, which means the face is a property of a text (and its
   kind), not of the whole interface: one skin-wide choice cannot satisfy both the display heading
   and the option that replaces it.
3. **Technical** — CSS `font-family` (MDN, CSS Fonts 4): "specifies a prioritized list of one or
   more font family names … for the selected element", and it is **inherited**. The web's model is
   therefore per element, with inheritance as a convenience, and every toolkit that draws text on
   that model (including Bevy 0.20: `TextFont { font: FontSource }` is a component on the text
   entity, and `TextSpan` children carry their own) lets the element's own face win. The crate is
   the only writer that takes that choice away.
4. **Shipping-game feel** — the consumer, `dark-incorporated/bevy_dark_factory` (the Cauldron
   editor): its `docs/ui/vision.md` "Prose and code" splits the interface into prose in Fira Sans
   and code, keys, ids and paths in Fira Mono, and `crates/bevy_dark_factory/src/prose.rs` runs
   `set_the_prose_face` after gooey's style pass to keep the prose face on. It is the shipping
   evidence: a real tool needed two faces on one screen and paid a frame-time system for it.

## Model adopted

A **component on the text**: `TextFace(pub FontSource)`, honoured by `apply_skin`.

- **Why a component and not a skin field or a resource.** The face belongs to one text: the crate
  cannot know which of a screen's labels is prose. `InkOverride` is the crate's own precedent for
  "the host pins one part of a node's look, the skin still writes the rest": a face is pinned, the
  ink, the size, the weight and the slant stay the skin's and the host's.
- **Why `FontSource` and not the issue's `Handle<Font>`.** `GooeyFonts` already holds `FontSource`,
  so a host writes `TextFace(handle.into())` or `TextFace::new(handle)` for a loaded face and
  `TextFace::new(FontSource::family("Fira Sans"))` for a system one — one type, no second
  convention. `FontSource` is not `Copy`, so it cannot ride the skin's `State`; `restyle` reads it
  from the query and clones a handle only on a restyle, never per frame.
- **No cascade.** `TextFace` applies to the node it is on, like `InkOverride`, and not to its
  children: a cascade would make a panel's face reach a markdown page's spans and a code editor's
  glyphs, which are exactly the texts that must not take it (the editor measures one column from
  `GooeyFonts::mono`). A host that wants a subtree in one face puts it on the texts it draws.
- **What still restyles it.** `Changed<TextFace>` and `RemovedComponents<TextFace>` join the
  restyle triggers, so writing one on a drawn text changes it in that frame and taking it off gives
  the role's face back in that frame.
- **What it does not touch.** Weight, slant, width, features and variations are kept: `restyle`
  clones the text's `TextFont` and writes only `font` and `font_size`, so a Markdown strong run
  stays bold under a custom face. The size stays the role's, snapped to whole device pixels, so a
  custom face cannot break the grid (`tests/text_grid.rs`).

Rejected: a `fonts` map keyed by name on `GooeyFonts` plus a `Role` per face (a second name table,
and a role is a widget's job, not a face's); a `SkinScope`-wide face (a preview of another skin is
a whole look, not a font, and `GooeyFonts` is global by design); a `Text` sub-bundle that carries
its own face (`label("x").face(f)`, which cannot reach a text the crate spawns itself, such as a
button's).

**Assumed decision (owner may override):** the component is `TextFace(FontSource)`, per node and no
cascade, with the role still choosing ink and size.

## Acceptance

1. `TextFace(f)` on a text draws in `f` on the frame it is spawned, after any number of restyles
   (state changes, a parent move, a scope change) and after a skin change — proved on measured
   glyph advances, not on the field, so a face that is written but not drawn fails.
2. A text without one keeps `GooeyFonts::body` (or `heading`) for its role exactly as before: the
   gallery's stills are pixel-identical, and keycaps and code stay in their own faces.
3. Removing `TextFace` gives the role's face back in the same frame.
4. Taking the consumer's shape (a prose label beside a code text) costs one component and no
   system of its own.
