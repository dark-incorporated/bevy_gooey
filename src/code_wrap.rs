//! Soft wrap for the code editor (#114, `docs/research/code-soft-wrap.md`): which display columns
//! a long line breaks at, and the one map between the editor's rows and the buffer's lines that
//! folds and wrapping share.
//!
//! Engine-free: nothing here knows a Bevy type. A line is cut by display columns, one column a
//! character and a tab to the next stop (the same columns [`display_col`] counts), after a blank
//! or a hyphen when it can be and in the middle of a word longer than the row when it cannot.
//! Blanks hang over the edge of a row rather than starting the next one, so a row never begins
//! with the space that ended the last.
//!
//! A hanging editor (`CodeWrapIndent::Same`, #126) stands a wrapped line's rows after the first
//! in by the line's [`hang_of`], so they start under the line's first word; those rows hold that
//! many columns fewer, and every place that draws or hit-tests a row reads the hang through
//! [`RowMap::origin_x`] or [`RowMap::hang`].

use std::sync::Arc;

use crate::code_buffer::{CodeBuffer, LINE_EDIT_LOG, LineEdit};
use crate::code_fold::FoldMap;

/// The display columns where a line of text starts a new row, ascending, each past the one before
/// and past 0; empty when the line fits one row.
///
/// `cols` is how many columns a row holds (at least 1), `hang` how many of them the rows after the
/// first stand in (those rows hold `cols - hang`, never under 1) and `tab` the tab width.
pub(crate) fn break_columns(line: &str, cols: usize, hang: usize, tab: usize) -> Vec<usize> {
    let cols = cols.max(1);
    let rest = cols.saturating_sub(hang).max(1);
    let tab = tab.max(1);
    let mut breaks = Vec::new();
    let mut start = 0;
    // How many columns the row being filled holds: the whole width for the first, less the hang
    // after it.
    let mut holds = cols;
    // The latest column after `start` a row may end at: after a blank run or a hyphen.
    let mut room = 0;
    let mut at = 0;
    for c in line.chars() {
        let blank = c == ' ' || c == '\t';
        let width = if c == '\t' { tab - at % tab } else { 1 };
        // A blank hangs over the edge; anything else that does not fit goes to the next row.
        while !blank && at + width > start + holds {
            let cut = if room > start && room <= at { room } else { at };
            breaks.push(cut);
            start = cut;
            holds = rest;
        }
        at += width;
        if blank || c == '-' {
            room = at;
        }
    }
    breaks
}

/// How many display columns a wrapped line's rows after the first stand in so they start under
/// its first word (#126): the line's leading blanks, then any Markdown quote marks (`>`, each with
/// the blanks after it), then one list marker with a blank after it (`-`, `*`, `+`, or up to nine
/// digits and `.` or `)`) and a task box after that (`[ ]`, `[x]`). At most half of `cols`, so a
/// deeply indented line still has room to wrap; a tab counts to its next stop.
pub(crate) fn hang_of(line: &str, cols: usize, tab: usize) -> usize {
    let tab = tab.max(1);
    let bytes = line.as_bytes();
    let blank = |at: usize| matches!(bytes.get(at), Some(b' ' | b'\t'));
    // Steps over the blanks from `at`, counting their columns onto `col`.
    let blanks = |at: &mut usize, col: &mut usize| {
        while let Some(&b) = bytes.get(*at) {
            match b {
                b' ' => *col += 1,
                b'\t' => *col += tab - *col % tab,
                _ => break,
            }
            *at += 1;
        }
    };
    let (mut at, mut col) = (0, 0);
    blanks(&mut at, &mut col);
    while bytes.get(at) == Some(&b'>') {
        at += 1;
        col += 1;
        blanks(&mut at, &mut col);
    }
    let marker = match bytes.get(at) {
        Some(b'-' | b'*' | b'+') => 1,
        Some(b'0'..=b'9') => {
            let digits = bytes[at..]
                .iter()
                .take_while(|b| b.is_ascii_digit())
                .count();
            match bytes.get(at + digits) {
                Some(b'.' | b')') if digits <= 9 => digits + 1,
                _ => 0,
            }
        }
        _ => 0,
    };
    if marker > 0 && blank(at + marker) {
        at += marker;
        col += marker;
        blanks(&mut at, &mut col);
        let task = bytes.get(at..at + 3);
        if matches!(task, Some(b"[ ]" | b"[x]" | b"[X]")) && blank(at + 3) {
            at += 3;
            col += 3;
            blanks(&mut at, &mut col);
        }
    }
    col.min(cols / 2)
}

