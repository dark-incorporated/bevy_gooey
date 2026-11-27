//! Soft wrap for the code editor (#114, `docs/research/code-soft-wrap.md`): which display columns
//! a long line breaks at, and the one map between the editor's rows and the buffer's lines that
//! folds and wrapping share.
//!
//! Engine-free: nothing here knows a Bevy type. A line is cut by display columns, one column a
//! character and a tab to the next stop (the same columns [`display_col`] counts), after a blank
//! or a hyphen when it can be and in the middle of a word longer than the row when it cannot.
//! Blanks hang over the edge of a row rather than starting the next one, so a row never begins
//! with the space that ended the last.

use std::sync::Arc;

use crate::code_buffer::{CodeBuffer, LINE_EDIT_LOG, LineEdit};
use crate::code_fold::FoldMap;

/// The display columns where a line of text starts a new row, ascending, each past the one before
/// and past 0; empty when the line fits one row.
///
/// `cols` is how many columns a row holds (at least 1) and `tab` the tab width.
pub(crate) fn break_columns(line: &str, cols: usize, tab: usize) -> Vec<usize> {
    let cols = cols.max(1);
    let tab = tab.max(1);
    let mut breaks = Vec::new();
    let mut start = 0;
    // The latest column after `start` a row may end at: after a blank run or a hyphen.
    let mut room = 0;
    let mut at = 0;
    for c in line.chars() {
        let blank = c == ' ' || c == '\t';
        let width = if c == '\t' { tab - at % tab } else { 1 };
        // A blank hangs over the edge; anything else that does not fit goes to the next row.
        while !blank && at + width > start + cols {
            let cut = if room > start && room <= at { room } else { at };
            breaks.push(cut);
            start = cut;
        }
        at += width;
        if blank || c == '-' {
            room = at;
        }
    }
    breaks
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
    /// The buffer revision whose text these breaks are for.
    revision: u64,
    /// The first row of each line, and the row count at the end; a line a fold hides has none.
    first_row: Vec<usize>,
    /// The columns each line breaks at, empty for a line that fits one row.
    breaks: Vec<Box<[usize]>>,
}

