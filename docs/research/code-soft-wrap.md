# Soft wrap and Markdown ink for `code_editor`

Written 2026-11-22 for issue #114, asked by bevy_dark_factory's markdown pane: "prose is read, not scanned"; a `code_editor` whose long line scrolls sideways is a bad place to write a paragraph, and a Markdown file drawn in one ink loses its headings. The editor's first note (`code-editor.md`) recorded "no soft wrap" as an assumed decision; this note reverses it for editors that ask.

## Sources

- **Craft**: WCAG 2.2, Understanding SC 1.4.10 Reflow (https://www.w3.org/WAI/WCAG22/Understanding/reflow.html): content must be usable with "Vertical scrolling content at a width equivalent to 320 CSS pixels", with the exception of "parts of the content which require two-dimensional layout for usage or meaning" (images, maps, data tables). A paragraph of prose is not one of those; a column-aligned table in source is.
- **Shipping game / shipping editors**: Roblox Studio, Script Editor docs (https://create.roblox.com/docs/studio/script-editor): "Text wrapping: If enabled, longer lines of code wrap to the next line." A per-user switch, off by default. Visual Studio Code, Basic editing (https://code.visualstudio.com/docs/editing/codebasics): "By default, `editor.wordWrap` is `off` but if you set it to `on`, text will wrap on the editor's viewport width"; "commands such as Cut and Copy apply to the whole wrapped line. Triple-click selects the whole wrapped line. Pressing Home twice moves the cursor to the very beginning of the line. Pressing End twice moves the cursor to the very end of the line."; a `logicalLine` argument ignores the wraps when adding cursors.
- **Technical**: Raph Levien, "Rope science, part 5 - incremental word wrapping" (xi-editor, https://xi-editor.io/docs/rope_science_05.html): "word wrapping can be defined as a scan through the document's 'candidate breaks,' and breaking either when the document has a hard break, or where there's a soft break and adding the word ... would cause the total line width to overflow"; "The result of word wrapping ... is a pure function of its input ... every paragraph is independent"; in the ASCII/monospace case "ligatures and kerning aren't in effect and can't change the result of a word wrap operation" and "in a monospace font, per-character is completely fine" for width. Unicode UAX #14 is the principled break source; xi stores the result so scrolling is O(log n) and jank-free.
- **Technical (Markdown ink)**: CommonMark spec 0.31 (https://spec.commonmark.org/): ATX headings, fenced code, emphasis, code spans, links and block quotes are all decided from the line's first characters or delimiter runs, so a line-scanner with one bit of carried state (inside a fence) is enough for an editor's ink; the crate's own `markdown` module already parses the full document for the view.
- Corpus (home-still, `hs distill search`): two queries ("soft wrap text editor line breaking", "line length readability") returned nothing on point (an unrelated boundary-labelling paper and software books). Noted, not padded.

## Craft principle

Text meant to be read wraps at the width of the room the reader has; a reader should never scroll sideways through a sentence. Code meant to be scanned keeps its columns. So wrap is a **choice per editor**, off for code and on for prose, and a wrapped editor must keep every promise of an unwrapped one: the caret is where the eye sees it, Up and Down move one line of what is drawn, and a selection is a shape, not a surprise.

## How shipping games do it

Roblox Studio and VS Code both default to off and offer one switch. Both keep the **logical line** as the unit of Cut, Copy, line numbers and triple-click, and the **visual row** as the unit of Up, Down and the page. The line number is shown on the first row only. Home goes to the start of the visual row first and the start of the line on a second press, and End likewise.

## Model adopted

- A **per-editor component** `CodeSoftWrap` (not a field of the global `CodeEditorSettings`): a markdown pane and a code pane live in one app. Absent is the old behaviour byte for byte.
- An engine-free `code_wrap` module: `Wrap::new(lines, cols, tab_width)` returns each line's break columns (display columns, the unit `display_col` already uses); `RowMap` joins it to the fold map so one value answers "which row is this line's first", "which line and column span is row r" and "which row holds this caret".
- **Break rule** on display columns: break after a run of blanks or after a hyphen; trailing blanks hang over the edge rather than start a row; a word longer than the row is cut at a character (a path, a URL). One column per character, a tab to the next stop, a wide glyph one column: the same measure as everywhere else in the editor, so the monospace case is exact.
- **Wrapped mode has no horizontal scroll**: `first_col` is 0, every row draws one segment of its line, only the first segment of a line carries its number, the selection band and every overlay (squiggles, find marks, brackets, external edits) are clipped to the segment.
- **Keys and pointer**: Up and Down move by visual row keeping the display column within the row; Home and End go to the segment's ends first, the line's on a second press (VS Code's rule; smart Home keeps its own first-non-blank step on the first segment); the pointer maps row to (line, segment start) and x to a column inside the segment, clamped to the segment's last character so a click past the end of a row lands before the break, not on the next row.
- The cache is keyed on buffer revision, fold, `cols` and tab width and rebuilt whole; the scan is O(text) and measured (`Parameters`). Like xi, the result is stored so scrolling costs nothing, but the incremental delta xi needs for 1.5 MB lines is not built: a line is wrapped whole and the rebuild is bounded by the text.
- **Markdown ink**: `markdown_highlights(text, revision) -> Highlights` is an engine-free line scanner (headings, fences, block quotes, list markers, code spans, emphasis, links) that fills the existing `Highlights` and so draws through the existing `Role::CodeSyntax` rows. A `CodeMarkdown` marker on the editor has a system keep the editor's `Highlights` current when the value's revision moves. One path: `Highlights` stays the only way a row gets ink; the marker is a host convenience that computes it.

## Parameters

- `cols` = floor((area width - 2 x pad) / advance), at least 1.
- Break set: blank, tab, `-` (after). No UAX #14: a code or prose pane in a monospace face; CJK breaks between any two characters is the open gap.
- Rebuild budget: wrapping 20,000 lines of 100 characters under 5 ms on the host (measured and recorded in the log).
- Markdown scanner: block level per line plus inline spans; a span never crosses a line; a fence carries state to its closing fence or the end.

## Validation targets

- A 300-character line in a 40-column editor is 8 rows or more; every row holds at most `cols` display columns (blanks hanging excepted).
- Caret column arithmetic round trip: for every column c of a wrapped line, `row_at(line, c)` and the segment window contain c, and a pointer press at the centre of a character lands on that character.
- Up from the second row of a line lands in the first row at the same column offset; Down from the last row of a line lands on the next line's first row.
- Home, End, Home twice, End twice.
- Existing `code_editor`, `code_decor`, `code_navigation` tests pass unchanged; with the component absent nothing about layout, rows or writes differs (a settled editor writes nothing).
- `markdown_highlights`: a heading, a fence, a code span, emphasis and a link each get a role; two or more distinct inks drawn on a sample document.
- Contrast: the roles reuse `GooeySkin::syntax`, already held at 7:1 (4.5:1 when text is lower).

## Deviations

- The issue asked for `soft_wrap` and `syntax_highlighting` on `CodeEditorSettings`. They are components here: the settings resource is global, so one bool would force a code pane and a prose pane in the same app to agree, and the acceptance is stated per `code_editor`. Assumed decision, owner may override.
- Highlighting is Markdown only. A general tokeniser per language is a dependency and a product of its own; the host that wants Rhai or YAML keeps writing `Highlights`.
- `LineBreak::WordBoundary` of Bevy's layout is not used: the editor draws one pooled row per visual row of an already-measured grid, so the break is decided in `code_wrap` where the caret arithmetic lives, and the row's `TextLayout` stays `NoWrap`. Two engines deciding the same break would disagree by a column and put the caret in the wrong place.

## Pending index

- home-still has no text-editor or line-breaking sources; re-scan when it does.
- UAX #14 and Knuth-Plass are not read in full; the monospace rule above does not need them.

## What great looks like

A paragraph typed into the pane reflows as the window narrows, with no sideways scroll at any width; the caret never jumps a row sideways; the gutter shows one number per paragraph; a heading is bold-bright, a fence and a code span are another ink, a link a third; resizing the window moves rows and nothing flickers; and the editor with the component off is the editor it was.