/// Where every line breaks, and the row each line starts on.
///
/// The expensive half — the character walk that decides a line's break columns — is kept per line,
/// so an edit re-walks only the lines it replaced and the lines after them only move by an integer
/// (`docs/research/code-wrap-incremental.md`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Wrap {
    /// The row width this was built for.
    cols: usize,
    /// The tab width this was built for.
    tab: usize,
    /// Whether a wrapped line's later rows hang under its first word ([`hang_of`]).
    hanging: bool,
    /// The buffer revision whose text these breaks are for.
    revision: u64,
    /// The first row of each line, and the row count at the end; a line a fold hides has none.
    first_row: Vec<usize>,
    /// Where each line breaks and how far its later rows stand in.
    breaks: Vec<LineWrap>,
}

/// How one line is wrapped: the default for a line that fits one row.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct LineWrap {
    /// The columns the line breaks at, empty for a line that fits one row.
    cuts: Box<[usize]>,
    /// How many columns the rows after the first stand in; 0 for a line that fits.
    hang: usize,
}

impl Wrap {
    fn build(buffer: &CodeBuffer, fold: &FoldMap, cols: usize, hanging: bool) -> Self {
        let tab = buffer.tab_width();
        let lines = buffer.len_lines();
        let mut breaks = Vec::with_capacity(lines);
        for line in 0..lines {
            breaks.push(breaks_of(buffer.line(line), cols, tab, hanging));
        }
        let mut first_row = Vec::with_capacity(lines + 1);
        let mut rows = 0;
        for (line, wrapped) in breaks.iter().enumerate() {
            first_row.push(rows);
            if !fold.is_hidden(line) {
                rows += 1 + wrapped.cuts.len();
            }
        }
        first_row.push(rows);
        Self {
            cols,
            tab,
            hanging,
            revision: buffer.revision(),
            first_row,
            breaks,
        }
    }

    fn rows(&self) -> usize {
        self.first_row.last().copied().unwrap_or(0)
    }

    /// Lays the row sums out again from the breaks this holds: the folds moved under the text, so
    /// a line that now shows takes rows it did not, and one that a fold took away takes none. An
    /// integer add per line, with no character walked.
    fn relay(&mut self, fold: &FoldMap) {
        let mut rows = 0;
        let mut first_row = Vec::with_capacity(self.breaks.len() + 1);
        for (line, wrapped) in self.breaks.iter().enumerate() {
            first_row.push(rows);
            if !fold.is_hidden(line) {
                rows += 1 + wrapped.cuts.len();
            }
        }
        first_row.push(rows);
        self.first_row = first_row;
    }

    /// Follows what `edits` did to the text: the lines an edit replaced are wrapped again, and the
    /// rows after them move by what their count became. Returns whether the map is a map of
    /// `buffer` now; `false` means it could not follow (a whole-text load or replace-all, a line
    /// count that does not add up) and the caller builds afresh.
    fn follow(&mut self, edits: &[LineEdit], buffer: &CodeBuffer, fold: &FoldMap) -> bool {
        for edit in edits {
            match *edit {
                LineEdit::Reset => return false,
                LineEdit::Splice {
                    first,
                    removed,
                    inserted,
                } => {
                    let lines = self.breaks.len();
                    if first > lines || first.checked_add(removed).is_none_or(|end| end > lines) {
                        return false;
                    }
                    let fresh: Vec<LineWrap> = (first..first + inserted)
                        .map(|line| breaks_of(buffer.line(line), self.cols, self.tab, self.hanging))
                        .collect();
                    let mut rows = vec![self.first_row[first]];
                    for (at, wrapped) in (first..first + inserted).zip(fresh.iter()) {
                        let last = *rows.last().unwrap_or(&0);
                        rows.push(last + self.rows_at_or(at, &wrapped.cuts, fold));
                    }
                    let old_end = first + removed;
                    let new_end = first + inserted;
                    let old_rows = self.first_row[old_end];
                    let new_rows = *rows.last().unwrap_or(&old_rows);
                    self.breaks.splice(first..old_end, fresh);
                    self.first_row
                        .splice(first + 1..=old_end, rows.into_iter().skip(1));
                    let delta = new_rows as i64 - old_rows as i64;
                    if delta != 0 {
                        for row in &mut self.first_row[new_end + 1..] {
                            *row = (*row as i64 + delta).max(0) as usize;
                        }
                    }
                }
            }
        }
        self.revision = buffer.revision();
        self.breaks.len() == buffer.len_lines() && self.first_row.len() == self.breaks.len() + 1
    }

    /// The rows a line the map does not hold yet takes: the `cuts` being spliced in, or 0 hidden.
    fn rows_at_or(&self, line: usize, cuts: &[usize], fold: &FoldMap) -> usize {
        if fold.is_hidden(line) {
            0
        } else {
            1 + cuts.len()
        }
    }
}

