# Style sheets

A style sheet styles nodes by **class**, the way a web page's CSS does: put `StyleClass::of("card")` on a node, give the app a `StyleSheet` whose rules name `.card`, and the cascade writes the node's box and colours. Change the sheet and every classed node restyles that frame, with no respawn. The host owns the files: gooey reads no file and knows no format. With the `serde` feature the rules read from any serde format; this page shows YAML, the form a host's sheet is usually written in.

```rust
use bevy::prelude::*;
use bevy_gooey::*;

fn ui(mut commands: Commands) {
    commands.spawn((panel(), StyleClass::of("card card-quiet")));
}

fn load(mut commands: Commands) {
    let rules: StyleRules = serde_yaml_ng::from_str(r#"
".card": {padding: !Step 2, radius: !Step 1}
".card-quiet": {background: !Tone quiet}
".list .row:hover": {background: !Host row_hover}
"#).expect("the sheet reads");
    commands.insert_resource(StyleSheet { rules, revision: 1 });
}
```

## Classes

A class is lower-case letters, digits and `-`, starting with a letter (`chrome-chip`, `row-2`). `StyleClass::of` splits on whitespace and drops a name that is not one, with a warning: no selector could name it. Use a class on a node that has a `Role` (every gooey constructor gives one): the components the cascade writes are drawn by `apply_skin`, which styles only `Role` nodes, so a class on a bare `Node` is written and warned about once, and draws nothing.

## Selectors

| Form | Matches |
|---|---|
| `.row` | a node with the class `row` |
| `.row.quiet` | a node with both classes |
| `.row:hover` | a `.row` whose Bevy `Hovered` reads true |
| `.list .row` | a `.row` with a `.list` anywhere above it |
| `.bar > .chip` | a `.chip` whose parent is a `.bar` |
| `.a, .b` | either |

The states are `:hover` (Bevy's `Hovered(true)`: only the nodes that carry `Hovered`, which are the interactive ones, a button, a chip, a row with a pointer), `:pressed` (`Pressed`), `:selected` (`Selected`), `:checked` (`Checked`), `:disabled` (`InteractionDisabled`) and `:focus` (the `InputFocus` entity). The last part of a selector names a class. Anything else is refused with its column: `.Row` is "at column 2: a class starts with a lower-case letter, not `R`", `.row:wobble` is "at column 6: unknown state `wobble`".

**Specificity** is the selector's classes plus its states: `.row` is 1, `.list .row:hover` is 3. For each property the matching rule with the highest specificity wins, and a tie goes to the rule written later. A side of `padding`, `margin` or `border` is a property of its own, so `.row: {padding: !Step 1}` then `.row.tight: {padding: {top: Zero}}` keeps three sides of the first.

## Declarations

Every property is optional; a rule sets only what it names. Each lands in the component a host would otherwise spawn by hand:

| Property | Component | YAML |
|---|---|---|
| `padding`, `margin` | `Spacing` | `!Step 2`, `!Px 6`, `Zero`, or `{all, x, y, top, right, bottom, left}` (a side over its axis over `all`) |
| `gap` | `GapPx` | a `Space`, resolved under the skin |
| `width`, `height`, `min_width`, `min_height`, `max_width`, `max_height` | `Sizing` | an `Extent`: `!Px 120`, `!Percent 100`, `!Text 12`, `Auto`, `!Lines 2` |
| `clip` | `Sizing` | `true` |
| `border` | `Shape` | line weights, one number or a side map |
| `border_ink`, `radius`, `fill` | `Shape` | `Accent`; `!Step 1` or `Full`; a surface level |
| `background`, `color`, `border_color` | `InkOverride` | a `Paint` |
| `z` | Bevy's `GlobalZIndex` | a number |

A property the sheet does not know (`paddin`) is refused when the sheet reads. Text size is not a declaration: it is a role of the type scale.

A `Paint` is a skin colour (`!Skin Text`), a step of the surface ladder (`!Surface 3`), a named tone (`!Tone warning`), a colour the host published in `HostPaints` (`!Host panel_bg`) or a literal in sRGB hex, as CSS writes it (`!Color "#ff8000"`, `#rgb`, `#rgba` and `#rrggbbaa` too). It resolves whenever the sheet, the skin or `HostPaints` change, so a theme step repaints every styled node. A tone, a host paint or a surface step that is not there is a line in `StyleSheetProblems` (`.row:hover background: no host paint `row_hoverr``), and that property is left as the node's base, never guessed.

## The base, and code that writes a styled component

The first time a node gets a class, the cascade keeps what its spawn code set of `Spacing`, `Sizing`, `Shape`, `GapPx`, `InkOverride` and `GlobalZIndex` (and its `Node`). What it writes is always that base with the matching declarations over it, so deleting a rule gives the node back what it was spawned with. Taking `StyleClass` off does the same.

A component the matching rules do not declare stays the code's: the host may write a classed panel's width every frame and the cascade takes each value as the base, silently. A component a rule declares, changed by the host's code afterwards, is a fight between the code and the sheet: the cascade notices before its next write, takes the code's value as the node's new base and warns once for that node and component. `CascadeBase::decides::<InkOverride>()` says whether the sheet decides a component on a node now, so a host that rewrites the colours it stored (a theme change) can leave the sheet's alone.

## When it runs

`apply_style_sheet` runs in `GooeySystems::Cascade`, after `Navigate` and before `Style`, only while a `StyleSheet` resource exists. A host writes `StyleSheet` and `HostPaints` before that set. It restyles every classed node when the sheet, the skin or `HostPaints` change; otherwise only the nodes whose class, state or parent changed (and their classed descendants when a rule looks up the tree).

## Inspecting

`explain(&world, entity)` returns a `StyleExplain`: the node's classes, each matching rule (selector, `source`, specificity, declaration) in cascade order, and what they add up to. `StyleRule::source` is the host's to fill after reading (the file a rule came from), so an inspector can say `.chrome-chip — editor.yaml:41 (specificity 1)`.
