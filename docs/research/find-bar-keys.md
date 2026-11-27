# The find bar's keys are the bar's

Researched 2026-11-27 for issue #128 (a `bug` filed by `dark-incorporated/bevy_dark_factory` as its
#612, `consumer:bevy_grimoire`), at bevy_gooey v0.62.0. In the shipped find bar, `Enter` in the query
**steps to the next match and inserts a line break over it**: the bar's text field is a child of the
editor, its `Enter` is deliberately let through (`text.rs`, "Bevy's handler lets Enter bubble"), and
the editor's key observer answers every `FocusedInput<KeyboardInput>` that reaches it without asking
whether the key started in the editor itself. Measured by the consumer: `Enter` ×4 in a query with
three matches read `1 of 3`, `2 of 3`, `3 of 3`, `1 of 3` and left the file with three new lines.

## Sources

1. **Shipping editor**: Visual Studio Code, `src/vs/editor/contrib/find/browser/findWidget.ts` (read at
   `main`, 2026-11-27): the find input's key handler is its own (`_onFindInputKeyDown`,
   `this._findInput.onKeyDown(...)`) and `Enter` there changes the search string and steps the match
   (`this._state.change({ searchString: ... }, true)` under `!findOnType`), while every key that
   handler does not take is consumed by the widget rather than the editor text.
   `findController.ts` binds `KeyCode.Escape` (weight `EditorContrib + 5`, `kbExpr:
   EditorContextKeys.focus`) to `FindCommand` with handler `x => x.closeFindWidget()`: **Escape closes
   the bar and gives the editor the keyboard back**.
2. **Craft / accessibility**: W3C, *Understanding SC 2.1.2: No Keyboard Trap* (WCAG 2.2) —
   https://www.w3.org/WAI/WCAG22/Understanding/no-keyboard-trap.html. "If keyboard focus can be moved
   to a component ... then focus can be moved away from that component using only a keyboard
   interface", and Escape is named as a standard exit method. A bar that kept Tab would trap the
   player inside it.
3. **Technical / Bevy 0.20**: `bevy_input_focus`'s `FocusedInput` is an entity event with
   `propagate = WindowTraversal, auto_propagate`: it starts at the focused entity and bubbles to each
   ancestor with an observer, up to the window, where `handle_tab_navigation` reads it. Two facts
   follow: an observer on an ancestor sees a descendant's key (which is the bug), and
   `On::original_event_target()` is the focused entity, so one line tells a key that started in the
   editor from one that bubbled up from its bar.

## Craft principle

> Keep users informed about what is going on (Nielsen, heuristic 1, *Visibility of system status*) —
> and never change the document except for the edit the player asked for.

Every shipped editor separates the chrome's keys from the text's: a query field is a text field, so
its Enter submits the query, never the document. The bar's count is the status of a keyboard walk;
a walk that edits the file as it goes is the one failure the count cannot survive.

## How shipping games do it

VS Code's find widget owns its keys (source 1) and `Escape` closes it. Its `Enter` also keeps the
widget open. The widget's Tab moves within the widget's own controls; the bar never traps. bevy_gooey
already lets Tab go to Bevy's global tab navigation (`text.rs`, "Tab is Bevy's navigation"), and that
stays right: the bar's exit is Escape, and Tab moves on out of it.

## Model adopted

- **The editor takes only the keys that started in it.** `code_on_key` returns unless
  `event.event_target() == event.original_event_target()`. One rule for every widget a host may put
  inside an editor (the bar's field, its toggles, its buttons, a host's own toolbar), not a list.
- **The bar stops every key it does not itself need at the panel**, as it already stops the pointer:
  an observer on the `Role::CodeFind` panel with `propagate(false)`. The exceptions are the keys
  something else already owns: `Tab` (Bevy's tab navigation, WCAG 2.1.2) and `Escape` (which the bar
  turns into `FindAction::Close` and also stops).
- **Escape in the bar closes it and returns focus to the editor** (`FindAction::Close` already set
  it), the same path the Close button takes.
- A key the field's own handler consumed (a character, Backspace, the arrows, Home and End) never
  reaches the panel; only Enter, Tab and Escape do. The rule is written for all of them anyway, so a
  new key Bevy starts letting through cannot leak into the editor.

## Validation targets

- `Enter` ×3 in a query of three matches: `current()` walks `1, 2, 0` and the buffer's text is
  byte-identical after every press (`tests/code_navigation.rs::enter_in_the_find_query_steps_the_match_and_leaves_the_text_alone`).
- `Escape`: `CodeFind::open == false`, `InputFocus` is the editor, and the next typed character lands
  in the file (`…::escape_in_the_bar_closes_it_and_hands_the_keyboard_back_to_the_editor`).
- A character typed in the query edits the query and not the file
  (`…::a_character_typed_in_the_query_never_reaches_the_editor`).
- Mutation checks: dropping the panel's key observer fails the first two; dropping the editor's
  origin guard fails the first three.

## Deviations

- No public API change; `FindAction` and the panel stay `pub(crate)`.
- Tab is deliberately not stopped: a bar that swallowed it would be a keyboard trap, which the issue
  does not ask for and WCAG 2.1.2 forbids.