/// How `line` wraps: no cuts when it fits, so a short line never walks its characters, and a hang
/// only for a line that breaks.
fn breaks_of(line: &str, cols: usize, tab: usize, hanging: bool) -> LineWrap {
    if line.len() <= cols {
        return LineWrap::default();
    }
    let hang = if hanging { hang_of(line, cols, tab) } else { 0 };
    let cuts = break_columns(line, cols, hang, tab);
    LineWrap {
        hang: if cuts.is_empty() { 0 } else { hang },
        cuts: cuts.into_boxed_slice(),
    }
}

/// The map between the editor's rows and the buffer's lines: a closed fold takes lines away and a
/// wrapping editor adds rows. Without wrapping it is its [`FoldMap`] and nothing else, which it
/// dereferences to; with wrapping, the row methods ([`row_of`](Self::row_of),
/// [`line_at`](Self::line_at), [`rows`](Self::rows)) count visual rows.
///
/// A map made for text that has since changed is stale for a frame (the editor's keys run before
/// the rebuild); every method answers from what it holds and never reads past it.
#[derive(Clone, Debug, Default)]
pub(crate) struct RowMap {
    fold: FoldMap,
    wrap: Option<Arc<Wrap>>,
    /// What the text did since the map was last synced, so the next `sync` can re-wrap only the
    /// lines an edit replaced.
    edits: Vec<LineEdit>,
    /// Whether the folds moved since the wrap was laid out.
    folds_moved: bool,
}

impl std::ops::Deref for RowMap {
    type Target = FoldMap;

    fn deref(&self) -> &FoldMap {
        &self.fold
    }
}

impl RowMap {
    /// Takes the folds a host or the player closed. The map itself catches up in `sync`, which
    /// only has to move the row sums, not walk the text again.
    pub(crate) fn set_fold(&mut self, fold: FoldMap) {
        if self.fold != fold {
            self.fold = fold;
            self.folds_moved = true;
        }
    }

    /// Takes what an edit did to the line numbers, so the next `sync` re-wraps the lines it
    /// replaced and shifts the rows after them instead of walking the whole file. A log too long
    /// to keep is given up for the rebuild it implies.
    pub(crate) fn follow_edits(&mut self, edits: &[LineEdit]) {
        self.edits.extend_from_slice(edits);
        if self.edits.len() > LINE_EDIT_LOG {
            self.edits.clear();
            self.edits.push(LineEdit::Reset);
        }
    }

    /// Makes the map answer for `buffer` in rows `cols` wide, or without wrapping at `None`; with
    /// `hanging`, a wrapped line's rows after the first stand in by its [`hang_of`].
    ///
    /// The character walk is paid only for lines the text replaced since the last `sync`; the
    /// lines after them move by an integer, and a fold that opened or closed moves the row sums
    /// alone. Anything the deltas cannot describe (a whole-text load, a width or hang change)
    /// rebuilds.
    pub(crate) fn sync(&mut self, cols: Option<usize>, hanging: bool, buffer: &CodeBuffer) {
        let Some(cols) = cols.map(|cols| cols.max(1)) else {
            self.wrap = None;
            self.edits.clear();
            self.folds_moved = false;
            return;
        };
        let edits = std::mem::take(&mut self.edits);
        let folds_moved = std::mem::take(&mut self.folds_moved);
        let tab = buffer.tab_width();
        let same = |wrap: &Wrap| wrap.cols == cols && wrap.tab == tab && wrap.hanging == hanging;
        let settled = !folds_moved
            && edits.is_empty()
            && self
                .wrap
                .as_ref()
                .is_some_and(|wrap| same(wrap) && wrap.revision == buffer.revision());
        if settled {
            return;
        }
        let reusable = self
            .wrap
            .take()
            .filter(|wrap| same(wrap))
            .map(Arc::unwrap_or_clone);
        self.wrap = Some(Arc::new(match reusable {
            Some(mut wrap) => {
                // The lines an edit replaced are wrapped again; a fold that opened or closed
                // moves the row sums, which is an add per line and no character walked. One
                // splice is what a keystroke, a paste or a delete makes; a frame that made
                // several (a replace-all, an undo group, an edit from the host) is rebuilt,
                // because each of their line numbers is the one it saw before the next.
                let followed = match edits.as_slice() {
                    [] => true,
                    [LineEdit::Splice { .. }] => wrap.follow(&edits, buffer, &self.fold),
                    // Even a load of the same number of lines is not the same breaks.
                    _ => false,
                };
                if !followed || wrap.breaks.len() != buffer.len_lines() {
                    Wrap::build(buffer, &self.fold, cols, hanging)
                } else {
                    if folds_moved {
                        wrap.relay(&self.fold);
                    }
                    wrap.revision = buffer.revision();
                    wrap
                }
            }
            None => Wrap::build(buffer, &self.fold, cols, hanging),
        }));
    }

