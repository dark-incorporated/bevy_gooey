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

use crate::code_buffer::CodeBuffer;
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
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Wrap {
    /// What this was built for: the buffer's revision, the row width and the tab width.
    key: (u64, usize, usize),
    /// The first row of each line, and the row count at the end; a line a fold hides has none.
    first_row: Vec<usize>,
    /// The columns each line breaks at.
    breaks: Vec<Box<[usize]>>,
}

impl Wrap {
    fn build(buffer: &CodeBuffer, fold: &FoldMap, cols: usize) -> Self {
        let tab = buffer.tab_width();
        let lines = buffer.len_lines();
        let mut first_row = Vec::with_capacity(lines + 1);
        let mut breaks = Vec::with_capacity(lines);
        let mut rows = 0;
        for line in 0..lines {
            first_row.push(rows);
            if fold.is_hidden(line) {
                breaks.push(Box::default());
                continue;
            }
            let text = buffer.line(line);
            // A line that is shorter than a row cannot break: skip the walk over it.
            let cuts = if text.len() <= cols {
                Vec::new()
            } else {
                break_columns(text, cols, tab)
            };
            rows += 1 + cuts.len();
            breaks.push(cuts.into_boxed_slice());
        }
        first_row.push(rows);
        Self {
            key: (buffer.revision(), cols, tab),
            first_row,
            breaks,
        }
    }

    fn rows(&self) -> usize {
        self.first_row.last().copied().unwrap_or(0)
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
}

impl std::ops::Deref for RowMap {
    type Target = FoldMap;

    fn deref(&self) -> &FoldMap {
        &self.fold
    }
}

impl RowMap {
    /// Takes the folds a host or the player closed; the wrapping is rebuilt on the next `sync`.
    pub(crate) fn set_fold(&mut self, fold: FoldMap) {
        if self.fold != fold {
            self.fold = fold;
            self.wrap = None;
        }
    }

    /// Makes the map answer for `buffer` in rows `cols` wide, or without wrapping at `None`.
    /// Rebuilds only when the text, the width, the tab or the folds are not what it was built for;
    /// the cost is one pass over the text then.
    pub(crate) fn sync(&mut self, cols: Option<usize>, buffer: &CodeBuffer) {
        let Some(cols) = cols else {
            self.wrap = None;
            return;
        };
        let key = (buffer.revision(), cols.max(1), buffer.tab_width());
        if self.wrap.as_ref().is_some_and(|wrap| wrap.key == key) {
            return;
        }
        self.wrap = Some(Arc::new(Wrap::build(buffer, &self.fold, cols.max(1))));
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