impl Wrap {
    fn build(buffer: &CodeBuffer, fold: &FoldMap, cols: usize) -> Self {
        let tab = buffer.tab_width();
        let lines = buffer.len_lines();
        let mut breaks = Vec::with_capacity(lines);
        for line in 0..lines {
            breaks.push(breaks_of(buffer.line(line), cols, tab));
        }
        let mut first_row = Vec::with_capacity(lines + 1);
        let mut rows = 0;
        for (line, cuts) in breaks.iter().enumerate() {
            first_row.push(rows);
            if !fold.is_hidden(line) {
                rows += 1 + cuts.len();
            }
        }
        first_row.push(rows);
        Self {
            cols,
            tab,
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
        for (line, cuts) in self.breaks.iter().enumerate() {
            first_row.push(rows);
            if !fold.is_hidden(line) {
                rows += 1 + cuts.len();
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
                    let fresh: Vec<Box<[usize]>> = (first..first + inserted)
                        .map(|line| breaks_of(buffer.line(line), self.cols, self.tab))
                        .collect();
                    let mut rows = vec![self.first_row[first]];
                    for (at, cuts) in (first..first + inserted).zip(fresh.iter()) {
                        let last = *rows.last().unwrap_or(&0);
                        rows.push(last + self.rows_at_or(at, cuts, fold));
                    }
                    let old_end = first + removed;
                    let new_end = first + inserted;
                    let old_rows = self.first_row[old_end];
                    let new_rows = *rows.last().unwrap_or(&old_rows);
                    self.breaks.splice(first..old_end, fresh);
                    self.first_row.splice(first + 1..=old_end, rows.into_iter().skip(1));
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

/// The columns `line` breaks at: none when it fits, so a short line never walks its characters.
fn breaks_of(line: &str, cols: usize, tab: usize) -> Box<[usize]> {
    if line.len() <= cols {
        return Box::default();
    }
    break_columns(line, cols, tab).into_boxed_slice()
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
        if self.edits.is_empty() && edits.contains(&LineEdit::Reset) {
            return;
        }
        self.edits.extend_from_slice(edits);
        if self.edits.len() > LINE_EDIT_LOG {
            self.edits.clear();
            self.edits.push(LineEdit::Reset);
        }
    }

    /// Makes the map answer for `buffer` in rows `cols` wide, or without wrapping at `None`.
    ///
    /// The character walk is paid only for lines the text replaced since the last `sync`; the
    /// lines after them move by an integer, and a fold that opened or closed moves the row sums
    /// alone. Anything the deltas cannot describe (a whole-text load, a width change) rebuilds.
    pub(crate) fn sync(&mut self, cols: Option<usize>, buffer: &CodeBuffer) {
        let Some(cols) = cols.map(|cols| cols.max(1)) else {
            self.wrap = None;
            self.edits.clear();
            self.folds_moved = false;
            return;
        };
        let edits = std::mem::take(&mut self.edits);
        let folds_moved = std::mem::take(&mut self.folds_moved);
        let tab = buffer.tab_width();
        let settled = !folds_moved
            && edits.is_empty()
            && self
                .wrap
                .as_ref()
                .is_some_and(|wrap| wrap.cols == cols && wrap.tab == tab && wrap.revision == buffer.revision());
        if settled {
            return;
        }
        let reusable = self
            .wrap
            .take()
            .filter(|wrap| wrap.cols == cols && wrap.tab == tab)
            .map(Arc::unwrap_or_clone);
        self.wrap = Some(Arc::new(match reusable {
            Some(mut wrap) => {
                let follows = !edits.is_empty() && wrap.follow(&edits, buffer, &self.fold);
                if !follows || wrap.breaks.len() != buffer.len_lines() {
                    Wrap::build(buffer, &self.fold, cols)
                } else {
                    if folds_moved {
                        wrap.relay(&self.fold);
                    }
                    wrap.revision = buffer.revision();
                    wrap
                }
            }
            None => Wrap::build(buffer, &self.fold, cols),
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
        let wrap = self.wrap.as_ref()?;
        let line = self.line_at(row);
        let breaks = wrap.breaks.get(line)?;
        let first = wrap.first_row.get(line).copied()?;
        let k = row.checked_sub(first)?;
        let from = if k == 0 { 0 } else { *breaks.get(k - 1)? };
        Some((line, from, breaks.get(k).copied()))
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
    /// row's first column with it.
    pub(crate) fn origin_x(&self, row: usize, scrolled_x: f32, advance: f32) -> f32 {
        self.window(row)
            .map_or(scrolled_x, |(_, from, _)| from as f32 * advance)
    }

    /// Where `line` breaks, `None` without wrapping or past what the map holds.
    pub(crate) fn breaks_of(&self, line: usize) -> Option<&[usize]> {
        self.wrap
            .as_ref()?
            .breaks
            .get(line)
            .filter(|_| !self.fold.is_hidden(line))
            .map(|breaks| &**breaks)
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
        let cuts = break_columns(line, cols, 4);
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
        assert!(break_columns("hello world", 11, 4).is_empty());
        assert!(break_columns("", 1, 4).is_empty());
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
        assert_eq!(break_columns("x\txy", 5, 4), [4]);
    }

    #[test]
    fn every_row_but_the_last_is_within_the_width_or_all_blank_hang() {
        let text = "lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod";
        for cols in 1..30 {
            let cuts = break_columns(text, cols, 4);
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
        map.sync(Some(10), &buffer);
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
        map.sync(None, &buffer);
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
        map.sync(Some(10), &buffer);
        buffer.load("1\n2\n3\n4\n5\n6\n7\n8");
        // The text has more lines than the map: it falls back to the folds' count.
        assert_eq!(map.rows(8), 8);
        let _ = map.row_of(7);
        let _ = map.line_at(40);
        assert_eq!(map.window(99), None);
        assert_eq!(map.row_at(7, 3), map.row_of(7));
        map.sync(Some(10), &buffer);
        assert_eq!(map.rows(8), 8);
    }

    #[test]
    fn a_rebuild_happens_only_for_a_change() {
        let buffer = buffer("aaaa bbbb cccc dddd");
        let mut map = RowMap::default();
        map.sync(Some(10), &buffer);
        let first = map.wrap.clone();
        map.sync(Some(10), &buffer);
        assert!(Arc::ptr_eq(
            first.as_ref().unwrap(),
            map.wrap.as_ref().unwrap()
        ));
        map.sync(Some(8), &buffer);
        assert!(!Arc::ptr_eq(
            first.as_ref().unwrap(),
            map.wrap.as_ref().unwrap()
        ));
    }

    /// A map that followed a stream of edits answers exactly what a map built from the text
    /// afterwards answers: same breaks per line, same first row per line, same line per row.
    #[test]
    fn an_incremental_rebuild_answers_exactly_what_a_fresh_one_does() {
        use crate::code_buffer::CodePos;
        let mut buffer = buffer("alpha beta gamma delta\nepsilon\n\nzeta eta theta\niota");
        let mut map = RowMap::default();
        map.sync(Some(8), &buffer);
        let mut seed: u64 = 0x5eed_1234_abcd_0001;
        let mut next = move |n: usize| {
            seed = seed
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((seed >> 33) % n as u64) as usize
        };
        let words = ["x", "long-word-here", "a b c d e f g", "", "tab\there"];
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
            map.sync(Some(8), &buffer);
            let fresh = Wrap::build(&buffer, &map.fold, 8);
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
            let line_on = |wrap: &Wrap, row: usize| wrap.first_row.partition_point(|&r| r <= row) - 1;
            for row in 0..map.rows(buffer.len_lines()) {
                assert_eq!(map.line_at(row), line_on(&fresh, row), "step {step} row {row}");
            }
        }
    }

    #[test]
    fn the_end_of_a_row_is_past_its_last_word_not_its_hanging_blank() {
        let buffer = buffer("aaaa bbbb cccc dddd");
        let mut map = RowMap::default();
        map.sync(Some(10), &buffer);
        // "aaaa bbbb " | "cccc dddd": the first row's text ends at 9.
        assert_eq!(map.row_text_end(&buffer, 0, 3), 9);
        assert_eq!(map.row_text_end(&buffer, 0, 12), 19);
    }
}