    /// The map counted in **lines that show**, not display rows: what the minimap's lane is an
    /// outline of. A wrapped line is one line here however many rows it takes; the row methods
    /// above count the rows.
    pub(crate) fn lines(&self) -> &FoldMap {
        &self.fold
    }

    /// Whether lines are wrapped into rows.
    pub(crate) fn wraps(&self) -> bool {
        self.wrap.is_some()
    }

    /// How many rows `lines` lines are drawn on.
    pub(crate) fn rows(&self, lines: usize) -> usize {
        match &self.wrap {
            Some(wrap) if wrap.breaks.len() == lines => wrap.rows(),
            _ => self.fold.rows(lines),
        }
    }

    /// The row a line starts on. A hidden line is on the row of the header that hides it.
    pub(crate) fn row_of(&self, line: usize) -> usize {
        match &self.wrap {
            Some(wrap) => {
                let line = self.fold.header_of(line).unwrap_or(line);
                wrap.first_row
                    .get(line)
                    .copied()
                    .unwrap_or_else(|| wrap.rows() + (line - wrap.breaks.len()))
            }
            None => self.fold.row_of(line),
        }
    }

    /// The line drawn on `row`.
    pub(crate) fn line_at(&self, row: usize) -> usize {
        match &self.wrap {
            Some(wrap) if row < wrap.rows() => wrap.first_row.partition_point(|&r| r <= row) - 1,
            Some(wrap) => wrap.breaks.len() + (row - wrap.rows()),
            None => self.fold.line_at(row),
        }
    }

    /// How many rows `line` is drawn on: 1 unwrapped, 0 hidden.
    pub(crate) fn rows_of(&self, line: usize) -> usize {
        if self.fold.is_hidden(line) {
            return 0;
        }
        1 + self.breaks_of(line).map_or(0, <[usize]>::len)
    }

    /// The row display column `col` of `line` is on.
    pub(crate) fn row_at(&self, line: usize, col: usize) -> usize {
        let first = self.row_of(line);
        match self.breaks_of(line) {
            Some(breaks) => first + breaks.partition_point(|&at| at <= col),
            None => first,
        }
    }

    /// The columns a row shows of its line: `from..to`, `to` `None` for the line's last row, and
    /// the line. `None` without wrapping, where every row shows the editor's own window.
    pub(crate) fn window(&self, row: usize) -> Option<(usize, usize, Option<usize>)> {
        self.piece(row).map(|(line, from, to, _)| (line, from, to))
    }

    /// How many columns `row` stands in from the text's left edge: its line's hang on a wrapped
    /// line's later rows (`hanging` in [`sync`](Self::sync)), 0 on a line's first row and
    /// without wrapping. A press that lands in it is on the row's first column.
    pub(crate) fn hang(&self, row: usize) -> usize {
        self.piece(row).map_or(0, |(.., hang)| hang)
    }

    /// [`window`](Self::window) and [`hang`](Self::hang) of `row` in one walk.
    fn piece(&self, row: usize) -> Option<(usize, usize, Option<usize>, usize)> {
        let wrap = self.wrap.as_ref()?;
        let line = self.line_at(row);
        let wrapped = wrap.breaks.get(line)?;
        let first = wrap.first_row.get(line).copied()?;
        let k = row.checked_sub(first)?;
        let (from, hang) = if k == 0 {
            (0, 0)
        } else {
            (*wrapped.cuts.get(k - 1)?, wrapped.hang)
        };
        Some((line, from, wrapped.cuts.get(k).copied(), hang))
    }

    /// The columns of `row` the text is cut to: the row's own piece of its line when wrapping, else
    /// `first_col..first_col + cols`, and where the line `row` draws is.
    pub(crate) fn span(&self, row: usize, first_col: usize, cols: usize) -> (usize, usize, usize) {
        match self.window(row) {
            Some((line, from, to)) => (line, from, to.unwrap_or(usize::MAX / 2)),
            None => (self.line_at(row), first_col, first_col + cols),
        }
    }

    /// How far the text of `row` is shifted left in px: the editor's scroll without wrapping, the
    /// row's first column less its [`hang`](Self::hang) with it. Every place that draws onto a
    /// row or hit-tests it reads this, so the text, the caret, the selection, the find bands and
    /// the squiggles move together.
    pub(crate) fn origin_x(&self, row: usize, scrolled_x: f32, advance: f32) -> f32 {
        self.piece(row).map_or(scrolled_x, |(_, from, _, hang)| {
            (from as f32 - hang as f32) * advance
        })
    }

