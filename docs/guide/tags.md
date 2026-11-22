# Tags

A name plate that says "ELITE", a "NEW" mark on a recipe, a level pill, a "2 STALE" count: a short word on a plate whose colour names a kind of thing. `tag(label)` is that, read-only.

```rust
fn plates(mut commands: Commands) {
    commands.spawn((
        row(0.5),
        children![
            label("Wolf"),
            (tag("ELITE"), TagTone("danger")),
            tag("NEW"),
        ],
    ));
}
```

- **The colour is the skin's.** `TagTone("danger")` names a tone of `GooeySkin::tones` or one of the skin's own colours (`accent`, `danger`, `warning`, `live`, `text_muted`, `outline`, ...). With no `TagTone`, or a name the skin does not know, the plate is the quiet one: `surface_raised` with the `outline` frame. A toned tag's frame is its own colour, so it reads as one solid shape.
- **The ink is picked for you.** `GooeySkin::tag_ink` takes whichever of black or white reads better on the plate (`GooeySkin::tag_fill` is the plate), so a host's own tone is never unreadable; the quiet plate's words are the skin's `text`. A test holds every preset against every named colour at 4.5:1.
- **Colour is never the only cue.** The words name the kind (WCAG 2.2 SC 1.4.1). A screen reader reads the tag as one label with its words as the value, once.
- **Empty words take no room.** `update_tags` hides a tag whose words are empty and shows it again when they are not, so `tag(format!("{n} STALE"))` can be rewritten to `""` without a `Shown`, a margin to undo or a gap left behind.
- **One line, cut at the host's edge.** The words never wrap; a tag wider than its host is clipped there. Bevy 0.20 draws no ellipsis.
- **No input.** A tag has no `Button`, no `TabIndex`, no hover; the pointer goes through it. A tag the player presses is a `chip`; a tally beside a name is a `count_text`.
- **Smaller than a label's line.** The tag is the skin's second type step with a fifth of a padding above and below, so it never makes its row taller than a `label` in it, at any text size.

There are no size variants, icons or a dismiss button: that is a `chip`.