    /// Where `line` breaks, `None` without wrapping or past what the map holds.
    pub(crate) fn breaks_of(&self, line: usize) -> Option<&[usize]> {
        self.wrap
            .as_ref()?
            .breaks
            .get(line)
            .filter(|_| !self.fold.is_hidden(line))
            .map(|wrapped| &*wrapped.cuts)
    }

    /// The first display column of the row display column `col` of `line` is on.
    pub(crate) fn row_start(&self, line: usize, col: usize) -> usize {
        self.breaks_of(line).map_or(0, |breaks| {
            breaks
                .partition_point(|&at| at <= col)
                .checked_sub(1)
                .map_or(0, |k| breaks[k])
        })
    }

    /// The display column just past the last character of the row display column `col` of `line`
    /// is on that is not a blank, for the End key; at least the row's start.
    pub(crate) fn row_text_end(&self, buffer: &CodeBuffer, line: usize, col: usize) -> usize {
        let from = self.row_start(line, col);
        let to = self
            .breaks_of(line)
            .and_then(|breaks| breaks.iter().copied().find(|&at| at > col))
            .unwrap_or(usize::MAX);
        let tab = buffer.tab_width();
        let mut at = 0;
        let mut end = from;
        for c in buffer.line(line).chars() {
            if at >= to {
                break;
            }
            let width = if c == '\t' { tab - at % tab } else { 1 };
            at += width;
            if at > from && c != ' ' && c != '\t' {
                end = at;
            }
        }
        end
    }

    /// Whether `col` is on the last row of `line`.
    pub(crate) fn is_last_row(&self, line: usize, col: usize) -> bool {
        self.breaks_of(line)
            .is_none_or(|breaks| breaks.last().is_none_or(|&at| at <= col))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pieces(line: &str, cols: usize) -> Vec<String> {
        let cuts = break_columns(line, cols, 0, 4);
        let chars: Vec<char> = line.chars().collect();
        let mut out = Vec::new();
        let mut from = 0;
        for cut in cuts.iter().copied().chain(std::iter::once(chars.len())) {
            out.push(chars[from..cut].iter().collect());
            from = cut;
        }
        out
    }

    #[test]
    fn a_line_that_fits_does_not_break() {
        assert!(break_columns("hello world", 11, 0, 4).is_empty());
        assert!(break_columns("", 1, 0, 4).is_empty());
    }

    #[test]
    fn a_line_breaks_after_the_last_blank_that_leaves_the_word_whole() {
        assert_eq!(
            pieces("the quick brown fox jumps", 10),
            ["the quick ", "brown fox ", "jumps"]
        );
    }

    #[test]
    fn a_word_longer_than_a_row_is_cut_in_the_word() {
        assert_eq!(pieces("abcdefghijklmnop", 6), ["abcdef", "ghijkl", "mnop"]);
        // After the short word's blank, the long one starts a row and is cut there.
        assert_eq!(pieces("ab cdefghijkl", 5), ["ab ", "cdefg", "hijkl"]);
    }

    #[test]
    fn a_hyphen_may_end_a_row() {
        assert_eq!(pieces("well-known fact", 8), ["well-", "known ", "fact"]);
    }

    #[test]
    fn blanks_hang_over_the_edge_and_never_start_a_row() {
        // Five blanks past a four-column row stay on the first row.
        assert_eq!(pieces("ab        cd", 4), ["ab        ", "cd"]);
    }

    #[test]
    fn a_tab_counts_to_its_next_stop() {
        // The tab spans columns 1..4, so "xy" would end at 6 and does not fit 5 columns.
        assert_eq!(break_columns("x\txy", 5, 0, 4), [4]);
    }

    #[test]
    fn every_row_but_the_last_is_within_the_width_or_all_blank_hang() {
        let text = "lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod";
        for cols in 1..30 {
            let cuts = break_columns(text, cols, 0, 4);
            let mut start = 0;
            for cut in cuts.iter().copied().chain(std::iter::once(text.len())) {
                assert!(cut > start, "{cols} wide: a row of nothing at {start}");
                let row = &text[start..cut];
                assert!(
                    row.trim_end().len() <= cols,
                    "{cols} wide: {row:?} is wider than the row"
                );
                start = cut;
            }
        }
    }

    // -- #126: a wrapped line's later rows hang under its first word --

    /// The rows of `line` at `cols` with its hang, each later row led by the blanks it stands in.
    fn hung(line: &str, cols: usize) -> Vec<String> {
        let hang = hang_of(line, cols, 4);
        let cuts = break_columns(line, cols, hang, 4);
        let mut out = Vec::new();
        let mut from = 0;
        for cut in cuts.iter().copied().chain(std::iter::once(line.len())) {
            let lead = if from == 0 { 0 } else { hang };
            out.push(format!("{}{}", " ".repeat(lead), &line[from..cut]));
            from = cut;
        }
        out
    }

    #[test]
    fn the_hang_is_the_indent_and_the_marker_before_the_first_word() {
        assert_eq!(hang_of("plain words", 40, 4), 0);
        assert_eq!(hang_of("- item", 40, 4), 2);
        assert_eq!(hang_of("* item", 40, 4), 2);
        assert_eq!(hang_of("+ item", 40, 4), 2);
        assert_eq!(hang_of("  - nested", 40, 4), 4);
        assert_eq!(hang_of("1. first", 40, 4), 3);
        assert_eq!(hang_of("12) twelfth", 40, 4), 4);
        assert_eq!(hang_of("> quoted", 40, 4), 2);
        assert_eq!(hang_of("> > - deep", 40, 4), 6);
        assert_eq!(hang_of("- [ ] task", 40, 4), 6);
        assert_eq!(hang_of("- [x] done", 40, 4), 6);
        assert_eq!(hang_of("    key: a long value", 40, 4), 4);
        assert_eq!(hang_of("\tvalue", 40, 4), 4, "a tab counts to its stop");
        assert_eq!(hang_of(" \tvalue", 40, 4), 4);
        // A marker counts only with a blank after it: these are words.
        assert_eq!(hang_of("-flag and more", 40, 4), 0);
        assert_eq!(hang_of("*emphasis* here", 40, 4), 0);
        assert_eq!(hang_of("3.14 is a number", 40, 4), 0);
        assert_eq!(hang_of("1234567890. too many digits", 40, 4), 0);
        assert_eq!(hang_of("- [y] not a box", 40, 4), 2);
    }

    #[test]
    fn the_hang_is_at_most_half_a_row_so_a_deep_line_still_wraps() {
        assert_eq!(hang_of(&format!("{}deep", " ".repeat(30)), 20, 4), 10);
        assert_eq!(hang_of("- item", 3, 4), 1);
        assert_eq!(hang_of("- item", 1, 4), 0);
        let deep = format!("{}alpha bravo charlie delta echo", " ".repeat(30));
        for row in hung(&deep, 20).iter().skip(1) {
            assert!(row.trim_end().len() <= 20, "{row:?} is wider than the row");
        }
    }

    #[test]
    fn a_list_item_continues_under_its_first_word() {
        assert_eq!(
            hung("- alpha bravo charlie delta echo foxtrot", 16),
            ["- alpha bravo ", "  charlie delta ", "  echo foxtrot"]
        );
        assert_eq!(
            hung("> quoted words that run on", 12),
            ["> quoted ", "  words that ", "  run on"]
        );
        // An unindented paragraph is cut as it was without a hang.
        let plain = "the quick brown fox jumps";
        assert_eq!(hang_of(plain, 10, 4), 0);
        assert_eq!(hung(plain, 10), pieces(plain, 10));
    }

    #[test]
    fn every_hung_row_is_within_the_width() {
        let text = "  - lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod";
        for cols in 1..40 {
            let rows = hung(text, cols);
            for (k, row) in rows.iter().enumerate() {
                assert!(
                    row.trim_end().len() <= cols || row.trim().chars().count() == 1,
                    "{cols} wide: row {k} {row:?} is wider than the row"
                );
            }
            let joined: String = rows
                .iter()
                .enumerate()
                .map(|(k, row)| {
                    if k == 0 {
                        row.as_str()
                    } else {
                        &row[hang_of(text, cols, 4)..]
                    }
                })
                .collect();
            assert_eq!(joined, text, "{cols} wide: the rows are the line");
        }
    }

    fn buffer(text: &str) -> CodeBuffer {
        CodeBuffer::new(text)
    }

    #[test]
    fn rows_and_lines_agree_with_wrapping_and_a_fold() {
        let long = "aaaa bbbb cccc dddd";
        let text = format!("one\n{long}\nfolded\nhidden\n{long}\nlast");
        let buffer = buffer(&text);
        let mut map = RowMap::default();
        map.set_fold(FoldMap::new(std::iter::once(2..4)));
        map.sync(Some(10), false, &buffer);
        assert!(map.wraps());
        // Lines: one(1 row) long(2) folded(1) hidden(0) long(2) last(1).
        assert_eq!(map.rows(buffer.len_lines()), 7);
        assert_eq!(map.row_of(0), 0);
        assert_eq!(map.row_of(1), 1);
        assert_eq!(map.row_of(2), 3);
        assert_eq!(map.row_of(3), 3, "a hidden line is on its header's row");
        assert_eq!(map.row_of(4), 4);
        assert_eq!(map.row_of(5), 6);
        for row in 0..7 {
            let line = map.line_at(row);
            assert!(row >= map.row_of(line), "row {row} line {line}");
        }
        assert_eq!(map.line_at(2), 1);
        assert_eq!(map.line_at(4), 4);
        assert_eq!(map.line_at(5), 4);
        assert_eq!(map.line_at(6), 5);
        // A column on the second piece of a long line is on its second row.
        assert_eq!(map.row_at(1, 0), 1);
        assert_eq!(map.row_at(1, 9), 1);
        assert_eq!(map.row_at(1, 10), 2);
        assert_eq!(map.window(2), Some((1, 10, None)));
        assert_eq!(map.window(1), Some((1, 0, Some(10))));
        assert_eq!(map.row_start(1, 12), 10);
        assert!(map.is_last_row(1, 12));
        assert!(!map.is_last_row(1, 3));
    }

    #[test]
    fn without_wrapping_it_is_the_fold_map() {
        let buffer = buffer("a\nb\nc\nd");
        let mut map = RowMap::default();
        map.set_fold(FoldMap::new(std::iter::once(0..2)));
        map.sync(None, false, &buffer);
        assert!(!map.wraps());
        // The header's range 0..2 hides line 1 only.
        assert_eq!(map.rows(4), 3);
        assert_eq!(map.row_of(1), 0);
        assert_eq!(map.row_of(2), 1);
        assert_eq!(map.line_at(1), 2);
        assert_eq!(map.window(0), None);
        assert_eq!(map.span(0, 5, 7), (0, 5, 12));
        assert_eq!(map.row_at(2, 9), 1);
    }

    #[test]
    fn a_stale_map_never_reads_past_what_it_holds() {
        let mut buffer = buffer("aaaa bbbb cccc dddd\nx");
        let mut map = RowMap::default();
        map.sync(Some(10), false, &buffer);
        buffer.load("1\n2\n3\n4\n5\n6\n7\n8");
        // The text has more lines than the map: it falls back to the folds' count.
        assert_eq!(map.rows(8), 8);
        let _ = map.row_of(7);
        let _ = map.line_at(40);
        assert_eq!(map.window(99), None);
        assert_eq!(map.row_at(7, 3), map.row_of(7));
        map.sync(Some(10), false, &buffer);
        assert_eq!(map.rows(8), 8);
    }

    #[test]
    fn a_rebuild_happens_only_for_a_change() {
        let buffer = buffer("aaaa bbbb cccc dddd");
        let mut map = RowMap::default();
        map.sync(Some(10), false, &buffer);
        let first = map.wrap.clone();
        map.sync(Some(10), false, &buffer);
        assert!(Arc::ptr_eq(
            first.as_ref().unwrap(),
            map.wrap.as_ref().unwrap()
        ));
        map.sync(Some(8), false, &buffer);
        assert!(!Arc::ptr_eq(
            first.as_ref().unwrap(),
            map.wrap.as_ref().unwrap()
        ));
    }

    #[test]
    fn a_hanging_map_moves_each_later_row_in_by_its_lines_hang() {
        let text = "- alpha bravo charlie delta echo foxtrot\nplain words that run past the edge";
        let buffer = buffer(text);
        let mut map = RowMap::default();
        map.sync(Some(16), true, &buffer);
        // "- alpha bravo " | "charlie delta " | "echo foxtrot", then "plain words " | ...
        assert_eq!(map.window(1), Some((0, 14, Some(28))));
        assert_eq!(map.hang(0), 0, "a line's first row stands at the edge");
        assert_eq!(map.hang(1), 2);
        assert_eq!(map.hang(2), 2);
        assert_eq!(map.hang(3), 0);
        assert_eq!(map.hang(4), 0, "an unindented line does not hang");
        let advance = 10.0;
        assert_eq!(map.origin_x(0, 99.0, advance), 0.0);
        assert_eq!(map.origin_x(1, 99.0, advance), (14.0 - 2.0) * advance);
        assert_eq!(
            map.origin_x(4, 99.0, advance),
            map.window(4).unwrap().1 as f32 * advance
        );
        // Without the hang the same rows stand at the edge.
        map.sync(Some(16), false, &buffer);
        assert_eq!(map.hang(1), 0);
        assert_eq!(map.window(1), Some((0, 14, Some(28))));
        assert_eq!(map.origin_x(1, 99.0, advance), 14.0 * advance);
    }

    #[test]
    fn switching_the_hang_rebuilds_the_map() {
        let buffer = buffer("- aaaa bbbb cccc dddd eeee");
        let mut map = RowMap::default();
        map.sync(Some(10), false, &buffer);
        let first = map.wrap.clone();
        map.sync(Some(10), true, &buffer);
        assert!(!Arc::ptr_eq(
            first.as_ref().unwrap(),
            map.wrap.as_ref().unwrap()
        ));
        assert_eq!(map.hang(1), 2);
    }

    /// A map that followed a stream of edits answers exactly what a map built from the text
    /// afterwards answers: same breaks per line, same first row per line, same line per row.
    #[test]
    fn an_incremental_rebuild_answers_exactly_what_a_fresh_one_does() {
        follows_like_a_fresh_build(
            false,
            &["x", "long-word-here", "a b c d e f g", "", "tab\there"],
        );
    }

    /// The same with hanging rows, whose hang moves as an edit adds or takes a marker or indent.
    #[test]
    fn an_incremental_rebuild_of_hanging_rows_answers_what_a_fresh_one_does() {
        follows_like_a_fresh_build(
            true,
            &[
                "x",
                "- ",
                "  ",
                "> ",
                "1. a b c d",
                "long-word-here",
                "tab\there",
            ],
        );
    }

    fn follows_like_a_fresh_build(hanging: bool, words: &[&str]) {
        use crate::code_buffer::CodePos;
        let mut buffer = buffer("alpha beta gamma delta\nepsilon\n\nzeta eta theta\niota");
        let mut map = RowMap::default();
        map.sync(Some(8), hanging, &buffer);
        let mut seed: u64 = 0x5eed_1234_abcd_0001;
        let mut next = move |n: usize| {
            seed = seed
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((seed >> 33) % n as u64) as usize
        };
        for step in 0..400 {
            match next(5) {
                0 => {
                    let line = next(buffer.len_lines());
                    let col = {
                        let text = buffer.line(line);
                        let mut col = next(text.len() + 1);
                        while !text.is_char_boundary(col) {
                            col -= 1;
                        }
                        col
                    };
                    buffer.select(CodePos::new(line, col), CodePos::new(line, col));
                    buffer.insert(words[next(words.len())], 0.0);
                }
                1 => {
                    let line = next(buffer.len_lines());
                    buffer.select(
                        CodePos::new(line, 0),
                        CodePos::new(line, buffer.line(line).len()),
                    );
                    buffer.delete_forward(false, 0.0);
                }
                2 => {
                    let line = next(buffer.len_lines());
                    let mut col = next(buffer.line(line).len() + 1);
                    while !buffer.line(line).is_char_boundary(col) {
                        col -= 1;
                    }
                    buffer.select(CodePos::new(line, col), CodePos::new(line, col));
                    buffer.insert("\n", 0.0);
                }
                3 => {
                    buffer.undo();
                }
                _ => {
                    let from = next(buffer.len_lines());
                    let to = next(buffer.len_lines());
                    let (from, to) = (from.min(to), from.max(to));
                    let end = buffer.line(to).len();
                    buffer.select(CodePos::new(from, 0), CodePos::new(to, end));
                    buffer.indent_here(0.0);
                }
            }
            map.follow_edits(&buffer.take_line_edits());
            map.sync(Some(8), hanging, &buffer);
            let fresh = Wrap::build(&buffer, &map.fold, 8, hanging);
            let held = map.wrap.as_ref().unwrap();
            assert_eq!(
                held.breaks,
                fresh.breaks,
                "step {step} ({} lines): the breaks disagreed with a fresh build",
                buffer.len_lines()
            );
            assert_eq!(
                held.first_row,
                fresh.first_row,
                "step {step} ({} lines): the row sums disagreed with a fresh build",
                buffer.len_lines()
            );
            let line_on =
                |wrap: &Wrap, row: usize| wrap.first_row.partition_point(|&r| r <= row) - 1;
            for row in 0..map.rows(buffer.len_lines()) {
                assert_eq!(
                    map.line_at(row),
                    line_on(&fresh, row),
                    "step {step} row {row}"
                );
            }
        }
    }

    #[test]
    fn the_end_of_a_row_is_past_its_last_word_not_its_hanging_blank() {
        let buffer = buffer("aaaa bbbb cccc dddd");
        let mut map = RowMap::default();
        map.sync(Some(10), false, &buffer);
        // "aaaa bbbb " | "cccc dddd": the first row's text ends at 9.
        assert_eq!(map.row_text_end(&buffer, 0, 3), 9);
        assert_eq!(map.row_text_end(&buffer, 0, 12), 19);
    }
}
